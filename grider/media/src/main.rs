#[derive(Debug)]
enum Media {
    Book{ title: String, author: String},
    Movie { title: String, director:String},
    Audiobook {title: String},
}

impl Media {
    fn description(&self) -> String {
        // VERBOSE SELF TYPE CHECK
        // if let Media::Book{title, author} = self {
        //     format!("Book: {} {}", title, author)
        // }else if let Media::Movie{title, director} = self  {
        //     format!("Movie: {} {}", title, director)
        // }else if let Media::Audiobook { title } = self  {
        //     format!("Audiobook: {}", title)
        // }else{
        //     format!("Unknown media!")
        // }

        // PATTERN MATCHING:
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            },
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
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

    let book = Media::Book { 
        title: String::from("Panchtantra!"), 
        author: String::from("Mishra"),
    };
    println!("{:#?}",book.description());
}
