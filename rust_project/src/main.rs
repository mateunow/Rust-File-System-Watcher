//use std::env;
use std::path::Path;
use notify::{RecursiveMode,Result, Watcher};
fn main() -> Result<()>
{
    // let args: Vec<String> = env::args().collect();
    // let first = &args[1];
    // let second = &args[2];
    // // dbg!(args);
    //
    //
    // println!("First argument: {}", first);
    // println!("Second argument: {}", second);
    let path = Path::new("./test");
    start_watching(Path:: new("Rust-project/rust_project/test"))?;
    Ok(())
}

fn start_watching(path: &Path) -> Result<()>
{
    let mut watcher = notify::recommended_watcher(handle_event)?;
    watcher.watch(path, RecursiveMode::Recursive)?;
    Ok(())
}
fn handle_event(result: notify::Result<notify::Event>)
{
    //Result<T, E> - 'hej mam coś, ale może to być ok(), albo err()
    println!("Got event: {:?}", result);
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};
    use std::{fs, thread};
    use std::time::Duration;

    #[test]
    fn test_start_watching_valid_path() {
        let path = PathBuf::from("./test");
        let result = start_watching(&path);
        assert!(result.is_ok(), "Watcher powinien się uruchomić na poprawnej ścieżce");
    }

    #[test]
    fn test_start_watching_invalid_path() {
        let path = PathBuf::from("./non_existent_folder_12345");
        let result = start_watching(&path);
        assert!(result.is_err(), "Watcher powinien zwrócić błąd dla nieistniejącej ścieżki");
    }
    #[test]
    fn test_start_watching_prints_event() {
        let test_dir = "./test_dir_for_test";
        std::fs::create_dir_all(test_dir).unwrap();

        // Przechwyć stdout
        let output = Arc::new(Mutex::new(Vec::new()));
        let output_clone = output.clone();

        let test_handle_event = move |result: notify::Result<notify::Event>| {
            let mut buf = output_clone.lock().unwrap();
            writeln!(buf, "Got event: {:?}", result).unwrap();
        };

        // Uruchom watcher z testowym callbackiem
        let mut watcher = notify::recommended_watcher(test_handle_event).unwrap();
        watcher.watch(PathBuf::from(test_dir).as_path(), notify::RecursiveMode::Recursive).unwrap();

        // Stwórz plik, który wywoła event
        let test_file = format!("{}/test1.txt", test_dir);
        std::fs::write(&test_file, "hello").unwrap();

        // Poczekaj, aby event został wykryty
        thread::sleep(Duration::from_millis(500));

        let buf = output.lock().unwrap();
        let output_str = String::from_utf8_lossy(&buf);
        assert!(output_str.contains("Got event"), "Brak oczekiwanego komunikatu");

        // Cleanup
        std::fs::remove_file(&test_file).unwrap();
        ::fs::remove_dir(test_dir).unwrap();
    }
}