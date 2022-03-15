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
use tracing::{debug, trace};
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
    if args.delete.is_some() {
        delete_entry(&args.delete.unwrap())?;
    }
    if args.begin.is_some() {
        alter_tasks(&args.begin.unwrap(), State::Started)?;
    }
    if args.check.is_some() {
        alter_tasks(&args.check.unwrap(), State::Completed)?;
    }

    Ok(())
}

enum State {
    Started,
    Completed,
}

fn alter_tasks(ids: &[String], state: State) -> Result<()> {
    let mut tasks = state_file_contents()?;
    for i in ids.iter() {
        let i: usize = i.parse().unwrap();
        let vals = tasks
            .clone()
            .into_iter()
            .map(|mut f| {
                if f.id as usize == i {
                    match state {
                        State::Started => {
                            f.in_progress = true;
                            debug!("starting task {}: {}", i, f.name);
                        }
                        State::Completed => {
                            f.in_progress = false;
                            f.is_done = true;
                            debug!("completing task {}: {}", i, f.name);
                        }
                    }
                }
                f
            })
            .collect();
        tasks = vals;
    }
    update_file(&tasks)?;
    debug!("{} tasks updated - ok", ids.len());
    Ok(())
}

fn delete_entry(ids: &[String]) -> Result<()> {
    let mut tasks = state_file_contents()?;
    for i in ids.iter() {
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
    update_file(&tasks)?;
    debug!("{} tasks deleted - ok", ids.len());
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
                       priority: &mut VecDeque<&PriorityLevel>| {
        let mut tasks: Tasks = state_file_contents().unwrap();
        {
            // Check if file has data in it
            if !tasks.is_empty() {
                trace!("found {} existing tasks", tasks.len());
            } else {
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
        entry_adder(tasks, true, &mut state_file(&path, false, true)?, &mut vd);
    }
    if let Some(ref notes) = args.note {
        entry_adder(notes, false, &mut state_file(&path, false, true)?, &mut vd);
    }
    Ok(())
}

fn state_file(path: &PathBuf, read: bool, write: bool) -> Result<File> {
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

fn state_file_contents() -> Result<Tasks> {
    let mut path = are_you_on_unix();
    path.push(".utd.json");
    let read_file = std::fs::OpenOptions::new().read(true).open(&path).unwrap();
    let mut buf_reader = std::io::BufReader::new(read_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    // empty list
    if contents.is_empty() {
        contents.push_str("[]");
    }
    let tasks: Tasks = serde_json::from_str(&contents)?;
    Ok(tasks)
}

fn update_file(tasks: &Tasks) -> Result<()> {
    let mut path = are_you_on_unix();
    path.push(".temp");
    let mut temp = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)?;
    write_to_file(&mut temp, tasks);
    let mut original = are_you_on_unix();
    original.push(".utd.json");
    std::fs::rename(path, original)?;
    Ok(())
}
