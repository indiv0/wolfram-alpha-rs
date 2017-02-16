// Copyright (c) 2016 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs, non_camel_case_types, warnings)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "nightly", feature(custom_derive, proc_macro))]

//! A library providing Rust bindings for the Wolfram|Alpha web API.
//!
//! The library provides a `WolframAlphaRequestSender` trait which can be
//! implemented by various request senders. These implementations may then be
//! used to execute requests to the API.
//!
//! If the `hyper` feature is enabled during compilation, then this library
//! provides an implementation of the `WolframAlphaRequestSender` trait for
//! the `hyper::Client` of the [`hyper`](https://github.com/hyperium/hyper)
//! library.
//!
//! Response bodies are deserialized from XML into structs via the
//! [`serde_xml`](https://github.com/serde-rs/xml) library.

#[cfg(feature = "hyper")]
extern crate hyper;

#[macro_use]
extern crate log;
extern crate serde;
#[cfg(feature = "nightly")]
#[macro_use]
extern crate serde_derive;
extern crate serde_xml;
extern crate url;

mod error;

pub use self::error::{Error, HttpRequestError, HttpRequestResult, Result};

pub mod model;
pub mod query;
// TODO: implement the `validate_query` function.

use std::collections::HashMap;
use std::fmt::Debug;

use serde::Deserialize;

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
    fn send<'a>(&self, method: &str, params: &mut HashMap<&str, &'a str>)
        -> HttpRequestResult<String>;

    /// Make an API call to Wolfram|Alpha that contains the configured App ID.
    ///
    /// Takes a map of parameters which get appended to the request as query
    /// parameters. Returns the response body string.
    fn send_authed<'a>(
        &self,
        method: &str,
        app_id: &'a str,
        params: &mut HashMap<&str, &'a str>,
    ) -> HttpRequestResult<String> {
        params.insert("appid", app_id);
        self.send(method, params)
    }
}

#[cfg(feature = "hyper")]
mod hyper_support {
    use std::collections::HashMap;
    use std::io::Read;

    use hyper;
    use url::Url;

    use error::{HttpRequestError, HttpRequestResult};

    use super::WolframAlphaRequestSender;

    impl WolframAlphaRequestSender for hyper::Client {
        fn send<'a>(&self, method: &str, params: &mut HashMap<&str, &'a str>)
            -> HttpRequestResult<String>
        {
            let url_string = format!("https://api.wolframalpha.com/v2/{}", method);
            let mut url = url_string.parse::<Url>().expect("Unable to parse URL");

            url.query_pairs_mut().extend_pairs(params.into_iter());

            trace!("Sending query \"{:?}\" to url: {}", params, url);
            let mut response = self.get(url).send()?;
            let mut result = String::new();
            response.read_to_string(&mut result)?;
            trace!("Query result: {}", result);

            Ok(result)
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

#[cfg(feature = "hyper")]
pub use hyper_support::*;
