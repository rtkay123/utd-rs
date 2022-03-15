use std::{
    collections::VecDeque,
    fs::File,
    io::Read,
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

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
    } else if args.delete.is_some() {
        delete_entry(&args.delete.unwrap())?;
    }

    Ok(())
}

fn delete_entry(ids: &[String]) -> Result<()> {
    let mut path = are_you_on_unix();
    path.push(".utd.json");
    let mut tasks: Tasks;
    {
        let read_file = std::fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut buf_reader = std::io::BufReader::new(read_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        tasks = serde_json::from_str(&contents).unwrap();
        for i in ids.to_vec().iter() {
            tasks = tasks
                .iter()
                .filter_map(|f| {
                    if f.id != i.parse::<i64>().unwrap() {
                        Some(f.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
        }
    }
    let mut path = are_you_on_unix();
    path.push(".temp");
    let mut temp = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)?;
    write_to_file(&mut temp, &tasks);
    let mut original = are_you_on_unix();
    original.push(".utd.json");
    std::fs::rename(path, original).unwrap();
    Ok(())
}

fn new_entry(args: &utd::args::Cli) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(@.\w+)").unwrap();
    }
    fn timestamp() -> std::time::Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time is going backwards")
    }
    let entry_adder = |list: &[String],
                       is_task: bool,
                       file: &mut File,
                       path: &PathBuf,
                       priority: &mut VecDeque<&PriorityLevel>| {
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
        for entry_name in list.iter() {
            let tags: Vec<_> = RE.find_iter(entry_name).map(|f| f.as_str()).collect();
            let title = RE.replace_all(entry_name, "");
            len += 1;
            let task = Task::new(
                &title,
                &tags.join(" "),
                is_task,
                len,
                *priority.pop_front().unwrap_or(&PriorityLevel::Normal),
                timestamp().as_nanos(),
            );
            entries.push(task);
        }
        tasks.append(&mut entries);
        write_to_file(file, &tasks);
    };
    let mut path = are_you_on_unix();
    path.push(".utd.json");
    // if note is some, iterate and add notes
    let default_vec = &vec![
        PriorityLevel::Normal;
        match args.add.as_ref() {
            Some(tasks) => tasks.len(),
            // Safe to unwrap since we are sure one of them is some
            None => args.note.as_ref().unwrap().len(),
        }
    ];

    let mut vd = VecDeque::from_iter(args.priority.as_ref().unwrap_or(default_vec));
    if let Some(ref tasks) = args.add {
        entry_adder(tasks, true, &mut file(&path, false, true)?, &path, &mut vd);
    }
    if let Some(ref notes) = args.note {
        entry_adder(notes, false, &mut file(&path, false, true)?, &path, &mut vd);
    }
    Ok(())
}

fn file(path: &PathBuf, read: bool, write: bool) -> Result<File> {
    Ok(std::fs::OpenOptions::new()
        .create(true)
        .write(write)
        .read(read)
        .open(&path)?)
}

fn write_to_file(file: &mut File, tasks: &Tasks) {
    writeln!(file, "{}", serde_json::to_string_pretty(tasks).unwrap()).unwrap();
    trace!("tasks updated");
}
