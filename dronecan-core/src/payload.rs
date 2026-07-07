use crate::utils::{Crc};
use crate::tailbyte::{Tailbyte};



pub struct StartMessagePayload {
    pub crc: Crc,
    pub payload: [u8; 5],
    pub tailbyte: Tailbyte,
}

impl StartMessagePayload {
    pub fn new(bits: u64) -> Self {
        let crc_1 = ((bits >> 56) & 0xFF) as u8;
        let crc_2 = ((bits >> 48) & 0xFF) as u8;

        let mut payload = [0u8; 5];
        for i in 0..5 {
            let shift = 40usize - (i * 8);
            payload[i] = ((bits >> shift) & 0xFF) as u8;
        }

        let tailbyte = Tailbyte {
            value: (bits & 0xFF) as u8,
        };

        Self {
            crc: Crc { crc_1, crc_2 },
            payload,
            tailbyte,
        }
    }
}

pub struct MiddleMessagePayload {
    pub crc: Crc,
    pub payload: [u8; 5],
    pub tailbyte: Tailbyte,
}

impl MiddleMessagePayload {
    pub fn new(bits: u64) -> Self {
        let crc_1 = ((bits >> 56) & 0xFF) as u8;
        let crc_2 = ((bits >> 48) & 0xFF) as u8;

        let mut payload = [0u8; 5];
        for i in 0..5 {
            let shift = 40usize - (i * 8);
            payload[i] = ((bits >> shift) & 0xFF) as u8;
        }

        let tailbyte = Tailbyte {
            value: (bits & 0xFF) as u8,
        };

        Self {
            crc: Crc { crc_1, crc_2 },
            payload,
            tailbyte,
        }
    }
}

pub struct EndMessagePayload {
    pub payload: Vec<u8>,
    pub tailbyte: Tailbyte,
    pub payload_len: u8,

}

impl EndMessagePayload {
    pub fn new(bits: u64, payload_len: usize) -> Self {
        let payload = (0..payload_len)
            .map(|i| {
                let shift = 56usize.saturating_sub(i * 8);
                ((bits >> shift) & 0xFF) as u8
            })
            .collect::<Vec<u8>>();

        let tail_shift = 56usize.saturating_sub(payload_len * 8);
        let tailbyte = Tailbyte {
            value: ((bits >> tail_shift) & 0xFF) as u8,
        };

        Self { payload, tailbyte, payload_len : payload_len as u8}
    }
}

pub struct SingleMessagePayload {
    pub payload: Vec<u8>,
    pub tailbyte: Tailbyte,
    pub payload_len: u8,
}

impl SingleMessagePayload {
    pub fn new(bits: u64, payload_len: usize) -> Self {
        let payload = (0..payload_len)
            .map(|i| {
                let shift = 56usize.saturating_sub(i * 8);
                ((bits >> shift) & 0xFF) as u8
            })
            .collect::<Vec<u8>>();

        let tail_shift = 56usize.saturating_sub(payload_len * 8);
        let tailbyte = Tailbyte {
            value: ((bits >> tail_shift) & 0xFF) as u8,
        };

        Self { payload, tailbyte, payload_len : payload_len as u8 }
    }
}

pub enum PayloadType {
    StartMessagePayload(StartMessagePayload),
    EndMessagePayload(EndMessagePayload),
    MiddleMessagePayload(MiddleMessagePayload),
    SingleMessagePayload(SingleMessagePayload),
}