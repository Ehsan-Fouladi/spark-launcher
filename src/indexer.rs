use crate::prelude::*;

#[derive(Clone)]
pub struct SearchEntry {
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub action: ActionType,
}

pub fn index_system_items() -> Vec<SearchEntry> {
    let mut items: Vec<SearchEntry> = Vec::new();

    let sys_commands = vec![
        (
            "Lock Screen",
            "Window Locked",
            "rundll32.exe user32.dll,LockWorkStation",
        ),
        ("Open Terminal", "Windows Terminal", "wt"),
        ("Task Manager", "Management Windows Task", "taskmgr"),
        ("System Settings", "Setting", "ms-settings:"),
    ];

    for (title, sub, cmd) in sys_commands {
        items.push(SearchEntry {
            title: title.to_string(),
            subtitle: sub.to_string(),
            category: "SYS".to_string(),
            action: ActionType::SystemCmd(cmd),
        });
    }

    let mut paths = vec![PathBuf::from(
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
    )];
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs"))
    }

    for base_path in paths {
        for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    items.push(SearchEntry {
                        title: stem.to_string(),
                        subtitle: path.to_string_lossy().to_string(),
                        category: "APP".to_string(),
                        action: ActionType::OpenPath(path.to_path_buf()),
                    })
                }
            }
        }
    }

    items
}
