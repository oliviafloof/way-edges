use cosmic_text::Color;
use educe::Educe;
use serde::Deserialize;
use util::color::COLOR_BLACK;

use crate::widgets::common::{self};

#[derive(Educe, Deserialize, Clone)]
#[educe(Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TextPreset {
    Time {
        #[serde(default = "dt_time_format")]
        format: String,
        #[serde(default)]
        time_zone: Option<String>,
        #[serde(default = "dt_time_update_interval")]
        update_interval: u64,
    },
    Custom {
        update_with_interval_ms: (u64, String),
    },
}
fn dt_time_format() -> String {
    "%Y-%m-%d %H:%M:%S".to_string()
}
fn dt_time_update_interval() -> u64 {
    1000
}

#[derive(Educe, Deserialize, Clone)]
#[educe(Debug)]
pub struct TextConfig {
    #[serde(default = "dt_fg_color")]
    #[serde(deserialize_with = "common::color_translate")]
    pub fg_color: Color,
    #[serde(default = "dt_font_size")]
    pub font_size: i32,
    #[serde(default)]
    pub font_family: Option<String>,

    pub preset: TextPreset,
}

fn dt_fg_color() -> Color {
    COLOR_BLACK
}
fn dt_font_size() -> i32 {
    24
}
