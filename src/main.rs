mod repl;

use runpack::{Pack, Cell, self};
use runpack_obj;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("RunPack REPL v{}\n", VERSION);

    let mut pack = Pack::new_dev_mode(true);

    println!("Loading...");

    println!("> Register runpack_obj.");
    runpack_obj::register(&mut pack);

    println!("> Register runpack_repl.");
    self::register(&mut pack);

    println!("Done!\n");

    pack.run().expect("Error running the prelude");

    //TODO: store defined stuff in a source file
    //TODO: if script.rnp exists, rename it and start a new one

    repl::cmd(word_list(&mut pack), "script.rnp", |line| {
        let backup_pack = pack.clone();
        pack.code(&line);
        if let Err(e) = pack.run() {
            println!("{}", e.msg);
            pack = backup_pack;
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
    pack.dictionary.native("help", help);
    pack.dictionary.native("list", list);
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
    for n in 0..pack.stack.size() {
        println!("\t{} : {:?}", n, pack.stack.get(n).unwrap());
    }
    Ok(true)
}

fn print_ret_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("{:?}", pack.ret);
    Ok(true)
}

fn help(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Word(word)) = pack.concat.next() {
        let stack_help_word = format!("?_{word}_stack_");
        let desc_help_word = format!("?_{word}_desc_");
        if pack.dictionary.dict.contains_key(&stack_help_word) && pack.dictionary.dict.contains_key(&desc_help_word) {
            pack.exec(&stack_help_word)?;
            pack.exec(&desc_help_word)?;
            if let (Some(Cell::String(desc)), Some(Cell::String(stack_effect))) = (pack.stack.pop(), pack.stack.pop()) {
                println!("Stack effect:\t{}", stack_effect);
                println!("Description:\t{}", desc);
                Ok(true)
            }
            else {
                Err(runpack::Error::new("help: Helper words didn't return data".into(), runpack::ErrCode::NoArgsStack.into()))
            }
        }
        else {
            println!("No help for word {}", word);
            Ok(true)
        }
    }
    else {
        Err(runpack::Error::new("help: No correct arguments in the concat".into(), runpack::ErrCode::NoArgsConcat.into()))
    }
}

fn list(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(Cell::Word(word)) = pack.concat.next() {
        if let Some(word_def) = pack.dictionary.dict.get(word) {
            match word_def {
                runpack::DictEntry::Native(_) => println!("Word is native"),
                runpack::DictEntry::Data(cell) => println!("{:?}", cell),
                runpack::DictEntry::Defined(block) => {
                    print!("{{ ");
                    for n in block.pos..(block.pos + block.len) {
                        match &pack.concat.array[n] {
                            Cell::Integer(i) => print!("{} ", i),
                            Cell::Float(f) => print!("{} ", f),
                            Cell::Boolean(b) => print!("{} ", b),
                            Cell::String(s) => print!("'{}' ", s),
                            Cell::Word(w) => print!("{} ", w),
                            Cell::Block(_) => print!("<BLOCK> "),
                            Cell::Object(_) => print!("<OBJECT> "),
                            Cell::Empty => print!("<EMPTY> "),
                        }
                    }
                    println!();
                },
            }
        }
        else {
            println!("Word doesn't exist");
        }
        Ok(true)
    }
    else {
        Err(runpack::Error::new("list: No correct arguments in the concat".into(), runpack::ErrCode::NoArgsConcat.into()))
    }
}