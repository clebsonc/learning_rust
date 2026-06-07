#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name: name,
            contents: Vec::new(),
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name: name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&mut self, index: usize) -> Option<&File> {
        if index >= 0 && index < self.contents.len() {
            return self.contents.get(index);
        }
        None
    }
}

fn main() {
    let mut folder = Folder::new(String::from("learning"));
    folder.create_file(String::from("games"));
    folder.create_file(String::from("musics"));
    folder.create_file(String::from("videos"));
    folder.create_file(String::from("pictures"));
    println!("{:?}", folder);
    let a = folder.get_file(2);
    match a {
        Some(b) => println!("{:#?}", b),
        _ => (),
    }
    println!("{:?}", folder);
    println!("{:?}", folder.delete_file(1));
    println!("{:?}", folder);
}
