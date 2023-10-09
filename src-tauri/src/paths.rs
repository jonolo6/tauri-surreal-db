use std::path::PathBuf;
use tauri::App;

/// Resolves the App's **file path** from the `AppHandle` context
pub fn app_path(app: &App) -> PathBuf {
    app.path_resolver()
        .app_data_dir()
        .expect("No App path was found!")
}

// Maps the user supplied DB connection string to a connection string with a fully qualified file path to the App's designed "app_path"
pub fn path_mapper(mut app_path: PathBuf, connection_string: &str) -> String {
    app_path.push(
        connection_string
            .split_once(':')
            .expect("Couldn't parse the connection string for DB!")
            .1,
    );

    format!(
        "file:{}",
        app_path
            .to_str()
            .expect("Problem creating fully qualified path to Database file!")
    )
}

#[allow(dead_code)]
pub fn path_homedir(connection_string: &str) -> String {
    format!(
        "sqlite:{}",
        connection_string
            .split_once(':')
            .expect("need a connection striong xxx:xxx")
            .1
    )
}
