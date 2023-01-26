use std::io;
use std::collections::HashMap;
use std::io::Write; // brings `flush` into scope. (Credit to https://stackoverflow.com/a/41387232)

// Intermediate, temporary or auxiliary concepts, mostly borrowed values:
type ProgramLine<'a> = (&'a Label, &'a Operation);
type WordSlices<'a> = Vec<&'a str>; // to avoid the "cannot move out of index of `Vec<String>`" bug! @credit to https://stackoverflow.com/q/27904864

// Main concepts, mostly owened vales
type Program = HashMap<Label, Operation>;
type Label = Option<String>;

#[derive(Debug)]
enum Operation {
    Inc(Label),
    Dec(Label, Label)
}

impl Operation {
    fn parse(word_slices: WordSlices) -> Option<Self> {
        if word_slices.len() == 2 && word_slices[0] == "inc"  {
            Some(
                Operation::Inc(
                    labelize("STOP", word_slices[1].to_string()) // @TODO: consider efficiency (but seems to be correct and unavoidable)
                )
            )
        } else if word_slices.len() == 3 && word_slices[0] == "dec" {
            Some(
                Operation::Dec(
                    labelize("STOP", word_slices[1].to_string()), // @TODO: consider efficiency (but seems to be correct and unavoidable)
                    labelize("STOP", word_slices[2].to_string())
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
        output_stream_channel.flush().unwrap(); // @credit to https://stackoverflow.com/a/41387232
        let mut line = String::new();
        input_stream_channel.read_line(&mut line).expect("Read error");
        if line.is_empty() {
            break;
        }
        trim_newline(&mut line);
        let mut words = line.split_whitespace();
        if let Some(label_or_direct) = words.next() {
            let operation_words: WordSlices = words.collect();
            match interpret_by_trailer(':', label_or_direct.to_string()) { // `label_or_direct` has just been moved
                Ok(label_word) => {
                    if let Some(operation) = Operation::parse(operation_words) {
                        program.insert(
                            labelize("START", label_word),
                            operation
                      );
                    } else {
                        println!("Syntax error in labelled program line, either in label or in operation arguments!")
                    }
                },
                Err(direct_command) => {
                    println!("Direct command: `{direct_command}`");
                    match direct_command.as_str() { // @credit to https://stackoverflow.com/a/29268076
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

fn listing(ref_program: &Program) {
    print_listing(listing_representation(ref_program));
}

fn listing_representation<'a>(program: &'a Program) -> Vec<ProgramLine<'a>> {
    let mut lines = vec![];
    if let Some(start_label_ref) = program.keys().find(|&label_ref| label_ref.is_none()) {
        let mut ref_current_label = start_label_ref;
        loop {
            if let Some(ref_operation) = program.get(ref_current_label) {
                lines.push((ref_current_label, ref_operation));
                ref_current_label = next_ref_label(ref_operation);
                if ref_current_label.is_none() {
                    break;
                }
            } else {
                println!("Inconsistency in label linearity");
                break;
            }
        }
    } else {
        println!("No `START` label!");
    }
    lines
}

fn print_listing(list: Vec<ProgramLine>) {
    for (i, o) in list.iter() {
        println!("{i:?} {o:?}");
    }
}

fn labelize(special: &'static str, word: String) -> Label {
    if word == special {
        None
    } else {
        Some(word)
    }
}


// Reference to enum variant: @credit to https://stackoverflow.com/q/36590549
fn next_ref_label<'a>(ref_operation: &'a Operation) -> &'a Label {
    match ref_operation {
        &Operation::Inc(ref next)         => next, // @credit to https://stackoverflow.com/a/36590693
        &Operation::Dec(ref _loopback_case, ref break_case) => break_case
    }
}
