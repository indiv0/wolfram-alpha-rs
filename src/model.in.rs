// TODO: add `MathmMl` struct.

/// `QueryResult` is the outer wrapper for all results from the query function.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct QueryResult {
    // Attributes
    pub success: bool,
    pub error: bool,
    pub numpods: u32,
    pub version: String, // TODO: replace this with a better type.
    pub datatypes: String, // TODO: possibly replace this with an enum?
    pub timing: f64,
    pub timedout: String,
    pub timedoutpods: String,
    pub parsetiming: f64,
    pub parsetimedout: bool,
    pub recalculate: String,
    pub id: String,
    //pub host: String, // FIXME
    pub server: u32,
    pub related: String,

    pub pod: Vec<Pod>,
    pub assumptions: Option<Assumptions>,
    pub sources: Option<Sources>,
    // TODO: find a way to parse errors.
    // The naive method (uncommenting the following line) doesn't work as
    // `QueryResult` also contains an `error` attribute.
    //pub error: Option<Error>,
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
    pub other: String,
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
}

/// A series of `State` attributes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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
