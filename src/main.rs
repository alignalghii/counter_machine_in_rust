use std::io;
use std::collections::HashMap;

type Label = Option<String>;
type Operation = Vec<String>;
type Program = HashMap<Label, Operation>;

fn main() {
    let mut program: Program = HashMap::new();
    let input_stream_channel = io::stdin();
    loop {
        let mut line = String::new();
        input_stream_channel.read_line(&mut line).expect("Read error");
        if line.is_empty() {
            break;
        }
        trim_newline(&mut line);
        let mut words = line.split_whitespace().map(|word| word.to_string());
        if let Some(mut label_or_direct) = words.next() {
            let operation: Vec<String> = words.collect();
            if let Some(trailer_symbol) = label_or_direct.pop() {
                if trailer_symbol == ':' {
                    if label_or_direct == "START" {
                        program.insert(None, operation);
                    } else {
                        program.insert(Some(label_or_direct), operation);
                    }
                } else {
                    label_or_direct.push(trailer_symbol);
                    println!("Direct command: {label_or_direct}");
                    if label_or_direct == "quit" {
                        break;
                    }
                    if label_or_direct == "list" {
                        listing(&program);
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
