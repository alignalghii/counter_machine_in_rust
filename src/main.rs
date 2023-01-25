use std::io;
use std::collections::HashMap;
use std::io::Write; // brings `flush` into scope. (Credit to https://stackoverflow.com/a/41387232)

type Label = Option<String>;
type Words = Vec<String>;
type Program = HashMap<Label, Operation>;

#[derive(Debug)]
enum Operation {
    Inc(Label),
    Dec(Label, Label)
}

impl Operation {
    fn parse(words: Vec<String>) -> Option<Self> {
        if words[0] == "inc" && words.len() == 2 {
            Some(
                Operation::Inc(
                    labelize("STOP".to_string(), words[1].clone()) // @TODO: inefficient, complete redesign needed
                )
            )
        } else if words[0] == "dec" && words.len() == 3 {
            Some(
                Operation::Dec(
                    labelize("STOP".to_string(), words[1].clone()), // @TODO: inefficient, complete redesign needed
                    labelize("STOP".to_string(), words[2].clone())
                )
            )
        } else {
            None
        }
    }
}

fn main() {
    let mut program: Program = HashMap::new();
    let input_stream_channel = io::stdin();
    let mut output_stream_channel = io::stdout();
    loop {
        println!("Type labeled program line to be stored in listing, or a direct command:");
        print!("> ");
        output_stream_channel.flush().unwrap(); // Credit to https://stackoverflow.com/a/41387232
        let mut line = String::new();
        input_stream_channel.read_line(&mut line).expect("Read error");
        if line.is_empty() {
            break;
        }
        trim_newline(&mut line);
        let mut words = line.split_whitespace().map(|word| word.to_string());
        if let Some(label_or_direct) = words.next() {
            let operation_words: Words = words.collect();
            match interpret_by_trailer(':', label_or_direct) { // `label_or_direct` has just been moved
                Ok(label_word) => {
                    if let Some(operation) = Operation::parse(operation_words) {
                        program.insert(
                            labelize("START".to_string(), label_word),
                            operation
                      );
                    } else {
                        println!("Syntax error in labelled program line, either in label or in operation arguments!")
                    }
                },
                Err(direct_command) => {
                    println!("Direct command: `{direct_command}`");
                    match direct_command.as_str() { // Credit to https://stackoverflow.com/a/29268076
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

fn interpret_by_trailer(expected_trailer: char, mut word: String) -> Result<String, String> {
    if let Some(trailer_symbol) = word.pop() {
        if trailer_symbol == expected_trailer {
            Ok(word)
        } else {
            word.push(trailer_symbol);
            Err(word)
        }
    } else {
        panic!("Empty labels or direct commands are not allowed!");
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

fn labelize(special: String, word: String) -> Label {
    if word == special {
        None
    } else {
        Some(word)
    }
}
