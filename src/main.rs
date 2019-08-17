use nannou::prelude::*;
use std::io::Write;

const ERROR_MESSAGE: &'static str = r"
This program draws a fractal.
As input it needs the number of iterations, and the type of fractal.
It currently supports Dragon, Semi_Dragon, Snowflake, Semi_Snowflake, Cross, Semi_Cross, and Flower
EX: fractal #_of_iterations fractal_type
EX: fractal 10 Snowflake
";

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
    fractal_type: String,
}


fn model(app: &App) -> Model {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        writeln!(std::io::stderr(), "Wrong number of parameters").unwrap();
        println!("{:?}", args);

        writeln!(std::io::stderr(), "{:?}", ERROR_MESSAGE).unwrap();
        std::process::exit(1);
    }
    let counter: usize = args[1].parse().unwrap();
    let fractal_type = args[2].clone();

    app.main_window().set_inner_size_points(720.0, 720.0);

    let mut points = Vec::new();
    match fractal_type.as_str() {
        "Dragon" => {
            points.push(Vector2::new(-180.0, 0.0));
            points.push(Vector2::new(180.0, 0.0));
        }
        "Semi_Dragon" => {
            points.push(Vector2::new(-180.0, 0.0));
            points.push(Vector2::new(180.0, 0.0));
        }
        "Snowflake" => {
            points.push(Vector2::new(-180.0, -100.0));
            points.push(Vector2::new(180.0, -100.0));
            let snowflake_vector1 =  points[1] - points[0];
            let snowflake_vector2 = Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());
            points.insert(1, points[0]+snowflake_vector2);
            points.push(points[0]);
        }
        "Semi_Snowflake" => {
            points.push(Vector2::new(-180.0, 0.0));
            points.push(Vector2::new(180.0, 0.0));
            points.push(Vector2::new(-180.0, 0.0));
        }
        "Cross" => {
            points.push(Vector2::new(-180.0, -180.0));
            points.push(Vector2::new(-180.0, 180.0));
            points.push(Vector2::new(180.0, 180.0));
            points.push(Vector2::new(180.0, -180.0));
            points.push(Vector2::new(-180.0, -180.0));
        }
        "Semi_Cross" => {
            points.push(Vector2::new(-100.0, -100.0));
            points.push(Vector2::new(-100.0, 100.0));
            points.push(Vector2::new(100.0, 100.0));
            points.push(Vector2::new(100.0, -100.0));
            points.push(Vector2::new(-100.0, -100.0));
            // points.push(Vector2::new(-180.0, 0.0));
            // points.push(Vector2::new(180.0, 0.0));
            // points.push(Vector2::new(-180.0, 0.0));
        }
        "Flower" => {
            points.push(Vector2::new(-180.0, -100.0));
            points.push(Vector2::new(180.0, -100.0));
            let snowflake_vector1 =  points[1] - points[0];
            let snowflake_vector2 = Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());
            points.insert(1, points[0]+snowflake_vector2);
            points.push(points[0]);
        }
        _ => {
            points.push(Vector2::new(-180.0, 0.0));
            points.push(Vector2::new(180.0, 0.0));
            points.push(Vector2::new(-180.0, 0.0));
        }
    }
    let active_spot = 0;
    Model {
        points,
        active_spot,
        counter,
        fractal_type,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.counter <= 0 {
        return;
    }

    if model.active_spot + 1 == model.points.len() {
        model.active_spot = 0;
        model.counter -= 1;
        if model.counter <= 0 {
            return;
        }
    }
    // let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];

    match &model.fractal_type.as_str() {

        &"Dragon" => {
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let dragon_vector = between_vector / 2.0 + Vector2::new(-between_vector.y, between_vector.x) / 2.0;
            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + dragon_vector);
            model.active_spot += 2;

        if model.active_spot + 1 == model.points.len() {
            model.active_spot = 0;
            model.counter -= 1;
            if model.counter <= 0 {
                return;
            }
        }
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let dragon_vector = between_vector / 2.0 + Vector2::new(between_vector.y, -between_vector.x) / 2.0;
            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + dragon_vector);
            model.active_spot += 2;
        },

        &"Semi_Dragon" => {
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let dragon_vector = between_vector / 2.0 + Vector2::new(-between_vector.y, between_vector.x) / 2.0;
            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + dragon_vector);
            model.active_spot += 2;
        },

        &"Snowflake" => {
            // Snowflake
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let snowflake_vector1 = between_vector / 3.0;
            let snowflake_vector2 = snowflake_vector1 +
                                    Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());
            let snowflake_vector3 = between_vector * 2.0 / 3.0;

            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + snowflake_vector1);
            model.points.insert(model.active_spot + 2, model.points[model.active_spot] + snowflake_vector2);
            model.points.insert(model.active_spot + 3, model.points[model.active_spot] + snowflake_vector3);
            model.active_spot += 4;
        },

        &"Semi_Snowflake" => {
            // Semi-Snowflake
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let snowflake_vector1 = between_vector / 2.0;
            let snowflake_vector2 = snowflake_vector1 +
                                    Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());

            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + snowflake_vector1);
            model.points.insert(model.active_spot + 2, model.points[model.active_spot] + snowflake_vector2);
            model.active_spot += 3;
        },

        // if model.active_spot + 1 == model.points.len() {
        //     model.active_spot = 0;
        //     model.counter -= 1;
        //     if model.counter <= 0 {
        //         return;
        //     }
        // }

        &"Cross" => {
            // Cross
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let cross_vector = between_vector / 3.0;
            let cross_vector_perp = Vector2::new(-cross_vector.y, cross_vector.x);

            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + cross_vector);
            model.points.insert(model.active_spot + 2, model.points[model.active_spot] + cross_vector + cross_vector_perp);
            model.points.insert(model.active_spot + 3, model.points[model.active_spot] + cross_vector + cross_vector_perp + cross_vector);
            model.points.insert(model.active_spot + 4, model.points[model.active_spot] + cross_vector + cross_vector);
            model.active_spot += 5;
        }

        &"Semi_Cross" => {
            // Cross
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let cross_vector = between_vector / 2.0;
            let cross_vector_perp = Vector2::new(-cross_vector.y, cross_vector.x);

            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + cross_vector);
            model.points.insert(model.active_spot + 2, model.points[model.active_spot] + cross_vector + cross_vector_perp);
            model.points.insert(model.active_spot + 3, model.points[model.active_spot] + cross_vector + cross_vector + cross_vector_perp);
            model.active_spot += 4;
        }

        &"Flower" => {
            // Flower
            let between_vector =  model.points[model.active_spot + 1] - model.points[model.active_spot];
            let flower_vector = between_vector * (2.0.sqrt() - 1.0);
            let flower_vector_turned = Vector2::new(flower_vector.x - flower_vector.y,
                                                    flower_vector.x+ flower_vector.y)
                                                    * (PI/4.0).sin();
            model.points.insert(model.active_spot + 1, model.points[model.active_spot] + flower_vector_turned);
            model.points.insert(model.active_spot + 2, model.points[model.active_spot] + flower_vector_turned + flower_vector);
            model.active_spot += 3;
        },
        _ => panic!("Invalid fractal type")
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for n in 0..model.points.len()-1 {
        draw.line().start(model.points[n]).end(model.points[n+1]).color(WHITE);
        // draw.ellipse().x_y(model.points[n].x, model.points[n].y).radius(3.0).color(INDIANRED);
    }
    draw.ellipse().x_y(model.points[model.active_spot].x, model.points[model.active_spot].y).radius(1.0).color(INDIANRED);

    draw.to_frame(app, &frame).unwrap();
}
