# Unicode Icons (Rust)

Total groups: **10**  
Total functions: **1869**  

Groups:
* Activities
* Animals & Nature
* Component
* Flags
* Food & Drink
* Objects
* People & Body
* Smileys & Emotion
* Symbols
* Travel & Places


full list at: [DOCS.RS](https://docs.rs/unicode-icons)

## Usage

````rust
// examples/main.rs

use unicode_icons::{activities, flags};

fn main() {
    println!("Christmas Tree: {}", activities::christmas_tree());
    println!("Rainbow Flag: {}", flags::rainbow_flag());
}
````

### Output

````shell
$ cargo run --example main
    Christmas Tree: 🎄
    Rainbow Flag: 🏳️‍🌈
````


## License

This project is licensed under the **MIT** License.

For more information, see the [LICENSE](LICENSE.md) file.

### Copyright (c) 2024 Ben
