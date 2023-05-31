use std::fs::File;
use std::io::{self, Read};

struct Directory {
    name: String,
    children: Vec<Box<Node>>, // Note that using Option here would give two representations for no children
    // !vec[] and None, so it's probably better to only keep the first representation
    parent: Option<Box<Directory>>,
}

impl Directory {
    fn new(name: String, parent: Option<Box<Directory>>) -> Directory {
        Directory {
            name,
            children: Vec::new(),
            parent,
        }
    }

    fn to_string(&self) -> String {
        String::from("")
    }
}

struct Document {
    name: String,
    parent: Box<Directory>,
    size: usize,
}

impl Document {
    fn new(name: String, parent: Box<Directory>, size: usize) -> Document {
        Document {
            name,
            parent,
            size,
        }
    }

    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.size).to_string()
    }
}

enum Node {
    Directory(Directory),
    Document(Document),
}

const FILENAME: &str = "day7";
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() -> io::Result<()> {
    let file = File::open(FILENAME);

    if let Ok(mut file) = File::open(FILENAME) {
        let mut body = String::new();
        file.read_to_string(&mut body)?;

        let mut root = Directory::new(String::from("/"), None);
        let doc = Document::new(String::from("testdoc.pdf"), Box::new(root), 342341);
        let doc2 = Document::new(String::from("testdoc.pdf"), Box::new(root), 990245);

        let mut dir = Directory::new(String::from("Images"), Some(Box::new(root)));
        let doc_dir_1 = Document::new(String::from("001.jpg"), Box::new(dir), 32423);

        println!("{}", root.name);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to open the file!",
        ));
    }

    Ok(())
}
