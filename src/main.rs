pub mod buffer;
pub mod tui;

fn main() {
    tui::Tui::new().run();
}
