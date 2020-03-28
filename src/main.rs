use std::io;
use std::io::Write;

fn main() {
    println!("drawerus");
    println!("Commands:");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
                let cmd = parse_input(input);
                match cmd.unwrap() {
                    Command::Quit => {
                        println!("Quitting...");
                        break;
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

enum Command {
    Quit,
}

// - LINE x,y
// - CIRC x,y,r
// - RECT x,y,x2,y2
// - CHAR ch              # sets character for the pen (default?)
// - LOAD <filename>
// - SAVE <filename>
// - READ <filename>      # for reading instructions from a file
// - CANV width,height    # setting canvas size
// - SIZE n               # size of the pen
fn parse_input(input: String) -> Result<Command, String> {
    let arg_err = || String::default();

    match input.trim() {
        "LINE" => return Err(arg_err()),
        "CIRC" => {}
        "RECT" => {}
        "CHAR" => {}
        "QUIT" => return Ok(Command::Quit),
        _ => return Err(arg_err()),
    }
    Err(arg_err())
}
