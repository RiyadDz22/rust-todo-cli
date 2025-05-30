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
    pub fn add(&mut self, title: String) {
        let todo = Todo {
            title: title,
            completed: false,
            created_at: chrono::Local::now().naive_local(),
        };
        self.list.push(todo);
    }
    pub fn list(&self) {
        if self.list.is_empty() {
            println!("There is nothing to list. Please add new tasks.");
        } else {
            for task in &self.list {
                println!(
                    "- [{}] {} (created at: {})",
                    if task.completed { "x" } else { " " },
                    task.title,
                    task.created_at
                );
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
