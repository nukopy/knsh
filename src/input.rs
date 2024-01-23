use crate::prompt::{print_prompt, PROMPT};

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

pub fn read_multilines_input() -> Vec<String> {
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        // check if line ends with a backslash. backslash means the line is not complete
        let (_, line) = trim_newline(line);
        let (is_trimmed, line) = trim_backslash(line);
        if is_trimmed {
            lines.push(line);
            print_prompt(PROMPT)
        } else {
            lines.push(line);
            break;
        }
    }

    lines
}
