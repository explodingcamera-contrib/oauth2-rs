use crate::{HttpClientError, HttpRequest, HttpResponse};

use http::{header::CONTENT_TYPE, method::Method};

use std::io::Read;

impl crate::SyncHttpClient for ureq::Agent {
    type Error = HttpClientError<ureq::Error>;

    fn call(&self, request: HttpRequest) -> Result<HttpResponse, Self::Error> {
        let (parts, body) = request.into_parts();
        let response = if parts.method == Method::POST {
            let mut request = self.post(parts.uri.to_string());
            request.headers_mut().unwrap().extend(parts.headers);
            request.send(body)
        } else {
            debug_assert_eq!(parts.method, Method::GET);
            let mut request = self.get(parts.uri.to_string());
            request.headers_mut().unwrap().extend(parts.headers);
            request.call()
        }
        .map_err(Box::new)?;

        let mut builder = http::Response::builder().status(response.status());

        if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
            builder = builder.header(CONTENT_TYPE, content_type);
        }

        let mut body = Vec::new();
        response.into_body().into_reader().read_to_end(&mut body)?;

        builder.body(body).map_err(HttpClientError::Http)
    }
}
