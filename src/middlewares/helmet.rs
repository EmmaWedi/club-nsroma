use std::{rc::Rc, task::{Context, Poll}};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http, Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};

pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();
            headers.insert(
                http::header::STRICT_TRANSPORT_SECURITY,
                "max-age=31536000; includeSubDomains".parse().unwrap(),
            );
            headers.insert(http::header::X_FRAME_OPTIONS, "DENY".parse().unwrap());
            headers.insert(
                http::header::X_CONTENT_TYPE_OPTIONS,
                "nosniff".parse().unwrap(),
            );
            headers.insert(
                http::header::X_XSS_PROTECTION,
                "1; mode=block".parse().unwrap(),
            );
            headers.insert(
                http::header::CONTENT_SECURITY_POLICY,
                "default-src 'self'".parse().unwrap(),
            );
            headers.insert(
                http::header::REFERRER_POLICY,
                "no-referrer".parse().unwrap(),
            );

            Ok(res)
        })
    }
}
//use this for intercepting responses