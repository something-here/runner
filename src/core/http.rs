use iron::prelude::*;
use iron::status;
use iron::status::Status;
use iron::Url;
use iron::modifiers::Redirect;
use hbs::Template;
use hbs::handlebars::to_json;
use persistent::Read;
use serde_json;
use serde_json::value::{Map, Value};
use serde_json::Value::String as SerdeString;
use iron_sessionstorage::Value as SessionValue;
use iron_sessionstorage::traits::SessionRequestExt;

use core::utils::*;

#[derive(Debug, Clone)]
pub struct SessionObject {
    pub user: String
}

impl SessionValue for SessionObject {

    fn get_key() -> &'static str {

        "runner"
    }

    fn into_raw(self) -> String {

        self.user
    }

    fn from_raw(value: String) -> Option<SessionObject> {

        if value.is_empty() {

            None
        } else {

            Some(SessionObject {
                user: value
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewData(Map<String, Value>);

impl ViewData {

    pub fn new(req: &mut Request) -> ViewData {

        let config = get_config(req);
        let path = config.get("path").unwrap();
        let static_path = config.get("static_path").unwrap();
        let session_wrapper = req.session().get::<SessionObject>().unwrap();

        let mut map = Map::new();
        map.insert("path".to_owned(), to_json(&path));
        map.insert("static_path".to_owned(), to_json(&static_path));

        if session_wrapper.is_some() {
            map.insert("user".to_owned(), to_json(&session_wrapper.unwrap().into_raw()));
        }

        ViewData(map)
    }

    pub fn insert(&mut self, key: &str, value: Value) -> &mut Self {

        self.0.insert(key.to_owned(), value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonData {
    pub success: bool,
    pub message: String,
    pub data: Value
}

impl JsonData {

    pub fn new() -> JsonData {

        JsonData {
            success: true,
            message: "".to_owned(),
            data: SerdeString("".to_owned())
        }
    }
}

pub fn respond_view(template_path: &str, data: &ViewData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(Template::new(template_path, data.0.clone()));

    Ok(res)
}

pub fn respond_json(data: &JsonData) -> IronResult<Response> {

    let mut res = Response::new();

    res.set_mut(status::Ok)
        .set_mut(mime!(Application/Json))
        .set_mut(serde_json::to_string(data).unwrap());

    Ok(res)
}

pub fn redirect_to(url: &str) -> IronResult<Response> {

    let url = Url::parse(url).unwrap();
    let res = Response::with((status::Found, Redirect(url)));

    return Ok(res);
}
