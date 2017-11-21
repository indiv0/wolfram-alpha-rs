// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs, non_camel_case_types)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "nightly", feature(custom_derive, proc_macro))]

//! A library providing Rust bindings for the Wolfram|Alpha web API.
//!
//! The library provides a `WolframAlphaRequestSender` trait which can be
//! implemented by various request senders. These implementations may then be
//! used to execute requests to the API.
//!
//! Response bodies are deserialized from XML into structs via the
//! [`serde_xml`](https://github.com/serde-rs/xml) library.

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
#[cfg(feature = "nightly")]
#[macro_use]
extern crate serde_derive;
extern crate serde_xml;
extern crate tokio_core;
extern crate url;

mod error;

pub use self::error::{Error, HttpRequestError, Result};

pub mod model;
pub mod query;
// TODO: implement the `validate_query` function.

use futures::Future;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;

fn parse_wolfram_alpha_response<T>(response: &str) -> Result<T>
    where T: Debug + Deserialize,
{
    let parsed_response = serde_xml::from_str(response)?;
    trace!("Parsed response: {:?}", parsed_response);
    Ok(parsed_response)
}

/// Functionality for sending requests to Wolfram|Alpha via HTTP.
///
/// Should be implemented for clients to send requests to Wolfram|Alpha.
pub trait WolframAlphaRequestSender {
    /// Performs an API call to Wolfram|Alpha.
    ///
    /// Takes a map of parameters which get appended to the request as query
    /// parameters. Returns the response body string.
    fn send<'a>(&'a self, method: &str, params: HashMap<&'a str, &'a str>)
        -> Box<'a + Future<Item = String, Error = HttpRequestError>>;

    /// Make an API call to Wolfram|Alpha that contains the configured App ID.
    ///
    /// Takes a map of parameters which get appended to the request as query
    /// parameters. Returns the response body string.
    fn send_authed<'a>(&'a self, method: &str, app_id: &'a str, mut params: HashMap<&'a str, &'a str>)
        -> Box<'a + Future<Item = String, Error = HttpRequestError>>
    {
        params.insert("appid", app_id);
        self.send(method, params)
    }
}

mod hyper_support {
    use error::HttpRequestError;
    use futures::{self, Future, Stream};
    use hyper::{self, Uri};
    use hyper::client::Connect;
    use std::collections::HashMap;
    use std::str::{self, FromStr};
    use super::WolframAlphaRequestSender;
    use url::Url;

    impl<C> WolframAlphaRequestSender for hyper::Client<C>
        where C: Connect,
    {
        fn send<'a>(&'a self, method: &str, mut params: HashMap<&'a str, &'a str>)
            -> Box<'a + Future<Item = String, Error = HttpRequestError>>
        {
            let url_string = format!("https://api.wolframalpha.com/v2/{}", method);
            let mut url = url_string.parse::<Url>().expect("Unable to parse URL");

            url.query_pairs_mut().extend_pairs((&mut params).into_iter());

            let uri = Uri::from_str(url.as_ref()).map_err(From::from);
            let res = futures::done(uri)
                .and_then(move |uri| {
                    trace!("Sending query \"{:?}\" to URL: {}", params, url.as_ref());
                    self.get(uri)
                })
                .and_then(|res| {
                    trace!("Response: {}", res.status());

                    res.body().concat2()
                })
                .map_err(From::from)
                .map(|body| {
                    str::from_utf8(&body)
                        .map_err(From::from)
                        .map(|string| string.to_string())
                })
                .and_then(|string| string);

            Box::new(res)
        }
    }

    impl From<hyper::Error> for HttpRequestError {
        fn from(error: hyper::Error) -> HttpRequestError {
            match error {
                hyper::Error::Io(e) => HttpRequestError::Io(e),
                e => HttpRequestError::Other(Box::new(e)),
            }
        }
    }
}

pub use hyper_support::*;
