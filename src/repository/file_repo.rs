use std::path::PathBuf;
use std::{fs, io};

use super::repo::TaskRepositor;
use crate::domain::task::Task;
use crate::errors::AppError;

pub struct FileTaskRepository {
    path: PathBuf,
}

impl FileTaskRepository {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }
}

impl TaskRepositor for FileTaskRepository {
    fn load(&self) -> Result<Vec<Task>, AppError> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let data = fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&data)?)
    }

    fn save(&self, tasks: &[Task]) -> Result<(), AppError> {
        let data = serde_json::to_string_pretty(tasks)?;
        fs::write(&self.path, data)?;
        Ok(())
    }
}
