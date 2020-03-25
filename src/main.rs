use std::path::{PathBuf, Path};
use dirs::home_dir;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use chrono::Local;
use std::num::NonZeroU32;
use std::fs;
use std::ffi::OsString;

#[cfg(windows)]
fn get_default_path() -> Result<PathBuf, &'static str> {
    match home_dir() {
        None => Err("Cannot get home directory"),
        Some(user_folder_path) =>
            Ok(user_folder_path.join("Documents\\Paradox Interactive\\Europa Universalis IV\\save games")),
    }

}

fn main() -> Result<(), &'static str> {
    let path = get_default_path()?;
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(1)).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(path))
                if path.extension().unwrap_or_default() == "eu4"
                && !path.file_stem().map(|file| file.to_string_lossy().ends_with("Backup")).unwrap_or(false) =>
                    handle_save_file(path, NonZeroU32::new(4).unwrap()),
            Ok(_) => {}, // ignore
            Err(e) => eprintln!("{} watch error: {:?}", Local::now().format("%Y-%m-%dT%H:%M:%S"), e),
        }
    }

    Ok(())
}

fn handle_save_file<P: AsRef<Path>>(path: P, frequency: NonZeroU32) {
    static mut CALLS: u32 = 0;

    let path = path.as_ref();
    println!("{} Handling file {:?}", Local::now().to_rfc3339(), path);
    unsafe {
        if CALLS % frequency.get() == 0 {
            let new_path = get_new_path(path);
            println!("{} Copying {:?} to {:?}", Local::now().to_rfc3339(), path, new_path);
            fs::copy(path, new_path);
        } else {
            println!("{} Skipping saving {:?}: Not time yet", Local::now().to_rfc3339(), path);
        }
        CALLS += 1;
    }
}

fn get_new_path(p: &Path) -> PathBuf {
    let mut result = OsString::from(p);
    result.push("_");
    result.push(format!("{}", Local::now().timestamp()));
    result.push("_Backup");
    result.into()
}