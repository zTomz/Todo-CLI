use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

use colored::*;

pub struct Todo {
    pub todos: Vec<TodoInstance>,
    file: File,
}

impl Todo {
    pub fn new() -> Todo {
        let file = match Path::new("todos.txt").exists() {
            true => std::fs::OpenOptions::new()
                .append(true)
                .open("todos.txt")
                .expect("cannot open file"),
            false => File::create("todos.txt").expect("Cannot create file"),
        };

        Todo {
            todos: Todo::read_todos(),
            file,
        }
    }

    pub fn add(&mut self, todos: &[String]) {
        for todo in todos {
            self.todos.push(TodoInstance::new(&todo));
        }

        self.save_to_file();
    }

    pub fn list(&self) {
        let mut count: u32 = 1;

        for todo in &self.todos {
            println!(
                "{}: {}",
                count.to_string().bold(),
                if todo.completed {
                    todo.to_str().strikethrough()
                } else {
                    todo.to_str().normal()
                }
            );
            count += 1;
        }
    }

    pub fn remove(&mut self, indexes_to_remove: &[String]) {
        if indexes_to_remove.is_empty() {
            return;
        }

        if indexes_to_remove.len() == 1
            && (indexes_to_remove[0] == "*" || indexes_to_remove[0] == "all")
        {
            self.clean();
            self.save_to_file();

            return;
        }

        for index in indexes_to_remove {
            let index = index.parse::<usize>().unwrap_or(991199);

            if index - 1 >= self.todos.len() {
                println!("You have to enter a number!");
                continue;
            }

            if index - 1 >= self.todos.len() {
                println!("Index: {index} out of range!");
                continue;
            }

            self.todos.remove(index - 1);
        }
        self.clear_file();

        self.save_to_file();
    }

    pub fn toggle(&mut self, indexes_to_toggle: &[String]) {
        if indexes_to_toggle.is_empty() {
            return;
        }

        if indexes_to_toggle.len() == 1
            && (indexes_to_toggle[0] == "*" || indexes_to_toggle[0] == "all")
        {
            for todo in &mut self.todos {
                todo.completed = !todo.completed;
            }
            self.save_to_file();

            return;
        }

        for index in indexes_to_toggle {
            let index = index.parse::<usize>().unwrap_or(991199);
            if index == 991199 {
                println!("You have to enter a number!");
                continue;
            }

            if index - 1 >= self.todos.len() {
                println!("Index: {index} out of range!");
                continue;
            }

            self.todos[index - 1].completed = !self.todos[index - 1].completed;
        }

        self.save_to_file();
    }

    pub fn sort(&mut self) {
        self.todos.sort_by(|a, b| a.completed.cmp(&b.completed));

        self.save_to_file();
    }

    pub fn clean(&mut self) {
        self.todos = Vec::new();
        self.clear_file();
    }

    fn save_to_file(&mut self) {
        self.clear_file();
        for todo in &self.todos {
            let content = format!(
                "{}: {}\n",
                if todo.completed { "DONE" } else { "TODO" },
                todo.to_str()
            );

            self.file
                .write_all(content.as_bytes())
                .expect("Failed to add todos.");
        }
    }

    fn clear_file(&self) {
        File::create("todos.txt").expect("Cannot clean todos");
    }

    fn read_todos() -> Vec<TodoInstance> {
        let mut todos = Vec::new();

        if read_to_string("todos.txt").is_err() {
            return todos;
        }

        for line in read_to_string("todos.txt").unwrap().lines() {
            todos.push(TodoInstance::from_str(line));
        }

        todos
    }
}

pub struct TodoInstance {
    pub title: String,
    pub completed: bool,
}

impl TodoInstance {
    pub fn new(title: &str) -> TodoInstance {
        TodoInstance {
            title: String::from(title),
            completed: false,
        }
    }

    pub fn from_str(text: &str) -> TodoInstance {
        let done = text.contains("DONE");

        TodoInstance {
            title: text[6..].to_string(),
            completed: done,
        }
    }

    pub fn to_str(&self) -> String {
        self.title.clone()
    } 
}