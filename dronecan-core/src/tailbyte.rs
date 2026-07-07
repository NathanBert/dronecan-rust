#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tailbyte {
    pub value: u8,
}

impl Tailbyte {
    // bit positions (LSB = bit 0)
    const START_OF_TRANSFER_BIT: u8 = 0;
    const END_OF_TRANSFER_BIT: u8 = 1;
    const TOGGLE_BIT: u8 = 2;
    const TRANSFER_ID_SHIFT: u8 = 3;
    const TRANSFER_ID_MASK: u8 = 0b1111; 

    pub fn new_from_parts(start_of_transfer: bool, end_of_transfer: bool, toggle: bool, transfer_id: u8) -> Self {
        let mut v: u8 = 0;
        if start_of_transfer { v |= 1 << Self::START_OF_TRANSFER_BIT; }
        if end_of_transfer   { v |= 1 << Self::END_OF_TRANSFER_BIT; }
        if toggle            { v |= 1 << Self::TOGGLE_BIT; }
        let tid = transfer_id & Self::TRANSFER_ID_MASK;
        v |= tid << Self::TRANSFER_ID_SHIFT;
        Tailbyte { value: v }
    }

    pub fn from_value(value: u8) -> Self {
        Tailbyte { value }
    }

    pub fn start_of_transfer(&self) -> bool {
        (self.value >> Self::START_OF_TRANSFER_BIT) & 1 != 0
    }

    pub fn set_start_of_transfer(&mut self, on: bool) {
        if on { self.value |= 1 << Self::START_OF_TRANSFER_BIT; } else { self.value &= !(1 << Self::START_OF_TRANSFER_BIT); }
    }

    pub fn end_of_transfer(&self) -> bool {
        (self.value >> Self::END_OF_TRANSFER_BIT) & 1 != 0
    }

    pub fn set_end_of_transfer(&mut self, on: bool) {
        if on { self.value |= 1 << Self::END_OF_TRANSFER_BIT; } else { self.value &= !(1 << Self::END_OF_TRANSFER_BIT); }
    }

    pub fn toggle(&self) -> bool {
        (self.value >> Self::TOGGLE_BIT) & 1 != 0
    }

    pub fn set_toggle(&mut self, on: bool) {
        if on { self.value |= 1 << Self::TOGGLE_BIT; } else { self.value &= !(1 << Self::TOGGLE_BIT); }
    }

    pub fn transfer_id(&self) -> u8 {
        (self.value >> Self::TRANSFER_ID_SHIFT) & Self::TRANSFER_ID_MASK
    }

    pub fn set_transfer_id(&mut self, id: u8) {
        let id = id & Self::TRANSFER_ID_MASK;
        self.value = (self.value & !(Self::TRANSFER_ID_MASK << Self::TRANSFER_ID_SHIFT)) | (id << Self::TRANSFER_ID_SHIFT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_parts_value() {
        let t = Tailbyte::new_from_parts(true, false, true, 0b1010);
        assert_eq!(t.start_of_transfer(), true);
        assert_eq!(t.end_of_transfer(), false);
        assert_eq!(t.toggle(), true);
        assert_eq!(t.transfer_id(), 0b1010);
        let raw = t.value;
        let mut t2 = Tailbyte::from_value(raw);
        assert_eq!(t, t2);

        t2.set_end_of_transfer(true);
        assert_eq!(t2.end_of_transfer(), true);
    }
}