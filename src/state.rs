use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use std::path::PathBuf;

use home::home_dir;

use crate::execute::ExitCode;

/// Args is used to initialize ShellState
pub struct Args {
    prompt: String,
    prompt_multilines: String,
}

impl Args {
    /// new creates a new Args
    pub fn new(prompt: String, prompt_multilines: String) -> Args {
        Args {
            prompt,
            prompt_multilines,
        }
    }
}

/// ShellState is used to store shell state
#[derive(Debug)]
pub struct ShellState {
    envs: HashMap<String, String>,
    current_working_dir: PathBuf,
    command_history: VecDeque<String>,
    last_exit_status: ExitCode,
    prompt: String,
    prompt_multilines: String,
}

impl ShellState {
    /// new creates a new ShellState
    pub fn new(args: Args) -> ShellState {
        // load environment variables
        let envs = ShellState::load_envs();

        // load current working directory
        let cwd = ShellState::load_current_working_dir();

        // initialize ShellState
        ShellState {
            envs,
            current_working_dir: cwd,
            command_history: VecDeque::new(),
            last_exit_status: 0,
            prompt: args.prompt,
            prompt_multilines: args.prompt_multilines,
        }
    }

    /// --------------------------------------------------
    /// loaders
    /// --------------------------------------------------

    /// load_envs loads environment variables
    fn load_envs() -> HashMap<String, String> {
        std::env::vars().collect::<HashMap<String, String>>()
    }

    /// load_envs_with_os loads environment variables with OsString
    #[allow(dead_code)]
    fn load_envs_with_os() -> HashMap<OsString, OsString> {
        // std::env::var_os returned iterator will not check if the environment variables are valid Unicode.
        // ref: https://doc.rust-lang.org/std/env/fn.vars_os.html
        std::env::vars_os().collect::<HashMap<OsString, OsString>>()
    }

    /// load_current_working_dir loads current working directory
    fn load_current_working_dir() -> PathBuf {
        let cwd = std::env::current_dir();
        match cwd {
            Ok(cwd) => cwd,
            Err(_) => {
                // fallback with home directory
                home_dir().unwrap_or_else(|| {
                    // fallback with root directory
                    PathBuf::from("/")
                })
            }
        }
    }

    /// --------------------------------------------------
    /// getters
    /// --------------------------------------------------

    /// get_current_dir returns current working directory
    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_working_dir
    }

    /// get_prompt returns prompt string
    pub fn get_prompt(&self) -> &String {
        &self.prompt
    }

    /// get_prompt_multilines returns prompt string for multiline input
    pub fn get_prompt_multilines(&self) -> &String {
        &self.prompt_multilines
    }

    /// --------------------------------------------------
    /// setters
    /// --------------------------------------------------

    /// set_last_exit_status sets last exit status
    fn set_last_exit_status(&mut self, status: ExitCode) {
        self.last_exit_status = status;
    }

    /// --------------------------------------------------
    /// state update methods
    /// --------------------------------------------------

    /// push_command_history pushes a new command to head of command history
    pub fn push_command_history(&mut self, cmd: String) {
        self.command_history.push_back(cmd);
    }

    /// update_last_exit_status updates last exit status
    pub fn update_last_exit_status(&mut self, status: ExitCode) {
        self.set_last_exit_status(status);
    }
}
