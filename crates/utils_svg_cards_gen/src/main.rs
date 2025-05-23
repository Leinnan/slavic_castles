use std::env;

fn main() {
    use game_core::data::card::*;
    use game_core::data::resource::ResourceType;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use std::fs;
    use std::time::Instant;

    let start = Instant::now();
    let directory = "assets/cards";
    if !std::path::Path::new("assets/all.deck.json").exists() {
        panic!("Called from wrong directory or assets/all.deck.json is missing.");
    }
    let cards: Vec<Card> =
        serde_json::from_str(include_str!("../../../assets/all.deck.json")).unwrap();
    println!(
        "Current working directory: {}",
        env::current_dir().expect("DD").display()
    );
    println!("Generating {} cards ", cards.len());
    let svg = include_str!("../card_project.svg");

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

    println!("Time elapsed for generate card gfx: {:?}", duration);
}
