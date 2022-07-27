mod repl;

use runpack::{Pack, Cell, self};
use runpack_obj;

fn main() {
    println!("RunPack REPL v0.1.0\n");

    let mut pack = Pack::new();
    runpack_obj::register(&mut pack);
    self::register(&mut pack);

    repl::cmd(word_list(&mut pack), "history.txt", |line| {
        let backpack = pack.clone();
        pack.code(&line);
        if let Err(e) = pack.run() {
            println!("{}", e.msg);
            pack = backpack;
        }
        // Update completion list
        word_list(&mut pack)
    }).expect("Error");
}

fn word_list(pack: &mut Pack) -> Vec<String> {
    let mut list: Vec<String> = pack.dictionary.dict.keys().map(|s| s.clone()).collect();
    list.sort();
    list
}

fn register(pack: &mut Pack) {
    pack.dictionary.native("print", print);
    pack.dictionary.native("print_stack", print_stack);
    pack.dictionary.native("print_ret_stack", print_ret_stack);
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