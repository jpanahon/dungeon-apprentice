use eframe::egui::{self, Color32};
// use rand::Rng;

mod parser;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    let monsters = parser::parse_xml_statblocks();

    for monster in &monsters? {
        println!("{:#?}", monster);
    }
    
    /*
    match monsters {
        Ok(m) => {
            let first_monster = &m[0];
            println!("Name: {}", first_monster.name);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    } */
    
    eframe::run_simple_native("Dungeon Apprentice", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(app_theme());
            ui.label(format!("Hello There!"));
        });
    })
}

fn app_theme() -> egui::Visuals {
    let mut visuals: egui::Visuals = egui::Visuals::default();
    visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));
    visuals
}