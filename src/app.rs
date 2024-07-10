use crate::arm::Arm;
use crate::constants::{DEFAULT_TIMESTEP, INACTIVE_GREY, SCALE_FACTOR};
use crate::joint::MotorType;
use crate::settings::Settings;
use log::debug;
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

pub struct Model {
    egui: Egui,
    arm: Arm,
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
    let arm = Arm::new(pt2(0.0, 0.0), MotorType::NEO550, 32.0, 0.5, 0.2);

    Model {
        egui,
        arm,
        settings: Settings {
            timestep: DEFAULT_TIMESTEP,
            current: 0.0,
            arm_length: 0.2,
        },
    }
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    let elapsed = update.since_last.as_secs_f32();

    // Update the model
    model.arm.set_link(0, model.settings.arm_length).unwrap();

    debug!("Solving with current {}", model.settings.current);
    model.arm.calc(model.settings.current, elapsed);

    // Egui updates
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // ui.label("Timestep");
        ui.add(egui::Slider::new(&mut model.settings.current, -100.0..=100.0).text("Current"));
        ui.add(egui::Slider::new(&mut model.settings.arm_length, 0.1..=0.4).text("Length"));
        // TODO: Add mass slider, add in model.settings.
        ui.add_space(5.0);
        // ui.add(egui::Label::new(format!(
        //     "Angular velocity: {:.2} rad/s",
        //     model.link.joint.v
        // )))
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect(); // Used to find the size of the window
    let draw = app.draw();

    // // Example rectangle
    // let r: Rect = Rect::from_w_h(50.0, 50.0);
    // draw.ellipse()

    // draw.ellipse() // Draw the bounding circle
    //     .xy(model.link.start)
    //     .radius(model.settings.arm_length * SCALE_FACTOR)
    //     .stroke_color(INACTIVE_GREY)
    //     .color(BLACK)
    //     .stroke_weight(2.0);

    // // Draw representation for the arm
    // model.link.draw_trig(&draw); // Draw the debug bounds
    model.arm.draw_links(&draw); // Draw the link

    // draw.ellipse() // Draw the end effector
    //     .xy(model.link.end)
    //     .radius(13.0)
    //     .color(WHITE);

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
