fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let mut color_iter = colors.iter();

    println!("{:#?}", color_iter.next());
    println!("{:#?}", color_iter.next());
    println!("{:#?}", color_iter.next());
    println!("{:#?}", color_iter.next());
}
