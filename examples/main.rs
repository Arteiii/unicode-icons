use unicode_icons::{checkmarks, flags, copyright};

fn main() {
    println!("Default Checkmark: {}", checkmarks::default()); // output: ✓

    // somehow I have rendering issues if there is no whitespace
    println!("White Heavy Checkmark: {} ", checkmarks::white_heavy()); // output: ✅

    println!("Rainbow Flag: {} ", flags::rainbow());
    println!("pirate_flag: {} ", flags::pirate());
    println!("pirate_flag: {} ", flags::triangular());
    println!("copyright default: {} ", copyright::default());
}
