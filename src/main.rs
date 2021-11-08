pub mod buffer;
pub mod term;
pub mod term_seq;
pub mod tui;

fn main() {
    tui::Tui::new().run();
}
