extern crate nannou;
use nannou::{color::encoding, prelude::*};

struct Slices(Vec<(String, rgb::Rgb<encoding::Srgb, u8>)>);
impl Slices {
    fn slice_width(&self) -> usize {
        360 / self.0.len() + (360 % self.0.len()) / self.0.len()
    }
}
fn main() {
    nannou::app(model).update(update).run();
}

enum ClickState {
    Clicked(Vec2),
    Released,
}

struct Model {
    rotation: f32,
    start_time: f32,
    angle_stopped: f32,
    running: bool,
    click_state: ClickState,
    slices: Slices,
    momentum: f32,
}
impl Model {
    fn stop(&mut self) {
        self.angle_stopped = self.rotation;
        self.running = false;
    }

    fn start(&mut self, app: &App) {
        self.start_time = app.time;
        self.running = true;
    }

    fn current_winner_slice(&self) -> (String, rgb::Rgb<encoding::Srgb, u8>) {
        let rot = rad_to_deg(self.rotation) % 360.0;
        self.slices.0[rot.trunc() as usize / self.slices.slice_width()].clone()
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .unwrap();
    Model {
        start_time: 0.0,
        rotation: 0.0,
        angle_stopped: 0.0,
        running: false,
        click_state: ClickState::Released,
        momentum: 0.0,

        slices: Slices(vec![
            ("foo".to_owned(), STEELBLUE),
            ("bar".to_owned(), RED),
            ("foo".to_owned(), STEELBLUE),
            ("baz".to_owned(), GREEN),
            ("bar".to_owned(), RED),
        ]),
    }
}

fn mouse_released(app: &App, model: &mut Model, _button: MouseButton) {
    match model.click_state {
        ClickState::Clicked(pos) => {
            model.momentum = pos.distance(app.mouse.position()) / 5.0;
            model.click_state = ClickState::Released;
            model.start(app);
        }
        ClickState::Released => {}
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    match model.click_state {
        ClickState::Clicked(_) => {}
        ClickState::Released => {
            model.click_state = ClickState::Clicked(app.mouse.position());
            model.stop();
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.rotation = if model.running {
        let speed = decelerate(app.time - model.start_time, model.momentum) / 100.0;
        let current_angle = model.rotation + speed;
        if speed < 0.001 {
            model.stop();
            model.angle_stopped
        } else {
            current_angle
        }
    } else {
        model.angle_stopped
    };
}

fn decelerate(time: f32, momentum: f32) -> f32 {
    momentum - time
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    let (winner, winner_color) = model.current_winner_slice();
    let text = format!("Winner: {winner}");
    draw.text(&text).color(winner_color).x_y(0.0, 350.0);
    let mut draw = draw.rotate(model.rotation);
    draw_wheel(&mut draw, &model.slices);
    draw.to_frame(app, &frame)
        .expect("To be able to draw to frame");
}

fn draw_wheel(draw: &mut Draw, slices: &Slices) {
    let slice_width = slices.slice_width();
    for (i, slice) in slices.0.iter().enumerate() {
        let start_deg = i * slice_width;
        let end_deg = if i == slices.0.len() - 1 {
            360
        } else {
            start_deg + slice_width
        };
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
