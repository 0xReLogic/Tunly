use async_trait::async_trait;
use url::Url;
use crate::moq::bridge::{RequestEnvelope, ResponseEnvelope};

#[async_trait]
pub trait MoqTransport {
    async fn connect(
        &mut self,
        remote: &str,
        token: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn publish_request(
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

#[async_trait]
impl MoqTransport for NullMoq {
    async fn connect(
        &mut self,
        _remote: &str,
        _token: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    async fn publish_request(
        &mut self,
        _request: RequestEnvelope,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }

    fn try_recv_response(&mut self) -> Option<ResponseEnvelope> {
        None
    }
}

pub struct MoqTailTransport {
    client: moq_native::Client,
}

impl MoqTailTransport {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = moq_native::ClientConfig::default();
        let client = moq_native::Client::new(config)?;
        Ok(Self { client })
    }
}

#[async_trait]
impl MoqTransport for MoqTailTransport {
    async fn connect(
        &mut self,
        remote: &str,
        token: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut url = Url::parse(remote)?;
        if let Some(t) = token {
            let q = match url.query() { Some(q) => format!("{}&token={}", q, t), None => format!("token={}", t) };
            url.set_query(Some(&q));
        }
        let _connection = self.client.connect(url).await?;
        Ok(())
    }

    async fn publish_request(
        &mut self,
        request: RequestEnvelope,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _ = request; // no-op for now until full MoQ wiring
        Ok(())
    }

    fn try_recv_response(&mut self) -> Option<ResponseEnvelope> {
        None
    }
}
