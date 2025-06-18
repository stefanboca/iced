use bitflags::bitflags;

bitflags! {
    /// Specifies which edges and corners a layer should be placed at in the anchor rectangle.
    ///
    /// A combination of two orthogonal edges will cause the layer's anchor point to be the intersection of
    /// the edges. For example [`Anchor::TOP`] and [`Anchor::LEFT`] will result in an anchor point in the top
    /// left of the anchor rectangle.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Anchor: u32 {
        /// Top edge of the anchor rectangle.
        const TOP = 1;

        /// The bottom edge of the anchor rectangle.
        const BOTTOM = 2;

        /// The left edge of the anchor rectangle.
        const LEFT = 4;

        /// The right edge of the anchor rectangle.
        const RIGHT = 8;
    }
}
