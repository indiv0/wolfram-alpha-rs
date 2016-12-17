// Copyright (c) 2016 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]
#![cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]

//! Struct and enum definitions of values in the Wolfram|Alpha model.
//!
//! For more information, see [Wolfram|Alpha's API
//! documentation](http://products.wolframalpha.com/api/documentation.html).

use serde::{Deserialize, Deserializer, Error as SerdeError};
use serde::de::{MapVisitor, SeqVisitor, Visitor};
use url::Url;

#[cfg(feature = "nightly")]
include!("model.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/model.rs"));

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use serde_xml::from_str;

    use super::{
        Definitions,
        DidYouMean,
        DidYouMeans,
        Img,
        Infos,
        LanguageMsg,
        Notes,
        Plaintext,
        Pod,
        QueryResult,
        Spellcheck,
        State,
        Statelist,
        States,
        Subpod,
        Tips,
        Warnings,
    };

    fn read_sample_data_from_path<P>(path: P) -> String
        where P: AsRef<Path>,
    {
        let mut file = File::open(path).unwrap();
        let mut body = String::new();
        file.read_to_string(&mut body).unwrap();
        body
    }

    #[test]
    fn test_query_result_deserializer() {
        from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_1.xml")).unwrap();
        from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_2.xml")).unwrap();
        //from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_3.xml")).unwrap();
        from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_4.xml")).unwrap();
        from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_5.xml")).unwrap();
        from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_6.xml")).unwrap();
        from_str::<QueryResult>(&read_sample_data_from_path("tests/sample-data/query_result_7.xml")).unwrap();
    }

    #[test]
    fn test_notes_deserializer() {
        from_str::<Notes>(&read_sample_data_from_path("tests/sample-data/notes.xml")).unwrap();
    }

    #[test]
    fn test_languagemsg_deserializer() {
        from_str::<LanguageMsg>(&read_sample_data_from_path("tests/sample-data/languagemsg.xml")).unwrap();
    }

    #[test]
    fn test_definitions_deserializer() {
        from_str::<Definitions>(&read_sample_data_from_path("tests/sample-data/definitions.xml")).unwrap();
    }

    #[test]
    fn test_didyoumean_deserializer() {
        from_str::<DidYouMean>(&read_sample_data_from_path("tests/sample-data/didyoumean.xml")).unwrap();
    }

    #[test]
    fn test_didyoumeans_deserializer() {
        from_str::<DidYouMeans>(&read_sample_data_from_path("tests/sample-data/didyoumeans.xml")).unwrap();
    }

    #[test]
    fn test_warning_deserializer() {
        from_str::<Spellcheck>(&read_sample_data_from_path("tests/sample-data/spellcheck.xml")).unwrap();
    }

    #[test]
    fn test_warnings_deserializer() {
        from_str::<Warnings>(&read_sample_data_from_path("tests/sample-data/warnings.xml")).unwrap();
    }

    #[test]
    fn test_tips_deserializer() {
        from_str::<Tips>(&read_sample_data_from_path("tests/sample-data/tips.xml")).unwrap();
    }

    #[test]
    fn test_state_deserializer() {
        from_str::<State>(&read_sample_data_from_path("tests/sample-data/state/state.xml")).unwrap();
    }

    #[test]
    fn test_statelist_deserializer() {
        from_str::<Statelist>(&read_sample_data_from_path("tests/sample-data/state/statelist.xml")).unwrap();
    }

    #[test]
    fn test_states_deserializer() {
        from_str::<States>(&read_sample_data_from_path("tests/sample-data/state/states.xml")).unwrap();
        from_str::<States>(&read_sample_data_from_path("tests/sample-data/state/states-multiple-states.xml")).unwrap();
        from_str::<States>(&read_sample_data_from_path("tests/sample-data/state/states-multiple-statelists.xml")).unwrap();
        from_str::<States>(&read_sample_data_from_path("tests/sample-data/state/states-out-of-order.xml")).unwrap();
        assert_eq!(
            from_str::<States>(&read_sample_data_from_path("tests/sample-data/state/states-out-of-order-complex.xml")).unwrap(),
            States {
                count: 5,
                state: vec![
                    State { name: 'a'.to_string(), input: 'a'.to_string(), },
                    State { name: 'c'.to_string(), input: 'c'.to_string(), },
                    State { name: 'f'.to_string(), input: 'f'.to_string(), },
                ],
                statelist: Some(vec![
                    Statelist {
                        count: 2,
                        value: 'b'.to_string(),
                        state: vec![
                            State { name: "b1".to_owned(), input: "b1".to_owned(), },
                            State { name: "b2".to_owned(), input: "b2".to_owned(), },
                        ],
                    },
                    Statelist {
                        count: 2,
                        value: 'd'.to_string(),
                        state: vec![
                            State { name: "d1".to_owned(), input: "d1".to_owned(), },
                            State { name: "d2".to_owned(), input: "d2".to_owned(), },
                        ],
                    },
                    Statelist {
                        count: 2,
                        value: 'e'.to_string(),
                        state: vec![
                            State { name: "e1".to_owned(), input: "e1".to_owned(), },
                            State { name: "e2".to_owned(), input: "e2".to_owned(), },
                        ],
                    },
                ]),
            }
        );
    }

    #[test]
    fn test_pod_deserializer() {
        from_str::<Pod>(&read_sample_data_from_path("tests/sample-data/pod.xml")).unwrap();
    }

    #[test]
    fn test_subpod_deserializer() {
        from_str::<Subpod>(&read_sample_data_from_path("tests/sample-data/subpod.xml")).unwrap();
    }

    #[test]
    fn test_img_deserializer() {
        from_str::<Img>(&read_sample_data_from_path("tests/sample-data/img.xml")).unwrap();
    }

    #[test]
    fn test_plaintext_deserializer() {
        from_str::<Plaintext>(&"<plaintext>pi</plaintext>".to_owned()).unwrap();
        from_str::<Option<Plaintext>>(&"<plaintext/>".to_owned()).unwrap();
    }

    #[test]
    fn test_infos_deserializer() {
        from_str::<Infos>(&read_sample_data_from_path("tests/sample-data/infos.xml")).unwrap();
    }
}
