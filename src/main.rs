use runpack::{Pack, Cell, DictEntry, self};
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        "TODO: veure diferents maneres de treballar amb el concat de la paraula cridada"
        
        lex 'count.'
            { dup print } def print
            { -- } def dec
            { dup 0 <= } def finished?
            { drop } def clean
            { recurs count.print count.dec count.finished? { count.clean end } if } def start
        lex ''
        
        10 count.start
        'Finished!' print

        '-------' print

        lex 'count.'
            { dup print } def print
            { -- } def dec
            { dup 0 <= } def finished?
            { drop } def clean
            { count.print count.dec count.finished? { count.clean ret } if again count.start } def start
        lex ''
        
        10 count.start
        'Finished!' print

        print_stack
        print_ret_stack

        '-------' print

        { 'Loooping!' print again infinite } def infinite
        infinite
    "#;

    // Create pack and register plugins
    let mut pack = Pack::new();
    runpack_obj::register(&mut pack);
    self::register(&mut pack);

    // Add script code and run
    pack.code(script);
    pack.run().expect("Failed running the script");
}

fn register(pack: &mut Pack) {
    pack.dictionary.native("print", print);
    pack.dictionary.native("print_stack", print_stack);
    pack.dictionary.native("print_ret_stack", print_ret_stack);
    pack.dictionary.native("recurs", recurs);
    pack.dictionary.native("end", end);
    pack.dictionary.native("again", again);
    pack.dictionary.native("ret", ret);
}

fn print(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(cell) = pack.stack.pop() {
        match cell {
            Cell::Empty => println!("<EMPTY>"),
            Cell::Integer(i) => println!("{}", i),
            Cell::Float(f) => println!("{}", f),
            Cell::Boolean(b) => println!("{}", b),
            Cell::String(s) => println!("{}", s),
            Cell::Word(w) => println!("{}", w),
            Cell::Block(b) => println!("{:?}", b),
            Cell::Object(o) => println!("{:?}", o),
        }
        Ok(true)
    }
    else {
        Err(runpack::Error::new("print: couldn't get a cell from the stack".into(), 1000))
    }
}

fn print_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("Stack:");
    for n in (0..pack.stack.size()).rev() {
        println!("\t{} : {:?}", n, pack.stack.get(n).unwrap());
    }
    Ok(true)
}

fn print_ret_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("{:?}", pack.ret);
    Ok(true)
}

//Experiment: recursive words (tail recursion):
//And an "end" word to abort recursion.

//Test 1:
// { recurs 'Loooooping!' print } def infinite
//Word 'recurs' puts in the ret stack position of itself, and } just returns to it

/*
Conclusions: No m'acaba d'agradar.

1. Es comporta com una mena de GOTO.
2. No veig que estalvii codi ni el faci més clar que emprant un loop.
 */

fn recurs(pack: &mut Pack) -> Result<bool, runpack::Error> {
    pack.ret.push(pack.concat.pointer - 1);
    Ok(true)
}

fn end(pack: &mut Pack) -> Result<bool, runpack::Error> {
    // Discard 2 ret positions, the call to 'end' and the 'recurs'.
    if let (Some(_), Some(_), Some(pos)) = (pack.ret.pop(), pack.ret.pop(), pack.ret.pop()) {
        pack.concat.pointer = pos;
        Ok(true)
    }
    else {
        Err(runpack::Error::new("end: Return stack underflow".into(), runpack::ErrCode::StackUnderflow.into()))
    }
}

//Test 2:
// { 'Loooping!' print again infinite } def infinite

/*
Conclusions: Força millor, però encara no veig que sigui millor que un simple loop.
 */

fn again(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Word(w)) = pack.concat.next() {
        if let Some(DictEntry::Defined(block)) = pack.dictionary.dict.get(w) {
            pack.concat.pointer = block.pos;
            Ok(true)
        }
        else {
            Err(runpack::Error::new("again: No word".into(), runpack::ErrCode::NotFound.into()))
        }
    }
    else {
        Err(runpack::Error::new("again: No word in the concat".into(), runpack::ErrCode::NoArgsConcat.into()))
    }
}

fn ret(pack: &mut Pack) -> Result<bool, runpack::Error> {
    // Fa igual que "}"
    // Discard 1 ret positions, the call to 'ret'
    if let (Some(_), Some(pos)) = ( pack.ret.pop(), pack.ret.pop()) {
        pack.concat.pointer = pos;
        Ok(true)
    }
    else {
        Err(runpack::Error::new("ret: Return stack underflow".into(), runpack::ErrCode::StackUnderflow.into()))
    }
}
