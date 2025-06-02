use std::env;
use std::path::Path;
use notify::{RecursiveMode,Result, Watcher};
fn main() -> Result<()>
{
    let args: Vec<String> = env::args().collect();
    // let first = &args[1];
    // let second = &args[2];
    // // dbg!(args);
    //
    //
    // println!("First argument: {}", first);
    // println!("Second argument: {}", second);
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
        Ok(event) => println!("Got event: {:?}", event),
        Err(e) => println!("Got error: {:?}", e),
    }
}