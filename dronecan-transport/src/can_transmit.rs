use embedded_can::Frame;

pub trait CanTransmit {
    type Frame: Frame;
    type Error;
    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error>;
}
