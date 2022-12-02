mod repl;
mod commands;

use runpack::{Pack, Cell, self};
use commands::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("RunPack REPL v{}\n", VERSION);

    let mut pack = Pack::new();
    pack.dictionary.data("?__", Cell::Boolean(true));

    println!("Loading...");
    println!("> Register runpack_repl.");
    self::register(&mut pack);

    println!("Done!\n");

    println!("Internal commands:");
    println!("\thelp WORD\t\tPrint usage information of WORD.");
    println!("\tlist WORD\t\tPrint definition of WORD.");
    println!("\tshow_stack\t\tShow stack contents.");
    println!("\tshow_ret_stack\t\tShow return stack contents.");
    println!("\tprint\t\t\tGet a cell from the stack and prints it.");
    println!("\tdoc x y ... N \\doc\tGenerate documentation file ./DOC.md for words x,y,z...");
    println!();

    pack.run().expect("Error running the prelude");

    //TODO: store defined stuff in a source file
    //      command to show source, like less, but we can edit lines: move, copy-paste, delete, modify.
    //TODO: pass argument to run script from cmd line and end, without opening the REPL mode
    //TODO: support scripting mode #!
    //TODO: save session (dictionary, stack, ret stack, and concat).

    repl::cmd(word_list(&mut pack), "history.txt", |line| {
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
    let mut list: Vec<String> = pack.dictionary.dict
        .keys()
        .filter(|x| !x.starts_with("?_") && !x.ends_with("_"))
        .map(|s| s.clone())
        .collect();
    list.sort();
    list
}

fn register(pack: &mut Pack) {
    pack.dictionary.native("print", print);
    pack.dictionary.native("show_stack", print_stack);
    pack.dictionary.native("show_ret_stack", print_ret_stack);
    pack.dictionary.native("help", help);
    pack.dictionary.native("list", list);
    pack.dictionary.native("doc", doc);
}