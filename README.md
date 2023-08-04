# Todo CLI

This is a project to practice and learn the Rust programming language. It is my implementation of a todo repository which already exists on [GitHub](https://github.com/sioodmy/todo/tree/master) from [sioodmy](https://github.com/sioodmy).

I have customized it so that multiple to-dos can now be deleted/toggled at once.

## Usage

```Usage: todo [COMMAND] [ARGUMENTS]
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add "buy carrots"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX INDEX ...]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX INDEX ...]
        remove one or more tasks
        Example: todo rm 4 5 7
    - reset
        deletes all tasks
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
```