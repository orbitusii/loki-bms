use iced::{
    canvas::{self, Cursor, Frame, Geometry, Path, Stroke, Text},
    Color, HorizontalAlignment, Point, Rectangle, Size, Vector, VerticalAlignment,
};

use crate::shapes;

#[derive(Default)]
pub struct ScopeState {
    pub some_state_value: f32,
    background_cache: canvas::Cache,
    midground_cache: canvas::Cache,
    foreground_cache: canvas::Cache,
}

impl ScopeState {
    pub fn new() -> ScopeState {
        ScopeState {
            some_state_value: 9.0,
            ..ScopeState::default()
        }
    }

    pub fn update(&mut self) {
        self.midground_cache.clear();
        self.foreground_cache.clear();
    }
}

impl<Message> canvas::Program<Message> for ScopeState {
    fn draw(&self, bounds: Rectangle, _: Cursor) -> Vec<Geometry> {
        let background = self.background_cache.draw(bounds.size(), |frame| {
            let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());

            frame.fill(&space, Color::BLACK);
        });

        let midground = self.midground_cache.draw(bounds.size(), |frame| {
            let radius = 3.0 + (self.some_state_value * 0.12);

            let circle = Path::circle(frame.center(), radius);
            let offsetpoint = frame.center() + Vector::new(10.0, 0.0);
            let pentagon = shapes::polygon::new(offsetpoint, radius, 4);

            let str_blue = Stroke {
                color: Color::from_rgb(0.0, 0.0, 1.0),
                width: 2.0,
                ..Stroke::default()
            };
            let str_red = Stroke {
                color: Color::from_rgb(1.0, 0.0, 0.0),
                width: 2.0,
                ..Stroke::default()
            };

            frame.fill(&circle, Color::from_rgba(0.0, 0.0, 1.0, 0.33));
            frame.stroke(&circle, str_blue);

            frame.fill(&&pentagon, Color::from_rgba(1.0, 0.0, 0.0, 0.33));
            frame.stroke(&pentagon, str_red);
        });

        let foreground = self.foreground_cache.draw(bounds.size(), |frame| {
            let text = Text {
                content: self.some_state_value.to_string(),
                position: Point::new(5.0, 5.0),
                color: Color::WHITE,
                size: 9.0,
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Top,
                ..Text::default()
            };

            frame.fill_text(text);
        });

        vec![background, midground, foreground]
    }
}
