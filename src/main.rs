use runpack::{Pack, Cell, self};
use runpack_obj;

fn main() {
    println!("RunPack Tutorial\n");

    // YOUR CODE GOES HERE
    let script = r#"
        
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

{ 0 @def } def zero
zero my_var

AAquesa paraula hipotètica, @def, pren una cel·la del concat de zero, no del seu propi concat.

Com ho fem?

*/