use crate::Lycus;

use eframe::egui;
use poll_promise::Promise;

use ares::perform_cracking;

pub fn ares_gui(lycus: &mut Lycus, ui: &mut egui::Ui) {
    egui::Grid::new("ares_gui")
        .spacing([40.0, 4.0])
        .min_col_width(200.0)
        .show(ui, |ui| {
            // Add some padding and spacing between the elements
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
            ui.style_mut().spacing.slider_width = 80.0;

            let ciphertext_label = ui.label("Enter ciphertext: ");
            ui.add(
                egui::TextEdit::multiline(&mut lycus.ciphertext)
                    .desired_width(400.0)
                    .hint_text("Enter ciphertext here"),
            )
            .labelled_by(ciphertext_label.id);

            ui.end_row();

            if ui.button("Crack").clicked() {
                let ciphertext = lycus.ciphertext.clone();
                let timeout = lycus.timeout;
                lycus.ares_promise = Some(Promise::spawn_thread("do_thing", move || {
                    crack_ciphertext(&ciphertext, timeout)
                }));
            }

            if let Some(promise) = lycus.ares_promise.take() {
                // we're doing some work, or we've finished but not taken results
                match promise.try_take() {
                    Ok((path, plaintext)) => {
                        // we're done!
                        lycus.plaintext = plaintext;
                        lycus.path = path;
                    }
                    Err(promise) => {
                        // it's cooking, put it back
                        lycus.ares_promise = Some(promise);
                        // show that we're still working on it
                        ui.add(egui::widgets::Spinner::new().size(24.0));
                    }
                }
            }
            ui.end_row();

            let plain_label = ui.label("Plaintext: ");
            ui.text_edit_multiline(&mut lycus.plaintext)
                .labelled_by(plain_label.id);

            ui.end_row();

            ui.horizontal(|ui| {
                let path_label = ui.label("Path: ");
                ui.text_edit_singleline(&mut lycus.path)
                    .labelled_by(path_label.id);
            });
            ui.end_row();
        });
}

pub fn crack_ciphertext(ciphertext: &str, timeout: u32) -> (String, String) {
    let config = ares::config::Config {
        timeout,
        ..Default::default()
    };
    let plaintext = perform_cracking(ciphertext, config);
    (
        plaintext
            .as_ref()
            .unwrap()
            .path
            .iter()
            .map(|c| c.decoder)
            .collect::<Vec<_>>()
            .join(" -> "),
        plaintext.unwrap().text[0].clone(),
    )
}
