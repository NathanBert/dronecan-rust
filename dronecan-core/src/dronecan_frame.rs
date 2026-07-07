use embedded_can::{Frame, Id};

use crate::message_type::{MessageTypeId, 
                          AnoMessageTypeId, 
                          ServiceTypeId, 
                          MessageIdMiddleBytes};


use crate::payload::{PayloadType, 
                     EndMessagePayload, 
                     MiddleMessagePayload, 
                     StartMessagePayload, 
                     SingleMessagePayload};




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

        // Une frame DroneCAN a au minimum un tailbyte (1 octet) et au max 8 octets
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

        // 3. Préparer le u64 brut depuis le slice de u8 pour tes constructeurs Payload
        let mut raw_data = [0u8; 8];
        raw_data[..len].copy_from_slice(data);
        let bits = u64::from_be_bytes(raw_data);

        // 4. Lire le Tailbyte (le dernier octet valide) pour choisir la variante Payload
        let tailbyte_val = data[len - 1];
        let start_of_transfer = (tailbyte_val & 0x80) != 0; // Bit 7
        let end_of_transfer = (tailbyte_val & 0x40) != 0;   // Bit 6
        
        let payload_len = len - 1; // La longueur du payload pur (sans le tailbyte)

        let payload = match (start_of_transfer, end_of_transfer) {
            (true, true) => {
                PayloadType::SingleMessagePayload(SingleMessagePayload::new(bits, payload_len))
            }
            (true, false) => {
                if len != 8 { return None; } // Start frame toujours pleine
                PayloadType::StartMessagePayload(StartMessagePayload::new(bits))
            }
            (false, false) => {
                if len != 8 { return None; } // Middle frame toujours pleine
                PayloadType::MiddleMessagePayload(MiddleMessagePayload::new(bits))
            }
            (false, true) => {
                PayloadType::EndMessagePayload(EndMessagePayload::new(bits, payload_len))
            }
        };

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