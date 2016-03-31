extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;
extern crate hyper;
extern crate dotenv;

use iron::status;
use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use hyper::Client;
use dotenv::dotenv;
use std::env;

fn main() {

    dotenv().ok();

    fn dist_cities(res: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        res.body.read_to_string(&mut payload).unwrap();
        println!("Response: {}", payload);
        let data = Json::from_str(&payload).unwrap();
        let obj = data.as_object().unwrap();
        let mut from = obj.get("from").unwrap().to_string();
        let mut to = obj.get("to").unwrap().to_string();

        from = (&from[1..from.len()-1]).to_string();
        to = (&to[1..to.len()-1]).to_string();

        /* TODO: HTTP post to map api */
        /* TODO: URL ENCODING */
        let key = env::var("KEY").unwrap();
        let url_query = format!("http://www.mapquestapi.com/directions/v2/route?from={}&to={}&key={}", from, to, key);

        let client = Client::new();
        let mut res = client.get(&url_query).send().unwrap();
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        println!("{}", s);

        let payload = json::encode(&s).unwrap();
        Ok(Response::with(((status::Ok), payload)))
    }

    let mut mount = Mount::new();
    mount
        .mount("/index.html", Static::new(Path::new("../index.html")))
        .mount("/api/citydistance/", dist_cities);
    println!("Listening on port 8000");
    Iron::new(mount).http("localhost:8000").unwrap();
}
