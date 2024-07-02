use crate::constants::{ARM_LENGTH, DEFAULT_TIMESTEP, INACTIVE_GREY};
use crate::link::Link;
use crate::settings::Settings;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

pub struct Model {
    egui: Egui,
    link: Link,
    settings: Settings,
}

pub fn model(app: &App) -> Model {
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
            end: pt2(0.0, ARM_LENGTH),
            angle: 0.0, // Angle in radians
        },
        settings: Settings {
            timestep: DEFAULT_TIMESTEP,
        },
    }
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    let time = app.time;
    let old_angle = model.link.angle;
    model.link.angle += model.settings.timestep;

    let mut pos = pt2(model.link.angle.cos(), model.link.angle.sin());
    pos *= ARM_LENGTH;

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
        .stroke_color(INACTIVE_GREY)
        .color(BLACK)
        .stroke_weight(2.0);

    // Draw representation for the arm
    model.link.draw_trig(&draw); // Draw the debug bounds
    model.link.draw_link(&draw); // Draw the link

    draw.ellipse() // Draw the end effector
        .xy(model.link.end)
        .radius(13.0)
        .color(WHITE);

    let text_bounds = win.pad(60.0);

    draw.text(&(app.fps().round().to_string() + " " + "fps"))
        .color(WHITE)
        .font_size(24)
        .wh(text_bounds.wh())
        .xy(text_bounds.bottom_left());

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();

    // Draw Egui window to frame
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
