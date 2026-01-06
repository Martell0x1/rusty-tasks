use crate::errors::AppError;
use crate::repository::repo::TaskRepositor;
use crate::ui::input::read_line;
use uuid::Uuid;

pub fn execute(repo: &impl TaskRepositor) -> Result<(), AppError> {
    println!("Task ID:");
    let id = Uuid::parse_str(&read_line()?)
        .map_err(|_| AppError::InvalidInput("Invalid UUID".into()))?;

    let mut tasks = repo.load()?;
    tasks.retain(|t| t.id != id);
    repo.save(&tasks)?;
    Ok(())
}
