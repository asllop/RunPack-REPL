use runpack::{Pack, Cell, DictEntry, BlockRef, IntegerType, self};
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        "TODO: veure diferents maneres de treballar amb el concat de la paraula cridada"
        
        '------- Recursion Test 1' print

        lex 'count.'
            { dup print } def print
            { -- } def dec
            { dup 0 <= } def finished?
            { drop } def clean
            { { count.clean end } } def end
            { here count.print count.dec count.finished? count.end if } def start
        lex ''
        
        10 count.start
        'Finished!' print

        print_stack
        print_ret_stack

        '------- Recursion Test 2' print

        lex 'count.'
            { dup print } def print
            { -- } def dec
            { dup 0 <= } def finished?
            { drop } def clean
            { { count.clean ret } } def end
            { count.print count.dec count.finished? count.end if again count.start } def start
        lex ''
        
        10 count.start
        'Finished!' print

        print_stack
        print_ret_stack

        '------- Recursion Test 3' print

        lex 'count.'
            { dup print } def print
            { -- } def dec
            { dup 0 <= } def finished?
            { drop } def clean
            { { count.clean ret } } def end
            { count.print count.dec count.finished? count.end if } recur def start
        lex ''
        
        10 count.start
        'Finished!' print

        print_stack
        print_ret_stack
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

    pack.dictionary.native("here", here);
    pack.dictionary.native("end", end);

    pack.dictionary.native("again", again);
    pack.dictionary.native("ret", ret);

    pack.dictionary.native("recur", recur);
    pack.dictionary.native("goto", goto);
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
// { here 'Loooooping!' print } def infinite
//Word 'here' puts in the ret stack position of itself, and } just returns to it

/*
Conclusions:

- Es comporta com una mena de GOTO, no gada.
 */

fn here(pack: &mut Pack) -> Result<bool, runpack::Error> {
    pack.ret.push(pack.concat.pointer - 1);
    Ok(true)
}

fn end(pack: &mut Pack) -> Result<bool, runpack::Error> {
    // Discard 2 ret positions, the call to 'end' and the 'here'.
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
Conclusions:

- Massa verbose
- Si en comptes de la paraula actual en posem un altra després d'again, problemes.
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

// TODO: Test 3:
// { 'Loooping!' print } recur def infinite
// La paraula "recur" pren de la pila un bloc, en modifica l'última paraula al concat per a que faci un salt al principi i
// retorna el block modificat a la pila.

/*
Conclusions:

- Codi una mica més net.
- Al final aconmseguim un resultat semblant al test 1 en estalvi de codi, però amb l'avantatge que no es un GOTO.
- Implementació massa complexa.
 */

// We copy the modfied block into the end of concat and use a ref to this block instead of the original.
fn recur(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Block(block)) = pack.stack.pop() {
        // New block will start at the end of current concat + 1 (the "{"" word).
        let new_block_pos = pack.concat.array.len() + 1;
        let new_block_len = block.len + 2;
        // Copy the block to the end of the concat. All but the last word "}"
        pack.concat.array.push(Cell::Word("{".into()));
        for n in block.pos..(block.pos + block.len - 1) {
            pack.concat.array.push(pack.concat.array[n].clone());
        }
        // Create new block
        let new_block = BlockRef { pos: new_block_pos, len: new_block_len };
        // Put an integer with the block starting position. This will be used by goto
        pack.concat.array.push(Cell::Integer(new_block.pos as IntegerType));
        // Put a GOTO
        pack.concat.array.push(Cell::Word("goto".into()));
        // Put a }
        pack.concat.array.push(Cell::Word("}".into()));
        // Put a drop after the block, in case this words get executed, we don't wanna leave a block in the stack
        pack.concat.array.push(Cell::Word("drop".into()));
        
        // Return the new block in the stack
        pack.stack.push(Cell::Block(new_block));

        Ok(true)
    }
    else {
        Err(runpack::Error::new("recur: Couldn't get block from stack".into(), runpack::ErrCode::NoArgsStack.into()))
    }
}

fn goto(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Integer(pos)) = pack.stack.pop() {
        pack.concat.pointer = pos as usize;
        Ok(true)
    }
    else {
        Err(runpack::Error::new("goto: Couldn't get int from stack".into(), runpack::ErrCode::NoArgsStack.into()))
    }
}