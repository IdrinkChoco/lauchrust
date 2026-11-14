use crate::core::app_model::AppModel;
use std::process::Command;
use std::path::Path;

pub struct Launcher;

impl Launcher {

    pub fn launch_path(
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {

        if !Path::new(path).exists() {
            return Err(
                format!("Executable not found: {}", path)
                    .into()
            );
        }

        Command::new(path)
            .spawn()?;

        Ok(())
    }

    pub fn launch(
        app: &AppModel,
    ) -> Result<(), Box<dyn std::error::Error>> {

        Self::launch_path(&app.path)

    }
}