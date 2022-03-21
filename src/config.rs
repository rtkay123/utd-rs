use std::io::Read;

use serde::Deserialize;
use serde::Serialize;
use tracing::error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub borders: Option<String>,
    pub tags: Option<Tags>,
    pub sections: Option<Sections>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            borders: Some(String::from("empty")),
            tags: Some(Tags::default()),
            sections: Some(Sections::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub colour: Option<String>,
    pub underline: Option<bool>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub icon: Option<String>,
    #[serde(rename = "icon-suffix")]
    pub icon_suffix: Option<bool>,
}

impl Default for Tags {
    fn default() -> Self {
        Self {
            colour: Some(String::from("#689d6a")),
            underline: Some(false),
            bold: Some(false),
            italic: Some(false),
            icon: Some(String::default()),
            icon_suffix: Some(false),
        }
    }
}

impl Tags {
    pub fn colour(&self) -> &str {
        match self.colour.as_ref() {
            Some(c) => c,
            None => "#689d6a",
        }
    }

    pub fn underline(&self) -> bool {
        self.underline.unwrap_or(false)
    }
    pub fn bold(&self) -> bool {
        self.bold.unwrap_or(false)
    }
    pub fn italic(&self) -> bool {
        self.italic.unwrap_or(false)
    }
    pub fn icon_suffix(&self) -> bool {
        self.icon_suffix.unwrap_or(false)
    }

