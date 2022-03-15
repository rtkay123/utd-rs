use std::{fs::File, io::Read, io::Write};

use clap::{lazy_static::lazy_static, StructOpt};
use regex::Regex;
use tracing::trace;
use utd::{are_you_on_unix, setup_logger, Task, Tasks};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let args = utd::args::Cli::parse();
    // don't drop guard
    let _guard = setup_logger(args.log.unwrap_or(utd::args::LogLevel::Trace));

    // Adding a new note/task
    if args.note.is_some() || args.add.is_some() {
        new_entry(&args)?;
    }
    Ok(())
}

fn new_entry(args: &utd::args::Cli) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(@.\w+)").unwrap();
    }
    let entry_adder = |list: &[String], is_task: bool, file: &mut File| {
        for entry_name in list.iter() {
            let tags: Vec<_> = RE.find_iter(entry_name).map(|f| f.as_str()).collect();
            let title = RE.replace_all(entry_name, "");
            let task = Task::new(&title, &tags.join(" "));
            let js = serde_json::to_string_pretty(&task).unwrap();
            writeln!(file, "{}", &js).unwrap();
        }
    };
    let mut path = are_you_on_unix();
    path.push(".utd.json");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(path)?;
    let mut contents = String::default();
    file.read_to_string(&mut contents).unwrap();
    // Check if file has data in it
    if contents.lines().count() > 0 {
        let /*mut*/ tasks: Tasks = serde_json::from_str(&contents).unwrap();
        trace!("found {} existing tasks", tasks.len());
    } else {
        trace!("found no existing tasks");
    }
    // if note is some, iterate and add notes
    entry_adder(args.add.as_ref().unwrap(), true, &mut file);
    Ok(())
}
