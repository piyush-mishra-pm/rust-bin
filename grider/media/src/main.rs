#[derive(Debug)]
enum Media {
    Book{ title: String, author: String},
    Movie { title: String, director:String},
    Audiobook {title: String},
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book{title, author} = self {
            format!("Book: {} {}", title, author)
        }else if let Media::Movie{title, director} = self  {
            format!("Movie: {} {}", title, director)
        }else if let Media::Audiobook { title } = self  {
            format!("Audiobook: {}", title)
        }else{
            format!("Unknown media!")
        }
    }
}

fn print_media(media: Media){
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook { 
        title: String::from("The three musketeers!"), 
    };
    println!("{:#?}",audiobook.description());

    let movie = Media::Movie { 
        title: String::from("Jurassic Park!"), 
        director: String::from("Stephen Hawking"),
    };
    println!("{:#?}",movie.description());
}
