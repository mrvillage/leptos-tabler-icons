use include_dir::{include_dir, Dir};

const SVG: Dir = include_dir!("$CARGO_MANIFEST_DIR/svg");

fn main() {
    println!("cargo:rerun-if-changed=svg");
    let re = regex::Regex::new("(?sU)<svg.*>(.*)</svg>").unwrap();
    let files = SVG
        .files()
        .map(|i| {
            let name = i
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".svg", "");
            let enum_name: String = name
                .split('-')
                .map(|i| {
                    format!(
                        "{}{}",
                        i.chars().next().unwrap().to_ascii_uppercase(),
                        &i[1..]
                    )
                })
                .collect();

            let cap = re.captures(i.contents_utf8().unwrap()).unwrap();
            let svg = cap.get(1).unwrap().as_str().trim().replace("\n", "");
            (name, enum_name, svg)
        })
        .collect::<Vec<_>>();
    let variants = files
        .iter()
        .map(|(_, e, _)| {
            format!(
                r#"
        #[cfg(feature = "Icon{e}")]
        Icon{e},
        "#
            )
        })
        .collect::<Vec<String>>()
        .join("");
    let names = files
        .iter()
        .map(|(n, e, _)| format!(r#"
        #[cfg(feature = "Icon{e}")]
        Icons::Icon{e} => write!(f, "{n}"),
        "#))
        .collect::<Vec<String>>()
        .join("");
    let froms = files
        .iter()
        .map(|(n, e, _)| format!(r#"
        #[cfg(feature = "Icon{e}")]
        "{n}" => Ok(Icons::Icon{e}),
        "#))
        .collect::<Vec<String>>()
        .join("");
    let svgs = files
        .iter()
        .map(|(_, e, s)| format!(r###"
        #[cfg(feature = "Icon{e}")]
        Icons::Icon{e} => r##"{s}"##,
        "###))
        .collect::<Vec<String>>()
        .join("");
    let features = files
        .iter()
        .map(|(_, e, _)| format!("Icon{e} = []"))
        .collect::<Vec<String>>()
        .join("\n");
    std::fs::write(
        std::env::current_dir().unwrap().join("Cargo.toml"),
        format!(
            r##"[package]
name = "leptos-tabler-icons"
version = "0.1.1"
edition = "2021"
license = "MIT"
description = "Tabler Icons for Leptos"
repository = "https://github.com/mrvillage/leptos-tabler-icons"

[dependencies]
tracing = "0.1"

[dependencies.leptos]
git = "https://github.com/leptos-rs/leptos.git"
branch = "main"
features = ["rustls", "serde", "nightly"]

[build-dependencies]
include_dir = "0.7.3"
regex = "1.9.1"

[features]
default = []

{features}
            "##,
        ),
    )
    .unwrap();
    std::fs::write(
        std::env::current_dir()
            .unwrap()
            .join("src")
            .join("icons.rs"),
        format!(
            r##"
// This file is generated by build.

#[derive(Clone, Debug)]
pub enum Icons {{
    {variants}
}}

impl std::fmt::Display for Icons {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match self {{
            {names}
        }}
    }}
}}

impl TryFrom<&str> for Icons {{
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {{
        match s {{
            {froms}
            _ => Err(())
        }}
    }}
}}

impl Icons {{
    pub fn svg(&self) -> &'static str {{
        match self {{
            {svgs}
        }}
    }}
}}
    "##,
        ),
    )
    .unwrap();
}
