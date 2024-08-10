fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }

    elements.iter().for_each(|el| println!("{}",el));

    elements
        .iter()
        .map(|el| format!("{} {}",el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(el: &mut [String]){
    el.iter_mut().for_each(|e| e.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|a|vec_b.push(a));
}

fn explode(elements: &[String]) -> Vec<Vec<String>>{
    elements.iter()
    .map(
        |el| el.chars()
                        .map(|c| c.to_string())
                        .collect()
    ).collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(
            String:: from(fallback),
            |el| el.to_string()
        )
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_iter = colors.iter();
    println!("{:#?}", colors_iter.next());

    // iter() and adapter, consumers, and for loop.
    print_elements(&colors);
    // works with slices too:
    print_elements(&colors[1..3]);

    // Mutating strings in `colors` using iter_mut 
    shorten_strings(&mut colors[1..3]);
    println!("{:#?}",colors);

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);

    // into_iter()
    let mut destination = vec![];
    move_elements(colors,&mut destination);
    println!("destination: {:#?}", destination);
    
    println!("Exploded: {:#?}",explode(&destination));

    let found_color = find_color_or(&destination, "reddy", "Orange");
    println!("{}", found_color);
}
