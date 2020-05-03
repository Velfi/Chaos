pub mod painter;
pub mod utils;

use nannou::color::RgbHue;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use painter::Painter;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    // ui_window: WindowId,
    art_window: WindowId,
    current_color: Hsv,
    fps_widget: widget::Id,
    mouse_xy: Point2<f32>,
    painter: Painter,
    show_fps: bool,
    ui: Ui,
}

fn model(app: &App) -> Model {
    let art_window = app
        .new_window()
        .title("Chaos")
        .event(event_art)
        .build()
        .unwrap();

    let mut ui = app.new_ui().build().unwrap();
    let fps_widget = ui.generate_widget_id();

    // let _window = app
    //     .new_window()
    //     .with_dimensions(DEFAULT_RESOLUTION_W, DEFAULT_RESOLUTION_H)
    //     .view(view::view)
    //     .mouse_moved(update::mouse_moved)
    //     .mouse_pressed(update::mouse_pressed)
    //     .mouse_released(update::mouse_released)
    //     .key_pressed(update::key_pressed)
    //     .resized(update::resized)
    //     .build()
    //     .unwrap();

    // let ui_window = app
    //     .new_window()
    //     .title("Chaos Controls")
    //     .event(event_ui)
    //     .build()
    //     .unwrap();

    let painter = Painter::default();
    let mouse_xy = pt2(0.0, 0.0);
    let current_color = Hsv::new(0.0, 0.75, 0.60);

    Model {
        // ui_window,
        art_window,
        current_color,
        fps_widget,
        mouse_xy,
        painter,
        show_fps: false,
        ui,
    }
}

fn event_art(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(xy) => model.mouse_xy = xy,
        KeyPressed(key) if key == Key::F => model.show_fps = !model.show_fps,
        _ => println!("Chaos Event: {:?}", event),
    };
}

// fn event_ui(_app: &App, _model: &mut Model, event: WindowEvent) {
//     println!("Chaos Controls Event: {:?}", event);
// }

fn update(app: &App, model: &mut Model, _update: Update) {
    let art_window_rect = app.window(model.art_window).unwrap().rect();

    model
        .painter
        .update(&art_window_rect, &model.mouse_xy, &model.current_color);

    model.current_color.hue = RgbHue::from_degrees(model.current_color.hue.to_degrees() + 1.0);

    let ui = &mut model.ui.set_widgets();
    widget::Text::new(&format!("{:.2}", app.fps()))
        .font_size(24)
        .top_right_with_margin(20.0)
        .set(model.fps_widget, ui);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    match frame.window_id() {
        id if id == model.art_window => {
            // TODO figure out a nice way to achieve ghosting
            draw.background().color(WHITE);

            model.painter.draw(&draw);
        }
        // id if id == model.ui_window => {
        //     draw.background().color(LIGHTGREEN);
        //     draw.tri().color(CORNFLOWERBLUE);
        // }
        _ => (),
    }

    draw.to_frame(app, &frame).unwrap();
    if model.show_fps {
        model.ui.draw_to_frame(app, &frame).unwrap()
    };
}
