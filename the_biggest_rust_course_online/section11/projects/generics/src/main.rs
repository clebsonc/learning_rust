use std::task::Context;

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        match self.content {
            DigitalContent::AudioFile => println!("Listening to Audio File"),
            DigitalContent::VideoFile => println!("Watching Video File"),
        };
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let cm = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("3 hours"),
    };
    let cm2 = ChatMessage {
        content: "To be determined",
        time: String::from("30 minutes"),
    };
    let cm3 = ChatMessage {
        content: String::from("travelling around"),
        time: String::from("30 minutes"),
    };

    cm.consume_entertainment();

    println!("{}", cm.retrieve_time());
    println!("{}", cm2.retrieve_time());
    println!("{}", cm3.retrieve_time());
}
