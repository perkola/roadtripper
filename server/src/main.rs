extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;
extern crate hyper;
extern crate dotenv;
extern crate rand;

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
use rand::Rng;

fn main() {

    dotenv().ok();

    fn dist_cities(res: &mut Request) -> IronResult<Response> {
        /* read payload to a string */
        let mut payload = String::new();
        res.body.read_to_string(&mut payload).unwrap();

        println!("Response: {}", payload);
        let data = Json::from_str(&payload).unwrap();
        let obj = data.as_object().unwrap();
        let mut from = obj.get("from").unwrap().to_string();
        let mut to = obj.get("to").unwrap().to_string();

        /* the strings are parsed as "city" rather than city. So we strip "". */
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

    fn save_roadtrip(res: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        res.body.read_to_string(&mut payload).unwrap();

        let id = rand::thread_rng()
            .gen_ascii_chars()
            .take(10)
            .collect::<String>();

        Ok(Response::with(((status::Ok, id))))
    }

    let mut mount = Mount::new();
    mount
        .mount("/index.html", Static::new(Path::new("../public/index.html")))
        .mount("/api/citydistance", dist_cities)
        .mount("/api/roadtrip/save", save_roadtrip);
    println!("Listening on port 8000");
    Iron::new(mount).http("localhost:8000").unwrap();
}
