mod app;


fn main() {
    let _ = eframe::run_native(
        "Tetris",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app::TetrisApp::new())),
    );
}