use std::{fs::File, io::Read, io::Write, path::PathBuf};

use clap::{lazy_static::lazy_static, StructOpt};
use regex::Regex;
use tracing::trace;
use utd::{are_you_on_unix, args::PriorityLevel, setup_logger, Task, Tasks};

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
    let entry_adder = |list: &[String],
                       is_task: bool,
                       file: &mut File,
                       path: &PathBuf,
                       priority: &Vec<PriorityLevel>| {
        let mut tasks: Tasks;
        {
            let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
            let mut buf_reader = std::io::BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            // Check if file has data in it
            if contents.lines().count() > 0 {
                tasks = serde_json::from_str(&contents).unwrap();
                trace!("found {} existing tasks", tasks.len());
            } else {
                tasks = Vec::with_capacity(list.len());
                trace!("found no existing tasks");
            }
        }
        let mut entries = Vec::with_capacity(list.len());
        let mut len = match tasks.iter().max_by_key(|f| f.id) {
            Some(task) => task.id,
            None => 0,
        };
        for (index, entry_name) in list.iter().enumerate() {
            let tags: Vec<_> = RE.find_iter(entry_name).map(|f| f.as_str()).collect();
            let title = RE.replace_all(entry_name, "");
            len += 1;
            let task = Task::new(
                &title,
                &tags.join(" "),
                is_task,
                len,
                *priority.get(index).unwrap_or(&PriorityLevel::Normal),
            );
            entries.push(task);
        }
        tasks.append(&mut entries);
        writeln!(file, "{}", serde_json::to_string_pretty(&tasks).unwrap()).unwrap();
    };
    let mut path = are_you_on_unix();
    path.push(".utd.json");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)?;

    // if note is some, iterate and add notes
    if let Some(ref tasks) = args.add {
        entry_adder(
            tasks,
            true,
            &mut file,
            &path,
            args.priority
                .as_ref()
                .unwrap_or(&vec![PriorityLevel::Normal; tasks.len()]),
        );
    }
    if let Some(ref notes) = args.note {
        entry_adder(
            notes,
            false,
            &mut file,
            &path,
            args.priority
                .as_ref()
                .unwrap_or(&vec![PriorityLevel::Normal; notes.len()]),
        );
    }
    Ok(())
}
