use std::fs;
use std::path::Path;
use std::io::{self, Write};
use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Result};

fn main() {
    let mut init_path = String::new();

    print!("Введите путь к папке: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut init_path)
        .expect("Не удалось прочитать каталог");

    let trimmed_path = init_path.trim();
    let exist_path = Path::new(trimmed_path);
    if exist_path.exists() {
        println!("Путь до каталога получен: {}", trimmed_path);

        if let Err(e) = scan_folder(exist_path) {
            eprintln!("Ошибка при чтении метаданных: {}", e);
        }
    } else {
        eprintln!("Ошибка! Путь '{}' не существует!", trimmed_path);
    }
}

fn scan_folder<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let paths = path.as_ref();

    let metadata = fs::metadata(paths)?;

    for path in fs::read_dir(paths)? {
        let entry = path.unwrap();
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;
        let type_str = if file_type.is_dir() {"Каталог"} else {"Файл"};
        println!("{}, [{}]", file_name.to_string_lossy(), type_str);

        // Смотрим, когда был создан файл.
        match metadata.created() {
            Ok(time) => {
                let datetime: DateTime<Local> = time.into();
                println!("Создан: {}", datetime.format("%Y-%m-%d %H:%M"));
            }
            Err(_) => {
                println!("Создан: Не поддерживается вашей ОС/файловой системой");
            }
        }
        // Смотрим, когда был изменен.
        if let Ok(time) = metadata.modified() {
            let datetime: DateTime<Local> = time.into();
            println!("Изменен: {}", datetime.format("%Y-%m-%d %H:%M"));
        }
        println!("\n");
    }
    Ok(())
}

// Инициализация БД и создание таблицы.
fn init_db() -> Result<Connection> {
    let conn = Connection::open("pth_hst.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS PATHS_HST(\
        path TEXT primary key\
        )",
        [],
    )?;
    Ok(conn)
}

// Получить список путей сохраненных в БД.
fn get_all_paths(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT path FROM PATHS_HST")?;

    let path_iter = stmt.query_map([],|row| {
        let path: String = row.get(0)?;
        Ok(path)
    })?;

    let mut paths = Vec::new();
    for path_result in path_iter {
        paths.push(path_result?);
    }
    Ok(paths)
}

// Добавить новый путь в БД.
fn add_path(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGRNORE INTO PATHS_HST (path) VALUES (?)",
        [path],
    )?;
    Ok(())
}