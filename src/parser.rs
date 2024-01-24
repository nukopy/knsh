pub fn lines_to_tokens(lines: Vec<String>) -> Vec<String> {
    let mut tokens = Vec::new();
    for line in lines {
        let line_tokens = line.split_whitespace();
        for token in line_tokens {
            tokens.push(token.to_string());
        }
    }

    tokens
}
