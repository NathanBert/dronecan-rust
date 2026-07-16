use embedded_can::Frame;

pub trait CanReceive {
    type Frame: Frame;
    type Error;
    fn receive(&mut self) -> Result<Option<Self::Frame>, Self::Error>;
}
