use std::env;
use std::path::Path;
use notify::{event::{AccessKind, RenameMode}, RecursiveMode, Result, Watcher};

// fn main() 
fn main() -> Result<()>
{
    // let path = Path::new("./foo/bar/bar.txt");

    // let parent = path.parent();
    // print!("{:?}", parent);
    // assert_eq!(parent, Some(Path::new("./bar")));
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    println!("Start: {:?}", path);
    start_watching(path)?;
    Ok(())
}

fn start_watching(path: &Path) -> Result<()>
{
    let mut watcher = notify::recommended_watcher(handle_event)?;
    watcher.watch(path, RecursiveMode::Recursive)?;
    println!("Observing path: {:?}", path);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
fn handle_event(result: notify::Result<notify::Event>)
{
    //Result<T, E> - 'hej mam coś, ale może to być ok(), albo err()
    //println!("Got event: {:?}", result);
    match result {
        Ok(event) => {
            println!("Got event: {:?}", event);
            // let parent = event.paths.to;
            // print!("Parent path: {:?}", parent);
            match event.kind {
                notify::EventKind::Access(AccessKind::Close(_)) => {
                    
                    println!("File closed in path: {:?}", event.paths);
                }
                notify::EventKind::Create(_) => {
                    println!("File created in path: {:?}", event.paths);
                }
                notify::EventKind::Modify(_) => {
                    println!("File modified in path: {:?}", event.paths);
                }
                notify::EventKind::Remove(_) => {
                    println!("File removed in path: {:?}", event.paths);
                }
                _ => {}
            }
        }
        Err(e) => println!("Got error: {:?}", e),

}
}