//! A crate that implements a LinkedList.
extern crate failure;
#[macro_use]
extern crate failure_derive;

pub use crate::error::Result;
pub use crate::linked_list::LinkedList;

mod error;
mod linked_list;
mod node;
