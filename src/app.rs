use crate::commands;
use crate::errors::AppError;
use crate::repository::file_repo::FileTaskRepository;
use crate::ui::input::read_choice;
use crate::ui::menu::show_menu;

pub struct App {
    repo: FileTaskRepository,
}

impl App {
    pub fn new() -> Self {
        Self {
            repo: FileTaskRepository::new("tasks.json"),
        }
    }

    pub fn run(&self) -> Result<(), AppError> {
        loop {
            show_menu();
            match read_choice()?.as_str() {
                "1" => commands::list::execute(&self.repo)?,
                "2" => commands::add::execute(&self.repo)?,
                "3" => commands::toggle::execute(&self.repo)?,
                "4" => commands::remove::execute(&self.repo)?,
                "5" => break,
                _ => println!("Invalid Choice"),
            }
        }
        Ok(())
    }
}
