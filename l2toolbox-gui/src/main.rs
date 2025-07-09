use iced::widget::{Column, button, text};

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i64,
}
impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
    fn view(&self) -> Column<'_, Message> {
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);
        let counter = text(self.value.to_string());

        let interface = Column::new().push(increment).push(counter).push(decrement);
        interface
    }
}
#[test]
fn it_counts_properly() {
    let mut counter = Counter::default();

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}
