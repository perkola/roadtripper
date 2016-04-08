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
use iron::error::HttpError;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
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
        from = (&from[1..from.len()-1]).to_string().replace(" ", "%20");
        to = (&to[1..to.len()-1]).to_string().replace(" ", "%20");

        /* TODO: HTTP post to map api */
        /* TODO: URL ENCODING */
        let key = env::var("KEY").unwrap();
        let url_query = format!("http://www.mapquestapi.com/directions/v2/route?from={}&to={}&key={}", from, to, key);

        println!("ENCODED: {}", url_query);

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

        let file_path_string = &("../roadtrips/".to_string() + &id + ".rdt");
        let file_path = Path::new(file_path_string);
        let mut f = File::create(file_path).unwrap();

        // TODO: Error handling
        f.write_all(&payload.into_bytes());

        println!("Wrote to file {}", file_path_string);

        Ok(Response::with(((status::Ok), id)))
    }

    fn get_roadtrip(res: &mut Request) -> IronResult<Response> {
        let mut id = String::new();

        // TODO: Error handling
        res.body.read_to_string(&mut id).unwrap();

        let file_path_string = &("../roadtrips/".to_string() + &id + ".rdt");

        // TODO: Error handling
        let mut f = match File::open(file_path_string) {
            Ok(val) => val,
            Err(err) => return Err(IronError::new(HttpError::Io(err), status::InternalServerError))
        };
        let mut s = String::new();
        f.read_to_string(&mut s);
        Ok(Response::with(((status::Ok), s)))
    }

    let mut mount = Mount::new();
    mount
        .mount("/index.html", Static::new(Path::new("../public/index.html")))
        .mount("/api/citydistance", dist_cities)
        .mount("/api/roadtrip/save", save_roadtrip)
        .mount("/roadtrip", get_roadtrip);
    println!("Listening on port 8000");
    Iron::new(mount).http("localhost:8000").unwrap();
}
