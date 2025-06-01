// Standard input module
use std::io::stdin;

// External crate for handling date and time
use chrono::NaiveDateTime;

/// Represents a single task in the todo list
pub struct Todo {
    pub title: String,             // Task title
    pub completed: bool,           // Completion status
    pub created_at: NaiveDateTime, // Time the task was created
}

/// Represents the entire todo list
pub struct Todos {
    pub list: Vec<Todo>, // A vector of Todo items
}

impl Todos {
    /// Creates and returns a new empty Todo list
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    /// Adds a new task to the todo list
    pub fn add(&mut self, title: String) {
        let todo = Todo {
            title,
            completed: false,
            created_at: chrono::Local::now().naive_local(), // Gets the current local time
        };
        self.list.push(todo);
        println!("Task added successfully!");
    }

    /// Displays all tasks in the todo list
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
                let formatted_time = task.created_at.format("%A %H:%M").to_string(); // Day and time
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

    /// Marks a task as completed by index
    pub fn mark_as_completed(&mut self, i: usize) {
        let index = i - 1; // Adjust from 1-based to 0-based index
        if let Some(task) = self.list.get_mut(index) {
            task.completed = true;
            println!("Task marked as completed! ✅");
        } else {
            println!("No task found at index {}", i);
        }
    }

    /// Deletes a task by index
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

/// Helper function to prompt the user and read input
fn take_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string() // Remove newline and return clean input
}

/// Converts string to integer (used for menu and task selection)
fn str_to_int(num: &str) -> i32 {
    num.trim().parse().expect("Invalid number")
}

/// Entry point of the application
fn main() {
    let mut task_list = Todos::new();

    loop {
        // Display menu
        println!(
            "Choose what you want to perform:
    1 - add new task
    2 - list all tasks
    3 - mark task as completed
    4 - delete a task
    5 - exit
    "
        );

        let choice = take_input("Enter your choice (1, 2, 3, 4, 5): ");

        if choice == "5" {
            println!("Exiting... bye! 👋");
            break;
        } else {
            match choice.as_str() {
                "1" => {
                    // Clear terminal
                    print!("\x1B[2J\x1B[1;1H");
                    let title = take_input("Enter the title of the task:");
                    task_list.add(title);
                }
                "2" => {
                    print!("\x1B[2J\x1B[1;1H");
                    task_list.list();
                }
                "3" => {
                    print!("\x1B[2J\x1B[1;1H");
                    task_list.list();
                    let selected = take_input("Enter the task index to mark as completed: ");
                    let selected = str_to_int(&selected);
                    task_list.mark_as_completed(selected as usize);
                }
                "4" => {
                    print!("\x1B[2J\x1B[1;1H");
                    task_list.list();
                    let selected = take_input("Enter the task index to delete: ");
                    let selected = str_to_int(&selected);
                    task_list.delete(selected as usize);
                }
                _ => println!("Invalid choice. Please try again."),
            }
        }
    }
}
