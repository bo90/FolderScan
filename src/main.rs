use std::fs;
use std::path::Path;
use chrono::{DateTime, Local};

fn main() {
    scan_folder(".").unwrap();
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