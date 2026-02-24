use http::header::{HeaderName, HeaderValue};
use http::{Request, Response};
use tower::{Layer, Service};

use underlay_core::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestId(pub Uuid);

impl RequestId {
    pub fn new() -> Self {
        RequestId(Uuid::new_v7())
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestId {
    pub fn from_header_value(value: &HeaderValue) -> Option<Self> {
        let raw = value.to_str().ok()?;
        let parsed = Uuid::parse_str(raw).ok()?;
        Some(RequestId(parsed))
    }

    pub fn to_header_value(self) -> Option<HeaderValue> {
        HeaderValue::from_str(&self.0.to_string()).ok()
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct RequestIdLayer {
    header_name: HeaderName,
}

impl Default for RequestIdLayer {
    fn default() -> Self {
        Self {
            header_name: HeaderName::from_static(REQUEST_ID_HEADER),
        }
    }
}

impl RequestIdLayer {
    pub fn new(header_name: HeaderName) -> Self {
        Self { header_name }
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService {
            inner,
            header_name: self.header_name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestIdService<S> {
    inner: S,
    header_name: HeaderName,
}

pin_project_lite::pin_project! {
    pub struct ResponseFuture<F> {
        #[pin]
        inner: F,
        header_name: HeaderName,
        request_id: Option<HeaderValue>,
    }
}

impl<F, B, E> std::future::Future for ResponseFuture<F>
where
    F: std::future::Future<Output = Result<Response<B>, E>>,
{
    type Output = Result<Response<B>, E>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        match this.inner.poll(cx) {
            std::task::Poll::Ready(Ok(mut res)) => {
                if let Some(request_id) = this.request_id.clone() {
                    res.headers_mut().insert(this.header_name.clone(), request_id);
                }
                std::task::Poll::Ready(Ok(res))
            }
            std::task::Poll::Ready(Err(err)) => std::task::Poll::Ready(Err(err)),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

impl<S, B> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>, Response = Response<B>>,
    S::Future: std::future::Future<Output = Result<Response<B>, S::Error>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let request_id = req
            .headers()
            .get(&self.header_name)
            .and_then(RequestId::from_header_value)
            .unwrap_or_default();

        req.extensions_mut().insert(request_id);

        ResponseFuture {
            inner: self.inner.call(req),
            header_name: self.header_name.clone(),
            request_id: request_id.to_header_value(),
        }
    }
}
