use heapless::{Deque, index_map::FnvIndexMap};
use dronecan_core::dronecan_frame::DroneCanFrame;

const MAX_PARALLEL_RECEPTION: usize = 2usize.pow(4);
const MAX_MESSAGE_SIZE: usize = 2usize.pow(4);

type TransferKey = (u8, u16, u8); // (source_node_id, data_type_id, transfer_id)

struct PendingTransfer {
    frames: Deque<DroneCanFrame, MAX_MESSAGE_SIZE>,
    last_toggle: bool,
    started_at: u32, // tick/timestamp pour timeout
}

struct ReceptionManager {
    pending: FnvIndexMap<TransferKey, PendingTransfer, MAX_PARALLEL_RECEPTION>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceptionError {
    TableFull,        
    WrongStartFrame,  
    ToggleBitError,
    MessageTooLarge,
}

impl ReceptionManager {
    pub fn on_frame(&mut self, key: TransferKey, frame: DroneCanFrame, now: u32) -> Result<(), ReceptionError> {

    
        let entry = self.pending.entry(key).or_insert_with(|| PendingTransfer {
             frames: Deque::new(),
             last_toggle: false,
             started_at: now,
         }).map_err(|_| ReceptionError::TableFull)?;

        match entry.frames.is_empty() {
            // Case 1: the frame queue is empty, this is the first frame of this transfer
            true => {
                // Check that this first frame carries the "Start of Transfer" bit.
                if !frame.start_of_transfer() {
                    return Err(ReceptionError::WrongStartFrame);
                }
            }

            // Case 2: the queue already contains at least one frame means this is not the start of the transfer
            false => {
                // Look at the last frame already stored (the most recent one, on the "rear" side)
                if let Some(last_frame) = entry.frames.back() {
                    // The toggle bit must alternate on every consecutive frame of the same transfer.
                    // If the new frame's toggle matches the previous one's,
                    // that's an anomaly: either a duplicated frame, or a lost intermediate frame
                    if last_frame.toggle() == frame.toggle() {
                        return Err(ReceptionError::ToggleBitError);
                    }

                }
            }
        }

        entry.frames.push_back(frame).map_err(|_| ReceptionError::MessageTooLarge)?;

        if entry.frames.back().unwrap().end_of_transfer() {
            let completed = self.pending.remove(&key).unwrap();
            // TODO: vérifier CRC puis traiter
        }

        Ok(())

    
    }

    
    //     // TODO: vérifier toggle bit ici avant push
    //     entry.frames.push_back(frame).map_err(|_| ReceptionError::MessageTooLarge)?;

    //     if frame.is_end_of_transfer() {
    //         let completed = self.pending.remove(&key).unwrap();
    //         // TODO: vérifier CRC puis traiter
    //         process_message(completed.frames);
    //     }
    //     Ok(())
    // }
}