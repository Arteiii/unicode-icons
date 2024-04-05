//! # Unicode Icons (Rust)
//!
//! Total groups: **10**
//! Total functions: **1869**
//!
//! features:
//! * [activities](./activities/index.html)
//! * [animals_and_nature](./animals_and_nature/index.html)
//! * [component](./component/index.html)
//! * [flags](./flags/index.html)
//! * [food_and_drink](./food_and_drink/index.html)
//! * [objects](./objects/index.html)
//! * [people_and_body](./people_and_body/index.html)
//! * [symbols](./symbols/index.html)
//! * [smileys_and_emotion](./smileys_and_emotion/index.html)
//! * [travel_and_places](./travel_and_places/index.html)
//!
//! all are default
//! 
//! ## Usage
//!
//! ```rust
//! use unicode_icons::{activities, flags};
//!
//! println!("Christmas Tree: {}", activities::christmas_tree());
//! println!("Cedy Flag: {}", flags::rainbow_flag());
//!
//! ```
//!
//!
//! ### Output
//!
//! ````shell
//! $ cargo run --example main
//!     Christmas Tree: 🎄
//!     Cedy Flag: 🏳️‍🌈
//! ````
//!
//! ## License
//!
//! This project is licensed under the **MIT** License.
//!
//! For more information, see the [LICENSE](LICENSE.md) file.
//!
//! ### Copyright (c) 2024 Ben

pub mod icons;

// from https://www.unicode.org/emoji/charts/full-emoji-list.html and others IDK rn
