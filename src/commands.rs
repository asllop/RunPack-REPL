use runpack::{Pack, Cell, self, DictEntry};
use std::collections::HashMap;

pub fn print(pack: &mut Pack) -> Result<bool, runpack::Error> {
    if let Some(cell) = pack.stack.pop() {
        match cell {
            Cell::Integer(i) => println!("{}", i),
            Cell::Float(f) => println!("{}", f),
            Cell::Boolean(b) => println!("{}", b),
            Cell::String(s) => println!("{}", s),
            Cell::Word(w) => println!("{}", w),
            Cell::Block(b) => println!("{:?}", b),
            Cell::Struct(s) => println!("{:?}", s),
        }
        Ok(true)
    }
    else {
        Err(runpack::Error::new("print: couldn't get a cell from the stack".into()))
    }
}

pub fn print_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("Stack:");
    for n in 0..pack.stack.size() {
        println!("\t{} : {:?}", n, pack.stack.get(n).unwrap());
    }
    Ok(true)
}

pub fn print_ret_stack(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    println!("{:?}", pack.ret);
    Ok(true)
}

pub fn help(pack: &mut Pack) -> Result<bool, runpack::Error> {
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
                Err(runpack::Error::new("help: Helper words didn't return data".into()))
            }
        }
        else {
            println!("No help for word {}", word);
            Ok(true)
        }
    }
    else {
        Err(runpack::Error::new("help: No correct arguments in the concat".into()))
    }
}

pub fn list(pack: &mut Pack) -> Result<bool, runpack::Error> {
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
                            Cell::Struct(_) => print!("<STRUCT> "),
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
        Err(runpack::Error::new("list: No correct arguments in the concat".into()))
    }
}

use std::fs::File;
use std::io::prelude::*;

/*
doc a b c d e f g /doc
*/
pub fn doc(pack: &mut Pack) -> Result<bool, runpack::Error> {
    let mut words = Vec::new();
    while let Some(Cell::Word(w)) = pack.concat.next() {
        if w == "/doc" {
            break;
        }
        else {
            words.push(w.clone());
        }
    }
    words.sort();
    let mut help = HashMap::new();
    for w in &words {
        let stack_help_word = format!("?_{w}_stack_");
        let desc_help_word = format!("?_{w}_desc_");
        if pack.dictionary.dict.contains_key(&stack_help_word) &&
        pack.dictionary.dict.contains_key(&desc_help_word) &&
        pack.dictionary.dict.contains_key(w) {
            pack.exec(&stack_help_word)?;
            pack.exec(&desc_help_word)?;
            let word_type = match pack.dictionary.dict.get(w) {
                Some(DictEntry::Native(_)) => "Native",
                Some(DictEntry::Defined(_)) => "Defined",
                Some(DictEntry::Data(_)) => "Data",
                None => return Err(runpack::Error::new("doc: word doesn't exist".into())),
            };
            if let (Some(Cell::String(desc)), Some(Cell::String(stack_effect))) = (pack.stack.pop(), pack.stack.pop()) {
                help.insert(w, (stack_effect, desc, word_type));
            }
            else {
                return Err(runpack::Error::new("doc: Helper words didn't return data".into()))
            }
        }
    }
    
    if let Ok(mut file) = File::create("DOC.md") {
        file.write_all(b"# Vocabulary\n\n").ok();
        /*
        for w in &words {
            file.write_fmt(format_args!("[{w}](#{w})&nbsp;&nbsp;")).ok();
        }
        file.write_all(b"\n\n").ok();
        file.write_all(b"# Description\n\n").ok();
        */
        for w in &words {
            if let Some((stack_effect, description, word_type)) = help.get(w) {
                file.write_fmt(format_args!("## {w}\n\n")).ok();
                file.write_fmt(format_args!("*{}*\n\n", word_type)).ok();
                file.write_all(b"Stack Effects:\n\n").ok();
                file.write_all(b"```\n").ok();
                file.write_fmt(format_args!("{stack_effect}\n")).ok();
                file.write_all(b"```\n").ok();
                file.write_all(b"Description:\n\n").ok();
                file.write_all(b"```\n").ok();
                file.write_fmt(format_args!("{description}\n")).ok();
                file.write_all(b"```\n").ok();
            }
            file.write_all(b"\n").ok();
        }
    }
    else {
        return Err(runpack::Error::new("doc: Unable to create DOC.md file".into()));
    }
    Ok(true)
}