pub struct MessageTypeId {
    mtid: u16,
}

impl MessageTypeId {
    pub fn new(mtid: u16) -> Self {
        Self { mtid }
    }
}

pub struct AnoMessageTypeId {
    discriminator: u16,
    lbmtid: u8,
}

impl AnoMessageTypeId {
    pub fn new(mtid_middle_bytes: u16) -> Self {
        Self {
            discriminator: (mtid_middle_bytes >> 2) & 0x3FFF,
            lbmtid: (mtid_middle_bytes & 0x0003) as u8,
        }
    }
}

pub struct ServiceTypeId {
    stid: u8,
    request_not_response: bool,
    dest_node_id: u8,
}

impl ServiceTypeId {
    pub fn new(mtid_middle_bytes: u16) -> Self {
        Self {
            stid: (mtid_middle_bytes >> 8) as u8,
            request_not_response: ((mtid_middle_bytes >> 7) & 0x0001) == 1,
            dest_node_id: (mtid_middle_bytes as u8) & 0x7F,
        }
    }

}

pub enum MessageIdMiddleBytes {
    MessageTypeId(MessageTypeId),
    AnoMessageTypeId(AnoMessageTypeId),
    ServiceTypeId(ServiceTypeId),
}

impl MessageIdMiddleBytes {
    pub fn get_data_type_id(&self) -> u16 {
        match self {
            MessageIdMiddleBytes::MessageTypeId(m)    => m.mtid,
            MessageIdMiddleBytes::AnoMessageTypeId(a) => a.lbmtid as u16,
            MessageIdMiddleBytes::ServiceTypeId(s)    => s.stid as u16,
        }
    }

}