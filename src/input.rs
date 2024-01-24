use crate::prompt::print_prompt;

fn trim_newline(str: String) -> (bool, String) {
    let mut is_trimmed = false;
    let mut s = str.clone();

    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }

        is_trimmed = true;
        return (is_trimmed, s);
    }

    (is_trimmed, s)
}

fn trim_backslash(str: String) -> (bool, String) {
    let mut is_trimmed = false;
    let mut s = str.clone();

    if s.ends_with('\\') {
        s.pop();
        is_trimmed = true;
        return (is_trimmed, s);
    }

    (is_trimmed, s)
}

pub fn read_multilines_input(prompt_multilines: &str) -> Vec<String> {
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        // check if line ends with a backslash. backslash means the input is multiline
        let (_, line) = trim_newline(line);
        let (is_trimmed, line) = trim_backslash(line);

        // empty line and empty line with backslash are not pushed to lines and break input loop
        if line.is_empty() {
            break;
        }

        if is_trimmed {
            lines.push(line);
            print_prompt(prompt_multilines)
        } else {
            lines.push(line);
            break;
        }
    }

    lines
}
