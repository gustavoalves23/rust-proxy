pub struct HTTPSClientBuilder;

impl HTTPSClientBuilder {
    pub fn build(
        &mut self,
    ) -> hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>
    {
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        )
    }
}

pub struct HTTPSClient;

impl HTTPSClient {
    pub fn builder() -> HTTPSClientBuilder {
        HTTPSClientBuilder
    }
}