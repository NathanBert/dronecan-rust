use embedded_can::{Frame, Id};

use crate::message_type::{MessageTypeId, 
                          AnoMessageTypeId, 
                          ServiceTypeId, 
                          MessageIdMiddleBytes};


use crate::payload::{PayloadType};




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
                    ((raw >> 24) & 0x1F) as u16,      // Priority: 5 bits
                    ((raw >> 8) & 0xFFFF) as u16,     // Middle bytes: 16 bits
                    ((raw >> 7) & 1) == 1,            // Service not message: 1 bit
                    (raw & 0x7F) as u8,               // Source Node ID: 7 bits
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
            PayloadType::StartMessagePayload(_) => {}, // OK
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
}