use embedded_can::{Frame, Id};

use crate::message_type::{AnoMessageTypeId, MessageIdMiddleBytes, MessageTypeId, ServiceTypeId};

use crate::payload::PayloadType;

pub struct DroneCanFrame {
    pub id: Id,
    pub priority: u16,
    pub mtid: MessageIdMiddleBytes,
    pub service_not_message: bool,
    pub source_node_id: u8,
    pub payload: PayloadType,

    raw_data: [u8; 8],
    dlc: usize,
}

impl Frame for DroneCanFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        let id = id.into();
        let len = data.len();

        // Une frame DroneCAN a
        // au minimum un tailbyte (1 octet) et au max 8 octets
        if len == 0 || len > 8 {
            return None;
        }

        // 1. Parser l'ID (DroneCAN utilise exclusivement des ID étendus 29-bits)
        let (priority, mtid_val, service_not_message, source_node_id) = match id {
            Id::Extended(ext_id) => {
                let raw = ext_id.as_raw();
                (
                    ((raw >> 24) & 0x1F) as u16,  // Priority: 5 bits
                    ((raw >> 8) & 0xFFFF) as u16, // Middle bytes: 16 bits
                    ((raw >> 7) & 1) == 1,        // Service not message: 1 bit
                    (raw & 0x7F) as u8,           // Source Node ID: 7 bits
                )
            }
            Id::Standard(_) => return None, // Refusé en DroneCAN
        };

        // 2. Construire le mtid
        let mtid = if service_not_message {
            MessageIdMiddleBytes::ServiceTypeId(ServiceTypeId::new(mtid_val))
        } else if source_node_id == 0 {
            // Node ID = 0 en mode Message correspond typiquement à un Anonymous Message
            MessageIdMiddleBytes::AnoMessageTypeId(AnoMessageTypeId::new(mtid_val))
        } else {
            MessageIdMiddleBytes::MessageTypeId(MessageTypeId::new(mtid_val))
        };

        // 3. Préparer le u64 brut depuis le slice de u8 pour les constructeurs Payload
        let mut raw_data = [0u8; 8];
        raw_data[..len].copy_from_slice(data);

        let payload = PayloadType::get_payload_type(raw_data, len)?;

        // 4. Une frame anonyme est obligatoirement un message single frame
        if matches!(mtid, MessageIdMiddleBytes::AnoMessageTypeId(_)) &&
        !matches!(payload, PayloadType::SingleMessagePayload(_)) {
            return None;
        }

        Some(DroneCanFrame {
            id,
            priority,
            mtid,
            service_not_message,
            source_node_id,
            payload,
            raw_data,
            dlc: len,
        })
    }

    // DroneCAN n'utilise pas les requêtes RTR (Remote Transmission Request)
    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        None
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        // Renvoie une slice exactement de la taille des données utiles
        &self.raw_data[..self.dlc]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_can::{ExtendedId, Frame, Id};

    #[test]
    fn test_create_and_parse_frame() {
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        let payload = [0x01, 0x02, 0xC0];

        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create frame");

        assert!(frame.is_extended());
        assert_eq!(frame.id(), Id::Extended(id));
        assert_eq!(frame.dlc(), 3);
        assert_eq!(frame.data(), &payload[..]);

        assert_eq!(frame.priority, 24); // 0x18FF0001 >> 24 = 0x18 = 24
        assert_eq!(frame.source_node_id, 1); // 0x18FF0001 & 0x7F = 1

        let roundtrip = DroneCanFrame::new(frame.id(), frame.data())
            .expect("Failed to create frame from parsed data");

        assert_eq!(roundtrip.priority, frame.priority);
        assert_eq!(roundtrip.source_node_id, frame.source_node_id);
    }

    #[test]
    fn test_start_frame() {
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        // 7 bytes de payload + 1 byte tailbyte (0x80 = start, not end)
        let payload = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x80];

        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create start frame");

        assert_eq!(frame.dlc(), 8);

        // Vérifier que c'est un StartMessagePayload
        match &frame.payload {
            PayloadType::StartMessagePayload(_) => {} // OK
            _ => panic!("Expected StartMessagePayload"),
        }
    }

    #[test]
    fn test_invalid_payload_length() {
        // Payload trop long (>8 bytes) doit retourner None
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        let long_payload = [0; 9];

        let result = DroneCanFrame::new(id, &long_payload);
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_payload() {
        // Payload vide est invalide (DLC=0)
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        let empty_payload: [u8; 0] = [];

        let frame = DroneCanFrame::new(id, &empty_payload);
        assert!(frame.is_none());
    }

    #[test]
    fn test_standard_id_rejected() {
        let id = embedded_can::StandardId::new(0x123).expect("Invalid StandardId");
        let payload = [0x01, 0xC0];
        assert!(DroneCanFrame::new(id, &payload).is_none());
    }

    #[test]
    fn test_service_frame_parsing() {
        // service_not_message = 1 (bit7), destination node id, service type id, request/response
        // Construire un ID où bit7 = 1 pour vérifier la branche ServiceTypeId
        let id = ExtendedId::new(0x1807_0081).expect("Invalid ExtendedId"); // bit7=1
        let payload = [0x01, 0xC0];
        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create service frame");

        assert!(frame.service_not_message);
        match &frame.mtid {
            MessageIdMiddleBytes::ServiceTypeId(_) => {}
            _ => panic!("Expected ServiceTypeId"),
        }
    }

    #[test]
    fn test_anonymous_message_parsing() {
        // source_node_id = 0, service_not_message = 0
        let id = ExtendedId::new(0x1800_0000).expect("Invalid ExtendedId");
        let payload = [0x01, 0xC0];
        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create anonymous frame");

        assert_eq!(frame.source_node_id, 0);
        assert!(!frame.service_not_message);
        match &frame.mtid {
            MessageIdMiddleBytes::AnoMessageTypeId(_) => {}
            _ => panic!("Expected AnoMessageTypeId"),
        }
    }

    #[test]
    fn test_middle_frame_tail_byte() {
        // Start=0, End=0, Toggle=1 → tail byte = 0x40
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        let payload = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x40];
        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create middle frame");

        match &frame.payload {
            PayloadType::StartMessagePayload(_) => panic!("Should not be Start"),
            _ => {} // vérifier ici le variant attendu pour "middle"
        }
    }

    #[test]
    fn test_end_frame_tail_byte() {
        // Start=0, End=1, Toggle=1 → tail byte = 0xC0... attention : 0xC0 = Start=1,End=1
        // End-only = 0x40 | 0x40(end bit) → à adapter selon le mapping exact bit7/bit6
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        let payload = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x60]; // End=1,Toggle=1,Start=0
        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create end frame");
        // vérifier le variant "EndMessagePayload" attendu
    }

    #[test]
    fn test_new_remote_always_none() {
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        assert!(DroneCanFrame::new_remote(id, 8).is_none());
    }

    #[test]
    fn test_priority_boundary_values() {
        // priority = 0 (bits 24-28 = 00000)
        let id_min = ExtendedId::new(0x0000_0001).expect("Invalid ExtendedId");
        let frame_min = DroneCanFrame::new(id_min, &[0xC0]).expect("Failed min priority");
        assert_eq!(frame_min.priority, 0);

        // priority = 31 (bits 24-28 = 11111)
        let id_max = ExtendedId::new(0x1F00_0001).expect("Invalid ExtendedId");
        let frame_max = DroneCanFrame::new(id_max, &[0xC0]).expect("Failed max priority");
        assert_eq!(frame_max.priority, 31);
    }

    #[test]
    fn test_source_node_id_boundary_values() {
        // node id = 1 (min valide, hors anonyme)
        let id_min = ExtendedId::new(0x1800_0001).expect("Invalid ExtendedId");
        let frame_min = DroneCanFrame::new(id_min, &[0xC0]).expect("Failed min node id");
        assert_eq!(frame_min.source_node_id, 1);

        // node id = 127 (max valide)
        let id_max = ExtendedId::new(0x1800_007F).expect("Invalid ExtendedId");
        let frame_max = DroneCanFrame::new(id_max, &[0xC0]).expect("Failed max node id");
        assert_eq!(frame_max.source_node_id, 127);
    }

    #[test]
    fn test_roundtrip_full_equality() {
        let id = ExtendedId::new(0x18FF0001).expect("Invalid ExtendedId");
        let payload = [0x01, 0x02, 0xC0];
        let frame = DroneCanFrame::new(id, &payload).expect("Failed to create frame");
        let roundtrip = DroneCanFrame::new(frame.id(), frame.data())
            .expect("Failed roundtrip");

        assert_eq!(roundtrip.priority, frame.priority);
        assert_eq!(roundtrip.source_node_id, frame.source_node_id);
        assert_eq!(roundtrip.service_not_message, frame.service_not_message);
        assert_eq!(roundtrip.dlc(), frame.dlc());
        assert_eq!(roundtrip.data(), frame.data());
        // Idéalement : comparer aussi les variants de mtid et payload (nécessite PartialEq/Debug)
    }

    #[test]
    fn test_anonymous_multiframe_rejected() {
        // source_node_id = 0 (anonyme) + tail byte Start (multi-trame) → doit être rejeté
        let id = ExtendedId::new(0x1800_0000).expect("Invalid ExtendedId");
        let payload = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x80];
        assert!(DroneCanFrame::new(id, &payload).is_none());
    }
}
