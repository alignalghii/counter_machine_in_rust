use std::io;
use std::collections::HashMap;
use std::io::Write; // brings `flush` into scope. (Credit to https://stackoverflow.com/a/41387232)

type Label = Option<String>;
type Words = Vec<String>;
type Program = HashMap<Label, Words>;

fn main() {
    let mut program: Program = HashMap::new();
    let input_stream_channel = io::stdin();
    let mut output_stream_channel = io::stdout();
    loop {
        println!("Type labeled program line to be stored in listing, or a direct command!");
        print!("> ");
        output_stream_channel.flush().unwrap(); // Credit to https://stackoverflow.com/a/41387232
        let mut line = String::new();
        input_stream_channel.read_line(&mut line).expect("Read error");
        if line.is_empty() {
            break;
        }
        trim_newline(&mut line);
        let mut words = line.split_whitespace().map(|word| word.to_string());
        if let Some(mut label_or_direct) = words.next() {
            let operation_words: Words = words.collect();
            if let Some(trailer_symbol) = label_or_direct.pop() {
                if trailer_symbol == ':' {
                    if label_or_direct == "START" {
                        program.insert(None, operation_words);
                    } else {
                        program.insert(Some(label_or_direct), operation_words);
                    }
                } else {
                    label_or_direct.push(trailer_symbol);
                    println!("Direct command: `{label_or_direct}`");
                    match label_or_direct.as_str() { // Credit to https://stackoverflow.com/a/29268076
                        "quit" => break,
                        "list" => listing(&program),
                        other  => println!("The `{other}` direct command  has no implementation yet!")

                    }
                }
            }
        } else {
            println!("Empty line skipped");
        }
    }
}

fn trim_newline(line: &mut String) {
    if let Some(last) = line.pop() {
        if last != '\n' {
            line.push(last);
        }
    }
}

fn listing(program: &Program) {
    println!("{program:?}");
}
