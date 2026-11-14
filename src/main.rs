mod core;

use crate::core::config::ConfigManager;
use crate::core::config::CONFIG_FILE;
use crate::core::launcher::Launcher;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {

    let config =
        ConfigManager::load(CONFIG_FILE)
            .expect("failed loading config");

    let app1 = config.apps.get(0).cloned();
    let app2 = config.apps.get(1).cloned();
    let app3 = config.apps.get(2).cloned(); 

    let ui = AppWindow::new()?;

    if let Some(app) = &app1 {
        ui.set_app1(app.name.clone().into());
    }

    if let Some(app) = &app2 {
        ui.set_app2(app.name.clone().into());
    }

    if let Some(app) = &app3 {
        ui.set_app3(app.name.clone().into());
    }

ui.on_app1_clicked(move || {
    if let Some(app) = &app1 {

        println!("Launching {}", app.name);

        if let Err(err) = Launcher::launch(app) {
            eprintln!("Launch error: {}", err);
        }
    }
});

ui.on_app2_clicked(move || {
    if let Some(app) = &app2 {

        println!("Launching {}", app.name);

        if let Err(err) = Launcher::launch(app) {
            eprintln!("Launch error: {}", err);
        }
    }
});

ui.on_app3_clicked(move || {
    if let Some(app) = &app3 {

        println!("Launching {}", app.name);

        if let Err(err) = Launcher::launch(app) {
            eprintln!("Launch error: {}", err);
        }
    }
});

    ui.run()?;

    Ok(())
}