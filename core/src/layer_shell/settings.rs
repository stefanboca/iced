//! Configure your layers.

use crate::layer_shell::{Anchor, KeyboardInteractivity, Layer};

use crate::{Padding, Size};

// TODO: custom size and margin types

/// The layer settings.
#[derive(Debug, Clone)]
pub struct Settings {
    /// Layer
    pub layer: Layer,
    /// Namespace
    pub namespace: Option<String>,
    /// Size
    pub size: Size<u32>,
    /// Anchor
    pub anchor: Anchor,
    /// Exclusive zone
    pub exclusive_zone: i32,
    /// Margin
    pub margin: Padding<i32>,
    /// Keyboard Interactivity
    pub keyboard_interactivity: KeyboardInteractivity,
    /// Output
    pub output: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            layer: Layer::Top,
            namespace: None,
            size: Size {
                width: 0,
                height: 30,
            },
            anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
            exclusive_zone: 30,
            margin: Padding::new(0),
            keyboard_interactivity: KeyboardInteractivity::None,
            output: None,
        }
    }
}
