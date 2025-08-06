struct Book {
    title: String,
    author: String,
    genere: String,
}
struct BookIterator {
    properties: Vec<String>,
}
impl Iterator for BookIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.properties.is_empty() {
            None
        } else {
            Some(self.properties.remove(0))
        }
    }
}
impl IntoIterator for Book {
    type Item = String;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genere],
        }
    }
}

fn main() {
    println!("=====Intoiterators in Rust=====");
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        genere: "Programming".to_string(),
    };
    let mut book_iter = book.into_iter();
    println!("Book Properties:");
    while let Some(property) = book_iter.next() {
        println!("{}", property);
    }
}
