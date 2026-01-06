use crate::errors::AppError;

use crate::domain::task::Task;

pub trait TaskRepositor {
    fn load(&self) -> Result<Vec<Task>, AppError>;
    fn save(&self, tasks: &[Task]) -> Result<(), AppError>;
}
