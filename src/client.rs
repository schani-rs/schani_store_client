use futures::{Future, Stream};
use hyper::{Client, Method, StatusCode, Uri};
use hyper::client::{HttpConnector, Request};
use hyper::error;
use tokio_core::reactor::Handle;

pub struct StoreClient {
    uri: Uri,
    client: Client<HttpConnector>,
}

impl StoreClient {
    pub fn new(uri: Uri, handle: &Handle) -> Self {
        StoreClient {
            uri: uri,
            client: Client::new(handle),
        }
    }

    fn build_uri(&self, path: &str) -> Uri {
        let mut uri_str = self.uri.to_string();
        uri_str.push_str(path);
        uri_str.parse().unwrap()
    }

    pub fn upload_raw_image(
        &self,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = error::Error>> {
        info!("uploading raw image to store ({} bytes)", data.len());
        let uri = self.build_uri("/raw");
        let mut req = Request::new(Method::Post, uri);
        req.set_body(data);

        Box::new(
            self.client
                .request(req)
                .and_then(|response| match response.status() {
                    StatusCode::Ok => Ok(response),
                    _ => Err(error::Error::Status),
                })
                .and_then(|response| response.body().concat2())
                .and_then(|body| {
                    let id = String::from_utf8(body.to_vec()).unwrap();
                    info!("raw image uploaded. Got ID {}", id);
                    Ok(id)
                })
                .map_err(|e| {
                    warn!("raw image upload failed: {}", e);
                    e
                }),
        )
    }

    pub fn upload_sidecar(
        &self,
        data: Vec<u8>,
    ) -> Box<Future<Item = String, Error = error::Error>> {
        info!("uploading sidecar to store ({} bytes)", data.len());
        let uri = self.build_uri("/sidecar");
        let mut req = Request::new(Method::Post, uri);
        req.set_body(data);

        Box::new(
            self.client
                .request(req)
                .and_then(|response| match response.status() {
                    StatusCode::Ok => Ok(response),
                    _ => Err(error::Error::Status),
                })
                .and_then(|response| response.body().concat2())
                .and_then(|body| {
                    let id = String::from_utf8(body.to_vec()).unwrap();
                    info!("sidecar uploaded. Got ID {}", id);
                    Ok(id)
                })
                .map_err(|e| {
                    warn!("sidecar upload failed: {}", e);
                    e
                }),
        )
    }

    pub fn upload_image(&self, data: Vec<u8>) -> Box<Future<Item = String, Error = error::Error>> {
        info!("uploading image to store ({} bytes)", data.len());
        let uri = self.build_uri("/image");
        let mut req = Request::new(Method::Post, uri);
        req.set_body(data);

        Box::new(
            self.client
                .request(req)
                .and_then(|response| match response.status() {
                    StatusCode::Ok => Ok(response),
                    _ => Err(error::Error::Status),
                })
                .and_then(|response| response.body().concat2())
                .and_then(|body| {
                    let id = String::from_utf8(body.to_vec()).unwrap();
                    info!("image uploaded. Got ID {}", id);
                    Ok(id)
                })
                .map_err(|e| {
                    warn!("image upload failed: {}", e);
                    e
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use tokio_core::reactor::Core;

    use super::*;

    #[test]
    fn upload_raw_image() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = StoreClient::new("http://localhost:8000".parse().unwrap(), &handle);

        let work = client
            .upload_raw_image(b"123".to_vec())
            .map_err(|e| panic!("{:?}", e));

        core.run(work).unwrap();
    }

    #[test]
    fn upload_sidecar() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = StoreClient::new("http://localhost:8000".parse().unwrap(), &handle);

        let work = client
            .upload_sidecar(b"123".to_vec())
            .map_err(|e| panic!("{:?}", e));

        core.run(work).unwrap();
    }

    #[test]
    fn upload_image() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = StoreClient::new("http://localhost:8000".parse().unwrap(), &handle);

        let work = client
            .upload_image(b"123".to_vec())
            .map_err(|e| panic!("{:?}", e));

        core.run(work).unwrap();
    }
}
