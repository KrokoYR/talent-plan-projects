//! Example of NON-consuming builder pattern
use std::io;

#[derive(Debug)]
pub struct Child {}

#[derive(Debug)]
pub struct ExampleCommand {
    pub program: String,
    args: Vec<String>,
    cwd: Option<String>,
}

impl ExampleCommand {
    pub fn new(program: String) -> Self {
        ExampleCommand {
            program: program,
            args: Vec::new(),
            cwd: None,
        }
    }

    /// Add an argument to pass to the program.
    pub fn arg(&mut self, arg: String) -> &mut Self {
        self.args.push(arg);
        self
    }

    /// Add multiple arguments to pass to the program.
    pub fn args(&mut self, args: &[String]) -> &mut Self {
        self.args.extend_from_slice(args);
        self
    }

    /// Set the working directory for the child process.
    pub fn current_dir(&mut self, dir: String) -> &mut Self {
        self.cwd = Some(dir);
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn spawn(&self) -> io::Result<Child> {
        /* ... */
        Ok(Child {})
    }
}

// Here is example of consuming builder
// fn main() {
// impl TaskBuilder {
//     /// Name the task-to-be.
//     pub fn named(mut self, name: String) -> TaskBuilder {
//         self.name = Some(name);
//         self
//     }

//     /// Redirect task-local stdout.
//     pub fn stdout(mut self, stdout: Box<io::Write + Send>) -> TaskBuilder {
//         self.stdout = Some(stdout);
//         self
//     }

//     /// Creates and executes a new child task.
//     pub fn spawn<F>(self, f: F) where F: FnOnce() + Send {
//         /* ... */
//     }
// }
// }
