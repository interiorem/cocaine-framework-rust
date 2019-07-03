//! Ticket Vending Machine security service.

use std::collections::HashMap;

use futures::Future;

use {Error, Request, Service};
use dispatch::PrimitiveDispatch;

enum Method {
    Ticket,
}

impl Into<u64> for Method {
    #[inline]
    fn into(self) -> u64 {
        match self {
            Method::Ticket => 0,
        }
    }
}

/// A service wrapper for the Yandex TVM service.
#[derive(Clone, Debug)]
pub struct Tvm {
    service: Service,
}

impl Tvm {
    /// Constructs a TVM service wrapper using the specified service.
    pub fn new(service: Service) -> Self {
        Self { service: service }
    }

    /// Unwraps this TVM service yielding the underlying service.
    pub fn into_inner(self) -> Service {
        self.service
    }

    /// Exchanges your credentials for a TVM ticket.
    pub fn ticket(&self, id: u32, secret: &str) ->
        impl Future<Item = String, Error = Error>
    {
        let method = Method::Ticket.into();

        let (dispatch, future) = PrimitiveDispatch::pair();

        self.service.call(Request::new(method, &(id, secret)).unwrap(), dispatch);
        
        future
    }
}

#[cfg(test)]
mod test {}
