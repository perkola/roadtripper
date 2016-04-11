extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate rustc_serialize;
extern crate hyper;
extern crate dotenv;
extern crate rand;
extern crate unicase;

use iron::{status, headers};
use iron::prelude::*;
use iron::error::HttpError;
use iron::method::Method::*;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use hyper::Client;
use dotenv::dotenv;
use std::env;
use rand::Rng;
use router::Router;
use unicase::UniCase;

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
        let key = env::var("GOOGLE_KEY").unwrap();
        let url_query = format!("https://maps.googleapis.com/maps/api/distancematrix/json?origins={}&destinations={}&key={}", from, to, key);

        println!("ENCODED: {}", url_query);

        let client = Client::new();
        let mut res = client.get(&url_query).send().unwrap();
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();
        println!("{}", s);

        let payload = json::encode(&s).unwrap();
        let resp = Response::with(((status::Ok), payload));
        Ok(resp)
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

        match f.write_all(&payload.into_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(IronError::new(HttpError::Io(err), status::InternalServerError))
        };

        println!("Wrote to file {}", file_path_string);

        Ok(Response::with(((status::Ok), id)))
    }

    fn get_roadtrip(res: &mut Request) -> IronResult<Response> {
        let mut id = String::new();

        // TODO: Error handling
        match res.body.read_to_string(&mut id) {
            Ok(_) => (),
            Err(err) => return Err(IronError::new(HttpError::Io(err), status::InternalServerError))
        };

        let file_path_string = &("../roadtrips/".to_string() + &id + ".rdt");

        // TODO: Error handling
        let mut f = match File::open(file_path_string) {
            Ok(val) => val,
            Err(err) => return Err(IronError::new(HttpError::Io(err), status::InternalServerError))
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(err) => return Err(IronError::new(HttpError::Io(err), status::InternalServerError))
        };
        Ok(Response::with(((status::Ok), s)))
    }

    let mut router = Router::new();
    router.get("/api/citydistance", dist_cities);
    router.post("/api/roadtrip/save", save_roadtrip);
    router.get("/roadtrip", get_roadtrip);

    let mut chain = Chain::new(router);
    chain.link_after(CorsFilter);
    println!("Listening on port 8000");
    Iron::new(chain).http("localhost:8000").unwrap();
}

struct CorsFilter;

impl iron::AfterMiddleware for CorsFilter {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Value("http://localhost:8080"));
        res.headers.set(headers::AccessControlAllowHeaders(
                vec![UniCase("accept".to_string()),
                UniCase("content-type".to_string())]));
        res.headers.set(headers::AccessControlAllowMethods(
                vec![Get]));
        Ok(res)
    }
}
