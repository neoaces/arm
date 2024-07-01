pub mod link;
pub mod constants;

use crate::link::Link;
use nannou::prelude::*;

fn main() {
    env_logger::init();

    nannou::app(model)
    .update(update)
    .simple_window(view)
    .run();
}

struct Model {
    link: Link,
}

fn model(_app: &App) -> Model {
    Model {
        link: Link {
            start: pt2(0.0, 0.0),
            end: pt2(0.0, constants::ARM_LENGTH),
            angle: 0.0 // Angle in radians
        } 
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // let time = app.time;
    model.link.angle = clamp(model.link.angle + 0.05, 0.0, 2.0 * PI);
    
    let mut pos = pt2(model.link.angle.cos(), model.link.angle.sin());
    pos *= constants::ARM_LENGTH;

    model.link.end = pos;

    dbg!("{:#?}", &model.link);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect(); // Used to find the size of the window
    let draw = app.draw();

    // // Example rectangle
    // let r: Rect = Rect::from_w_h(50.0, 50.0);
    // draw.ellipse()

    // Draw representation for the arm
    model.link.draw_link(&draw); // Draw the link
    draw.ellipse() // Draw the end effector
        .xy(model.link.end)
        .radius(10.0)
        .color(WHITE);

    draw.background().color(BLUE);
    draw.to_frame(app, &frame).unwrap();
}