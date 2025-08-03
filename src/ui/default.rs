use indexmap::IndexMap;
use once_cell::sync::Lazy;
use serde_json::Number;

use super::types;

pub static UI_CONFIGURATION: Lazy<types::UI> = Lazy::new(|| types::UI {
    name: String::from("Seyuna"),
    slogan: String::from("Another cool Seyuna app."),
    theme: types::Theme {
        colors: IndexMap::from([
            ("alpha".to_string(), Number::from(0)),
            ("beta".to_string(), Number::from(15)),
            ("gamma".to_string(), Number::from(30)),
            ("delta".to_string(), Number::from(45)),
            ("epsilon".to_string(), Number::from(60)),
            ("zeta".to_string(), Number::from(75)),
            ("eta".to_string(), Number::from(90)),
            ("theta".to_string(), Number::from(105)),
            ("iota".to_string(), Number::from(120)),
            ("kappa".to_string(), Number::from(135)),
            ("lambda".to_string(), Number::from(150)),
            ("mu".to_string(), Number::from(165)),
            ("nu".to_string(), Number::from(180)),
            ("xi".to_string(), Number::from(195)),
            ("omicron".to_string(), Number::from(210)),
            ("pi".to_string(), Number::from(225)),
            ("rho".to_string(), Number::from(240)),
            ("sigma".to_string(), Number::from(255)),
            ("tau".to_string(), Number::from(270)),
            ("upsilon".to_string(), Number::from(285)),
            ("phi".to_string(), Number::from(300)),
            ("chi".to_string(), Number::from(315)),
            ("psi".to_string(), Number::from(330)),
            ("omega".to_string(), Number::from(345)),
        ]),
        light: types::Palette {
            chroma: Number::from_f64(0.70).unwrap(),
            lightness: Number::from_f64(0.9).unwrap(),
            background: types::Color {
                hue: Number::from(0),
                chroma: Number::from_f64(0.0).unwrap(),
                lightness: Number::from_f64(1.0).unwrap(),
            },
            text: types::Color {
                hue: Number::from(0),
                chroma: Number::from_f64(0.0).unwrap(),
                lightness: Number::from_f64(0.0).unwrap(),
            },
        },
        dark: types::Palette {
            chroma: Number::from_f64(0.70).unwrap(),
            lightness: Number::from_f64(0.9).unwrap(),
            background: types::Color {
                hue: Number::from(0),
                chroma: Number::from_f64(0.0).unwrap(),
                lightness: Number::from_f64(0.0).unwrap(),
            },
            text: types::Color {
                hue: Number::from(0),
                chroma: Number::from_f64(0.0).unwrap(),
                lightness: Number::from_f64(1.0).unwrap(),
            },
        },
    },
    mode: types::Mode::System,
    output_dir: Some(String::from("styles")),
});
