use std::env;
use std::path::{Path, PathBuf};
use std::io;
use std::fs;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let work_dir = env::current_dir().unwrap();
    
    walk(&work_dir, |e| print_entry(e, true), |e| print_entry(e, false))?;

    Ok(())
}

fn print_entry(entry: &Entry, is_dir: bool) {
    if entry.nesting > 3 {
        return;
    }
    if let Some(name) = entry.path.file_name() {
        println!("{}{}{}", "  ".repeat(entry.nesting),
            name.to_string_lossy(),
            if is_dir { "/" } else { "" }
        );
    }
}

struct Entry {
    path: PathBuf,
    nesting: usize
}

fn walk<D, F>(dir: &Path, dir_action: D, file_action: F) -> io::Result<()>
    where D: Fn(&Entry), F: Fn(&Entry)
{
    let mut queue: VecDeque<Entry> = VecDeque::new();
    let entry = Entry {
        path: dir.canonicalize()?,
        nesting: 0
    };
    queue.push_back(entry);
    while !queue.is_empty() {
        let dir = queue.pop_front().unwrap();
        let cur_nesting = dir.nesting;
        dir_action(&dir);
        for dir_entry in fs::read_dir(&dir.path)? {
            let dir_entry = dir_entry?;
            let entry = Entry {
                path: dir_entry.path(),
                nesting: cur_nesting + 1
            };
            if dir_entry.path().is_dir() {
                queue.push_back(entry);
            } else {
                file_action(&entry);
            }
        }
    }

    Ok(())
}