//! # Unicode Icons (Rust)
//!
//! Total groups: **10**
//! Total functions: **1869**
//!
//! Groups:
//! * [Activities](./activities/index.html)
//! * [Animals & Nature](./animals_and_nature/index.html)
//! * [Component](./component/index.html)
//! * [Flags](./flags/index.html)
//! * [Food & Drink](./food_and_drink/index.html)
//! * [Objects](./objects/index.html)
//! * [People & Body](./people_and_body/index.html)
//! * [Smileys & Emotion](./smileys_and_emotion/index.html)
//! * [Symbols](./symbols/index.html)
//! * [Travel & Places](./travel_and_places/index.html)
//!
//! ## Usage
//!
//! ````rust
//! // examples/main.rs
//!
//! use unicode_icons::{activities, flags};
//!
//! fn main() {
//!     println!("Christmas Tree: {}", activities::christmas_tree());
//!     println!("Rainbow Flag: {}", flags::rainbow_flag());
//! }
//! ````
//!
//! ### Output
//!
//! ````shell
//! $ cargo run --example main
//!     Christmas Tree: 🎄
//!     Rainbow Flag: 🏳️‍🌈
//! ````
//!
//! ## License
//!
//! This project is licensed under the **MIT** License.
//!
//! For more information, see the [LICENSE](LICENSE.md) file.
//!
//! ### Copyright (c) 2024 Ben

mod icons;

pub use icons::{
    activities, animals_and_nature, component, flags, food_and_drink, objects, people_and_body,
    smileys_and_emotion, symbols, travel_and_places,
};

// from https://www.unicode.org/emoji/charts/full-emoji-list.html and others IDK rn
