#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::future::{ready, Ready};
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    use http::{Request, Response};
    use tower::{Layer, Service};

    use crate::request_id::RequestId;
    use crate::RequestIdLayer;

    fn noop_waker() -> Waker {
        unsafe fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        unsafe fn wake(_: *const ()) {}
        unsafe fn wake_by_ref(_: *const ()) {}
        unsafe fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }

    #[test]
    fn request_id_round_trips_as_header_value() {
        let id = RequestId::new();
        let header = id.to_header_value();
        let parsed = RequestId::from_header_value(&header).expect("should parse");
        assert_eq!(id, parsed);
    }

    #[test]
    fn invalid_header_value_is_rejected() {
        let header = http::header::HeaderValue::from_static("not-a-uuid");
        assert!(RequestId::from_header_value(&header).is_none());
    }

    #[tokio::test]
    async fn request_id_layer_inserts_extension_and_response_header() {
        #[derive(Clone, Debug)]
        struct InnerService;

        impl Service<Request<()>> for InnerService {
            type Response = Response<()>;
            type Error = Infallible;
            type Future = Ready<Result<Self::Response, Self::Error>>;

            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }

            fn call(&mut self, req: Request<()>) -> Self::Future {
                assert!(req.extensions().get::<RequestId>().is_some());
                ready(Ok(Response::new(())))
            }
        }

        let layer = RequestIdLayer::default();
        let mut svc = layer.layer(InnerService);

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        match svc.poll_ready(&mut cx) {
            Poll::Ready(Ok(())) => {}
            other => panic!("service should be ready, got: {other:?}"),
        }

        let req = Request::builder().uri("/").body(()).unwrap();
        let res = svc.call(req).await.expect("service call should succeed");

        let header = res
            .headers()
            .get(crate::REQUEST_ID_HEADER)
            .expect("x-request-id should be set");
        assert!(RequestId::from_header_value(header).is_some());
    }
}
