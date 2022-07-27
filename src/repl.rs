use rustyline::completion::Completer;
use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::completion::Pair;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::validate::MatchingBracketValidator;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

#[derive(Default)]
struct MyCompleter {
    words: Vec<String>,
}

impl MyCompleter {
    // Return a list of words that starts with "part"
    pub fn find_word(&self, word_start: &str) -> Vec<Pair> {
        let mut vec = Vec::new();
        for w in self.words.iter() {
            if w.starts_with(word_start) {
                let pair = Pair {
                    display: w.clone(),
                    replacement: w.clone().strip_prefix(word_start).unwrap().into(),
                };
                vec.push(pair);
            }
        }
        vec
    }

    pub fn update_word(&mut self, words: Vec<String>) {
        self.words = words;
    }
}

impl Completer for MyCompleter {
    type Candidate = Pair;
    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // from line at pos, extract a word: go back until pos 0 or first space found
        let mut i = pos as isize - 1;
        let mut word_start = Vec::new();
        while i >= 0 {
            let ch = line.chars().nth(i as usize).unwrap();
            i -= 1;
            if ch != ' ' {
                word_start.push(ch);    
            }
            else {
                break;
            }
        }
        word_start.reverse();

        let word_start = String::from_iter(word_start);
        Ok((pos, self.find_word(&word_start)))
    }

    fn update(&self, line: &mut rustyline::line_buffer::LineBuffer, start: usize, elected: &str) {
        let end = line.pos();
        line.replace(start..end, elected);
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: MyCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

fn new_editor() -> rustyline::Result<Editor<MyHelper>> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();
    let h = MyHelper {
        completer: MyCompleter::default(),
        highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));
    Ok(rl)
}

pub fn cmd<F: FnMut(String) -> Vec<String>>(words: Vec<String>, history_file: &str, mut exec: F) -> rustyline::Result<()> {
    let mut rl = new_editor()?;
    // Set initial completion list
    rl.helper_mut().expect("No helper found").completer.update_word(words);
    rl.load_history(history_file).ok();
    let mut count = 1;
    loop {
        let p = format!("{}> ", count);
        rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let helper = rl.helper_mut().expect("No helper found");
                helper.completer.update_word(exec(line));
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Encountered Eof");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }
    rl.append_history(history_file)
}
