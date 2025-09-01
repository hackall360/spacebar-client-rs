/// Simple bitfield helper inspired by the frontend implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitField {
    bits: u64,
}

impl BitField {
    /// Creates a new bitfield from the provided bits.
    pub fn new(bits: u64) -> Self {
        Self { bits }
    }

    /// Checks whether any of the provided bits are set.
    pub fn any(&self, bit: u64) -> bool {
        self.bits & bit != 0
    }

    /// Checks whether all provided bits are set.
    pub fn has(&self, bit: u64) -> bool {
        self.bits & bit == bit
    }

    /// Returns the bits that are missing from the provided mask.
    pub fn missing(&self, bits: u64) -> u64 {
        bits & !self.bits
    }

    /// Adds bits to the bitfield.
    pub fn add(&mut self, bits: u64) {
        self.bits |= bits;
    }

    /// Removes bits from the bitfield.
    pub fn remove(&mut self, bits: u64) {
        self.bits &= !bits;
    }

    /// Returns the underlying raw value.
    pub fn bits(&self) -> u64 {
        self.bits
    }
}
