use crate::infrastructure::adapters::http::cookies::RequestResponseCookies;
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web::body::MessageBody;
use actix_web::cookie::Cookie;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform, forward_ready};
use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::rc::Rc;

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub struct CookieMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CookieMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CookieMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CookieMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct CookieMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for CookieMiddlewareService<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        req.extensions_mut()
            .insert(RequestResponseCookies::default());

        let svc = self.service.clone();

        Box::pin(async move {
            let mut res = svc.call(req).await?;

            let cookies = res
                .request()
                .extensions()
                .get::<RequestResponseCookies>()
                .cloned()
                .unwrap_or_default();

            for raw in cookies.cookies {
                if let Ok(cookie) = Cookie::parse_encoded(raw) {
                    res.response_mut().add_cookie(&cookie)?;
                }
            }

            Ok(res)
        })
    }
}
