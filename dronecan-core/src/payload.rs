use crate::tailbyte::Tailbyte;
use crate::utils::CrcData;

#[derive(Debug, Hash)]
pub struct StartMessagePayload {
    pub crc: CrcData,
    pub payload: [u8; 5],
    pub tailbyte: Tailbyte,
}

impl StartMessagePayload {
    pub fn new(bits: u64) -> Option<Self> {
        let crc_1 = ((bits >> 56) & 0xFF) as u8;
        let crc_2 = ((bits >> 48) & 0xFF) as u8;

        let mut payload = [0u8; 5];
        for i in 0..5 {
            let shift = 40usize - (i * 8);
            payload[i] = ((bits >> shift) & 0xFF) as u8;
        }

        let tailbyte = Tailbyte::from_value((bits & 0xFF) as u8);

        Some(Self {
            crc: CrcData { crc_1, crc_2 },
            payload,
            tailbyte,
        })
    }
}

#[derive(Debug, Hash)]
pub struct MiddleMessagePayload {
    pub crc: CrcData,
    pub payload: [u8; 5],
    pub tailbyte: Tailbyte,
}

impl MiddleMessagePayload {
    pub fn new(bits: u64) -> Option<Self> {
        let crc_1 = ((bits >> 56) & 0xFF) as u8;
        let crc_2 = ((bits >> 48) & 0xFF) as u8;

        let mut payload = [0u8; 5];
        for i in 0..5 {
            let shift = 40usize - (i * 8);
            payload[i] = ((bits >> shift) & 0xFF) as u8;
        }

        let tailbyte = Tailbyte::from_value((bits & 0xFF) as u8);

        Some(Self {
            crc: CrcData { crc_1, crc_2 },
            payload,
            tailbyte,
        })
    }
}

#[derive(Debug, Hash)]
pub struct EndMessagePayload {
    pub payload: Vec<u8>,
    pub tailbyte: Tailbyte,
    pub payload_len: u8,
}

impl EndMessagePayload {
    pub fn new(bits: u64, payload_len: usize) -> Option<Self> {
        let payload = (0..payload_len)
            .map(|i| {
                let shift = 56usize.saturating_sub(i * 8);
                ((bits >> shift) & 0xFF) as u8
            })
            .collect::<Vec<u8>>();

        let tail_shift = 56usize.saturating_sub(payload_len * 8);

        let tailbyte = Tailbyte::from_value(((bits >> tail_shift) & 0xFF) as u8);

        Some(Self {
            payload,
            tailbyte,
            payload_len: payload_len as u8,
        })
    }
}

#[derive(Debug, Hash)]
pub struct SingleMessagePayload {
    pub payload: Vec<u8>,
    pub tailbyte: Tailbyte,
    pub payload_len: u8,
}

impl SingleMessagePayload {
    pub fn new(bits: u64, payload_len: usize) -> Option<Self> {
        let payload = (0..payload_len)
            .map(|i| {
                let shift = 56usize.saturating_sub(i * 8);
                ((bits >> shift) & 0xFF) as u8
            })
            .collect::<Vec<u8>>();

        let tail_shift = 56usize.saturating_sub(payload_len * 8);
        let tailbyte = Tailbyte::from_value(((bits >> tail_shift) & 0xFF) as u8);

        Some(Self {
            payload,
            tailbyte,
            payload_len: payload_len as u8,
        })
    }
}

#[derive(Debug, Hash)]
pub enum PayloadType {
    StartMessagePayload(StartMessagePayload),
    EndMessagePayload(EndMessagePayload),
    MiddleMessagePayload(MiddleMessagePayload),
    SingleMessagePayload(SingleMessagePayload),
}