    pub fn icon(&self) -> &str {
        match self.icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sections {
    pub title: Option<Title>,
    pub todo: Option<Todo>,
    #[serde(rename = "in-progress")]
    pub in_progress: Option<InProgress>,
    pub notes: Option<Notes>,
}

impl Default for Sections {
    fn default() -> Self {
        Self {
            title: Some(Title::default()),
            todo: Some(Todo::default()),
            in_progress: Some(InProgress::default()),
            notes: Some(Notes::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub underline: Option<bool>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub icon: Option<String>,
    #[serde(rename = "icon-suffix")]
    pub icon_suffix: Option<bool>,
    pub colour: Option<String>,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            underline: Some(false),
            bold: Some(false),
            italic: Some(false),
            icon: Some(String::default()),
            icon_suffix: Some(false),
            colour: Some(String::from("#ebdbb2")),
        }
    }
}

impl Configurable for Title {
    fn indent_spaces(&self) -> u8 {
        unimplemented!()
    }
    fn title_bold(&self) -> bool {
        self.bold.unwrap_or(false)
    }

    fn title_italic(&self) -> bool {
        self.italic.unwrap_or(false)
    }

    fn title_underline(&self) -> bool {
        self.underline.unwrap_or(false)
    }

    fn title_icon_suffix(&self) -> bool {
        self.icon_suffix.unwrap_or(false)
    }

    fn entry_icon_suffix(&self) -> bool {
        unimplemented!()
    }

    fn entry_bold(&self) -> bool {
        unimplemented!()
    }

    fn entry_italic(&self) -> bool {
        unimplemented!()
    }

    fn dim_completed(&self) -> bool {
        unimplemented!()
    }

    fn title_icon(&self) -> &str {
        match self.icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }

    fn entry_icon(&self) -> &str {
        unimplemented!()
    }

    fn colour_low(&self) -> &str {
        unimplemented!()
    }

    fn colour_normal(&self) -> &str {
        unimplemented!()
    }

    fn colour_high(&self) -> &str {
        unimplemented!()
    }

    fn colour_completed(&self) -> &str {
        unimplemented!()
    }
    fn title_colour(&self) -> &str {
        match self.colour.as_ref() {
            Some(c) => c,
            None => "#ebdbb2",
        }
    }

    fn completed_icon(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    #[serde(rename = "title-colour")]
    pub title_colour: Option<String>,
    #[serde(rename = "indent-spaces")]
    pub indent_spaces: Option<i64>,
    #[serde(rename = "title-bold")]
    pub title_bold: Option<bool>,
    #[serde(rename = "title-italic")]
    pub title_italic: Option<bool>,
    #[serde(rename = "title-underline")]
    pub title_underline: Option<bool>,
    #[serde(rename = "title-icon")]
    pub title_icon: Option<String>,
    #[serde(rename = "title-icon-suffix")]
    pub title_icon_suffix: Option<bool>,
    #[serde(rename = "entry-icon")]
    pub entry_icon: Option<String>,
    #[serde(rename = "entry-icon-suffix")]
    pub entry_icon_suffix: Option<bool>,
    #[serde(rename = "entry-bold")]
    pub entry_bold: Option<bool>,
    #[serde(rename = "entry-italic")]
    pub entry_italic: Option<bool>,
    #[serde(rename = "dim-completed")]
    pub dim_completed: Option<bool>,
    #[serde(rename = "colour-low")]
    pub colour_low: Option<String>,
    #[serde(rename = "colour-normal")]
    pub colour_normal: Option<String>,
    #[serde(rename = "colour-high")]
    pub colour_high: Option<String>,
    #[serde(rename = "colour-completed")]
    pub colour_completed: Option<String>,
    #[serde(rename = "completed-icon")]
    pub completed_icon: Option<String>,
}

pub trait Configurable {
    fn indent_spaces(&self) -> u8;
    fn title_bold(&self) -> bool;
    fn title_italic(&self) -> bool;
    fn title_underline(&self) -> bool;
    fn title_icon_suffix(&self) -> bool;
    fn entry_icon_suffix(&self) -> bool;
    fn entry_bold(&self) -> bool;
    fn entry_italic(&self) -> bool;
    fn dim_completed(&self) -> bool;
    fn title_icon(&self) -> &str;
    fn entry_icon(&self) -> &str;
    fn colour_low(&self) -> &str;
    fn colour_normal(&self) -> &str;
    fn colour_high(&self) -> &str;
    fn colour_completed(&self) -> &str;
    fn title_colour(&self) -> &str;
    fn completed_icon(&self) -> &str;
}

impl Configurable for Todo {
    fn indent_spaces(&self) -> u8 {
        self.indent_spaces.unwrap_or(4).try_into().unwrap()
    }
    fn title_bold(&self) -> bool {
        self.title_bold.unwrap_or(true)
    }

    fn title_italic(&self) -> bool {
        self.title_italic.unwrap_or(false)
    }

    fn title_underline(&self) -> bool {
        self.title_underline.unwrap_or(true)
    }

    fn title_icon_suffix(&self) -> bool {
        self.title_icon_suffix.unwrap_or(false)
    }

    fn entry_icon_suffix(&self) -> bool {
        self.entry_icon_suffix.unwrap_or(false)
    }

    fn entry_bold(&self) -> bool {
        self.entry_bold.unwrap_or(false)
    }

    fn entry_italic(&self) -> bool {
        self.entry_italic.unwrap_or(false)
    }

    fn dim_completed(&self) -> bool {
        self.dim_completed.unwrap_or(false)
    }

    fn title_icon(&self) -> &str {
        match self.title_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }

    fn entry_icon(&self) -> &str {
        match self.entry_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }

    fn colour_low(&self) -> &str {
        match self.colour_low.as_ref() {
            Some(c) => c,
            None => "#458588",
        }
    }

    fn colour_normal(&self) -> &str {
        match self.colour_normal.as_ref() {
            Some(c) => c,
            None => "#d65d0e",
        }
    }

    fn colour_high(&self) -> &str {
        match self.colour_high.as_ref() {
            Some(c) => c,
            None => "#cc241d",
        }
    }

    fn colour_completed(&self) -> &str {
        match self.colour_completed.as_ref() {
            Some(c) => c,
            None => "#98971a",
        }
    }
    fn title_colour(&self) -> &str {
        match self.title_colour.as_ref() {
            Some(c) => c,
            None => "#458588",
        }
    }

    fn completed_icon(&self) -> &str {
        match self.completed_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InProgress {
    #[serde(rename = "title-colour")]
    pub title_colour: Option<String>,
    #[serde(rename = "indent-spaces")]
    pub indent_spaces: Option<i64>,
    #[serde(rename = "title-bold")]
    pub title_bold: Option<bool>,
    #[serde(rename = "title-italic")]
    pub title_italic: Option<bool>,
    #[serde(rename = "title-underline")]
    pub title_underline: Option<bool>,
    #[serde(rename = "title-icon")]
    pub title_icon: Option<String>,
    #[serde(rename = "title-icon-suffix")]
    pub title_icon_suffix: Option<bool>,
    #[serde(rename = "entry-icon")]
    pub entry_icon: Option<String>,
    #[serde(rename = "entry-icon-suffix")]
    pub entry_icon_suffix: Option<bool>,
    #[serde(rename = "entry-bold")]
    pub entry_bold: Option<bool>,
    #[serde(rename = "entry-italic")]
    pub entry_italic: Option<bool>,
    #[serde(rename = "colour-low")]
    pub colour_low: Option<String>,
    #[serde(rename = "colour-normal")]
    pub colour_normal: Option<String>,
    #[serde(rename = "colour-high")]
    pub colour_high: Option<String>,
}
impl Configurable for InProgress {
    fn indent_spaces(&self) -> u8 {
        self.indent_spaces.unwrap_or(8).try_into().unwrap()
    }

    fn title_bold(&self) -> bool {
        self.title_bold.unwrap_or(false)
    }

    fn title_italic(&self) -> bool {
        self.title_italic.unwrap_or(false)
    }

    fn title_underline(&self) -> bool {
        self.title_underline.unwrap_or(true)
    }

    fn title_icon_suffix(&self) -> bool {
        self.title_icon_suffix.unwrap_or(false)
    }

    fn entry_icon_suffix(&self) -> bool {
        self.entry_icon_suffix.unwrap_or(false)
    }

    fn entry_bold(&self) -> bool {
        self.entry_bold.unwrap_or(false)
    }

    fn entry_italic(&self) -> bool {
        self.entry_italic.unwrap_or(true)
    }

    fn dim_completed(&self) -> bool {
        true
    }

    fn title_icon(&self) -> &str {
        match self.title_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }

    fn entry_icon(&self) -> &str {
        match self.entry_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }
    fn colour_low(&self) -> &str {
        match self.colour_low.as_ref() {
            Some(c) => c,
            None => "#458588",
        }
    }

    fn colour_normal(&self) -> &str {
        match self.colour_normal.as_ref() {
            Some(c) => c,
            None => "#d65d0e",
        }
    }

    fn colour_high(&self) -> &str {
        match self.colour_high.as_ref() {
            Some(c) => c,
            None => "#cc241d",
        }
    }

    fn colour_completed(&self) -> &str {
        unimplemented!()
    }
    fn title_colour(&self) -> &str {
        match self.title_colour.as_ref() {
            Some(c) => c,
            None => "#d79921",
        }
    }
    fn completed_icon(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    #[serde(rename = "title-colour")]
    pub title_colour: Option<String>,
    #[serde(rename = "indent-spaces")]
    pub indent_spaces: Option<i64>,
    #[serde(rename = "title-bold")]
    pub title_bold: Option<bool>,
    #[serde(rename = "title-italic")]
    pub title_italic: Option<bool>,
    #[serde(rename = "title-underline")]
    pub title_underline: Option<bool>,
    #[serde(rename = "title-icon")]
    pub title_icon: Option<String>,
    #[serde(rename = "title-icon-suffix")]
    pub title_icon_suffix: Option<bool>,
    #[serde(rename = "entry-icon")]
    pub entry_icon: Option<String>,
    #[serde(rename = "entry-icon-suffix")]
    pub entry_icon_suffix: Option<bool>,
    #[serde(rename = "entry-bold")]
    pub entry_bold: Option<bool>,
    #[serde(rename = "entry-italic")]
    pub entry_italic: Option<bool>,
    #[serde(rename = "dim-noted")]
    pub dim_noted: Option<bool>,
    #[serde(rename = "colour-low")]
    pub colour_low: Option<String>,
    #[serde(rename = "colour-normal")]
    pub colour_normal: Option<String>,
    #[serde(rename = "colour-high")]
    pub colour_high: Option<String>,
    #[serde(rename = "colour-completed")]
    pub colour_completed: Option<String>,
    #[serde(rename = "completed-icon")]
    pub completed_icon: Option<String>,
}

impl Default for Notes {
    fn default() -> Self {
        Self {
            title_colour: Some(String::from("#d79921")),
            indent_spaces: Some(4),
            title_bold: Some(false),
            title_italic: Some(false),
            title_underline: Some(false),
            title_icon: Some(String::default()),
            title_icon_suffix: Some(false),
            entry_icon: Some(String::default()),
            entry_icon_suffix: Some(false),
            entry_bold: Some(false),
            entry_italic: Some(false),
            dim_noted: Some(true),
            colour_low: Some("#ebdbb2".to_owned()),
            colour_normal: Some("#ebdbb2".to_owned()),
            colour_high: Some("#ebdbb2".to_owned()),
            colour_completed: Some("#458588".to_owned()),
            completed_icon: Some(String::default()),
        }
    }
}

impl Configurable for Notes {
    fn indent_spaces(&self) -> u8 {
        self.indent_spaces.unwrap_or(4).try_into().unwrap()
    }
    fn title_bold(&self) -> bool {
        self.title_bold.unwrap_or(false)
    }

    fn title_italic(&self) -> bool {
        self.title_italic.unwrap_or(false)
    }

    fn title_underline(&self) -> bool {
        self.title_underline.unwrap_or(false)
    }

    fn title_icon_suffix(&self) -> bool {
        self.title_icon_suffix.unwrap_or(false)
    }
    fn entry_icon_suffix(&self) -> bool {
        self.entry_icon_suffix.unwrap_or(false)
    }
    fn entry_bold(&self) -> bool {
        self.entry_bold.unwrap_or(false)
    }

    fn entry_italic(&self) -> bool {
        self.entry_italic.unwrap_or(false)
    }

    fn dim_completed(&self) -> bool {
        self.dim_noted.unwrap_or(true)
    }

    fn title_icon(&self) -> &str {
        match self.title_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }

    fn entry_icon(&self) -> &str {
        match self.entry_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }
    fn colour_low(&self) -> &str {
        match self.colour_low.as_ref() {
            Some(c) => c,
            None => "#ebdbb2",
        }
    }

    fn colour_normal(&self) -> &str {
        match self.colour_normal.as_ref() {
            Some(c) => c,
            None => "#ebdbb2",
        }
    }

    fn colour_high(&self) -> &str {
        match self.colour_high.as_ref() {
            Some(c) => c,
            None => "#ebdbb2",
        }
    }

    fn colour_completed(&self) -> &str {
        match self.colour_completed.as_ref() {
            Some(c) => c,
            None => "#ebdbb2",
        }
    }

    fn title_colour(&self) -> &str {
        match self.title_colour.as_ref() {
            Some(c) => c,
            None => "#d79921",
        }
    }

    fn completed_icon(&self) -> &str {
        match self.completed_icon.as_ref() {
            Some(c) => c,
            None => "",
        }
    }
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub fn read_config_file(retry: bool) -> Result<Config> {
    let op = std::fs::OpenOptions::new()
        .read(true)
        .open(get_config_file_path(retry));
    match op {
        Ok(contents) => {
            let mut buf_reader = std::io::BufReader::new(contents);
            let mut contents = String::new();
            match buf_reader.read_to_string(&mut contents) {
                Ok(_) => Ok(toml::from_str(&contents)?),
                Err(e) => {
                    error!("{}, using default configuration", e);
                    let file = include_bytes!("../config.toml");
                    let contents = String::from_utf8_lossy(file);
                    Ok(toml::from_str(&contents)?)
                }
            }
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                if !retry {
                    read_config_file(true)
                } else {
                    let file = include_bytes!("../config.toml");
                    let contents = String::from_utf8_lossy(file);
                    Ok(toml::from_str(&contents)?)
                }
            }
            _ => {
                error!("{}", e);
                Err(Box::new(e))
            }
        },
    }
}

fn get_config_file_path(retry: bool) -> std::path::PathBuf {
    use directories::ProjectDirs;
    let dirs = ProjectDirs::from("org", "Ugly Todo", "utd").unwrap();
    let dirs = dirs.config_dir();
    if retry {
        dirs.with_file_name("utd.toml")
    } else {
        dirs.with_file_name("utd/config.toml")
    }
}
