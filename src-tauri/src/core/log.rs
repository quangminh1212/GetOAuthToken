use std::fs::OpenOptions;
use std::io::Write;

pub fn log_to_file(message: &str) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let line = format!("[{}] {}\n", timestamp, message);
    println!("{}", line.trim_end());

    if let Ok(cwd) = std::env::current_dir() {
        let path = cwd.join("log").join("log.txt");
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = file.write_all(line.as_bytes());
        }
    }
}
