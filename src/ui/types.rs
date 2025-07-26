use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct UI {
    pub name: String,
    pub slogan: String,
    pub theme: Theme,
    pub mode: Mode,
    #[schemars(skip)]
    #[serde(skip)]
    pub breakpoints: Option<BreakpointsConfig>,
    #[schemars(skip)]
    #[serde(skip)]
    pub upscale_from: Option<Breakpoint>,
    #[schemars(skip)]
    pub spacing: Number,
    pub output_dir: Option<String>,
}

pub type ViewportBreakpoints = HashMap<Breakpoint, Number>;
pub type ContainerBreakpoints = Vec<Number>;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct BreakpointsConfig {
    pub viewport: ViewportBreakpoints,
    pub container: ContainerBreakpoints,
}

#[derive(Debug, Serialize, PartialEq, Eq, Hash, Deserialize, JsonSchema, Clone, Copy)]
pub enum Breakpoint {
    SM,
    MD,
    LG,
    XL,
    _2XL,
    _3XL,
    _4XL,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    System,
    Light,
    Dark,
}

pub type Hue = Number;
pub type Chroma = Number;
pub type Lightness = Number;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Color {
    pub hue: Hue,
    pub chroma: Chroma,
    pub lightness: Lightness,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Palette {
    pub chroma: Chroma,
    pub lightness: Lightness,
    pub background: Color,
    pub text: Color,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Theme {
    pub colors: IndexMap<String, Hue>,
    pub light: Palette,
    pub dark: Palette,
}

impl UI {
    pub fn merge(self, other: Self) -> Self {
        Self {
            name: if other.name.is_empty() {
                self.name
            } else {
                other.name
            },
            slogan: if other.slogan.is_empty() {
                self.slogan
            } else {
                other.slogan
            },
            theme: self.theme.merge(other.theme),
            mode: other.mode,
            breakpoints: other.breakpoints.or(self.breakpoints),
            upscale_from: other.upscale_from.or(self.upscale_from),
            spacing: other.spacing,
            output_dir: other.output_dir.or(self.output_dir),
        }
    }
}

impl Theme {
    pub fn merge(self, other: Self) -> Self {
        let mut merged_colors = self.colors;

        for (key, value) in other.colors {
            merged_colors.insert(key, value);
        }

        Self {
            colors: merged_colors,
            light: self.light.merge(other.light),
            dark: self.dark.merge(other.dark),
        }
    }
}

impl Palette {
    pub fn merge(self, other: Self) -> Self {
        Self {
            chroma: other.chroma,
            lightness: other.lightness,
            background: self.background.merge(other.background),
            text: self.text.merge(other.text),
        }
    }
}

impl Color {
    pub fn merge(self, other: Self) -> Self {
        Self {
            hue: other.hue,
            chroma: other.chroma,
            lightness: other.lightness,
        }
    }
}
