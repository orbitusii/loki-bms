use iced::{
    Color, HorizontalAlignment, Point, Rectangle, Size, VerticalAlignment, 
    canvas::{
        self, Cursor, Frame, Geometry, Path, Stroke, Text
    }
};

#[derive(Default)]
pub struct ScopeState {
    pub some_state_value: f32,
    background_cache: canvas::Cache,
    midground_cache: canvas::Cache,
    foreground_cache: canvas::Cache,
}

impl ScopeState {
    pub fn new () -> ScopeState {
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
    fn draw(
        &self,
        bounds: Rectangle,
        _: Cursor
    ) -> Vec<Geometry> {
        
        let background = self.background_cache.draw(bounds.size(), |frame| {
            let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());

            frame.fill(&space, Color::BLACK);
        });

        let midground = self.midground_cache.draw(bounds.size(), |frame| {
            let circle = Path::circle(
                frame.center(),
                3.0 + (self.some_state_value * 0.12)
            );
            let stroke = Stroke {
                color: Color::from_rgb(0.0, 0.0, 1.0),
                width: 2.0,
                ..Stroke::default()
            };

            frame.fill(&circle, Color::from_rgb(0.0, 0.0, 0.2));
            frame.stroke(&circle, stroke);
        });

        let foreground = self.foreground_cache.draw(bounds.size(), |frame| {
            let text = Text {
                content : self.some_state_value.to_string(),
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