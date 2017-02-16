// TODO: add `MathmMl` struct.

/// `QueryResult` is the outer wrapper for all results from the query function.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryResult {
    // Attributes
    pub success: bool,
    pub error_flag: bool,
    pub numpods: u32,
    pub version: Option<String>, // TODO: replace this with a better type.
    pub datatypes: String, // TODO: possibly replace this with an enum?
    pub timing: f64,
    pub timedout: String,
    pub timedoutpods: Option<String>,
    pub parsetiming: f64,
    pub parsetimedout: Option<bool>,
    pub recalculate: Option<String>,
    pub id: Option<String>,
    //pub host: Option<String>, // FIXME
    pub server: Option<u32>,
    pub related: Option<String>,

    pub pod: Option<Vec<Pod>>,
    pub assumptions: Option<Assumptions>,
    pub sources: Option<Sources>,
    pub error: Option<Error>,
    pub tips: Option<Tips>,
    pub didyoumeans: Option<DidYouMeans>,
    pub languagemsg: Option<LanguageMsg>,
    pub futuretopic: Option<FutureTopic>,
    pub relatedexamples: Option<RelatedExamples>,
    pub examplepage: Option<ExamplePage>,
    pub generalization: Option<Generalization>,
    pub warnings: Option<Warnings>,
}

/// A series of `Assumption` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Assumptions {
    // Attributes
    pub count: u32,

    pub assumption: Vec<Assumption>,
}

/// A single assumption, typically about the meaning of a word or phrase.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Assumption {
    // Attributes
    #[serde(rename="type")]
    pub assumption_type: String,
    //pub desc: Option<String>,
    //pub word: Option<String>,
    //pub template: Option<String>,
    //pub current: Option<u32>,
    pub count: u32,

    pub value: Vec<Value>,
}

/// A series of `Tip` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Tips {
    // Attributes
    pub count: u32,

    pub tip: Vec<Tip>,
}

/// A single tip, typically regarding suggestions for fixing the query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Tip {
    // Attributes
    pub text: String,
}

/// A series of `DidYouMean` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DidYouMeans {
    // Attributes
    pub count: u32,

    pub didyoumean: Vec<DidYouMean>,
}

/// Provides a suggestion for a different query than the one provided.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DidYouMean {
    // Attributes
    pub score: String, // TODO: find a way to put a floating point here.
    pub level: String,

    #[serde(rename="$value")]
    pub value: String,
}

/// Generated when a foreign language is detected in a query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LanguageMsg {
    // Attributes
    pub english: String,
    pub other: Option<String>,
}

/// Provides information for queries regarding a topic under development.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FutureTopic {
    // Attributes
    pub topic: String,
    pub msg: String,
}

/// A series of `RelatedExample` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RelatedExamples {
    // Attributes
    pub count: u32,

    #[serde(rename = "relatedexample")]
    pub related_example: Vec<RelatedExample>,
}

/// Provides a link to an HTML page of related examples on the requested topic.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RelatedExample {
    // Attributes
    pub input: String,
    pub desc: String,
    pub category: String,
    pub categorythumb: Url,
    pub categorypage: Url,
}

/// Provides a link to an HTML page of sample queries on the requested topic.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ExamplePage {
    // Attributes
    pub category: String,
    pub url: Url,
}

/// A series of `Warning` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Warnings {
    // Attributes
    pub count: u32,

    // TODO: find a way to merge these into an enum?
    pub spellcheck: Option<Vec<Spellcheck>>,
    pub delimeters: Option<Vec<Delimeters>>,
    pub translation: Option<Vec<Translation>>,
    pub reinterpret: Option<Vec<Reinterpret>>,
}

/// Provides word and suggestion attributes as alternative spellings for a word
/// in the query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Spellcheck {
    // Attributes
    word: String,
    suggestion: String,
    text: String,
}

/// Represents a warning regarding mismatched delimiters in a query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Delimeters {
    // Attributes
    text: String,
}

/// Represents a word or a phrase which was translated in the query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Translation {
    // Attributes
    phrase: String,
    trans: String,
    lang: String,
    text: String,
}

/// Represents a warning that the query was reinterpred.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Reinterpret {
    // Attributes
    text: String,

    alternative: Vec<Alternative>,
}

/// An alternative interpretation of an element in a query.
pub type Alternative = String;

