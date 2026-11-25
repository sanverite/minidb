use std::io::{self, Write};

fn main() {
    let mut line = String::new();

    loop {
        print!("minidb> ");
        io::stdout().flush().unwrap();
        line.clear();
        let bytes = io::stdin().read_line(&mut line).unwrap();

        if bytes == 0 {
            break; // EOF: Ctrl-D
        }

        match line.trim() {
            ".exit" => break,
            "" => continue,
            "version" => println!("0.1.0"),
            cmd => println!("unrecognized command: {cmd}"),
        }
    }
    println!("bye");
}
