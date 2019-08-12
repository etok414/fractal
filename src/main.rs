use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    points: Vec<Vector2>,
    active_spot: usize,
    counter: usize,
}


fn model(app: &App) -> Model {
    app.main_window().set_inner_size_points(720.0, 720.0);
    let points = vec![Vector2::new(-360.0, 0.0), Vector2::new(360.0, 0.0)];
    let active_spot = 0;
    let counter = 0;
    Model {
        points,
        active_spot,
        counter,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.counter >= 8 {
        return;
    } else {
        model.counter += 1;
    }
    if model.active_spot + 1 == model.points.len() {
        model.active_spot = 0;
    }
    let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];

    // Dragon
    let mut dragon_vector = Vector2::new(between_vector.x - between_vector.y,
                                         between_vector.x + between_vector.y);
    dragon_vector *= (PI/2.0).sin();
    dragon_vector *= 2.0.sqrt()/2.0;
    dragon_vector += model.points[model.active_spot];
    model.points.insert(model.active_spot, dragon_vector);
    model.active_spot += 2;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK); // Comment this out to activate tracks.
    for n in 0..model.points.len()-1 {
        draw.line().start(model.points[n]).end(model.points[n+1]).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}
