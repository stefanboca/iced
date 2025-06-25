use crate::Size;

/// A window-related event.
#[derive(PartialEq, Clone, Debug)]
pub enum Event {
    /// A layer was opened.
    Opened {
        /// The created size of the layer window.
        size: Size,
    },
}
