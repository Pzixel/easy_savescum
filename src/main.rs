use chrono::Local;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs;
use std::path::{PathBuf};
use std::sync::mpsc::{channel};
use std::time::Duration;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    path: String,
    #[clap(short, long)]
    savescum_dir_path: String,
    #[clap(short, long, default_value_t = 1)]
    frequency: u64,
}

fn main() {
    let args: Args = Args::parse();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(1)).unwrap();
    watcher.watch(args.path, RecursiveMode::NonRecursive).unwrap();
    let mut i = 0;
    println!("Running...");
    loop {
        match rx.recv() {
            | Ok(DebouncedEvent::Write(path))
            | Ok(DebouncedEvent::NoticeWrite(path))
            | Ok(DebouncedEvent::Create(path)) =>
                {
                    if i % args.frequency == 0 {
                        let file_name = path.file_name().unwrap().to_str().unwrap();
                        let file_name_prefix = Local::now().format("%FT%H-%M-%S");
                        let new_path = PathBuf::from(&args.savescum_dir_path)
                                .join(format!("{}_{}", file_name_prefix, file_name));
                        if !new_path.exists() {
                            println!("Saving {} to {:?}", file_name, new_path);
                            fs::copy(path, new_path).unwrap();
                        }
                    }
                    i += 1;
                }
            Ok(ev) => println!("{} Ignore event {:?}", Local::now().to_rfc3339(), ev),
            Err(e) => eprintln!("{} watch error: {:?}", Local::now().to_rfc3339(), e),
        }
    }
}
