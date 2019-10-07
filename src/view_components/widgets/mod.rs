pub mod widget;

pub mod button;
pub mod checkbox;
pub mod text;
pub mod textbox;

pub use crate::backend::system::state::GenerateView;

// Simplified widget imports
// Allows user to type `::widget::Name` or `::widget::*` rather than full namespaces
pub use button::Button;
pub use checkbox::CheckBox;
pub use text::Text;
pub use textbox::TextBox;