use crate::Lycus;

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use poll_promise::Promise;

use lemmeknow::Identifier;

pub fn lemmeknow_gui(lycus: &mut Lycus, ui: &mut egui::Ui) {
    egui::Grid::new("lemmeknow_gui")
        .spacing([40.0, 4.0])
        .min_col_width(200.0)
        .show(ui, |ui| {
            // Add some padding and spacing between the elements
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
            ui.style_mut().spacing.slider_width = 80.0;

            let text_label = ui.label("Enter text: ");
            ui.add(
                egui::TextEdit::multiline(&mut lycus.text_input)
                    .desired_width(400.0)
                    .hint_text("Enter text here"),
            )
            .labelled_by(text_label.id);

            ui.end_row();

            if ui.button("Identify").clicked() {
                let text = lycus.text_input.clone();
                lycus.lemmeknow_promise = Some(Promise::spawn_thread("do_thing", move || {
                    identify_text(&text)
                }));
            }

            if let Some(promise) = lycus.lemmeknow_promise.take() {
                // we're doing some work, or we've finished but not taken results
                match promise.try_take() {
                    Ok(result) => {
                        // we're done!
                        if result.is_empty() {
                            lycus.lemmeknow_name = String::from("No results found");
                        } else {
                            let (name, description, tags, rarity) = process_results(result);
                            lycus.lemmeknow_name = name;
                            lycus.lemmeknow_description = description;
                            lycus.lemmeknow_tags = tags;
                            lycus.lemmeknow_rarity = rarity;
                        }
                    }
                    Err(promise) => {
                        // it's cooking, put it back
                        lycus.lemmeknow_promise = Some(promise);
                        // show that we're still working on it
                        ui.add(egui::widgets::Spinner::new().size(24.0));
                    }
                }
            }
            ui.end_row();
        });
    TableBuilder::new(ui)
        .column(Column::remainder())
        .column(Column::remainder())
        .column(Column::remainder())
        .column(Column::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Match");
            });
            header.col(|ui| {
                ui.heading("Description");
            });
            header.col(|ui| {
                ui.heading("Tags");
            });
            header.col(|ui| {
                ui.heading("Rarity");
            });
        })
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    ui.text_edit_singleline(&mut lycus.lemmeknow_name);
                });
                row.col(|ui| {
                    ui.text_edit_singleline(&mut lycus.lemmeknow_description);
                });
                row.col(|ui| {
                    ui.text_edit_singleline(&mut lycus.lemmeknow_tags);
                });
                row.col(|ui| {
                    ui.text_edit_singleline(&mut lycus.lemmeknow_rarity);
                });
            });
        });
}

pub fn process_results(results: Vec<lemmeknow::Match>) -> (String, String, String, String) {
    let name = results[0].data.name.to_string();
    let description = results[0]
        .data
        .description
        .unwrap_or("No description")
        .to_string();
    let tags = results[0].data.tags.join(", ");
    let rarity = results[0].data.rarity.to_string();
    (name, description, tags, rarity)
}

pub fn identify_text(text: &str) -> Vec<lemmeknow::Match> {
    let identifier = Identifier::default();
    identifier.identify(text)
}
