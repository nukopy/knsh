use crate::builtin::{commands::EXIT, exit::exit};

type ExitCode = i32;
const EXIT_SUCCESS: ExitCode = 0;
const EXIT_FAILURE: ExitCode = 1;
// const EXIT_BUILTIN: ExitCode = 2;
const EXIT_COMMAND_NOT_FOUND: ExitCode = 127;

pub fn execute(cmd: String, args: Vec<String>) -> ExitCode {
    match cmd.as_str() {
        // TODO: implement built-in commands
        EXIT => {
            exit();

            EXIT_SUCCESS // unreachable
        }
        cmd => {
            let output = std::process::Command::new(cmd).args(args).output();
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    let stderr = String::from_utf8(output.stderr).unwrap();
                    print!("{}", stdout);
                    eprint!("{}", stderr);

                    output.status.code().unwrap_or(EXIT_FAILURE)
                }
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        eprintln!("knsh: command not found: {}", cmd);

                        EXIT_COMMAND_NOT_FOUND
                    }
                    _ => {
                        eprintln!("Error: {}", e);

                        EXIT_FAILURE
                    }
                },
            }
        }
    }
}
