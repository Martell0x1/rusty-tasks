use uuid::Uuid;

use crate::{errors::AppError, repository::repo::TaskRepositor, ui::input::read_line};

pub fn execute(repo: &impl TaskRepositor) -> Result<(), AppError> {
    println!("Task Id:");
    let id = Uuid::parse_str(&read_line()?)
        .map_err(|_| AppError::InvalidInput("Invalid UUID".into()))?;

    let mut tasks = repo.load()?;
    for task in &mut tasks {
        if task.id == id {
            task.toggle();
        }
    }
    repo.save(&tasks)?;
    Ok(())
}
