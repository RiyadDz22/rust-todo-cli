use std::io::stdin;

use chrono::NaiveDateTime;
pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

pub struct Todos {
    pub list: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, title: String) {
        let todo = Todo {
            title: title,
            completed: false,
            created_at: chrono::Local::now().naive_local(),
        };
        self.list.push(todo);
        println!("Task added successfully!");
    }
    pub fn list(&self) {
        if self.list.is_empty() {
            println!("\n🔍 There is nothing to list. Please add new tasks.\n");
        } else {
            println!("\n📋 Task List:");
            println!("-------------------------------------------");
            for (i, task) in self.list.iter().enumerate() {
                let status = if task.completed {
                    "✅ DONE"
                } else {
                    "❌ TODO"
                };
                let formatted_time = task.created_at.format("%A %H:%M").to_string();
                println!(
                    "{:>2}. {:<20} | {:<8} | {}",
                    i + 1,
                    task.title,
                    status,
                    formatted_time
                );
            }
            println!("-------------------------------------------\n");
        }
    }

    pub fn mark_as_completed(&mut self, i: usize) {
        let index = i - 1;
        if let Some(task) = self.list.get_mut(index) {
            task.completed = true;
            println!("task is marked as completed x try to list them")
        } else {
            println!("No task found at index {}", i);
        }
    }
    pub fn delete(&mut self, i: usize) {
        let index = i - 1;
        if index < self.list.len() {
            self.list.remove(index);
            println!("Task is removed.");
        } else {
            println!("No task found at index {}", i);
        }
    }
}

fn take_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string() // trim newline and return clean input
}

fn str_to_int(num: &str) -> i32 {
    num.trim().parse().expect("Invalid number")
}

fn main() {
    let mut task_list = Todos::new();
    loop {
        println!(
            "Choose what you want to perform:
    1 - add new task
    2 - list all tasks
    3 - mark task as completed
    4 - delete a task
    5 - exit
    "
        );

        let choice = take_input("enter your choice (1, 2, 3, 4, 5): ");
        if choice == "5" {
            println!("exiting...bye!");
            break;
        } else {
            match choice.as_str() {
                "1" => {
                    print!("\x1B[2J\x1B[1;1H");
                    let title = take_input("enter the title of the task");
                    task_list.add(title);
                }
                "2" => {
                    print!("\x1B[2J\x1B[1;1H");
                    task_list.list();
                }
                "3" => {
                    print!("\x1B[2J\x1B[1;1H");
                    task_list.list();
                    let seleted = take_input("enter the task index u wanna mark as completed: ");
                    let seleted = str_to_int(&seleted);
                    task_list.mark_as_completed(seleted as usize);
                }
                "4" => {
                    print!("\x1B[2J\x1B[1;1H");
                    task_list.list();
                    let seleted = take_input("enter the task index u wanna delete: ");
                    let seleted = str_to_int(&seleted);
                    task_list.delete(seleted as usize);
                }
                _ => println!("Invalid operator."),
            }
        }
    }
}
