// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::SpeeDb, sql::Thing, Surreal};
use tauri::Manager;
// use tauri::{State, Manager};

mod paths;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

// #[tokio::main]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let path = paths::app_path(app);
            create_dir_all(path).expect("problems creating App directory!");

            tauri::async_runtime::block_on(async move {
                // let db = "file:surreal.db";
                // let fqdb = paths::path_mapper(paths::app_path(app), db);

                let db = "surreal.db";
                let fqdb = paths::append(paths::app_path(app), db);
                let context = ApplicationContext::new(&fqdb).await;
                println!("fqdb: {}", fqdb);

                // let fqdb = path_homedir(&db);
                // let pool = db::setup(&fqdb).await.expect("no pool generated!");
                // let instance = db::DbInstance(Mutex::new(pool));
                // let lock = instance.0.lock().await;
                // drop(lock);
                app.manage(context);

                Ok(())
            })
        })
        // .manage(context)
        // .invoke_handler(tauri::generate_handler![db::select, db::move_item_above])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

struct ApplicationContext {
    // action_dispatchers: HashMap<String, Arc<dyn ActionDispatcher + Sync + Send>>,
}

impl ApplicationContext {
    async fn new(url: &str) -> Self {
        println!("Creating db at {}", url);
        // let db = Surreal::new::<SpeeDb>("file://./dbs").await.unwrap();
        // "/Users/polofsson/Library/Application Support/com.tauri.dev/surreal.db",
        let db = Surreal::new::<SpeeDb>(url).await.unwrap();

        db.use_ns("test").use_db("test").await.unwrap();
        // let repository = Box::new(SurrealRepository::new(Box::new(surreal_db), "classifiers"));
        // let service = Arc::new(ClassifierService::new(repository));
        // let mut action_dispatchers: HashMap<String, Arc<dyn ActionDispatcher + Sync + Send>> =
        //     HashMap::new();
        // action_dispatchers.insert(
        //     actions::classifier_action::CLASSIFIER_DOMAIN.to_string(),
        //     service.clone(),
        // );
        // action_dispatchers.insert(
        //     actions::application_action::APPLICATION_DOMAIN.to_string(),
        //     service.clone(),
        // );
        // Self { action_dispatchers }

        let created: Vec<Record> = db
            .create("person")
            .content(Person {
                title: "Founder & CEO",
                name: Name {
                    first: "Tobie",
                    last: "Morgan Hitchcock",
                },
                marketing: true,
            })
            .await
            .unwrap();
        dbg!(created);

        // Update a person record with a specific id
        let updated: Option<Record> = db
            .update(("person", "jaime"))
            .merge(Responsibility { marketing: true })
            .await
            .unwrap();
        dbg!(updated);

        // Select all people records
        let people: Vec<Record> = db.select("person").await.unwrap();
        dbg!(people);

        // Perform a custom advanced query
        let groups = db
            .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
            .bind(("table", "person"))
            .await
            .unwrap();
        dbg!(groups);

        // tokio::task::spawn(async move {
        //     let groups = db
        //         .query("LIVE SELECT * FROM type::table($table)")
        //         .bind(("table", "person"))
        //         .notifications().await
        //
        //     while let Ok(v) = db.query().unwrap().notifications().recv().await {
        //         println!("received: {}", v);
        //     }
        // });
        Self {}
    }
}
