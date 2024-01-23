mod builtin;
mod execute;
mod input;
mod parser;
mod prompt;

use execute::execute;
use input::read_multilines_input;
use parser::lines_to_tokens;
use prompt::{print_prompt, PROMPT};

fn main() {
    loop {
        // 1. print prompt
        print_prompt(PROMPT);

        // 2. read input lines
        let input_lines = read_multilines_input();

        // 3. parse input
        let tokens = lines_to_tokens(input_lines);
        // println!("tokens: {:?}", tokens);
        if tokens.is_empty() {
            continue;
        }

        // 4. execute
        let cmd = tokens[0].clone();
        let args = tokens[1..].to_vec();
        let _ = execute(cmd, args);
    }
}
