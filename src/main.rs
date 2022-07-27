use runpack::{Pack, Cell, self};
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        { @@ print } def test_one
        test_one symbol_1

        print_stack
        print_ret_stack

        '------' print

        { { @@ } exe symbol_2 print } def test_two
        test_two

        print_stack
        print_ret_stack

        '------' print

        { 0 @@ @def } def zero
        zero my_zero
        my_zero print

        "Create a clone of the 'def' word"
        { @@ @def } def var

        'Andreu' var name
        name print

        { 'Hello Andreu' print } var hello
        hello

        "Create a clone of the '@' word"
        { @@ } def at

        at any_word print

        "
        TODO: experiments with obj words that use the concat ':' and '.'
        see if we can implement them in RunPack using @@
        "
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

    pack.dictionary.native("@@", atat);
    pack.dictionary.native("@def", atdef);
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

/*
Podem treballar amb el concat de forma fàcil amb les següents paraules:

@ word          "fica ref a word dins la pila"
10 def word     "defineix paraula word"
lex 'count.'    "estableix prefix the lexicon"
{ xxxx }        "defineix block"
[ a | a a ]     "realitza tranferencia de pila"

Però què passa si definim una paraula:

{ . . . } def word

I la volem poder cridar així:

word blabla

De manera que la definició de word pugui obtenir coses del concat del lloc on ha estat cridada.

És més, també podríem voler cridar una de les paraules "standard" que fan servir el concat amb una referència que no es troba 
al just després d'elles:

{ ... } def zero
zero my_var

Aquesa paraula hipotètica, zero, pren una cel·la del seu concat i defineix una paraula que conté un 0.

En realitat només ens cal una paraula, @@, que actua com @ però del concat "extern".
Després podríem tenir versions de totes les altres paraules que fan servir la pila en comptes del concat.

{ 0 @@ @def } def zero
zero my_var

Aquest @def pren de la pila una ref a una paraula, en comptes del concat. Aquesta ref hi ha estat ficada per @@.

Com ho fem?

Quan executem una paraula, la següent pos del concat es fica a la pila de retorn, per tant n'hi hauria d'haver
prou de llegir aquest index i obtenir una paraula del concat d'allà.
*/

fn atat(pack: &mut Pack) -> Result<bool, runpack::Error>  {
    if let Some(parent_concat_pos) = pack.ret.pop() {
        if let Some(cell) = pack.concat.array.get(parent_concat_pos) {
            pack.ret.push(parent_concat_pos + 1);
            pack.stack.push(cell.clone());
            Ok(true)
        }
        else {
            Err(runpack::Error::new("atat: couldn't get a cell from the concat".into(), 1000))
        }
    }
    else {
        Err(runpack::Error::new("atat: couldn't get ret pos".into(), 1000))
    }
}

// Usage: 10 @ num @def
fn atdef(pack: &mut Pack) -> Result<bool, runpack::Error> {
    let (word, data) = (pack.stack.pop(), pack.stack.pop());
    if let Some(Cell::Word(word)) = word {
        if let Some(Cell::Block(block)) = data {
            pack.dictionary.block(&word, block);
        }
        else if let Some(cell) = data {
            pack.dictionary.data(&word, cell);
        }
        else {
            return Err(runpack::Error::new("atdef: Expecting a block or a cell".into(), 1000));
        }
    }
    else {
        return Err(runpack::Error::new("atdef: Expecting a word in the stack".into(), 1000));
    }
    Ok(true)
}
