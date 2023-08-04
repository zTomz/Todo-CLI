use std::env;

mod todo;

fn main() {
    let mut todo = todo::Todo::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "add" => todo.add(&args[2..]),
            "list" => todo.list(),
            "reset" => todo.clean(),
            "rm" => todo.remove(&args[2..]),
            "toggle" => todo.toggle(&args[2..]),
            "sort" => todo.sort(),
            _ => (),
        }
    } else {
        todo.list();
    }
}
