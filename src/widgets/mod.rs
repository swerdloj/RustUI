pub mod widget;

pub mod button;
pub mod checkbox;
pub mod text;

// Simplified widget imports
// Allows user to type `::widget::Name` or `::widget::*` rather than full namespaces
pub type Button<T> = button::Button<T>;
pub type CheckBox<T> = checkbox::CheckBox<T>;
pub type Text<T> = text::Text<T>;