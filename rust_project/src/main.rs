use std::env;
use std::path::Path;
use notify::{event::{AccessKind, RemoveKind}, RecursiveMode, Result, Watcher};
use notify::event::AccessMode;


fn main() -> Result<()>
{
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    println!("Starting watcher on path: {:?}", path);
    start_watching(path)?;
    Ok(())
}

fn start_watching(path: &Path) -> Result<()>
{
    let base_path = path.to_path_buf();
    let mut watcher = notify::recommended_watcher(move |event| handle_event(event, &base_path))?;
    watcher.watch(path, RecursiveMode::Recursive)?;
    println!("Observing path: {:?}", path);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
fn handle_event(result: notify::Result<notify::Event>, base_path: &Path)
{
    //Result<T, E> - 'hej mam coś, ale może to być ok(), albo err()
    match result {
        Ok(event) => {
            if let Some(path) = event.paths.last() {
                let rel_path = path.strip_prefix(base_path).unwrap_or(path);
                if let Some(name) = path.file_name() {
                    for path in &event.paths {
                                if path.is_file() {
                                    println!("File: {:?}", name);
                                } else if path.is_dir() {
                                    println!("Folder: {:?}", name);
                                } else {
                                    println!("Unknown: {:?}", name);
                                }
                            }
                } else {
                    println!("Path has no file name: {:?}", rel_path);
                }
            } else {
                println!("No paths in event");
            }
            match event.kind {
                notify::EventKind::Access(AccessKind::Close(access)) => {
                    match access {

                        AccessMode::Read => {
                            if let Some(path) = event.paths.last() {
                                println!("File read closed in path: {:?}", path);
                            }
                        }
                        AccessMode::Write => {
                            if let Some(path) = event.paths.last() {
                                println!("File write closed in path: {:?}", path);
                            }
                        }
                        AccessMode::Execute =>
                         if let Some(path) = event.paths.last() {
                                println!("File execute closed in path: {:?}", path);
                            }
                        _ => {}
                    }
                    println!("File closed in path: {:?}", event.paths);
                }
                notify::EventKind::Create(kind) => {
                    match kind {
                        notify::event::CreateKind::File => {
                            println!("File created in path: {:?}", event.paths);
                        }
                        notify::event::CreateKind::Folder => {
                            println!("Folder created in path: {:?}", event.paths);
                        }
                        notify::event::CreateKind::Any => {
                            for path in &event.paths {
                                if path.is_file() {
                                    println!("File created in path: {:?}", path);
                                } else if path.is_dir() {
                                    println!("Folder created in path: {:?}", path);
                                } else {
                                    println!("Unknown created in path: {:?}", path);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                
                notify::EventKind::Modify(_) => {
                    println!("File modified in path: {:?}", event.paths[0]);
                }
                notify::EventKind::Remove(RemoveKind::File) => {
                    println!("File removed in path: {:?}", event.paths);
                }
                notify::EventKind::Remove(RemoveKind::Folder) => {
                    println!("Folder removed in path: {:?}", event.paths);
                }
                notify::EventKind::Remove(RemoveKind::Any) => {
                    println!("File or Folder removed in path: {:?}", event.paths);
                }
                _ => {}
            }
        }
        Err(e) => println!("Got error: {:?}", e),

}
}