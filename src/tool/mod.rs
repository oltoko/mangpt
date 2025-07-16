use std::{error::Error, process::Command, str};

pub fn find_for(name: &str) -> Result<Tool, Box<dyn Error>> {
    let tool = Tool::new(name);

    tool.validate_exists()?;

    Ok(tool)
}

#[cfg(target_os = "windows")]
const WHICH_COMMAND: &str = "where";

#[cfg(unix)]
const WHICH_COMMAND: &str = "which";

pub struct Tool {
    name: String,
}

impl Tool {
    fn new(name: &str) -> Tool {
        Tool {
            name: String::from(name),
        }
    }

    fn validate_exists(&self) -> Result<(), Box<dyn Error>> {
        match Command::new(WHICH_COMMAND).arg(&self.name).output() {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err(format!(
                "The Program with the name {} wasn't found on the Path",
                self.name
            )
            .into()),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn fetch_man_page(&self) -> Result<String, Box<dyn Error>> {
        self.try_to_fetch_help()
    }

    #[cfg(unix)]
    pub fn fetch_man_page(&self) -> Result<String, Box<dyn Error>> {
        match self.try_to_fetch_man_page() {
            Ok(man_page) => Ok(man_page),
            Err(error) => {
                eprintln!("{error}... fall back to --help...");
                self.try_to_fetch_help()
            }
        }
    }

    #[cfg(unix)]
    fn try_to_fetch_man_page(&self) -> Result<String, Box<dyn Error>> {
        match Command::new("man").arg(&self.name).output() {
            Ok(output) if output.status.success() => {
                let help = str::from_utf8(&output.stdout)?.trim();
                Ok(String::from(help))
            }
            _ => Err(format!("The Program with the name {} has no man page", self.name).into()),
        }
    }

    fn try_to_fetch_help(&self) -> Result<String, Box<dyn Error>> {
        match Command::new(&self.name).arg("--help").output() {
            Ok(output) if output.status.success() => {
                let help = str::from_utf8(&output.stdout)?.trim();
                Ok(String::from(help))
            }
            _ => Err(format!(
                "The Program with the name {} doesn't support --help",
                self.name
            )
            .into()),
        }
    }
}
