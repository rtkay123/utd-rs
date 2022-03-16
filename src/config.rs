use std::io::Read;

use serde::Deserialize;
use serde::Serialize;
use tracing::error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub borders: String,
    pub tags: Tags,
    pub sections: Sections,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub colour: String,
    pub underline: bool,
    pub bold: bool,
    pub italic: bool,
    pub icon: String,
    #[serde(rename = "icon-suffix")]
    pub icon_suffix: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sections {
    pub title: Title,
    pub todo: Todo,
    #[serde(rename = "in-progress")]
    pub in_progress: InProgress,
    pub notes: Notes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub underline: bool,
    pub bold: bool,
    pub italic: bool,
    pub icon: String,
    #[serde(rename = "icon-suffix")]
    pub icon_suffix: bool,
    pub colour: String,
}
impl Configurable for Title {
    fn title_colour(&self) -> &str {
        &self.colour
    }
    fn indent_spaces(&self) -> u8 {
        unimplemented!()
    }

    fn title_bold(&self) -> bool {
        self.bold
    }

    fn title_italic(&self) -> bool {
        self.italic
    }

    fn title_underline(&self) -> bool {
        self.underline
    }

    fn title_icon_suffix(&self) -> bool {
        self.icon_suffix
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
        &self.icon
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

    fn completed_icon(&self) -> &str {
        unimplemented!()
    }
    fn colour_high(&self) -> &str {
        unimplemented!()
    }

    fn colour_completed(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    #[serde(rename = "title-colour")]
    pub title_colour: String,
    #[serde(rename = "indent-spaces")]
    pub indent_spaces: i64,
    #[serde(rename = "title-bold")]
    pub title_bold: bool,
    #[serde(rename = "title-italic")]
    pub title_italic: bool,
    #[serde(rename = "title-underline")]
    pub title_underline: bool,
    #[serde(rename = "title-icon")]
    pub title_icon: String,
    #[serde(rename = "title-icon-suffix")]
    pub title_icon_suffix: bool,
    #[serde(rename = "entry-icon")]
    pub entry_icon: String,
    #[serde(rename = "entry-icon-suffix")]
    pub entry_icon_suffix: bool,
    #[serde(rename = "entry-bold")]
    pub entry_bold: bool,
    #[serde(rename = "entry-italic")]
    pub entry_italic: bool,
    #[serde(rename = "dim-completed")]
    pub dim_completed: bool,
    #[serde(rename = "colour-low")]
    pub colour_low: String,
    #[serde(rename = "colour-normal")]
    pub colour_normal: String,
    #[serde(rename = "colour-high")]
    pub colour_high: String,
    #[serde(rename = "colour-completed")]
    pub colour_completed: String,
    #[serde(rename = "completed-icon")]
    pub completed_icon: String,
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
    fn title_colour(&self) -> &str {
        &self.title_colour
    }
    fn indent_spaces(&self) -> u8 {
        self.indent_spaces as u8
    }

    fn title_bold(&self) -> bool {
        self.title_bold
    }

    fn title_italic(&self) -> bool {
        self.title_italic
    }

    fn title_underline(&self) -> bool {
        self.title_underline
    }

    fn title_icon_suffix(&self) -> bool {
        self.title_icon_suffix
    }

    fn entry_icon_suffix(&self) -> bool {
        self.entry_icon_suffix
    }

    fn entry_bold(&self) -> bool {
        self.entry_bold
    }

    fn entry_italic(&self) -> bool {
        self.entry_italic
    }

    fn dim_completed(&self) -> bool {
        self.dim_completed
    }

    fn title_icon(&self) -> &str {
        &self.title_icon
    }

    fn entry_icon(&self) -> &str {
        &self.entry_icon
    }

    fn colour_low(&self) -> &str {
        &self.colour_low
    }

    fn colour_normal(&self) -> &str {
        &self.colour_normal
    }

    fn colour_high(&self) -> &str {
        &self.colour_high
    }
    fn completed_icon(&self) -> &str {
        &self.completed_icon
    }

    fn colour_completed(&self) -> &str {
        &self.colour_completed
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InProgress {
    #[serde(rename = "title-colour")]
    pub title_colour: String,
    #[serde(rename = "indent-spaces")]
    pub indent_spaces: i64,
    #[serde(rename = "title-bold")]
    pub title_bold: bool,
    #[serde(rename = "title-italic")]
    pub title_italic: bool,
    #[serde(rename = "title-underline")]
    pub title_underline: bool,
    #[serde(rename = "title-icon")]
    pub title_icon: String,
    #[serde(rename = "title-icon-suffix")]
    pub title_icon_suffix: bool,
    #[serde(rename = "entry-icon")]
    pub entry_icon: String,
    #[serde(rename = "entry-icon-suffix")]
    pub entry_icon_suffix: bool,
    #[serde(rename = "entry-bold")]
    pub entry_bold: bool,
    #[serde(rename = "entry-italic")]
    pub entry_italic: bool,
    #[serde(rename = "colour-low")]
    pub colour_low: String,
    #[serde(rename = "colour-normal")]
    pub colour_normal: String,
    #[serde(rename = "colour-high")]
    pub colour_high: String,
}
impl Configurable for InProgress {
    fn indent_spaces(&self) -> u8 {
        self.indent_spaces as u8
    }

    fn title_bold(&self) -> bool {
        self.title_bold
    }

    fn title_italic(&self) -> bool {
        self.title_italic
    }

    fn title_underline(&self) -> bool {
        self.title_underline
    }

    fn title_icon_suffix(&self) -> bool {
        self.title_icon_suffix
    }

    fn entry_icon_suffix(&self) -> bool {
        self.entry_icon_suffix
    }

    fn entry_bold(&self) -> bool {
        self.entry_bold
    }

    fn entry_italic(&self) -> bool {
        self.entry_italic
    }

    fn dim_completed(&self) -> bool {
        todo!()
    }

    fn title_icon(&self) -> &str {
        &self.title_icon
    }

    fn colour_completed(&self) -> &str {
        unimplemented!()
    }
    fn entry_icon(&self) -> &str {
        &self.entry_icon
    }

    fn colour_low(&self) -> &str {
        &self.colour_low
    }

    fn colour_normal(&self) -> &str {
        &self.colour_normal
    }

    fn colour_high(&self) -> &str {
        &self.colour_high
    }
    fn title_colour(&self) -> &str {
        &self.title_colour
    }
    fn completed_icon(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
    #[serde(rename = "title-colour")]
    pub title_colour: String,
    #[serde(rename = "indent-spaces")]
    pub indent_spaces: i64,
    #[serde(rename = "title-bold")]
    pub title_bold: bool,
    #[serde(rename = "title-italic")]
    pub title_italic: bool,
    #[serde(rename = "title-underline")]
    pub title_underline: bool,
    #[serde(rename = "title-icon")]
    pub title_icon: String,
    #[serde(rename = "title-icon-suffix")]
    pub title_icon_suffix: bool,
    #[serde(rename = "entry-icon")]
    pub entry_icon: String,
    #[serde(rename = "entry-icon-suffix")]
    pub entry_icon_suffix: bool,
    #[serde(rename = "entry-bold")]
    pub entry_bold: bool,
    #[serde(rename = "entry-italic")]
    pub entry_italic: bool,
    #[serde(rename = "dim-noted")]
    pub dim_noted: bool,
    #[serde(rename = "colour-low")]
    pub colour_low: String,
    #[serde(rename = "colour-normal")]
    pub colour_normal: String,
    #[serde(rename = "colour-high")]
    pub colour_high: String,
    #[serde(rename = "colour-completed")]
    pub colour_completed: String,
    #[serde(rename = "completed-icon")]
    pub completed_icon: String,
}

impl Configurable for Notes {
    fn title_colour(&self) -> &str {
        &self.title_colour
    }
    fn indent_spaces(&self) -> u8 {
        self.indent_spaces as u8
    }

    fn title_bold(&self) -> bool {
        self.title_bold
    }

    fn title_italic(&self) -> bool {
        self.title_italic
    }

    fn title_underline(&self) -> bool {
        self.title_underline
    }
    fn colour_completed(&self) -> &str {
        &self.colour_completed
    }
    fn title_icon_suffix(&self) -> bool {
        self.title_icon_suffix
    }

    fn entry_icon_suffix(&self) -> bool {
        self.entry_icon_suffix
    }

    fn entry_bold(&self) -> bool {
        self.entry_bold
    }

    fn entry_italic(&self) -> bool {
        self.entry_italic
    }

    fn completed_icon(&self) -> &str {
        &self.completed_icon
    }
    fn dim_completed(&self) -> bool {
        self.dim_noted
    }

    fn title_icon(&self) -> &str {
        &self.title_icon
    }

    fn entry_icon(&self) -> &str {
        &self.entry_icon
    }

    fn colour_low(&self) -> &str {
        &self.colour_low
    }

    fn colour_normal(&self) -> &str {
        &self.colour_normal
    }

    fn colour_high(&self) -> &str {
        &self.colour_high
    }
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub fn read_config_file() -> Result<Config> {
    let op = std::fs::OpenOptions::new()
        .read(true)
        .open(get_config_file_path());
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
                let file = include_bytes!("../config.toml");
                let contents = String::from_utf8_lossy(file);
                Ok(toml::from_str(&contents)?)
            }
            _ => {
                error!("{}", e);
                Err(Box::new(e))
            }
        },
    }
}

#[cfg(not(target_family = "unix"))]
fn get_config_file_path() {
    // TODO windows
}

#[cfg(target_family = "unix")]
fn get_config_file_path() -> std::path::PathBuf {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("utd").unwrap();
    xdg_dirs.get_config_file("config.toml")
}
