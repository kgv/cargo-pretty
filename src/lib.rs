#![feature(or_patterns)]

#[doc(inline)]
pub use self::{
    format::Format,
    inline::Inline,
    order::{Order, Ordered},
    settings::Settings,
};

pub mod settings;

mod format;
mod inline;
mod order;
mod sort;