/// Provides a link to a generalization of the requested query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Generalization {
    pub topic: String,
    pub desc: String,
    pub url: Url,
}

/// A series of `Source` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sources {
    // Attributes
    pub count: u32,

    pub source: Vec<Source>,
}

/// A link to a web page of source information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Source {
    // Attributes
    pub url: String,
    pub text: String,
}

/// The value of an assumption.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Value {
    // Attributes
    pub name: String,
    pub desc: String,
    pub input: String,
    //pub word: Option<String>,
}

/// `Pod` elements are subelements of `QueryResult`. Each contains the results
/// for a single pod.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Pod {
    // Attributes
    pub title: String,
    pub error: bool,
    pub position: u32,
    pub scanner: String,
    pub id: String,
    pub numsubpods: u32,
    pub primary: Option<bool>,

    pub subpod: Vec<Subpod>,
    pub states: Option<States>,
    pub infos: Option<Infos>,
    // TODO: find a way to parse errors.
    // The naive method (uncommenting the following line) doesn't work as `Pod`
    // also contains an `error` attribute.
    //pub error: Option<Error>,
    pub sounds: Option<Sounds>,
    pub definitions: Option<Definitions>,
    pub notes: Option<Notes>,
}

/// A series of `State` attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct States {
    // Attributes
    pub count: u32,

    pub state: Vec<State>,
    pub statelist: Option<Vec<Statelist>>,
}

/// A series of `State` attributes, combined into a list.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Statelist {
    // Attributes
    pub count: u32,
    pub value: String,

    pub state: Vec<State>,
}

/// An alternative state which is available to a `Pod` or `Subpod`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct State {
    // Attributes
    pub name: String,
    pub input: String,
}

/// A series of `Info` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Infos {
    // Attributes
    pub count: u32,

    pub info: Vec<Info>,
}

/// A container for extra information regarding the contents of a pod.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Info {
    // TODO: uncomment this once `serde-xml` implements `Option` deserialization
    // in this situation.
    //pub text: Option<String>,
    pub img: Option<Vec<Img>>,
    pub link: Option<Vec<Link>>,
    pub units: Option<Vec<Units>>,
}

/// A series of `Sound` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sounds {
    // Attributes
    pub count: u32,

    pub sound: Vec<Sound>,
}

/// Provides a link to a playable sound file.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sound {
    // Attributes
    pub url: Url,
    #[serde(rename = "type")]
    pub mimetype: String, // TODO: replace this with a Mimetype type.
}

/// A series of `Definition` attributes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Definitions {
    // Attributes
    pub count: u32,

    pub definition: Vec<Definition>,
}

/// An element containing a definition of a concept used in the query result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Definition {
    // Attributes
    pub word: String,
    pub desc: String,
}

/// A series of `Note` attributes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Notes {
    // Attributes
    pub count: u32,

    pub note: Vec<Note>,
}

/// A note made by Wolfram|Alpha regarding a query.
pub type Note = String;

/// A subelement of `Pod`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Subpod {
    // Attributes
    pub title: String,

    pub img: Option<Img>,
    pub plaintext: Option<Plaintext>,
    pub states: Option<States>,
}

/// A textual representation of a single subpod.
pub type Plaintext = String;

/// An HTML img element suitable for direct inclusion in a web page.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Img {
    pub src: Url,
    //pub alt: Option<String>,
    //pub title: Option<String>,
    pub width: u32,
    pub height: u32,
}

/// An HTML image map.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ImageMap {
    rect: Vec<Rect>,
}

/// A rectangular section of an HTML image map.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Rect {
    // Attributes
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub query: String,
    pub assumptions: String,
    pub title: String,
}

/// A series of `Unit` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Units {
    // Attributes
    pub count: u32,

    pub unit: Vec<Unit>,
    pub img: Img,
}

/// A table of unit abbreviations used inside a `Pod`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Unit {
    // Attributes
    pub short: String,
    pub long: String,
}

/// A standard link of some text pointing to a URL.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Link {
    // Attributes
    pub url: String,
    pub text: String,
    //pub title: Option<String>,
}

/// `Error` elements provide a code and a description of an error which has
/// occured while performing a query to the API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Error {
    pub code: u32,
    pub msg: String,
}

impl Deserialize for States {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer,
    {
        enum Field { Count, State, Statelist };

