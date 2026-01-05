use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(fname: &str) {
    println!("Reading file <{}>", fname);
    let contents: Vec<u8> = match std::fs::read(fname) {
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(42);
        }
        Ok(v) => v,
    };

    println!("Read {} bytes", contents.len());
    run(contents);
}

fn run(input: Vec<u8>) {
    _ = input;
    todo!("run input");
}

fn run_prompt() {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("failed to flush stdout");

        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(0) => break, // EOF reached
            Ok(_) => {}
            Err(e) => {
                println!("error: {e}");
                continue;
            }
        };

        match buf.trim() {
            "quit" => break,
            _ => println!("<{}>", buf.trim()),
        }
    }
}
