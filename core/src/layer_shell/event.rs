use crate::time::Instant;
use crate::Size;

/// A window-related event.
#[derive(PartialEq, Clone, Debug)]
pub enum Event {
    /// A layer was opened.
    Opened {
        /// The created size of the layer window.
        size: Size,
    },

    /// A layer was closed.
    Closed,

    /// A Layer was resized.
    Resized(Size),

    /// A layer redraw was requested.
    ///
    /// The [`Instant`] contains the current time.
    RedrawRequested(Instant),

    /// A layer was focused.
    Focused,

    /// A layer was unfocused.
    Unfocused,
}
