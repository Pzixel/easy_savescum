use chrono::Local;
use dirs::home_dir;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use retry;
use std::env::args;
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufRead};
use std::num::{NonZeroU32, ParseIntError};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::mpsc::{Sender, TryRecvError};
use std::thread;
use std::time::Duration;

#[cfg(windows)]
fn get_default_path() -> Result<PathBuf, String> {
    match home_dir() {
        None => Err("Cannot get home directory".into()),
        Some(user_folder_path) => {
            Ok(user_folder_path.join("Documents\\Paradox Interactive\\Europa Universalis IV\\save games"))
        }
    }
}

fn main() -> Result<(), String> {
    let args = args().skip(1).next();
    let path = match args {
        None => {
            let path = get_default_path()?;
            println!("No custom path for save files provided. Using default: {:?}", path);
            path
        }
        Some(path) => {
            let path: PathBuf = path.into();
            println!("Custom path for save files provided: {:?}", path);
            path
        }
    };

    const DEFAULT_FREQUENCY: u32 = 4;
    let mut frequency = NonZeroU32::new(DEFAULT_FREQUENCY).unwrap();
    println!(
        "This program will monitor all saves in directory {:?} and backup some of them.\nYou can change the default \
         directory by running passing an argument to the program.\nYou can change frequency dynamically by typing it \
         below.\nPress 'Ctrl+C' to break the loop",
        &path
    );
    loop {
        let (stop_tx, stop_rx) = channel();
        let (stopped_tx, stopped_rx) = channel();
        run_watch_loop(path.clone(), frequency, stop_rx, stopped_tx);
        println!(
            "Running with frequency of {}. Type a number below if you want to change it",
            frequency.get()
        );
        frequency = io::stdin()
            .lock()
            .lines()
            .map(|x| {
                let x = x.map_err(|e| e.to_string())?;
                x.parse().map_err(|e: ParseIntError| e.to_string())
            })
            .inspect(|r| {
                if r.is_err() {
                    println!("Couldn't parse line as a number");
                }
            })
            .filter_map(|x| x.ok())
            .next()
            .unwrap();
        stop_tx.send(()).unwrap();
        println!("Got a new frequency! Change will be applied at next tick");
        stopped_rx.recv().unwrap();
    }
}

fn run_watch_loop(path: PathBuf, frequency: NonZeroU32, stop_rx: Receiver<()>, stopped_tx: Sender<()>) {
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_millis(1)).unwrap();
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
        let mut handler = SaveHandler::default();
        loop {
            match stop_rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Aborting watch");
                    stopped_tx.send(()).unwrap();
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }

            match rx.recv() {
                Ok(DebouncedEvent::Write(path))
                    if path.extension().unwrap_or_default() == "eu4"
                        && !path
                            .file_stem()
                            .map(|file| file.to_string_lossy().ends_with("Backup"))
                            .unwrap_or(false) =>
                {
                    let save_result = handler.handle_save_file(path, frequency).map_err(|e| e.to_string());
                    if let Err(e) = save_result {
                        eprintln!("{} save error: {:?}", Local::now().to_rfc3339(), e)
                    }
                }
                Ok(ev) => println!("{} Ignore event {:?}", Local::now().to_rfc3339(), ev),
                Err(e) => eprintln!("{} watch error: {:?}", Local::now().to_rfc3339(), e),
            }
        }
    });
}

#[derive(Default)]
struct SaveHandler {
    calls: u32,
}

impl SaveHandler {
    fn handle_save_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        frequency: NonZeroU32,
    ) -> Result<(), retry::Error<io::Error>> {
        let path = path.as_ref();
        println!("{} Handling file {:?}", Local::now().to_rfc3339(), path);
        if self.calls % frequency.get() == 0 {
            let new_path = get_new_path(path);
            println!("{} Copying {:?} to {:?}", Local::now().to_rfc3339(), path, new_path);
            retry::retry(retry::delay::Fixed::from_millis(200).take(10), move || {
                fs::copy(&path, &new_path)
            })?;
        } else {
            println!("{} Skipping saving {:?}: Not time yet", Local::now().to_rfc3339(), path);
        }
        self.calls += 1;
        Ok(())
    }
}

fn get_new_path(p: &Path) -> PathBuf {
    let mut result = OsString::from(p);
    result.push("_");
    result.push(format!("{}", Local::now().timestamp()));
    result.push("_Backup");
    result.into()
}