        impl Deserialize for Field {
            #[inline]
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: Deserializer,
            {
                struct FieldVisitor;

                impl Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_usize<E>(&mut self, value: usize) -> Result<Field, E>
                        where E: SerdeError,
                    {
                        match value {
                            0usize => { Ok(Field::Count) },
                            1usize => { Ok(Field::State) },
                            2usize => { Ok(Field::Statelist) },
                            _ => Err(SerdeError::unknown_field(value.to_string().as_ref())),
                        }
                    }

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: SerdeError,
                    {
                        match value {
                            "count" => { Ok(Field::Count) },
                            "state" => { Ok(Field::State) },
                            "statelist" => { Ok(Field::Statelist) },
                            _ => Err(SerdeError::unknown_field(value)),
                        }
                    }

                    fn visit_bytes<E>(&mut self, value: &[u8]) -> Result<Field, E>
                        where E: SerdeError,
                    {
                        match value {
                            b"count" => { Ok(Field::Count) },
                            b"state" => { Ok(Field::State) },
                            b"statelist" => { Ok(Field::Statelist) },
                            _ => Err(SerdeError::unknown_field(String::from_utf8_lossy(value).as_ref())),
                        }
                    }
                }

                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct StatesVisitor;

        impl Visitor for StatesVisitor {
            type Value = States;

