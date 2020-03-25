use chrono::Local;
use dirs::home_dir;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use retry;
use std::env::args;
use std::error::Error;
use std::ffi::OsString;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{fs, io};

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
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(1)).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    let mut handler = SaveHandler::default();
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(path))
                if path.extension().unwrap_or_default() == "eu4"
                    && !path
                        .file_stem()
                        .map(|file| file.to_string_lossy().ends_with("Backup"))
                        .unwrap_or(false) =>
            {
                handler
                    .handle_save_file(path, NonZeroU32::new(4).unwrap())
                    .map_err(|e| e.to_string())?
            }
            Ok(ev) => println!("{} Ignore event {:?}", Local::now().to_rfc3339(), ev),
            Err(e) => eprintln!("{} watch error: {:?}", Local::now().to_rfc3339(), e),
        }
    }

    Ok(())
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
