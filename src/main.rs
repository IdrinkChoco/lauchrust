mod core;

slint::include_modules!();
///use crate::core::app_model::AppModel;
use crate::core::config::{ConfigManager, DockConfig, CONFIG_FILE,};
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

/*
fn main() {

    let config =
        ConfigManager::load(CONFIG_FILE)
            .unwrap();

    if let Some(app) =
        config.apps.first()
    {
        Launcher::launch(app)
            .unwrap();

        println!("Launched: {}", app.name);
    } else {
        println!("No apps configured");
    }

    //disini ui
    let ui = AppWindow::new().unwrap();
    
    ui.run().unwrap();

} */

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.run()?;
    Ok(())
}