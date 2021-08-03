use crate::{
    settings::structs::{RateLimitConfig, Settings},
    utils::get_ip,
    IpAddr,
    MyError,
};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{ok, Ready};
use rate_limiter::{RateLimitType, RateLimiter};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::sync::Mutex;

pub mod rate_limiter;

#[derive(Debug, Clone)]
pub struct RateLimit {
    // it might be reasonable to use a std::sync::Mutex here, since we don't need to lock this
    // across await points
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

#[derive(Debug, Clone)]
pub struct RateLimited {
    rate_limiter: Arc<Mutex<RateLimiter>>,
    type_: RateLimitType,
}

pub struct RateLimitedMiddleware<S> {
    rate_limited: RateLimited,
    service: S,
}


impl RateLimited {
    pub async fn wrap<T, E>(
        self,
        ip_addr: IpAddr,
        fut: impl Future<Output=Result<T, E>>,
    ) -> Result<T, E>
        where
            E: From<MyError>,
    {
        let rate_limit: RateLimitConfig = Settings::get().rate_limit();
        let mut limiter = self.rate_limiter.lock().await;
        // Does not need to be blocking because the RwLock in settings never held across await points,
        // and the operation here locks only long enough to clone
        match self.type_ {
            RateLimitType::Register => {
                limiter.check_rate_limit_full(
                    self.type_,
                    &ip_addr,
                    rate_limit.register,
                    rate_limit.register_per_second,
                    true,
                )?;
            }
        };
        let res = fut.await;
        // after
        {
            let mut limiter = self.rate_limiter.lock().await;
            if res.is_ok() {
                match self.type_ {
                    RateLimitType::Register => {
                        limiter.check_rate_limit_full(
                            self.type_,
                            &ip_addr,
                            rate_limit.register,
                            rate_limit.register_per_second,
                            false,
                        )?;
                    }
                };
            }
        }
        res
    }
}

impl RateLimit {
    pub fn register(&self) -> RateLimited {
        self.kind(RateLimitType::Register)
    }
    fn kind(&self, type_: RateLimitType) -> RateLimited {
        RateLimited {
            rate_limiter: self.rate_limiter.clone(),
            type_,
        }
    }
}

impl<S> Transform<S, ServiceRequest> for RateLimited
    where
        S: Service<ServiceRequest, Response=ServiceResponse, Error=actix_web::Error>,
        S::Future: 'static,
{
    type Response = S::Response;
    type Error = actix_web::Error;
    type Transform = RateLimitedMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimitedMiddleware {
            rate_limited: self.clone(),
            service,
        })
    }
}

type FutResult<T, E> = dyn Future<Output=Result<T, E>>;

impl<S> Service<ServiceRequest> for RateLimitedMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse, Error=actix_web::Error>,
        S::Future: 'static,
{
    type Response = S::Response;
    type Error = actix_web::Error;
    type Future = Pin<Box<FutResult<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip_addr = get_ip(&req.connection_info());

        let fut = self
            .rate_limited
            .clone()
            .wrap(ip_addr, self.service.call(req));

        Box::pin(async move { fut.await.map_err(actix_web::Error::from) })
    }
}
