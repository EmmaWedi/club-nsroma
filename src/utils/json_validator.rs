use actix_web::{dev::Payload, error, web, FromRequest, HttpRequest};
use futures_util::future::{FutureExt, LocalBoxFuture};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

pub struct ValidatedPath<T>(pub T);

pub struct ValidatedQuery<T>(pub T);

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_fut = web::Json::<T>::from_request(req, payload);

        async move {
            let json = json_fut.await?;
            json.validate().map_err(|e| error::ErrorBadRequest(e))?;
            Ok(ValidatedJson(json.into_inner()))
        }
        .boxed_local()
    }
}

impl<T> FromRequest for ValidatedPath<T>
where
    T: serde::de::DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let path = web::Path::<T>::from_request(req, &mut Payload::None);

        async move {
            let path = path.await?;
            path.validate().map_err(|e| error::ErrorBadRequest(e))?;
            Ok(ValidatedPath(path.into_inner()))
        }
        .boxed_local()
    }
}

impl<T> FromRequest for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let query_fut = web::Query::<T>::from_request(req, &mut Payload::None);

        async move {
            let query = query_fut.await?;
            query.validate().map_err(|e| error::ErrorBadRequest(e))?;
            Ok(ValidatedQuery(query.into_inner()))
        }
        .boxed_local()
    }
}
