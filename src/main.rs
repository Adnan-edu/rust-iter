fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    //elements.iter().for_each(|el| println!("{}", el)); //Iterator consumer
    //Iterator adapter
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1))
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el))
}
fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|e| e.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}
fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}
fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    //print_elements(&colors);
    //print_elements(&colors[1..3]);

    //shorten_strings(&mut colors);
    //shorten_strings(&mut colors[1..3]);
    //println!("{:#?}", colors);

    //let uppercased = to_uppercase(&colors);
    //println!("{:#?}", uppercased);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    // let exploded = explode(&colors);
    // println!("{:#?}", exploded);

    let find_color = find_color_or(&colors, "re", "Orange");
    println!("{}", find_color);
}
