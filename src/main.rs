//editor
//2026_11_01

use eframe::egui;


fn main() -> eframe::Result {
    // mutable text
    let mut text = String::new();

    // frame
    eframe::run_ui_native(
        "Editor", // window name
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            // get the window size
            let available = ui.available_size();
            // add ui text field
            ui.add_sized(
                available, // max size
                egui::TextEdit::multiline(&mut text)
                    .frame(
                        egui::Frame::new()
                            .corner_radius(egui::CornerRadius::same(7))
                            .inner_margin(egui::Margin::same(8))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
                    ),
            );
        },
    )
}

