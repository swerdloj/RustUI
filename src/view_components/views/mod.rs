// TODO: Move view.rs functionality to this module, then create view types which implement a base View trait

// TODO: Where should items such as dividers and spacers go? Would these be views?

pub mod view;
pub mod vstack;
pub mod hstack;

pub type VStack<T> = vstack::VStack<T>;
pub type HStack<T> = hstack::HStack<T>;