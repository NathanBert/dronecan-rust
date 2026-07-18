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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_matches_reference_check_value() {
        // Valeur officielle de référence de l'algo CRC-16-CCITT-FALSE
        let payload = b"123456789";
        assert_eq!(TRANSFER_CRC.checksum(payload), 0x29B1);
    }

    #[test]
    fn crc_with_data_type_signature() {
        // TODO: remplacer par la vraie signature DSDL du message ciblé
        let data_type_signature: u64 = 0x0000_0000_0000_03F3;
        let mut buf = [0u8; 8 + 3];
        buf[..8].copy_from_slice(&data_type_signature.to_le_bytes());
        buf[8..].copy_from_slice(&[0x01, 0x02, 0x03]);
        let value = TRANSFER_CRC.checksum(&buf);
        // Comparer avec une capture réseau réelle (Wireshark/candump) ou pyuavcan
    }
}
