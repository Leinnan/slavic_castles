use serde::{Deserialize, Serialize};
use std::fmt;
use std::process::Command;

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize, Default)]
pub enum ResourceType {
    #[default]
    Tools,
    Magic,
    Soldiers,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ResourceType::Magic => "Magic",
                ResourceType::Tools => "Tools",
                ResourceType::Soldiers => "Soldiers",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Card {
    pub name: String,
    pub id: i32,
    pub cost_amount: i32,
    pub cost_resource: ResourceType,
    pub effects: Vec<CardEffect>,
}

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone, Deserialize, Serialize, Default)]
pub enum EffectType {
    /// dont use multiple production change per user
    ProductionChange(ResourceType, i32),
    /// if true ignores wall
    Damage(i32, bool),
    ResourceChange(ResourceType, i32),
    TowerGrowth(i32),
    WallsGrowth(i32),
    #[default]
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CardEffect {
    pub affects_user: bool,
    pub effect_type: EffectType,
}

impl fmt::Display for CardEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = match self.effect_type {
            EffectType::ProductionChange(resource, amount) => {
                let sign = if amount > 0 { "Increase" } else { "Decrease" };
                let target = if self.affects_user { "" } else { " enemy" };

                return write!(
                    f,
                    "{}{} prod of {} by {}",
                    sign,
                    target,
                    &resource,
                    amount.abs()
                );
            }
            EffectType::Damage(amount, ignore_wall) => {
                let ignore_wall = if ignore_wall { "(ignores shield)" } else { "" };
                let target = if self.affects_user {
                    "Takes"
                } else {
                    "Deliver"
                };
                return write!(f, "{} {} damage{}", target, amount, ignore_wall);
            }
            EffectType::ResourceChange(resource, amount) => {
                let sign = if amount > 0 { "Gives" } else { "Takes" };
                let target_suffix = if self.affects_user {
                    ""
                } else if amount > 0 {
                    " to enemy"
                } else {
                    " from enemy"
                };

                return write!(
                    f,
                    "{} {} of {}{}",
                    sign,
                    amount.abs(),
                    &resource,
                    target_suffix
                );
            }
            EffectType::TowerGrowth(growth) => {
                let sign = if growth > 0 { "+" } else { "-" };
                format!("{}{} Health", sign, growth)
            }
            EffectType::WallsGrowth(growth) => {
                let sign = if growth > 0 { "+" } else { "-" };
                format!("{}{} Shield", sign, growth)
            }
            EffectType::None => {
                return write!(f, "");
            }
        };
        if !self.affects_user {
            result.push_str(" to enemy");
        }
        write!(f, "{}", result)
    }
}

fn main() {
    println!("cargo::rerun-if-changed=card_project.svg");
    {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", &git_hash[..7]);
    }
    {
        let output = Command::new("git")
            .args(["log", "-1", "--date=format:%Y/%m/%d %T", "--format=%ad"])
            .output()
            .unwrap();
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_DATE={}", git_hash);
    }
    generate_card_gfx();
}
fn generate_card_gfx() {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use std::fs;
    use std::time::Instant;

    let start = Instant::now();
    let directory = "assets/cards";
    let cards: Vec<Card> = serde_json::from_str(include_str!("assets/all.deck.json")).unwrap();
    let svg = include_str!("card_project.svg");
    if std::path::Path::new(directory).exists() {
        std::fs::remove_dir_all(directory).unwrap();
    }
    fs::create_dir(directory).unwrap();
    cards.into_par_iter().for_each(|card| {
        let base_color = match &card.cost_resource {
            ResourceType::Tools => "#2a9efe",
            ResourceType::Magic => "#339820",
            ResourceType::Soldiers => "#bb332a",
        };
        let filename = format!("{}/{}.png", directory, card.id);
        let description: Vec<String> = card.effects.into_iter().map(|e| e.to_string()).collect();

        let svg_data = svg
            .to_owned()
            .replace("fill:#ff5555", &format!("fill:{}", &base_color))
            .replace(">99<", &format!(">{}<", card.cost_amount))
            .replace(
                "FIRST LINE TO REPLACE",
                description.first().unwrap_or(&String::new()),
            )
            .replace("CARD_NAME_HERE", &card.name)
            .replace(
                "SECOND LINE TO REPLACE",
                description.get(1).unwrap_or(&String::new()),
            )
            .replace(
                "THIRD LINE TO REPLACE",
                description.get(2).unwrap_or(&String::new()),
            );

        let mut fontdb = usvg::fontdb::Database::new();

        for entry in glob::glob("assets/fonts/*ttf").expect("Failed to load fonts") {
            let Ok(path) = entry else {
                continue;
            };
            fontdb.load_font_file(path).unwrap();
        }

        let tree =
            usvg::Tree::from_data(svg_data.as_bytes(), &usvg::Options::default(), &fontdb).unwrap();
        let pixmap_size = tree.size().to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
        pixmap.save_png(filename).unwrap();
    });
    let duration = start.elapsed();

    println!(
        "cargo:warning=Time elapsed for generate card gfx: {:?}",
        duration
    );
}
