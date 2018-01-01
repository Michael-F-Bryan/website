//! Utility types and wrappers for altering how a Rocket response is generated.

use std::ops::Deref;
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use rocket::{Outcome, Request, Response, Route};
use rocket::response::Responder;
use rocket::http::hyper::header::{CacheControl, CacheDirective};
use rocket::http::uri::URI;

/// Generate a new list of routes who's URL is prefixed with the provided
/// string.
///
/// This is essentially a carbon copy of what `Rocket::mount()` does.
pub fn prefix_routes(prefix: &str, mut routes: Vec<Route>) -> Vec<Route> {
    for route in &mut routes {
        let uri = URI::new(format!("{}/{}", prefix, route.uri));

        route.set_base(prefix);
        route.set_uri(uri.to_string());
    }

    routes
}

/// A wrapper request guard which will *require* the inner guard to succeed,
/// returning a 401 Unauthorized otherwise.
pub struct LoginRequired<T>(pub T);

impl<'a, 'r, T: FromRequest<'a, 'r>> FromRequest<'a, 'r> for LoginRequired<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match T::from_request(request) {
            Outcome::Success(s) => Outcome::Success(LoginRequired(s)),
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl<T> Deref for LoginRequired<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A wrapper which will return the inner `Responder`, but then set the
/// `Cache-Control` header so the browser knows to cache this item in the
/// future.
#[derive(Debug, Clone, PartialEq)]
pub struct Cached<T> {
    pub inner: T,
    pub seconds: u32,
}

/// Default cache duration is 7 days.
pub const DEFAULT_CACHE_DURATION: u32 = 3600 * 24 * 7;

impl<T> Cached<T> {
    pub fn new(inner: T) -> Cached<T> {
        Cached {
            inner: inner,
            seconds: DEFAULT_CACHE_DURATION,
        }
    }
}

impl<T> From<T> for Cached<T> {
    fn from(other: T) -> Cached<T> {
        Cached::new(other)
    }
}

impl<T> Deref for Cached<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'r, T: Responder<'r>> Responder<'r> for Cached<T> {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        let Cached { inner, seconds } = self;

        let mut original = inner.respond_to(request)?;
        original.set_header(CacheControl(vec![CacheDirective::MaxAge(seconds)]));

        Ok(original)
    }
}
