#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
#![allow(dead_code)]
mod editor;
use editor::Editor;

fn main() {
    Editor::new().run();
}