impl PayloadType {
    pub fn get_payload_type(raw_data: [u8; 8], dlc: usize) -> Option<Self> {
        // DLC Value check
        if !(1..=raw_data.len()).contains(&dlc) {
            return None;
        }
        let tailbyte = Tailbyte::from_value(*raw_data.get(dlc - 1)?);

        let mut arr = [0u8; 8];
        arr.copy_from_slice(&raw_data);
        let data = u64::from_be_bytes(arr);

        if tailbyte.start_of_transfer() {
            if tailbyte.end_of_transfer() {
                return Some(PayloadType::SingleMessagePayload(
                    SingleMessagePayload::new(data, dlc)?,
                ));
            }
            return Some(PayloadType::StartMessagePayload(StartMessagePayload::new(
                data,
            )?));
        } else if tailbyte.end_of_transfer() {
            return Some(PayloadType::EndMessagePayload(EndMessagePayload::new(
                data, dlc,
            )?));
        }

        Some(PayloadType::MiddleMessagePayload(
            MiddleMessagePayload::new(data)?,
        ))
    }

    pub fn get_transfert_id(&self) -> u8 {
        match self {
            PayloadType::MiddleMessagePayload(m) => m.tailbyte.transfer_id(),
            PayloadType::EndMessagePayload(e) => e.tailbyte.transfer_id(),
            PayloadType::StartMessagePayload(s) => s.tailbyte.transfer_id(),
            PayloadType::SingleMessagePayload(s) => s.tailbyte.transfer_id(),
        }
    }

    pub fn end_of_transfer(&self) -> bool {
        match self {
            PayloadType::EndMessagePayload(_) => true,
            PayloadType::SingleMessagePayload(_) => true,
            _ => false,
        }
    }

    pub fn start_of_transfer(&self) -> bool {
        match self {
            PayloadType::StartMessagePayload(_) => true,
            PayloadType::SingleMessagePayload(_) => true,
            _ => false,
        }
    }

    pub fn toggle(&self) -> bool{
        match self {
            PayloadType::MiddleMessagePayload(m) => m.tailbyte.toggle(),
            PayloadType::EndMessagePayload(e) => e.tailbyte.toggle(),
            PayloadType::StartMessagePayload(s) => s.tailbyte.toggle(),
            PayloadType::SingleMessagePayload(s) => s.tailbyte.toggle(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIDDLE_MESSAGE_FRAME: [u8; 8] = [0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0b00100000];
    const START_MESSAGE_FRAME: [u8; 8] = [0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0b10100000];
    const END_MESSAGE_FRAME: [u8; 8] = [0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0b01100000];
    const SINGLE_MESSAGE_FRAME: [u8; 8] = [0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0b11100000];

    const END_MESSAGE_FRAME_4: [u8; 8] = [0x44, 0x44, 0x44, 0b01100000, 0x00, 0x00, 0x00, 0x00];
    const SINGLE_MESSAGE_FRAME_6: [u8; 8] = [0x44, 0x44, 0x44, 0x44, 0x44, 0b11100000, 0x00, 0x00];

    #[test]
    fn check_message_type_assignation() {
        let type_message =
            PayloadType::get_payload_type(MIDDLE_MESSAGE_FRAME, MIDDLE_MESSAGE_FRAME.len());
        assert!(matches!(
            type_message,
            Some(PayloadType::MiddleMessagePayload(_))
        ));

        let type_message =
            PayloadType::get_payload_type(START_MESSAGE_FRAME, START_MESSAGE_FRAME.len());
        assert!(matches!(
            type_message,
            Some(PayloadType::StartMessagePayload(_))
        ));

        let type_message =
            PayloadType::get_payload_type(END_MESSAGE_FRAME, END_MESSAGE_FRAME.len());

        assert!(matches!(
            type_message,
            Some(PayloadType::EndMessagePayload(_))
        ));

        let type_message =
            PayloadType::get_payload_type(SINGLE_MESSAGE_FRAME, SINGLE_MESSAGE_FRAME.len());

        assert!(matches!(
            type_message,
            Some(PayloadType::SingleMessagePayload(_))
        ));

        let type_message = PayloadType::get_payload_type(END_MESSAGE_FRAME_4, 4);

        assert!(matches!(
            type_message,
            Some(PayloadType::EndMessagePayload(_))
        ));

        let type_message = PayloadType::get_payload_type(SINGLE_MESSAGE_FRAME_6, 6);

        assert!(matches!(
            type_message,
            Some(PayloadType::SingleMessagePayload(_))
        ));
    }
}
