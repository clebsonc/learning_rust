#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        match &self.content {
            DigitalContent::AudioFile => println!("Listening to audio file."),
            DigitalContent::VideoFile => println!("Watching Video File."),
        }
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) {
        println!("Content duration: {}", self.time);
    }
}

fn main() {
    let cm = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("60 segundos"),
    };
    cm.consume_entertainment();
    cm.retrieve_time();
    println!("---------");

    let cm2 = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("60 segundos"),
    };
    cm2.consume_entertainment();
    cm2.retrieve_time();
    println!("---------");

    let cm3 = ChatMessage {
        content: String::from("text content"),
        time: String::from("10 minutes"),
    };
    cm3.retrieve_time();
    println!("---------");
}
