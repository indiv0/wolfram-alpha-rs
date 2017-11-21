// Copyright (c) 2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate wolfram_alpha;

use futures::future::Future;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use wolfram_alpha::query::query;

const APP_ID: &str = "YOUR_APP_ID";

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let input = "distance from the Sun to the Earth";
    let work = query(&client, APP_ID, input, None)
        .and_then(|res| {
            let subpod = &res.pod.unwrap()[1].subpod[0];
            let plaintext = subpod.plaintext.as_ref().unwrap();
            println!("Distance from the Sun to the Earth: {:?}", plaintext);
            Ok(())
        });

    core.run(work).unwrap();
}
