// examples/main.rs

use unicode_icons::icons::{activities, flags};

fn main() {
    let format_string = format!("{} a string using format", activities::bullseye());

    println!("{}", format_string);
    println!("Christmas Tree: {}", activities::christmas_tree());
    println!("Cedy Flag: {}", flags::rainbow_flag());
}