            #[inline]
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<States, V::Error>
                where V: SeqVisitor,
            {
                let count = match match visitor.visit::<u32>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(0usize));
                    },
                };
                let state = match match visitor.visit::<Vec<State>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(1usize));
                    },
                };
                let statelist = match match visitor.visit::<Option<Vec<Statelist>>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(2usize));
                    },
                };
                match visitor.end() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                };
                Ok(States{
                    count: count,
                    state: state,
                    statelist: statelist,
                })
            }

            #[inline]
            fn visit_map<V>(&mut self, mut visitor: V) -> Result<States, V::Error>
                where V: MapVisitor,
            {
                let mut count: Option<u32> = None;
                let mut state: Option<Vec<State>> = None;
                let mut statelist: Option<Option<Vec<Statelist>>> = None;
                while let Some(key) = match visitor.visit_key::<Field>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    match key {
                        Field::Count => {
                            if count.is_some() {
                                return Err(<V::Error as
                                               SerdeError>::duplicate_field("count"));
                            }
                            count = Some(match visitor.visit_value::<u32>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        },
                        // The next two match arms alone have been modified from
                        // the original, generated deserializer in order to
                        // support out of order elements.
                        Field::State => {
                            let more_state = Some(match visitor.visit_value::<Vec<State>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                            if let Some(mut more_state) = more_state {
                                if state.is_none() {
                                    state = Some(more_state);
                                } else {
                                    if let Some(mut state) = state.as_mut() {
                                        state.append(&mut more_state);
                                    }
                                }
                            }
                        },
                        Field::Statelist => {
                            let more_statelist = Some(match visitor.visit_value::<Option<Vec<Statelist>>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                            if let Some(more_statelist) = more_statelist {
                                if let Some(mut more_statelist) = more_statelist {
                                    if statelist.is_none() {
                                        statelist = Some(Some(more_statelist));
                                    } else {
                                        if let Some(mut statelist) = statelist.as_mut() {
                                            if let Some(statelist) = statelist.as_mut() {
                                                statelist.append(&mut more_statelist);
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
                match visitor.end() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                };
                let count = match count {
                    Some(count) => count,
                    None => match visitor.missing_field("count") {
                        Result::Ok(val) => val,
                        Result::Err(err) => return Result::Err(From::from(err)),
                    },
                };
                let state = match state {
                    Some(state) => state,
                    None => match visitor.missing_field("state") {
                        Result::Ok(val) => val,
                        Result::Err(err) => return Result::Err(From::from(err)),
                    },
                };
                let statelist = match statelist {
                    Some(statelist) => statelist,
                    None => match visitor.missing_field("statelist") {
                        Result::Ok(val) => val,
                        Result::Err(err) => return Result::Err(From::from(err)),
                    },
                };
                Ok(States{
                    count: count,
                    state: state,
                    statelist: statelist,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "count",
            "state",
            "statelist",
        ];
        deserializer.deserialize_struct("States", FIELDS, StatesVisitor)
    }
}

// This is a custom deserializer implementation, written for the purpose of
// being able to handle Wolfram|Alpha `QueryResult`s properly. This is necessary
// because Wolfram|Alpha returns an `error` attribute (always) and an `error`
// element (only when there's an error).
//
// This is nearly identical to the generated deserializer, except for in certain
// locations, where modifications have been indicated.
impl Deserialize for QueryResult {
    fn deserialize<__D>(deserializer: &mut __D) -> Result<QueryResult, __D::Error>
        where __D: Deserializer,
    {
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __field7,
            __field8,
            __field9,
            __field10,
            __field11,
            __field12,
            __field13,
            __field14,
            __field15,
            __field16,
            __field17,
            __field18,
            __field19,
            __field20,
            __field21,
            __field22,
            __field23,
            __field24,
            __field25,
            __ignore,
        }

        impl Deserialize for __Field {
            #[inline]
            fn deserialize<__D>(deserializer: &mut __D)
                                -> Result<__Field, __D::Error>
                where __D: Deserializer,
            {
                struct __FieldVisitor;

                impl Visitor for __FieldVisitor {
                    type Value = __Field;

                    fn visit_usize<__E>(&mut self,
                                        value: usize)
                                        -> Result<__Field, __E>
                        where __E: SerdeError
                    {
                        match value {
                            0usize => Ok(__Field::__field0),
                            1usize => Ok(__Field::__field1),
                            2usize => Ok(__Field::__field2),
                            3usize => Ok(__Field::__field3),
                            4usize => Ok(__Field::__field4),
                            5usize => Ok(__Field::__field5),
                            6usize => Ok(__Field::__field6),
                            7usize => Ok(__Field::__field7),
                            8usize => Ok(__Field::__field8),
                            9usize => Ok(__Field::__field9),
                            10usize => Ok(__Field::__field10),
                            11usize => Ok(__Field::__field11),
                            12usize => Ok(__Field::__field12),
                            13usize => Ok(__Field::__field13),
                            14usize => Ok(__Field::__field14),
                            15usize => Ok(__Field::__field15),
                            16usize => Ok(__Field::__field16),
                            17usize => Ok(__Field::__field17),
                            18usize => Ok(__Field::__field18),
                            19usize => Ok(__Field::__field19),
                            20usize => Ok(__Field::__field20),
                            21usize => Ok(__Field::__field21),
                            22usize => Ok(__Field::__field22),
                            23usize => Ok(__Field::__field23),
                            24usize => Ok(__Field::__field24),
                            25usize => Ok(__Field::__field25),
                            _ => Ok(__Field::__ignore),
                        }
                    }

                    fn visit_str<__E>(&mut self, value: &str) -> Result<__Field, __E>
                        where __E: SerdeError,
                    {
                        match value {
                            "success" => { Ok(__Field::__field0) }
                            // Renamed from `error_flag` to `error`.
                            "error" => {
                                Ok(__Field::__field1)
                            }
                            "numpods" => { Ok(__Field::__field2) }
                            "version" => { Ok(__Field::__field3) }
                            "datatypes" => {
                                Ok(__Field::__field4)
                            }
                            "timing" => { Ok(__Field::__field5) }
                            "timedout" => {
                                Ok(__Field::__field6)
                            }
                            "timedoutpods" => {
                                Ok(__Field::__field7)
                            }
                            "parsetiming" => {
                                Ok(__Field::__field8)
                            }
                            "parsetimedout" => {
                                Ok(__Field::__field9)
                            }
                            "recalculate" => {
                                Ok(__Field::__field10)
                            }
                            "id" => { Ok(__Field::__field11) }
                            "server" => { Ok(__Field::__field12) }
                            "related" => {
                                Ok(__Field::__field13)
                            }
                            "pod" => { Ok(__Field::__field14) }
                            "assumptions" => {
                                Ok(__Field::__field15)
                            }
                            "sources" => {
                                Ok(__Field::__field16)
                            }
                            // Commented out to avoid match arm conflicts..
                            //"error" => { Ok(__Field::__field17) }
                            "tips" => { Ok(__Field::__field18) }
                            "didyoumeans" => {
                                Ok(__Field::__field19)
                            }
                            "languagemsg" => {
                                Ok(__Field::__field20)
                            }
                            "futuretopic" => {
                                Ok(__Field::__field21)
                            }
                            "relatedexamples" => {
                                Ok(__Field::__field22)
                            }
                            "examplepage" => {
                                Ok(__Field::__field23)
                            }
                            "generalization" => {
                                Ok(__Field::__field24)
                            }
                            "warnings" => {
                                Ok(__Field::__field25)
                            }
                            _ => Ok(__Field::__ignore),
                        }
                    }

                    fn visit_bytes<__E>(&mut self,
                                        value: &[u8])
                                        -> Result<__Field, __E>
                        where __E: SerdeError
                    {
                        match value {
                            b"success" => Ok(__Field::__field0),
                            // Renamed from `error_flag` to `error`.
                            b"error" => Ok(__Field::__field1),
                            b"numpods" => Ok(__Field::__field2),
                            b"version" => Ok(__Field::__field3),
                            b"datatypes" => Ok(__Field::__field4),
                            b"timing" => Ok(__Field::__field5),
                            b"timedout" => Ok(__Field::__field6),
                            b"timedoutpods" => Ok(__Field::__field7),
                            b"parsetiming" => Ok(__Field::__field8),
                            b"parsetimedout" => Ok(__Field::__field9),
                            b"recalculate" => Ok(__Field::__field10),
                            b"id" => Ok(__Field::__field11),
                            b"server" => Ok(__Field::__field12),
                            b"related" => Ok(__Field::__field13),
                            b"pod" => Ok(__Field::__field14),
                            b"assumptions" => Ok(__Field::__field15),
                            b"sources" => Ok(__Field::__field16),
                            // Commented out to avoid match arm conflicts..
                            // b"error" => { Ok(__Field::__field17) }
                            b"tips" => Ok(__Field::__field18),
                            b"didyoumeans" => Ok(__Field::__field19),
                            b"languagemsg" => Ok(__Field::__field20),
                            b"futuretopic" => Ok(__Field::__field21),
                            b"relatedexamples" => Ok(__Field::__field22),
                            b"examplepage" => Ok(__Field::__field23),
                            b"generalization" => Ok(__Field::__field24),
                            b"warnings" => Ok(__Field::__field25),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                }

                deserializer.deserialize_struct_field(__FieldVisitor)
            }
        }

        struct __Visitor;

        impl Visitor for __Visitor {
            type Value = QueryResult;

            #[inline]
            fn visit_seq<__V>(&mut self,
                              mut visitor: __V)
                              -> Result<QueryResult, __V::Error>
                where __V: SeqVisitor
            {
                let __field0 = match match visitor.visit::<bool>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(0usize));
                    }
                };
                let __field1 = match match visitor.visit::<bool>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(1usize));
                    }
                };
                let __field2 = match match visitor.visit::<u32>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(2usize));
                    }
                };
                let __field3 = match match visitor.visit::<Option<String>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(3usize));
                    }
                };
                let __field4 = match match visitor.visit::<String>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(4usize));
                    }
                };
                let __field5 = match match visitor.visit::<f64>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(5usize));
                    }
                };
                let __field6 = match match visitor.visit::<String>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(6usize));
                    }
                };
                let __field7 = match match visitor.visit::<Option<String>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(7usize));
                    }
                };
                let __field8 = match match visitor.visit::<f64>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(8usize));
                    }
                };
                let __field9 = match match visitor.visit::<Option<bool>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(9usize));
                    }
                };
                let __field10 = match match visitor.visit::<Option<String>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(10usize));
                    }
                };
                let __field11 = match match visitor.visit::<Option<String>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(11usize));
                    }
                };
                let __field12 = match match visitor.visit::<Option<u32>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(12usize));
                    }
                };
                let __field13 = match match visitor.visit::<Option<String>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(13usize));
                    }
                };
                let __field14 = match match visitor.visit::<Option<Vec<Pod>>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(14usize));
                    }
                };
                let __field15 = match match visitor.visit::<Option<Assumptions>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(15usize));
                    }
                };
                let __field16 = match match visitor.visit::<Option<Sources>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(16usize));
                    }
                };
                let __field17 = match match visitor.visit::<Option<Error>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(17usize));
                    }
                };
                let __field18 = match match visitor.visit::<Option<Tips>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(18usize));
                    }
                };
                let __field19 = match match visitor.visit::<Option<DidYouMeans>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(19usize));
                    }
                };
                let __field20 = match match visitor.visit::<Option<LanguageMsg>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(20usize));
                    }
                };
                let __field21 = match match visitor.visit::<Option<FutureTopic>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(21usize));
                    }
                };
                let __field22 = match match visitor.visit::<Option<RelatedExamples>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(22usize));
                    }
                };
                let __field23 = match match visitor.visit::<Option<ExamplePage>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(23usize));
                    }
                };
                let __field24 = match match visitor.visit::<Option<Generalization>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(24usize));
                    }
                };
                let __field25 = match match visitor.visit::<Option<Warnings>>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    Some(value) => value,
                    None => {
                        match visitor.end() {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        };
                        return Err(SerdeError::invalid_length(25usize));
                    }
                };
                match visitor.end() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                };
                Ok(QueryResult {
                    success: __field0,
                    error_flag: __field1,
                    numpods: __field2,
                    version: __field3,
                    datatypes: __field4,
                    timing: __field5,
                    timedout: __field6,
                    timedoutpods: __field7,
                    parsetiming: __field8,
                    parsetimedout: __field9,
                    recalculate: __field10,
                    id: __field11,
                    server: __field12,
                    related: __field13,
                    pod: __field14,
                    assumptions: __field15,
                    sources: __field16,
                    error: __field17,
                    tips: __field18,
                    didyoumeans: __field19,
                    languagemsg: __field20,
                    futuretopic: __field21,
                    relatedexamples: __field22,
                    examplepage: __field23,
                    generalization: __field24,
                    warnings: __field25,
                })
            }

            #[inline]
            fn visit_map<__V>(&mut self,
                              mut visitor: __V)
                              -> Result<QueryResult, __V::Error>
                where __V: MapVisitor
            {
                let mut __field0: Option<bool> = None;
                let mut __field1: Option<bool> = None;
                let mut __field2: Option<u32> = None;
                let mut __field3: Option<Option<String>> = None;
                let mut __field4: Option<String> = None;
                let mut __field5: Option<f64> = None;
                let mut __field6: Option<String> = None;
                let mut __field7: Option<Option<String>> = None;
                let mut __field8: Option<f64> = None;
                let mut __field9: Option<Option<bool>> = None;
                let mut __field10: Option<Option<String>> = None;
                let mut __field11: Option<Option<String>> = None;
                let mut __field12: Option<Option<u32>> = None;
                let mut __field13: Option<Option<String>> = None;
                let mut __field14: Option<Option<Vec<Pod>>> = None;
                let mut __field15: Option<Option<Assumptions>> = None;
                let mut __field16: Option<Option<Sources>> = None;
                let mut __field17: Option<Option<Error>> = None;
                let mut __field18: Option<Option<Tips>> = None;
                let mut __field19: Option<Option<DidYouMeans>> = None;
                let mut __field20: Option<Option<LanguageMsg>> = None;
                let mut __field21: Option<Option<FutureTopic>> = None;
                let mut __field22: Option<Option<RelatedExamples>> = None;
                let mut __field23: Option<Option<ExamplePage>> = None;
                let mut __field24: Option<Option<Generalization>> = None;
                let mut __field25: Option<Option<Warnings>> = None;
                while let Some(key) = match visitor.visit_key::<__Field>() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                } {
                    match key {
                        __Field::__field0 => {
                            if __field0.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("success"));
                            }
                            __field0 = Some(match visitor.visit_value::<bool>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        // Modified to handle both the `error` attribute and
                        // element. If the attribute has already been
                        // deserialized, assume the next item is the `error`
                        // element.
                        __Field::__field1 => {
                            if __field1.is_some() {
                                if __field17.is_some() {
                                    return Err(<__V::Error as
                                                   SerdeError>::duplicate_field("error"));
                                }
                                __field17 = Some(match visitor.visit_value::<Option<Error>>() {
                                    Result::Ok(val) => val,
                                    Result::Err(err) => return Result::Err(From::from(err)),
                                });
                                // return Err(<__V::Error as
                                // SerdeError>::duplicate_field("error_flag"));
                            } else {
                                __field1 = Some(match visitor.visit_value::<bool>() {
                                    Result::Ok(val) => val,
                                    Result::Err(err) => return Result::Err(From::from(err)),
                                });
                            }
                        }
                        __Field::__field2 => {
                            if __field2.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("numpods"));
                            }
                            __field2 = Some(match visitor.visit_value::<u32>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field3 => {
                            if __field3.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("version"));
                            }
                            __field3 = Some(match visitor.visit_value::<Option<String>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field4 => {
                            if __field4.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("datatypes"));
                            }
                            __field4 = Some(match visitor.visit_value::<String>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field5 => {
                            if __field5.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("timing"));
                            }
                            __field5 = Some(match visitor.visit_value::<f64>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field6 => {
                            if __field6.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("timedout"));
                            }
                            __field6 = Some(match visitor.visit_value::<String>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field7 => {
                            if __field7.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("timedoutpods"));
                            }
                            __field7 = Some(match visitor.visit_value::<Option<String>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field8 => {
                            if __field8.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("parsetiming"));
                            }
                            __field8 = Some(match visitor.visit_value::<f64>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field9 => {
                            if __field9.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("parsetimedout"));
                            }
                            __field9 = Some(match visitor.visit_value::<Option<bool>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field10 => {
                            if __field10.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("recalculate"));
                            }
                            __field10 = Some(match visitor.visit_value::<Option<String>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field11 => {
                            if __field11.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("id"));
                            }
                            __field11 = Some(match visitor.visit_value::<Option<String>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field12 => {
                            if __field12.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("server"));
                            }
                            __field12 = Some(match visitor.visit_value::<Option<u32>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field13 => {
                            if __field13.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("related"));
                            }
                            __field13 = Some(match visitor.visit_value::<Option<String>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field14 => {
                            if __field14.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("pod"));
                            }
                            __field14 = Some(match visitor.visit_value::<Option<Vec<Pod>>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field15 => {
                            if __field15.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("assumptions"));
                            }
                            __field15 = Some(match visitor.visit_value::<Option<Assumptions>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field16 => {
                            if __field16.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("sources"));
                            }
                            __field16 = Some(match visitor.visit_value::<Option<Sources>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field17 => {
                            if __field17.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("error"));
                            }
                            __field17 = Some(match visitor.visit_value::<Option<Error>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field18 => {
                            if __field18.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("tips"));
                            }
                            __field18 = Some(match visitor.visit_value::<Option<Tips>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field19 => {
                            if __field19.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("didyoumeans"));
                            }
                            __field19 = Some(match visitor.visit_value::<Option<DidYouMeans>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field20 => {
                            if __field20.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("languagemsg"));
                            }
                            __field20 = Some(match visitor.visit_value::<Option<LanguageMsg>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field21 => {
                            if __field21.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("futuretopic"));
                            }
                            __field21 = Some(match visitor.visit_value::<Option<FutureTopic>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field22 => {
                            if __field22.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("relatedexamples"));
                            }
                            __field22 =
                                Some(match visitor.visit_value::<Option<RelatedExamples>>() {
                                    Result::Ok(val) => val,
                                    Result::Err(err) => return Result::Err(From::from(err)),
                                });
                        }
                        __Field::__field23 => {
                            if __field23.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("examplepage"));
                            }
                            __field23 = Some(match visitor.visit_value::<Option<ExamplePage>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        __Field::__field24 => {
                            if __field24.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("generalization"));
                            }
                            __field24 =
                                Some(match visitor.visit_value::<Option<Generalization>>() {
                                    Result::Ok(val) => val,
                                    Result::Err(err) => return Result::Err(From::from(err)),
                                });
                        }
                        __Field::__field25 => {
                            if __field25.is_some() {
                                return Err(<__V::Error as
                                               SerdeError>::duplicate_field("warnings"));
                            }
                            __field25 = Some(match visitor.visit_value::<Option<Warnings>>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            });
                        }
                        _ => {
                            let _ = match visitor.visit_value::<IgnoredAny>() {
                                Result::Ok(val) => val,
                                Result::Err(err) => return Result::Err(From::from(err)),
                            };
                        }
                    }
                }
                match visitor.end() {
                    Result::Ok(val) => val,
                    Result::Err(err) => return Result::Err(From::from(err)),
                };
                let __field0 = match __field0 {
                    Some(__field0) => __field0,
                    None => {
                        match visitor.missing_field("success") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field1 = match __field1 {
                    Some(__field1) => __field1,
                    // TODO: modified
                    None => {
                        match visitor.missing_field("error") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field2 = match __field2 {
                    Some(__field2) => __field2,
                    None => {
                        match visitor.missing_field("numpods") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field3 = match __field3 {
                    Some(__field3) => __field3,
                    None => {
                        match visitor.missing_field("version") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field4 = match __field4 {
                    Some(__field4) => __field4,
                    None => {
                        match visitor.missing_field("datatypes") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field5 = match __field5 {
                    Some(__field5) => __field5,
                    None => {
                        match visitor.missing_field("timing") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field6 = match __field6 {
                    Some(__field6) => __field6,
                    None => {
                        match visitor.missing_field("timedout") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field7 = match __field7 {
                    Some(__field7) => __field7,
                    None => {
                        match visitor.missing_field("timedoutpods") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field8 = match __field8 {
                    Some(__field8) => __field8,
                    None => {
                        match visitor.missing_field("parsetiming") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field9 = match __field9 {
                    Some(__field9) => __field9,
                    None => {
                        match visitor.missing_field("parsetimedout") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field10 = match __field10 {
                    Some(__field10) => __field10,
                    None => {
                        match visitor.missing_field("recalculate") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field11 = match __field11 {
                    Some(__field11) => __field11,
                    None => {
                        match visitor.missing_field("id") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field12 = match __field12 {
                    Some(__field12) => __field12,
                    None => {
                        match visitor.missing_field("server") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field13 = match __field13 {
                    Some(__field13) => __field13,
                    None => {
                        match visitor.missing_field("related") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field14 = match __field14 {
                    Some(__field14) => __field14,
                    None => {
                        match visitor.missing_field("pod") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field15 = match __field15 {
                    Some(__field15) => __field15,
                    None => {
                        match visitor.missing_field("assumptions") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field16 = match __field16 {
                    Some(__field16) => __field16,
                    None => {
                        match visitor.missing_field("sources") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field17 = match __field17 {
                    Some(__field17) => __field17,
                    None => {
                        match visitor.missing_field("error") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field18 = match __field18 {
                    Some(__field18) => __field18,
                    None => {
                        match visitor.missing_field("tips") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field19 = match __field19 {
                    Some(__field19) => __field19,
                    None => {
                        match visitor.missing_field("didyoumeans") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field20 = match __field20 {
                    Some(__field20) => __field20,
                    None => {
                        match visitor.missing_field("languagemsg") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field21 = match __field21 {
                    Some(__field21) => __field21,
                    None => {
                        match visitor.missing_field("futuretopic") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field22 = match __field22 {
                    Some(__field22) => __field22,
                    None => {
                        match visitor.missing_field("relatedexamples") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field23 = match __field23 {
                    Some(__field23) => __field23,
                    None => {
                        match visitor.missing_field("examplepage") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field24 = match __field24 {
                    Some(__field24) => __field24,
                    None => {
                        match visitor.missing_field("generalization") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                let __field25 = match __field25 {
                    Some(__field25) => __field25,
                    None => {
                        match visitor.missing_field("warnings") {
                            Result::Ok(val) => val,
                            Result::Err(err) => return Result::Err(From::from(err)),
                        }
                    }
                };
                Ok(QueryResult {
                    success: __field0,
                    error_flag: __field1,
                    numpods: __field2,
                    version: __field3,
                    datatypes: __field4,
                    timing: __field5,
                    timedout: __field6,
                    timedoutpods: __field7,
                    parsetiming: __field8,
                    parsetimedout: __field9,
                    recalculate: __field10,
                    id: __field11,
                    server: __field12,
                    related: __field13,
                    pod: __field14,
                    assumptions: __field15,
                    sources: __field16,
                    error: __field17,
                    tips: __field18,
                    didyoumeans: __field19,
                    languagemsg: __field20,
                    futuretopic: __field21,
                    relatedexamples: __field22,
                    examplepage: __field23,
                    generalization: __field24,
                    warnings: __field25,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "success",
            "error_flag",
            "numpods",
            "version",
            "datatypes",
            "timing",
            "timedout",
            "timedoutpods",
            "parsetiming",
            "parsetimedout",
            "recalculate",
            "id",
            "server",
            "related",
            "pod",
            "assumptions",
            "sources",
            "error",
            "tips",
            "didyoumeans",
            "languagemsg",
            "futuretopic",
            "relatedexamples",
            "examplepage",
            "generalization",
            "warnings",
        ];
        deserializer.deserialize_struct("QueryResult", FIELDS, __Visitor)
    }
}
