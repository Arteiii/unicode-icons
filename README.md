# Unicode Icons (Rust)

Total groups: **10**  
Total functions: **1869**  

Features:
* activities
* animals_and_nature
* component
* flags
* food_and_drink
* objects
* people_and_body
* symbols
* smileys_and_emotion
* travel_and_places

full list at: [DOCS.RS](https://docs.rs/unicode-icons)

## Usage

````rust
// examples/main.rs

use unicode_icons::{activities, flags};

fn main() {
    println!("Christmas Tree: {}", activities::christmas_tree());
    println!("Cedy Flag: {}", flags::rainbow_flag());
}
````

### Output

````shell
$ cargo run --example main
    Christmas Tree: 🎄
    Cedy Flag: 🏳️‍🌈
````


## License

This project is licensed under the **MIT** License.

For more information, see the [LICENSE](LICENSE.md) file.

### Copyright (c) 2024 Ben
