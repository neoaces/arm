pub mod constants;
pub mod link;
pub mod settings;

use crate::link::Link;
use crate::settings::Settings;
use constants::ARM_LENGTH;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

fn main() {
    env_logger::init();

    nannou::app(model)
        .update(update)
        // .simple_window(view)
        .run();
}

struct Model {
    egui: Egui,
    link: Link,
    settings: Settings,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        link: Link {
            start: pt2(0.0, 0.0),
            end: pt2(0.0, constants::ARM_LENGTH),
            angle: 0.0, // Angle in radians
        },
        settings: Settings { timestep: 0.05 },
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let time = app.time;
    let old_angle = model.link.angle;
    model.link.angle += model.settings.timestep;

    let mut pos = pt2(model.link.angle.cos(), model.link.angle.sin());
    pos *= constants::ARM_LENGTH;

    model.link.end = pos;

    dbg!("{:#?}", &model.link);

    // Egui updates
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // ui.label("Timestep");
        ui.add(egui::Slider::new(&mut model.settings.timestep, 0.0..=0.05).text("Speed"));
        ui.add_space(5.0);
        ui.add(egui::Label::new(format!(
            "{:.2} rad/s",
            (model.link.angle - old_angle) / update.since_last.as_secs_f32()
        )))
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect(); // Used to find the size of the window
    let draw = app.draw();

    // // Example rectangle
    // let r: Rect = Rect::from_w_h(50.0, 50.0);
    // draw.ellipse()

    draw.ellipse() // Draw the bounding circle
        .xy(model.link.start)
        .radius(ARM_LENGTH)
        .stroke_color(WHITE)
        .color(BLACK)
        .stroke_weight(2.0);

    // Draw representation for the arm
    model.link.draw_link(&draw); // Draw the link
    model.link.draw_trig(&draw); // Draw the debug bounds

    draw.ellipse() // Draw the end effector
        .xy(model.link.end)
        .radius(10.0)
        .color(WHITE);

    draw.text(&(app.fps().round().to_string() + " " + "fps"))
        .color(WHITE)
        .font_size(24)
        .wh(win.pad(15.0).wh())
        .align_text_bottom();

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();

    // Draw Egui window to frame
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
