use crate::builtin::{
    commands::{EXIT, PWD},
    exit::exit,
};
use crate::state::ShellState;

// ExitCode range is from 0 to 255
pub type ExitCode = u8;

const EXIT_SUCCESS: ExitCode = 0;
const EXIT_FAILURE: ExitCode = 1;
// const EXIT_BUILTIN: ExitCode = 2;
const EXIT_COMMAND_NOT_FOUND: ExitCode = 127;
const EXIT_OUT_OF_RANGE: ExitCode = 255; // if exit code is out of range, it will be set to 255

pub fn execute(state: &mut ShellState, tokens: Vec<String>) -> ExitCode {
    // add command to history
    let cmd_str = tokens.join(" ");
    state.push_command_history(cmd_str);

    // execute cmd
    let cmd = tokens[0].clone();
    let args = tokens[1..].to_vec();
    match cmd.as_str() {
        EXIT => {
            exit();
            unreachable!();
        }
        PWD => {
            let cwd = state.get_current_dir();
            println!("{}", cwd.to_string_lossy());

            EXIT_SUCCESS
        }
        // TODO: implement built-in commands
        cmd => {
            let output = std::process::Command::new(cmd).args(args).output();
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    let stderr = String::from_utf8(output.stderr).unwrap();
                    print!("{}", stdout);
                    eprint!("{}", stderr);

                    // calculate exit status
                    let exit_status = output.status.code().unwrap_or(1);
                    if exit_status > (EXIT_OUT_OF_RANGE as i32) {
                        // exit_status is out of range, set it to 255. 255 means exit status is out of range.
                        eprintln!("knsh: exit code out of range: {}", exit_status);
                        EXIT_OUT_OF_RANGE
                    } else {
                        exit_status as ExitCode
                    }
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
