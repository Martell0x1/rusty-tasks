use crate::errors::AppError;
use crate::repository::repo::TaskRepositor;
use crate::ui::render;

pub fn execute(repo: &impl TaskRepositor) -> Result<(), AppError> {
    let data = repo.load()?;
    render::render_tasks(&data);
    Ok(())
}
