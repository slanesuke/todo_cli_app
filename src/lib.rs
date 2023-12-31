use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::{env, process};
use std::fmt::format;

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_bak: String,
    pub no_backup: bool,
}

impl Todo {
    pub fn new() -> Result<Self,String> {
        /// This variable is expected to hold the path to the to-do file.
        let todo_path: String = match env::var("TODO_PATH") {
            //If the environment variable exists , the fn assigns its value to the todo_path variable
            Ok(t) => t,
            //the function proceeds to create a default todo_path based on the user's home directory
            Err(_) => {
                let home =  env::var("HOME").unwrap();

                // Look for a legacy TODO file path
                let legacy_todo = format!("{}/TODO,", &home);
                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/todo", &home),
                }
            }
        };

        //  todo_bak attempts to read the value of an environment variable called
        let todo_bak: String = match env::var("TODO_BAK_DIR") {
            // if the environment var exists, it's variable is assigned to todo_back
            Ok(t) => t,
            // otherwise the below string is assigned to todo_bak
            Err(_) => String::from("/tmp/todo.bak"),
        };

        // no_backup  attempts to read the value of an environment variable called "TODO_NOBACKUP."
        let no_backup = env::var("TODO_NOBACKUP").is_ok();
        // .is_ok() is called on the result of env::var("TODO_NOBACKUP"). This method returns true
        // if the result is Ok, indicating that the environment variable exists and has a value.
        // It returns false if the result is Err, indicating that the environment variable does not
        // exist or could not be read.

        //OpenOptions = Options and flags which can be used to configure how a file is opened.
        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("Couldn't open the todo file");

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todo_file);

        // Empty String ready to be filled with TODO's
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of the TODO file into a todo vector
        let todo = contents.to_string().lines().map(str::to_string).collect();

        // Returns todo
        Ok(Self {
            todo,
            todo_path,
            todo_bak,
            no_backup,
        })
    }

    // Prints every todo saved
    pub fn list(&self) {
        let handle = io::stdout().lock();

        // Buffered writer for stdout stream
        let mut writer = BufWrriter::new(handle);
        let mut data = String::new();

        // This loop will repeat itself for each task in TODO file
        for (number, task) in self.todo.iter().enumerate() {
            if task.len() > 5 {
                // Converts virgin default number into a chad BOLD string
                let number = (number + 1).to_string().bold();

                // Saves the symbol of the current task
                let symbol = &task[..4];

                // Saves a task without a symbol
                let task = &task[4..];

                // Checks if tteh current task is completed or not...
                if symbol == "[*] " {
                    // DONE
                    // If the task is completed, then it prints it with a strikethrough
                    data = format!("{} {}\n", number, task.strikethrough());
                } else if symbol == "[ ] " {
                    // NOT DONE
                    // If the task is not completed yet, then it will print it as it is
                    data = format!("{} {}\n", number, task);
                }
                writer
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdout");
            }
        }
    }

    // left off on lib.rs fn raw()
    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("todo rarw takes only 1 argument, not {}", arg.len())
        } else if arg.is_empty() {
            eprintln!("todo raw takes 1 argument (done/todo)");
        } else {
            let handle = io::stdout().lock();
            // Buffer writer for stdout stream
            let mut writer = BufWriter::new(handle);
            let mut data = String::new();
            // This loop will repeat itself for each task in TODO file
            for task in self.todo.iter() {
                if task.len() > 5 {
                    // Saves the symbol for current ask
                    let symbol = &task[..4];
                    // Saves task without a symbol
                    let task = &task[4..];

                    // Check if the current task is completed or not
                    if symbol == "[*] " && arg[0] == "done" {
                        // DONE
                        // If the task is completed, then it prints it with a strikethrough
                        data = format!("{}\n", task.strikethrough()); // fix on github
                    } else if symbol == "[ ] " && arg[0] == "todo" {
                        // NOT DONE
                        // If the task is not completed yet, then it will print it as it is
                        data = format!("{}\n", task);
                    }
                    writer
                        .write_all(data.as_bytes())
                        .expect("Failed to write to stdout");
                }
            }
        }
    }

    // Adds a new todo
    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }
        // Opens the TODO file with a permission to:
        let todo_file = OpenOptions::new()
            .create(true) // create the file if it does not exist
            .append(true) // append a line to it
            .open(&self.todo_path)
            .expect("Could not open todo file");

        let mut buffer = BufWriter::new(todo_file);
        for arg in args {
            if arg.trim().is_empty() {
                continue
            }

            // Appends a new task/s to the file
            let line = format!("[ ] {}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("Unable to write data");
        }
    }

    // Removes a task
    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo remove takes at least 1 argument");
            process::exit(1);
        }
        // Opens the todo file with permission to:
        let todo_file = OpenOptions::new()
            .write(true) // write
            .truncate(true) // truncate // Fix on github
            .open(&self.todo_path)
            .expect("Couldn't open todo file");

        let mut buffer = BufWriter::new(todo_file);

        for (position, line) in self.todo.iter().enumerate() {
            if args.contains(&"done".to_string()) && &line[..4] == "[*] " {
                continue;
            }

            if args.contains(&(position + 1).to_string()) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("Unable to write data.");
        }
    }

    // Remove file
    fn remove_file(&self) {
        match fs::remove_file(&self.todo_path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error while clearing todo file: {}", e)
            }
        };
    }

    // Clear todo by removing todo file
    pub fn reset(&self) { // fix the absence of & in github
        if !&self.no_backup {
            match fs::copy(&self.todo_path, &self.todo_bak) {
                Ok(_) => self.remove_file(),
                Err(_) => {
                    eprintln!("Couldn't backup the todo file")
                }
            }
        } else {
            self.remove_file();
        }
    }

    // Restore backup
    pub fn restore(&self) {
        fs::copy(&self.todo_bak, &self.todo_path)
            .expect("Unable to restore the backup");
    }

    // Sorts done tasks
    pub fn sort(&self) {
       // Create new empty string
        let mut new_todo = String::new();

        let mut todo = String::new();
        let mut done = String::new();

        for line in self.todo.iter() {
            if line.len() > 5 {
                if &line[..4] == "[ ] " {
                    let line = format!("{}\n", line);
                    todo.push_str(&line);
                }else if &line[..4] = "[*] " {
                    let line = format!("{}\n", line);
                    done.push_str(&line);
                }
            }
        }

        new_todo = format!("{}{}", &todo, &done);

        // Open the TODO file with permission
        let mut todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Unable to open todo file");

        // Write contents of a newtodo variable into the TODO file
        todo_file
            .write_all(new_todo.as_bytes())
            .expect("Error while trying to save the todo file")
    }

    // implement fn done()
    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            println!("todo done takes at least 1 argument");
            process::exit(1);
        }

        let todo_file = OpenOptions::new()
            .write(true)
            .open(&self.todo_path)
            .expect("Couldn't open todo file");
        let mut buffer = BufWriter::new(todo_file);

        for (position, line) in self.todo.iter().enumerate() {
            if line.len() > 5 {
                if args.contains(&(position + 1).to_string()) {
                    if &line[..4] == "[ ] " {
                        let line = format!("[*] {}\n", &line[4..]);
                        buffer
                            .write_all(line.as_bytes())
                            .expect("unable to write data");
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        buffer
                            .write(&line.as_bytes())
                            .expect("unable to write data");
                    }
                }else if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                    let line = format!("{}\n", line);
                    buffer
                        .write_all(line.as_bytes())
                        .expect("unable to write data");
                }

                }
            }
        }

}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";

pub fn help() {
    println!("{}\n", TODO_HELP);
}







