use iced::{
    canvas, slider, widget, Canvas, Column, Container, Element, Length, Row, Sandbox, Slider, Text,
};

mod scope_canvas;
use scope_canvas::ScopeState;

#[derive(Default)]
pub struct MainScope {
    slider: slider::State,
    state: ScopeState,
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(f32),
}

impl Sandbox for MainScope {
    type Message = Message;

    fn new() -> Self {
        println!("Launching Main Scope...");
        MainScope::default()
    }

    fn title(&self) -> String {
        String::from("LOKI - Main Scope")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(value) => {
                self.state.some_state_value = value;
                println!("some_value updated to {}", self.state.some_state_value);
            }
        }

        self.state.update();
    }

    fn view(&mut self) -> Element<Message> {
        let slider = Slider::new(
            &mut self.slider,
            0.0..=100.0,
            self.state.some_state_value,
            Message::SliderChanged,
        );

        let text = Text::new("Hello world!");
        let text2 = Text::new(self.state.some_state_value.to_string());

        let canvas: Canvas<Message, &mut ScopeState> = Canvas::new(&mut self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        let left_column = Column::new()
            .spacing(20)
            .padding(20)
            .width(Length::FillPortion(1))
            .max_width(100)
            .push(slider)
            .push(text)
            .push(text2);

        let scope_column = Column::new()
            .width(Length::FillPortion(4))
            .push(Text::new("Scope Area"))
            .push(canvas)
            .push(widget::Rule::horizontal(5))
            .push(Text::new("Lower info area"))
            .push(widget::Space::new(Length::Fill, Length::Units(200)));

        let right_column = Column::new()
            .spacing(20)
            .padding(20)
            .width(Length::FillPortion(1))
            .max_width(100)
            .push(Text::new("Right column"));

        let vertsplitters = Row::new()
            .push(left_column)
            .push(widget::Rule::vertical(5))
            .push(scope_column)
            .push(widget::Rule::vertical(5))
            .push(right_column);

        Container::new(vertsplitters)
            .width(Length::Fill)
            .height(Length::Fill)
            //.center_x()
            //.center_y()
            .into()
    }
}
