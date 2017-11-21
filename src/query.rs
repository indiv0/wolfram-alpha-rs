// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Performs a query to Wolfram|Alpha.
//!
//! For more information, see [Wolfram|Alpha's API
//! documentation](http://products.wolframalpha.com/api/documentation.html).

use error::Error;
use futures::Future;
use model::QueryResult;
use std::collections::HashMap;
use super::{WolframAlphaRequestSender, parse_wolfram_alpha_response};

/// A container struct for the parameters for a query to the Wolfram|Alpha API.
// TODO: replace these with concrete types.
#[allow(missing_docs)]
pub struct OptionalQueryParameters<'a> {
    pub format: Option<&'a str>,
    pub includepodid: Option<&'a str>,
    pub excludepodid: Option<&'a str>,
    pub podtitle: Option<&'a str>,
    pub podindex: Option<&'a str>,
    pub scanner: Option<&'a str>,
    pub async: Option<&'a str>,
    pub ip: Option<&'a str>,
    pub latlong: Option<&'a str>,
    pub location: Option<&'a str>,
    pub assumption: Option<&'a str>,
    pub podstate: Option<&'a str>,
    pub units: Option<&'a str>,
    pub width: Option<&'a str>,
    pub maxwidth: Option<&'a str>,
    pub plotwidth: Option<&'a str>,
    pub mag: Option<&'a str>,
    pub scantimeout: Option<&'a str>,
    pub podtimeout: Option<&'a str>,
    pub formattimeout: Option<&'a str>,
    pub parsetimeout: Option<&'a str>,
    pub reinterpret: Option<&'a str>,
    pub translation: Option<&'a str>,
    pub ignorecase: Option<&'a str>,
    pub sig: Option<&'a str>,
}

/// Performs a query to the Wolfram|Alpha API.
pub fn query<'a, R>(
    client: &'a R, appid: &'a str, input: &'a str,
    optional_query_parameters: Option<OptionalQueryParameters<'a>>
) -> Box<'a + Future<Item = QueryResult, Error = Error>>
    where R: WolframAlphaRequestSender,
{
    let mut params = HashMap::new();
    params.insert("input", input);

    // If present, we insert the optional parameters.
    if let Some(v) = optional_query_parameters {
        for &(name, value) in &[("format", v.format),
                                ("includepodid", v.includepodid),
                                ("excludepodid", v.excludepodid),
                                ("podtitle", v.podtitle),
                                ("podindex", v.podindex),
                                ("scanner", v.scanner),
                                ("async", v.async),
                                ("ip", v.ip),
                                ("latlong", v.latlong),
                                ("location", v.location),
                                ("assumption", v.assumption),
                                ("podstate", v.podstate),
                                ("units", v.units),
                                ("width", v.width),
                                ("maxwidth", v.maxwidth),
                                ("plotwidth", v.plotwidth),
                                ("mag", v.mag),
                                ("scantimeout", v.scantimeout),
                                ("podtimeout", v.podtimeout),
                                ("formattimeout", v.formattimeout),
                                ("parsetimeout", v.parsetimeout),
                                ("reinterpret", v.reinterpret),
                                ("translation", v.translation),
                                ("ignorecase", v.ignorecase)] {
            if let Some(value) = value {
                params.insert(name, value);
            }
        }
    }

    let res = client.send_authed("query", appid, params)
        .map_err(From::from)
        .and_then(|res| {
            parse_wolfram_alpha_response(&res)
        });

    Box::new(res)
}
