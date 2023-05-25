use iced::{
    executor, theme,
    widget::{button, column, text},
    Application, Command, Element, Settings, Theme,
};

pub fn main() -> iced::Result {
    IcedApp::run(Settings::default())
}

#[derive(Clone)]
struct IcedApp(i32);

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for IcedApp {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self(0), Command::<Self::Message>::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Self::Message::IncrementPressed => {
                self.0 += 1;
            }
            Self::Message::DecrementPressed => {
                self.0 -= 1;
            }
        }
        Command::<Self::Message>::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let (inc_btn, dec_btn) = (
            button("Increment")
                .on_press(Message::IncrementPressed)
                .style(theme::Button::Positive),
            button("Decrement")
                .on_press(Message::DecrementPressed)
                .style(theme::Button::Destructive),
        );
        column![inc_btn, text(format!("counter: {:?}", self.0)), dec_btn].into()
    }
}
