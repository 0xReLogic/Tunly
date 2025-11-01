use crate::moq::bridge::{RequestEnvelope, ResponseEnvelope};

pub trait MoqTransport {
    fn connect(
        &mut self,
        remote: &str,
        token: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    fn publish_request(
        &mut self,
        request: RequestEnvelope,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    fn try_recv_response(&mut self) -> Option<ResponseEnvelope>;
}

pub struct NullMoq;

impl NullMoq {
    pub fn new() -> Self {
        Self
    }
}

impl MoqTransport for NullMoq {
    fn connect(
        &mut self,
        _remote: &str,
        _token: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    fn publish_request(
        &mut self,
        _request: RequestEnvelope,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    fn try_recv_response(&mut self) -> Option<ResponseEnvelope> {
        None
    }
}
