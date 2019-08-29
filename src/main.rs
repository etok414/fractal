use nannou::prelude::*;
// use nannou::color::Color;
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

struct Point {
    pos: Vector2,
    color: Rgb,
    forwards: bool,
}

struct Model {
    points: Vec<Point>,
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
    let fractal_type = args[2].to_lowercase();

    app.main_window().set_inner_size_points(720.0, 720.0);

    let mut pos_points = Vec::new();
    match fractal_type.as_str() {
        "dragon" => {
            pos_points.push(Vector2::new(-180.0, 0.0));
            pos_points.push(Vector2::new(180.0, 0.0));
        }
        "semi_dragon" => {
            pos_points.push(Vector2::new(-180.0, 0.0));
            pos_points.push(Vector2::new(180.0, 0.0));
            pos_points.push(Vector2::new(-180.0, 0.0));
        }
        "snowflake" => {
            pos_points.push(Vector2::new(-180.0, -100.0));
            pos_points.push(Vector2::new(180.0, -100.0));
            let snowflake_vector1 =  pos_points[1] - pos_points[0];
            let snowflake_vector2 = Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());
            pos_points.insert(1, pos_points[0]+snowflake_vector2);
            pos_points.push(pos_points[0]);
        }
        "semi_snowflake" => {
            pos_points.push(Vector2::new(-180.0, 0.0));
            pos_points.push(Vector2::new(180.0, 0.0));
            pos_points.push(Vector2::new(-180.0, 0.0));
        }
        "cross" => {
            pos_points.push(Vector2::new(-180.0, -180.0));
            pos_points.push(Vector2::new(-180.0, 180.0));
            pos_points.push(Vector2::new(180.0, 180.0));
            pos_points.push(Vector2::new(180.0, -180.0));
            pos_points.push(Vector2::new(-180.0, -180.0));
        }
        "semi_cross" => {
            pos_points.push(Vector2::new(-100.0, -100.0));
            pos_points.push(Vector2::new(-100.0, 100.0));
            pos_points.push(Vector2::new(100.0, 100.0));
            pos_points.push(Vector2::new(100.0, -100.0));
            pos_points.push(Vector2::new(-100.0, -100.0));
            // pos_points.push(Vector2::new(-180.0, 0.0));
            // pos_points.push(Vector2::new(180.0, 0.0));
            // pos_points.push(Vector2::new(-180.0, 0.0));
        }
        "flower" => {
            pos_points.push(Vector2::new(-180.0, -100.0));
            pos_points.push(Vector2::new(180.0, -100.0));
            let snowflake_vector1 =  pos_points[1] - pos_points[0];
            let snowflake_vector2 = Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());
            pos_points.insert(1, pos_points[0]+snowflake_vector2);
            pos_points.push(pos_points[0]);
        }
        "gosper" => {
            pos_points.push(Vector2::new(-180.0, 0.0));
            pos_points.push(Vector2::new(180.0, 0.0));
        }
        _ => {
            pos_points.push(Vector2::new(-180.0, 0.0));
            pos_points.push(Vector2::new(180.0, 0.0));
            pos_points.push(Vector2::new(-180.0, 0.0));
        }
    }
    let mut points = Vec::new();
    for point_pos in pos_points {
        let new_point = Point {
            pos: point_pos,
            color: Rgb::new(1.0, 1.0, 1.0),
            forwards: true,
        };
        points.push(new_point);
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
    // let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
    let current_color = Rgb::new(1.0, 1.0, 1.0);

    match &model.fractal_type.as_str() {

        &"dragon" => {
            if model.points[model.active_spot].forwards {
                let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
                let dragon_vector = between_vector / 2.0 + Vector2::new(-between_vector.y, between_vector.x) / 2.0;
                model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + dragon_vector, color:current_color, forwards:false,});
                model.active_spot += 2;
            } else {
                let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
                let dragon_vector = between_vector / 2.0 + Vector2::new(between_vector.y, -between_vector.x) / 2.0;
                model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + dragon_vector, color:current_color, forwards:false,});
                model.points[model.active_spot].forwards = true;
                model.active_spot += 2;
            }
        },

        &"semi_dragon" => {
            let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            let dragon_vector = between_vector / 2.0 + Vector2::new(-between_vector.y, between_vector.x) / 2.0;
            model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + dragon_vector, color:current_color, forwards:true,});
            model.active_spot += 2;
        },

        &"snowflake" => {
            // Snowflake
            let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            let snowflake_vector1 = between_vector / 3.0;
            let snowflake_vector2 = snowflake_vector1 +
                                    Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());
            let snowflake_vector3 = between_vector * 2.0 / 3.0;

            model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + snowflake_vector1, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot].pos + snowflake_vector2, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 3, Point {pos: model.points[model.active_spot].pos + snowflake_vector3, color:current_color, forwards:true,});
            model.active_spot += 4;
        },

        &"semi_snowflake" => {
            // Semi-Snowflake
            let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            let snowflake_vector1 = between_vector / 2.0;
            let snowflake_vector2 = snowflake_vector1 +
                                    Vector2::new(snowflake_vector1.x * (PI/3.0).cos() -
                                                 snowflake_vector1.y * (PI/3.0).sin(),
                                                 snowflake_vector1.x * (PI/3.0).sin() +
                                                 snowflake_vector1.y * (PI/3.0).cos());

            model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + snowflake_vector1, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot].pos + snowflake_vector2, color:current_color, forwards:true,});
            model.active_spot += 3;
        },

        // if model.active_spot + 1 == model.points.len() {
        //     model.active_spot = 0;
        //     model.counter -= 1;
        //     if model.counter <= 0 {
        //         return;
        //     }
        // }

        &"cross" => {
            // Cross
            let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            let cross_vector = between_vector / 3.0;
            let cross_vector_perp = Vector2::new(-cross_vector.y, cross_vector.x);

            model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + cross_vector, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot].pos + cross_vector + cross_vector_perp, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 3, Point {pos: model.points[model.active_spot].pos + cross_vector + cross_vector_perp + cross_vector, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 4, Point {pos: model.points[model.active_spot].pos + cross_vector + cross_vector, color:current_color, forwards:true,});
            model.active_spot += 5;
        }

        &"semi_cross" => {
            // Cross
            let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            let cross_vector = between_vector / 2.0;
            let cross_vector_perp = Vector2::new(-cross_vector.y, cross_vector.x);

            model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + cross_vector, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot].pos + cross_vector + cross_vector_perp, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 3, Point {pos: model.points[model.active_spot].pos + cross_vector + cross_vector + cross_vector_perp, color:current_color, forwards:true,});
            model.active_spot += 4;
        }

        &"flower" => {
            // Flower
            let between_vector =  model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            let flower_vector = between_vector * (2.0.sqrt() - 1.0);
            let flower_vector_turned = Vector2::new(flower_vector.x - flower_vector.y,
                                                    flower_vector.x+ flower_vector.y)
                                                    * (PI/4.0).sin();
            model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + flower_vector_turned, color:current_color, forwards:true,});
            model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot].pos + flower_vector_turned + flower_vector, color:current_color, forwards:true,});
            model.active_spot += 3;
        },

        &"gosper" => {
            // Cross
            let mut between_vector = model.points[model.active_spot + 1].pos - model.points[model.active_spot].pos;
            if !model.points[model.active_spot].forwards {
                between_vector *= -1.0;
            }

            let gosper_vector = between_vector / 7.0.sqrt();
            let first_angle = PI/6.0 - (1.0/(3.0*3.0.sqrt())).atan();
            let gosper_vector1 = Vector2::new(gosper_vector.x * (-first_angle).cos() -
                                              gosper_vector.y * (-first_angle).sin(),
                                              gosper_vector.x * (-first_angle).sin() +
                                              gosper_vector.y * (-first_angle).cos());

            let gosper_vector2 = gosper_vector1 + Vector2::new(gosper_vector1.x * (PI/3.0).cos() -
                                                               gosper_vector1.y * (PI/3.0).sin(),
                                                               gosper_vector1.x * (PI/3.0).sin() +
                                                               gosper_vector1.y * (PI/3.0).cos());

            let gosper_vector3 = gosper_vector2 - gosper_vector1;

            let gosper_vector4 = gosper_vector3 + Vector2::new(gosper_vector3.x * (PI/3.0).cos() -
                                                               gosper_vector3.y * (PI/3.0).sin(),
                                                               gosper_vector3.x * (PI/3.0).sin() +
                                                               gosper_vector3.y * (PI/3.0).cos());

            let gosper_vector5 = gosper_vector3 * 2.0;

            let gosper_vector6 = gosper_vector5 + gosper_vector1;


            if model.points[model.active_spot].forwards {
                model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot].pos + gosper_vector1, color:current_color, forwards:!model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot].pos + gosper_vector2, color:current_color, forwards:!model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 3, Point {pos: model.points[model.active_spot].pos + gosper_vector3, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 4, Point {pos: model.points[model.active_spot].pos + gosper_vector4, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 5, Point {pos: model.points[model.active_spot].pos + gosper_vector5, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 6, Point {pos: model.points[model.active_spot].pos + gosper_vector6, color:current_color, forwards:!model.points[model.active_spot].forwards,});
            } else {
                model.points.insert(model.active_spot + 1, Point {pos: model.points[model.active_spot + 1].pos + gosper_vector6, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 2, Point {pos: model.points[model.active_spot + 2].pos + gosper_vector5, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 3, Point {pos: model.points[model.active_spot + 3].pos + gosper_vector4, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 4, Point {pos: model.points[model.active_spot + 4].pos + gosper_vector3, color:current_color, forwards:!model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 5, Point {pos: model.points[model.active_spot + 5].pos + gosper_vector2, color:current_color, forwards:!model.points[model.active_spot].forwards,});
                model.points.insert(model.active_spot + 6, Point {pos: model.points[model.active_spot + 6].pos + gosper_vector1, color:current_color, forwards:model.points[model.active_spot].forwards,});
                model.points[model.active_spot].forwards = !model.points[model.active_spot].forwards;
            }
            model.active_spot += 7;
        }
        _ => panic!("Invalid fractal type")
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for n in 0..model.points.len()-1 {
        // draw.line().start(model.points[n].pos).end(model.points[n+1].pos).color(if model.points[n].forwards {model.points[n].color} else {Rgb::new(1.0, 0.0, 0.0)});
        draw.line().start(model.points[n].pos).end(model.points[n+1].pos).color(model.points[n].color);
        // draw.ellipse().x_y(model.points[n].x, model.points[n].y).radius(3.0).color(INDIANRED);
    }
    draw.ellipse().x_y(model.points[model.active_spot].pos.x, model.points[model.active_spot].pos.y).radius(1.0).color(INDIANRED);

    draw.to_frame(app, &frame).unwrap();
}
