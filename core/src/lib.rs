//! The core library of [Iced].
//!
//! This library holds basic types that can be reused and re-exported in
//! different runtime implementations.
//!
//! ![The foundations of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/foundations.png?raw=true)
//!
//! [Iced]: https://github.com/iced-rs/iced
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(
    missing_debug_implementations,
    missing_docs,
    unused_results,
    rustdoc::broken_intra_doc_links
)]
pub mod alignment;
pub mod border;
pub mod clipboard;
pub mod event;
pub mod font;
pub mod gradient;
pub mod image;
pub mod keyboard;
pub mod layout;
pub mod mouse;
pub mod overlay;
pub mod renderer;
pub mod svg;
pub mod text;
pub mod time;
pub mod touch;
pub mod widget;
pub mod window;

mod angle;
mod background;
mod color;
mod content_fit;
mod element;
mod hasher;
mod length;
mod padding;
mod pixels;
mod point;
mod rectangle;
mod shadow;
mod shell;
mod size;
mod vector;

pub use alignment::Alignment;
pub use angle::{Degrees, Radians};
pub use background::Background;
pub use border::Border;
pub use clipboard::Clipboard;
pub use color::Color;
pub use content_fit::ContentFit;
pub use element::Element;
pub use event::Event;
pub use font::Font;
pub use gradient::Gradient;
pub use hasher::Hasher;
pub use layout::Layout;
pub use length::Length;
pub use overlay::Overlay;
pub use padding::Padding;
pub use pixels::Pixels;
pub use point::Point;
pub use rectangle::Rectangle;
pub use renderer::Renderer;
pub use shadow::Shadow;
pub use shell::Shell;
pub use size::Size;
pub use text::Text;
pub use vector::Vector;
pub use widget::Widget;

pub use smol_str::SmolStr;
