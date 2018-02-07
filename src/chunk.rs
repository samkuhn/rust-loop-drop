pub use std::time::SystemTime;
pub use ::output_value::OutputValue;

pub trait Triggerable {
    // TODO: or should this be MidiTime??
    fn trigger (&mut self, id: u32, value: OutputValue, at: SystemTime);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Coords {
    pub row: u32,
    pub col: u32
}

impl Coords {
    pub fn new (row: u32, col: u32) -> Coords {
        Coords { row, col }
    }

    pub fn from (id: u32) -> Coords {
        Coords {
            row: id / 8, 
            col: id % 8
        }
    }

    pub fn id_from (row: u32, col: u32) -> u32 {
        (row * 8) + col
    }

    pub fn id (&self) -> u32 {
        Coords::id_from(self.row, self.col)
    }
}

pub struct Shape {
    pub rows: u32,
    pub cols: u32
}

impl Shape {
    pub fn new (rows: u32, cols: u32) -> Shape {
        Shape { rows, cols }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct MidiMap {
    pub chunk_index: usize,
    pub id: u32
}

pub struct ChunkMap {
    pub coords: Coords,
    pub shape: Shape,
    pub chunk: Box<Triggerable + Send>
}

impl ChunkMap {
    pub fn new (chunk: Box<Triggerable + Send>, coords: Coords, shape: Shape) -> Box<Self> {
        Box::new(ChunkMap {
            chunk, coords, shape
        })
    }
}