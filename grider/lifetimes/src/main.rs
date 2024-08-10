use std::result;

fn next_language<'a>(langs: &'a[String], current: &str) -> &'a str {
    let mut found = false;
    
    for lang in langs {
        if found {
            return lang
        }

        if lang == current {
            found = true;
        }
    }
    langs.last().unwrap()
}

fn last_language(langs: &[String]) -> &str {
    langs.last().unwrap()
}

fn longest<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let langs = vec![
        String::from("rust"),
        String::from("go"),
        String::from("ts"),
    ];
    
    let  result =  next_language(&langs, "go");
    println!("{}",result);
    println!("{}",last_language(&langs));
    println!("{}",longest("ts", "typescript"));
}
