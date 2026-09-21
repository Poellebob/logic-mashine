mod core;
mod editor;

fn main() {
    let mut app = editor::App::init();
    app.run();
}
