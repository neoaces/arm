use crate::constants::{DEFAULT_TIMESTEP, INACTIVE_GREY, SCALE_FACTOR};
use crate::joint::{Joint, MotorType};
use crate::link::Link;
use crate::settings::Settings;
use crate::utils::rk4::solve_rk4;
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
            m: 0.05, // 50 kg
            l: 0.2,  // 20cm - used as ARM_LENGTH
            start: pt2(0.0, 0.0),
            end: pt2(0.2 * SCALE_FACTOR, 0.0),
            angle: 0.0, // Angle in radians
            joint: Joint::new(MotorType::NEO550, 32.0),
        },
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
    model.link.l = model.settings.arm_length;

    model.link.joint.v = solve_rk4(
        model.link.joint.v,
        model.settings.current,
        elapsed,
        model.link.alpha(),
    ) * elapsed;

    model.link.angle += model.link.joint.v * elapsed;

    model.link.end = pt2(
        model.settings.arm_length * SCALE_FACTOR * model.link.angle.cos(),
        model.settings.arm_length * SCALE_FACTOR * model.link.angle.sin(),
    );

    dbg!("{:?}", &model.link);

    // Egui updates
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // ui.label("Timestep");
        ui.add(egui::Slider::new(&mut model.settings.current, -100.0..=100.0).text("Current"));
        ui.add(egui::Slider::new(&mut model.settings.arm_length, 0.1..=0.4).text("Length"));
        ui.add_space(5.0);
        ui.add(egui::Label::new(format!(
            "Angular velocity: {:.2} rad/s",
            model.link.joint.v
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
        .radius(model.settings.arm_length * SCALE_FACTOR)
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
