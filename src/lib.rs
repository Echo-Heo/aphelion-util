#![warn(clippy::pedantic)]
#![allow(clippy::unusual_byte_groupings)]

/*!
![Aphelion](https://github.com/orbit-systems/aphelion/blob/main/readme-assets/aphelion64.png?raw=true)

TODO: put some good documentation thats not just copy pasted from the typst doc...
*/

pub mod instruction;
pub mod interrupt;
pub mod io;
pub mod nibble;
pub mod registers;
// TODO: useful operations here
pub mod helper;

/// DOCUMENTATION NEEDED!
#[doc(hidden)]
pub const TODO: () = ();
