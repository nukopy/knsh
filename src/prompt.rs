use std::io::Write;

pub const PROMPT: &str = "> ";

pub fn print_prompt(prompt: &str) {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_print_prompt() {
        // print_prompt(PROMPT);
    }
}
