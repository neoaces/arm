use app::{model, update};

pub mod app;
pub mod constants;
pub mod link;
pub mod settings;

fn main() {
    env_logger::init();
    nannou::app(model).update(update).run();
}
