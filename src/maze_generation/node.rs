use bitflags::bitflags;

bitflags! {
    /// This structure contains a desciptor of each node in the maze. Specifically,
    /// whether or not it is marked, and the pathways to other cells that are open.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Node: u8 {
        const marked = 0b00010000;
        const north = 0b00001000;
        const east = 0b00000100;
        const south = 0b00000010;
        const west = 0b00000001;
        const directions = Node::east.bits() | Node::north.bits() | Node::south.bits() | Node::west.bits();
    }
}
