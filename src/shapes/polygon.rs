use iced::{Point, canvas::{Path, path}};

pub fn new (center: Point, radius: f32, points: u32) -> Path {
    
    let mut angle: f32 = (0.0 as f32).to_radians();
    let increment = (360.0 / points as f32).to_radians();
    let mut points: Vec<Point> = Vec::new();

    let first_point = Point::new(
        center.x,
        center.y - radius
    );

    while angle < 6.283185 - increment {
        angle += increment;

        let point = Point::new(
            center.x + radius * angle.sin(),
            center.y - radius * angle.cos()
        );
        points.push(point);
    };

    let mut builder = path::Builder::new();

    builder.move_to(first_point);

    for point in points.into_iter() {
        builder.line_to(point);
    };

    builder.close();
    builder.build()
}