use common_infrastructure::messages::MasterMessage;
use anyhow::Result;
use esp_idf_svc::http::client::{EspHttpConnection, Configuration};
use core::time::Duration;
use serde_json;
use embedded_svc::http::client::Client;
use embedded_svc::io::Write;
use log::info;

const URL_MASTER: &str = common_infrastructure::URL_MASTER;

pub struct MessageSender{
    config: Configuration
}


impl MessageSender {

    pub fn new() -> Self{
        let mut config = Configuration::default();
        config.timeout = Some(Duration::from_millis(10_000));
        Self{
            config
        }
    }
    
    pub fn send_message(&self, message: MasterMessage) -> Result<()>{
        let mut client = Client::wrap(EspHttpConnection::new(&self.config)?);

        let message = serde_json::to_string(&message)?;
        let payload = message.as_bytes();

        // Prepare headers and URL
        let content_length_header = format!("{}", payload.len());
        let headers = [
            ("content-type", "text/plain"),
            ("content-length", &*content_length_header),
        ];

        // Send request
        let mut request = client.post(URL_MASTER, &headers)?;
        request.write_all(payload)?;
        request.flush()?;
        info!("-> POST {}", URL_MASTER);
        let _ = request.submit()?;

        return Ok(());
    }
}
