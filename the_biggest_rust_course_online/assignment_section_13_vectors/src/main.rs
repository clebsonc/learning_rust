#[derive(Debug)]
struct File {
    field: String,
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
        self.contents.push(File { field: name })
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.swap_remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut f = Folder::new("musics".to_string());

    for music_artists in vec![
        String::from("abba"),
        String::from("Bom Jovi"),
        String::from("bee gees"),
    ] {
        f.create_file(music_artists);
    }
    println!("{:#?}", f);

    let removed = f.delete_file(1);
    println!("File Removed: {}", removed.field);

    let retrieved = f.get_file(2);
    if let Some(file) = retrieved {
        println!("File at index: {:#?}", file);
    } else {
        println!("There was no file");
    }
}
