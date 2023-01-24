use std::io;

fn main() {
    let input_stream_channel = io::stdin();
    loop {
        let mut line = String::new();
        input_stream_channel.read_line(&mut line).expect("Read error");
        trim_newline(&mut line);
        if line.is_empty() || line == "quit" {
            break;
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
