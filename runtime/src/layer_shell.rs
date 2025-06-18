//! Build GUI applications with wayland wlr_layer_shell.
use crate::core::layer_shell::Settings;
use crate::core::window::Id;
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

/// An operation to be performed on some layer.
#[allow(missing_debug_implementations)]
pub enum Action {
    /// Opens a new layer with some [`Settings`].
    Open(Id, Settings, oneshot::Sender<Id>),

    /// Close the layer.
    Close(Id),
}

/// Opens a new window with the given [`Settings`]; producing the [`Id`]
/// of the new window on completion.
pub fn open(settings: Settings) -> (Id, Task<Id>) {
    let id = Id::unique();

    (
        id,
        task::oneshot(|channel| {
            crate::Action::LayerShell(Action::Open(id, settings, channel))
        }),
    )
}

/// Closes the layer with `id`.
pub fn close<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::LayerShell(Action::Close(id)))
}
