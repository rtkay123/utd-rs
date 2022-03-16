use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub borders: bool,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InProgress {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notes {
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
}
