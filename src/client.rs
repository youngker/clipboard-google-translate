use hyper::rt::{self, Future, Stream};
use hyper::Client;
use std::sync::mpsc;
use url::form_urlencoded;

const TRANSLATE_URL: &'static str = "http://translate.googleapis.com/translate_a/single?";

enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }
    pub fn make_url(&self, search: &str) -> hyper::Uri {
        form_urlencoded::Serializer::new(String::from(TRANSLATE_URL))
            .append_pair("client", "gtx")
            .append_pair("ie", "UTF-8")
            .append_pair("oe", "UTF-8")
            .append_pair("sl", "auto")
            .append_pair("tl", "ko")
            .append_pair("q", search)
            .append_pair("dt", "t")
            .finish()
            .parse::<hyper::Uri>()
            .unwrap()
    }

    fn fetch_json(
        &mut self,
        uri: hyper::Uri,
    ) -> impl Future<Item = serde_json::Value, Error = FetchError> {
        Client::new()
            .get(uri)
            .and_then(|res| res.into_body().concat2())
            .from_err::<FetchError>()
            .and_then(|body| {
                let s = ::std::str::from_utf8(&body).expect("httpbin sends utf-8 JSON");
                Ok(serde_json::from_str(&s).unwrap())
            })
            .from_err()
    }

    pub fn request(&mut self, uri: hyper::Uri) -> String {
        let (tx, rx) = mpsc::channel();

        let fut = self
            .fetch_json(uri)
            .map(move |json| {
                let mut msg = String::new();
                let v: Vec<_> = json[0].as_array().unwrap().to_vec();
                for item in &v {
                    msg.push_str(item[0].as_str().unwrap());
                }
                println!("{}", msg);
                tx.send(msg).unwrap();
            })
            .map_err(|e| match e {
                FetchError::Http(e) => eprintln!("http error: {}", e),
                FetchError::Json(e) => eprintln!("json parsing error: {}", e),
            });
        rt::run(fut);
        let received = rx.recv().unwrap();
        received
    }
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
