//! Draws pretty things in Typst
//!
//! The [`typst_writer`](crate) crate handles __almost__ all types of Typst formatting that the PDF
//! requires. This includes constructing the preamble document, drawing graphs and formatting solutions.
//!
//! The only Typst-related functions that aren't handled by this crate are `Display`-related
//! implementations in the [`math`] crate.
pub mod colors;
pub mod custom_math;
pub mod drawing;
pub mod formatting;
pub mod graphing;
pub mod preamble;
pub mod prefix_handler;
mod solution_with_steps;
pub mod typst_file_builder;
