use crate::domain::{status::Status, task::Task};
use colored::{Color, Colorize, Styles};
use tabled::{Table, Tabled, derive};

#[derive(Tabled)]
struct TaskRaw {
    id: String,
    title: String,
    status: String,
    description: String,
    priority: String,
    created_at: String,
}

pub fn render_tasks(tasks: &[Task]) {
    let rows: Vec<TaskRaw> = tasks
        .iter()
        .map(|t| TaskRaw {
            id: t.id.to_string().cyan().to_string(),
            title: t.title.bright_blue().bold().to_string(),
            status: match t.status {
                Status::ToDo => "ToDo".red().to_string(),
                Status::Done => "Done".green().to_string(),
            },
            description: t
                .description
                .as_deref()
                .unwrap_or("None")
                .to_string()
                .blue()
                .to_string(),
            priority: format!("{:?}", t.priority),
            created_at: t.created_at.to_string(),
        })
        .collect();

    let table = Table::new(rows);
    println!("{}", "Your Tasks".bold().underline());
    println!("{}", table);
}
