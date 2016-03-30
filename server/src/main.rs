extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;

use iron::status;
use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::json::Json;

#[derive(RustcEncodable)]
struct Distance {
    from: String,
    to: String,
    dist: i32
}

fn main() {

    fn dist_cities(res: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        res.body.read_to_string(&mut payload).unwrap();
        println!("Response: {}", payload);
        let data = Json::from_str(&payload).unwrap();
        let obj = data.as_object().unwrap();
        let from = obj.get("from").unwrap();
        let to = obj.get("to").unwrap();
        let dist = Distance {from: from.to_string(), to: to.to_string(), dist: 1337};
        let payload = json::encode(&dist).unwrap();
        Ok(Response::with(((status::Ok), payload)))
    }

    let mut mount = Mount::new();
    let path = Path::new("../index.html");
    mount
        .mount("/", Static::new(path))
        .mount("/citydistance/", dist_cities);
    println!("Listening on port 8000");
    Iron::new(mount).http("localhost:8000").unwrap();
}
