mod core;
use crate::core::app_model::AppModel;
///use crate::core::config::{ConfigManager, DockConfig, CONFIG_FILE,};
use crate::core::launcher::Launcher;
/*
fn main() {

    let config = DockConfig {

        apps: vec![

            AppModel {
                name: "Chrome".into(),
                path: "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe".into(),
            },

            AppModel {
                name: "VS Code".into(),
                path: "C:\\Program Files\\Microsoft VS Code\\Code.exe".into(),
            },

        ],

    };

    ConfigManager::save(
        CONFIG_FILE,
        &config,
    ).unwrap();

    
    println!("Config saved");
    let loaded =
        ConfigManager::load_default()
            .unwrap();
    println!("{:#?}", loaded);

}
*/


fn main() {

    let app = AppModel {

        name: "Notepad".into(),

        path:
            "C:\\Windows\\System32\\notepad.exe"
                .into(),

    };

    Launcher::launch_path(&app.path)
    .unwrap();

}