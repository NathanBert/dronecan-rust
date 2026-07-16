#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Tailbyte {
    pub value: u8,
}

impl Tailbyte {
    const START_OF_TRANSFER_BIT: u8 = 7;
    const END_OF_TRANSFER_BIT: u8 = 6;
    const TOGGLE_BIT: u8 = 5;

    const TRANSFER_ID_SHIFT: u8 = 0;
    const TRANSFER_ID_MASK: u8 = 0b0001_1111;

    pub fn new_from_parts(
        start_of_transfer: bool,
        end_of_transfer: bool,
        toggle: bool,
        transfer_id: u8,
    ) -> Self {
        let mut value = transfer_id & Self::TRANSFER_ID_MASK;

        if start_of_transfer {
            value |= 1 << Self::START_OF_TRANSFER_BIT;
        }

        if end_of_transfer {
            value |= 1 << Self::END_OF_TRANSFER_BIT;
        }

        if toggle {
            value |= 1 << Self::TOGGLE_BIT;
        }

        Self { value }
    }

    pub fn from_value(value: u8) -> Self {
        Self { value }
    }

    pub fn start_of_transfer(&self) -> bool {
        (self.value & (1 << Self::START_OF_TRANSFER_BIT)) != 0
    }

    pub fn set_start_of_transfer(&mut self, on: bool) {
        if on {
            self.value |= 1 << Self::START_OF_TRANSFER_BIT;
        } else {
            self.value &= !(1 << Self::START_OF_TRANSFER_BIT);
        }
    }

    pub fn end_of_transfer(&self) -> bool {
        (self.value & (1 << Self::END_OF_TRANSFER_BIT)) != 0
    }

    pub fn set_end_of_transfer(&mut self, on: bool) {
        if on {
            self.value |= 1 << Self::END_OF_TRANSFER_BIT;
        } else {
            self.value &= !(1 << Self::END_OF_TRANSFER_BIT);
        }
    }

    pub fn toggle(&self) -> bool {
        (self.value & (1 << Self::TOGGLE_BIT)) != 0
    }

    pub fn set_toggle(&mut self, on: bool) {
        if on {
            self.value |= 1 << Self::TOGGLE_BIT;
        } else {
            self.value &= !(1 << Self::TOGGLE_BIT);
        }
    }

    pub fn transfer_id(&self) -> u8 {
        (self.value >> Self::TRANSFER_ID_SHIFT) & Self::TRANSFER_ID_MASK
    }

    pub fn set_transfer_id(&mut self, transfer_id: u8) {
        let transfer_id = transfer_id & Self::TRANSFER_ID_MASK;

        self.value =
            (self.value & !Self::TRANSFER_ID_MASK) | (transfer_id << Self::TRANSFER_ID_SHIFT);
    }
}

#[test]
fn roundtrip_parts_value() {
    let t = Tailbyte::new_from_parts(true, false, true, 0b01010);

    assert_eq!(t.value, 0b1010_1010);
    assert!(t.start_of_transfer());
    assert!(!t.end_of_transfer());
    assert!(t.toggle());
    assert_eq!(t.transfer_id(), 0b01010);

    let mut t2 = Tailbyte::from_value(t.value);
    assert_eq!(t, t2);

    t2.set_end_of_transfer(true);

    assert_eq!(t2.value, 0b1110_1010);
    assert!(t2.end_of_transfer());
}
