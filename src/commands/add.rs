use crate::domain::{priority::Priority, task::Task};
use crate::errors::AppError;
use crate::repository::repo::TaskRepositor;

use crate::ui::input::read_line;

pub fn execute(repo: &impl TaskRepositor) -> Result<(), AppError> {
    print!("Task Title");
    // to do input from ui;
    let title = read_line()?;

    let mut tasks = repo.load()?;

    print!("Task Description: ");
    let desc = read_line()?;

    let mut new_task = Task::new(title, Priority::Medium);
    new_task.set_description(desc);

    tasks.push(new_task);

    repo.save(&tasks)?;
    Ok(())
}
