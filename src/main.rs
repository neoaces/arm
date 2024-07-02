use app::{model, update};

pub mod app;
pub mod constants;
pub mod joint;
pub mod link;
pub mod motor;
pub mod settings;
pub mod utils;

fn main() {
    env_logger::init();
    nannou::app(model).update(update).run();
}
