mod content;

use content::media::Media;
use content::catalog::Catalog;

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

    let podcast = Media::Podcast(99);
    let placeholder = Media:: Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);
    println!("{:#?}", catalog);

    println!("{:#?}",catalog.get_by_index(105));
    println!("{:#?}",catalog.get_by_index(3));

    match catalog.get_by_index(0) {
        Option::Some(v) => {
            println!("Item: {:#?}", v);
        },
        Option::None => {
            println!("‼ None");
        }
    }

    match catalog.get_by_index(100) {
        Option::Some(v) => {
            println!("Item: {:#?}", v);
        },
        Option::None => {
            println!("‼ None");
        }
    }
}
