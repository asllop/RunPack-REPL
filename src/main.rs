use runpack::{Pack, Cell, self};
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
            
            { recurs count.print count.dec count.finished? { end } if } def ten
        lex ''
        
        count.ten

        'Finished!' print
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
    pack.dictionary.native("recurs", recurs);
    pack.dictionary.native("end", end);
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

//Experiment: recursive words (tail recursion):
//And an "end" word to abort recursion.
//Implementation:
// { recurs 'Again!' print } def infinite     "Word 'recurs' puts in the ret stack position of itself, and } just returns to it"

fn recurs(pack: &mut Pack) -> Result<bool, runpack::Error> {
    // push position of current word into ret stack
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
