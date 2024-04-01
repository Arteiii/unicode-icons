use unicode_icons::{checkmarks, copyright, flags, punctuation};

fn main() {
    println!("Default Checkmark: {}", checkmarks::default()); // output: ✓
    println!("White Heavy Checkmark: {} ", checkmarks::white_heavy()); // output: ✅
    println!("Heavy Checkmark: {} ", checkmarks::heavy());
    println!("Light Checkmark: {} ", checkmarks::light());
    println!("Not Checkmark: {} ", checkmarks::not());
    println!("Ballot Box Checkmark: {} ", checkmarks::ballot_box());
    println!("Ballot Box with Bold Checkmark: {} ", checkmarks::ballot_box_with_bold());
    println!("Aegean Checkmark: {} ", checkmarks::aegean());

    println!("Rainbow Flag: {} ", flags::rainbow());
    println!("Pirate Flag: {} ", flags::pirate());
    println!("Black Flag: {} ", flags::black());
    println!("White Flag: {} ", flags::white());
    println!("Triangular Flag: {} ", flags::triangular());
    println!("Chequered Flag: {} ", flags::chequered());
    println!("Crossed Flag: {} ", flags::crossed());

    println!("Copyright Default: {} ", copyright::default());

    println!("Double Exclamation: {} ", punctuation::double_exclamation());
    println!("Exclamation Question: {} ", punctuation::exclamation_question());
    println!("Red Question: {} ", punctuation::red_question());
    println!("White Question: {} ", punctuation::white_question());
    println!("White Exclamation: {} ", punctuation::white_exclamation());
    println!("Red Exclamation: {} ", punctuation::red_exclamation());
    println!("Wavy Dash: {} ", punctuation::wavy_dash());
}
