extern crate nannou;
use nannou::{color::encoding, prelude::*};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    start_time: f32,
    angle: f32,
    running: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    Model {
        start_time: 0.0,
        angle: 0.0,
        running: true,
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    if model.running {
        model.angle = get_current_angle(app, model);
        model.running = false;
    } else {
        model.start_time = app.time;
        model.running = true;
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let rotation = get_current_angle(app, model);
    let draw = app.draw();

    let mut draw = draw.rotate(rotation);
    draw.background().color(BLACK);
    draw_wheel(&mut draw);
    draw.to_frame(app, &frame)
        .expect("To be able to draw to frame");
}

fn draw_wheel(draw: &mut Draw) {
    let slices: Vec<(&str, rgb::Rgb<encoding::Srgb, u8>)> = vec![
        ("foo", STEELBLUE),
        ("bar", RED),
        ("baz", GREEN),
        ("foo", STEELBLUE),
        ("bar", RED),
        ("bar", RED),
        ("baz", GREEN),
        ("foo", STEELBLUE),
        ("bar", RED),
        ("baz", GREEN),
    ];

    for (i, slice) in slices.iter().enumerate() {
        let slice_width = 360 / slices.len();
        let start_deg = i * slice_width;
        let end_deg = start_deg + slice_width;
        let radius = 300.0;

        let (text, color) = slice;

        draw.polygon()
            .points(make_cake_slice(start_deg, end_deg, radius))
            .color(*color);
        draw.text(text)
            .color(WHITE)
            .x_y(
                deg_to_rad(middle_between(start_deg, end_deg)).sin() * (radius / 2.0),
                deg_to_rad(middle_between(start_deg, end_deg)).cos() * (radius / 2.0),
            )
            .rotate(-deg_to_rad(middle_between(start_deg, end_deg) - 90.0));
    }
}

fn middle_between(a: usize, b: usize) -> f32 {
    let a = a as f32;
    let b = b as f32;
    a + ((b - a) / 2.0)
}
fn get_current_angle(app: &App, model: &Model) -> f32 {
    if model.running {
        decelerate(app.time - model.start_time)
    } else {
        model.angle
    }
}

fn decelerate(time: f32) -> f32 {
    (1.0 / time) * 10.0
}

fn make_cake_slice(from_deg: usize, to_deg: usize, radius: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = (from_deg..=to_deg)
        .map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            pt2(x, y)
        })
        .collect();
    let mut res = vec![pt2(0.0, 0.0)];
    res.append(&mut points);
    res.push(pt2(0.0, 0.0));
    res
}
