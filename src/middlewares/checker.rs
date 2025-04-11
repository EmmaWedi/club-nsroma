use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use std::{rc::Rc, sync::Arc};

use crate::{
    app::{customers::dtos::dto::get_customer_session, employees::dtos::dto::get_employee_session, users::dtos::dto::get_user_session},
    libs::jwt::Claims,
    AppState,
};

pub struct CheckUserMiddleware {
    state: web::Data<AppState>,
    account: &'static str,
}

impl CheckUserMiddleware {
    pub fn new(state: web::Data<AppState>, account: &'static str) -> Self {
        Self { state, account }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CheckUserMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckUserMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckUserMiddlewareInner {
            service: Rc::new(service),
            state: self.state.clone(),
            account: self.account,
        })
    }
}

pub struct CheckUserMiddlewareInner<S> {
    service: Rc<S>,
    state: web::Data<AppState>,
    account: &'static str,
}

impl<S, B> Service<ServiceRequest> for CheckUserMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut session = uuid::Uuid::nil();

        if let Some(claims) = req.extensions().get::<Arc<Claims>>() {
            if let Ok(s_uuid) = uuid::Uuid::parse_str(&claims.id.clone()) {
                session = s_uuid;
            }
        }

        let service = self.service.clone();
        let state = self.state.clone();
        let account = self.account;

        let fut = async move {
            let result: Result<(), ()> = match account {
                "User" => {
                    let res = get_user_session(session, &state).await;
                    if let Err(_) = res {
                        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                    };
                    req.extensions_mut().insert(Arc::new(res.unwrap()));
                    Ok(())
                }
                "Employee" => {
                    let res = get_employee_session(session, &state).await;
                    if let Err(_) = res {
                        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                    };
                    req.extensions_mut().insert(Arc::new(res.unwrap()));
                    Ok(())
                }
                "Customer" => {
                    let res = get_customer_session(session, &state).await;
                    if let Err(_) = res {
                        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                    };
                    req.extensions_mut().insert(Arc::new(res.unwrap()));
                    Ok(())
                }
                _ => Err(()),
            };

            match result {
                Ok(_) => service.call(req).await,
                Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
            }
        };
        Box::pin(fut)
    }
}
