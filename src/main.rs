mod builtin;
mod execute;
mod input;
mod parser;
mod prompt;
mod state;

use execute::execute;
use input::read_multilines_input;
use parser::lines_to_tokens;
use prompt::{print_prompt, PROMPT, PROMPT_MULTILINE};
use state::{Args, ShellState};

fn main() {
    let args = Args::new(PROMPT.to_string(), PROMPT_MULTILINE.to_string());
    let mut state = ShellState::new(args);

    loop {
        // 1. print prompt
        print_prompt(state.get_prompt());

        // 2. read input lines
        let input_lines = read_multilines_input(state.get_prompt_multilines());

        // 3. parse input
        let tokens = lines_to_tokens(input_lines);
        if tokens.is_empty() {
            continue;
        }

        // 4. execute
        let exit_status = execute(&mut state, tokens);
        state.update_last_exit_status(exit_status);
    }
}
