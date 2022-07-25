use runpack::{Pack, Cell, self};
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        'Hello, World!' print
    "#;

    let mut pack = Pack::new(script);
    runpack_obj::register(&mut pack);
    register(&mut pack);
    pack.run().expect("Failed running the script");
}

fn register(pack: &mut Pack) {
    pack.dictionary.native("print", print);
    pack.dictionary.native("print_stack", print_stack);
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