use tauri::*;
use std::process;
use rusqlite::{Connection, Result};

#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(Clone, serde::Serialize)]
struct Payload {
    screen: String,
}

struct Snippet {
  id: Option<i32>,
  title: String,
  content: String,
}

#[derive(Clone, serde::Serialize)]
struct SnippetResponse {
  id: Option<i32>,
  title: String,
  content: String,
}

#[derive(Clone, serde::Serialize)]
struct ResultSearch {
    snippets: Vec<SnippetResponse>,
}

static mut CONN: Option<Connection> = None;

fn run() -> Result<()> {
  unsafe {
    CONN = Some(establish_connection()?);
  
    initialize_database()?;

    Ok(())
  }
}

fn main() {
  unsafe {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
  }

  tauri::Builder::default()
    .setup(|app| {
      {
          let _handle = app.handle();
          let window = app.get_window("main").unwrap();

          app.global_shortcut_manager()
            .register("CONTROL + SPACE", move || {
              if window.is_visible().unwrap() {
                println!("Hide find");
                let _ = window.hide();
              } else {
                println!("Show find");
                let _ = window.show();
                let screen = String::from("FIND");
                window.emit("channel", Payload { screen }).unwrap();
              }
            })
      };
      {
        let _handle = app.handle();
        let window = app.get_window("main").unwrap();

        app.global_shortcut_manager()
          .register("ALT + C", move || {
            if window.is_visible().unwrap() {
              println!("Hide save");
              let _ = window.hide();
            } else {
              println!("Show save");
              let _ = window.show();
              let screen = String::from("SAVE");
              window.emit("channel", Payload { screen }).unwrap();
            }
          })
          .unwrap();
      }

      Ok(())
    })
    .system_tray(system_tray())
    .on_system_tray_event(handle_tray_event)
    .invoke_handler(tauri::generate_handler![hide_window, find_snippet, save_snippet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn system_tray() -> SystemTray {     // <- a function that creates the system tray
  let menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
    .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
  return SystemTray::new().with_menu(menu);
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
  if let SystemTrayEvent::MenuItemClick { id, .. } = event {
    if id.as_str() == "quit" {
      process::exit(0);
    }
    if id.as_str() == "toggle" {
      let window = app.get_window("main").unwrap();
      let menu_item = app.tray_handle().get_item("toggle");
      if window.is_visible().unwrap() {
        let _ = window.hide();
        let _ = menu_item.set_title("Show");
      } else {
        let _ = window.show();
        let _ = window.center();
        let _ = menu_item.set_title("Hide");
      }
    }
  }
}

#[tauri::command]
fn hide_window(app: AppHandle) {
  let window = app.get_window("main").unwrap();
  let menu_item = app.tray_handle().get_item("toggle");
  window.hide();
  menu_item.set_title("Show");
}

#[tauri::command]
fn find_snippet(window: Window, query: String) {
    println!("query: {}", query);

    let snippets_result = search_items(&query);

    let snippets = match snippets_result {
      Ok(snippets) => {
        let snippets_response: Vec<SnippetResponse> = snippets
            .iter()
            .map(|snippet| SnippetResponse {
                id: snippet.id.clone(),
                title: snippet.title.clone(),
                content: snippet.content.clone(),
            })
            .collect();
          snippets_response
        }
        Err(err) => {
            eprintln!("Error searching snippets: {}", err);
            Vec::new() // Fallback to an empty Vec<Snippet> if an error occurred
        }
    };

    let result = ResultSearch { snippets };
    window.emit("find_result", result).unwrap();
}


#[tauri::command]
fn save_snippet(title: String, snippet: String) {
  println!("title: {}", title);
  println!("snippet: {}", snippet);

  insert_item(&title, &snippet)
    .map_err(|err| {
        eprintln!("Error inserting item: {}", err);
        err
    });
}

//Database
fn establish_connection() -> Result<Connection> {
  Connection::open("mysnippets.db")
}

fn initialize_database() -> Result<()> {
  unsafe {
    let conn = CONN.as_ref().expect("Database connection not initialized");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS snippets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
  }
}

fn insert_item(title: &str, content: &str) -> Result<()> {
  unsafe {
    let conn = CONN.as_ref().expect("Database connection not initialized");
    conn.execute(
        "INSERT INTO snippets (title, content) VALUES (?1, ?2)",
        [title, content],
    )?;

    Ok(())
  }
}

fn search_items(search_query: &str) -> Result<Vec<Snippet>> {
  unsafe {
    let conn = CONN.as_ref().expect("Database connection not initialized");
    let mut stmt = conn.prepare("SELECT id, title, content FROM snippets WHERE title LIKE '%' || ?1 || '%' OR content LIKE '%' || ?1 || '%'")?;

    let snippet_iter = stmt.query_map([search_query], |row| {
        Ok(Snippet {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
        })
    })?;

    let mut snippets = Vec::new();

    for snippet in snippet_iter {
        snippets.push(snippet?);
    }

    Ok(snippets)
  }
}
