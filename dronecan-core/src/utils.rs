use crc::Crc;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct CrcData {
    pub crc_1: u8,
    pub crc_2: u8,
}

const TRANSFER_CRC: Crc<u16> = Crc::<u16>::new(&crc::CRC_16_IBM_3740);

impl CrcData {
    pub fn from_payload(payload: &[u8]) -> Self {
        let value = TRANSFER_CRC.checksum(payload);
        Self {
            crc_1: (value >> 8) as u8,
            crc_2: (value & 0x00FF) as u8,
        }
    }

    pub fn as_u16(&self) -> u16 {
        ((self.crc_1 as u16) << 8) | self.crc_2 as u16
    }
}

// D2FINIR UNE REFERENCE CALCULEE A PARTIR DE L'ALGO DEPUIS EXTERIEUR DU CODE
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_of_known_payload_matches_reference() {
        let payload = [0x01, 0x02, 0x03];
        assert_eq!(
            CrcData::from_payload(&payload),
            CrcData {
                crc_1: 0xAD,
                crc_2: 0xAD
            }
        );
    }
}
