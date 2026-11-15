use crate::prelude::*;

#[derive(Clone)]
pub enum ActionType {
    OpenPath(PathBuf),
    CopyText(String),
    OpenUrl(String),
    SystemCmd(&'static str),
}

impl ActionType {
    pub fn execute(&self) {
        match self {
            ActionType::OpenPath(path) => {
                let _ = open::that(path);
            }
            ActionType::CopyText(text) => {
                if let Ok(mut clipboard) = Clipboard::new() {
                    let _ = clipboard.set_text(text.clone());
                }
            }
            ActionType::OpenUrl(url) => {
                let _ = open::that(url);
            }
            ActionType::SystemCmd(cmd) => {
                let _ = open::that(cmd);
            }
        }
    }
}
