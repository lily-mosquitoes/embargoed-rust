//! `embargoed` is a Rust library for [Rocket](https://rocket.rs/), which implements a fairing that can be used to block all requests from Russian IP addresses and display a [pro-Ukraine message instead](https://raw.githubusercontent.com/rameerez/embargoed/main/public/embargoed-message.jpg).
//!
//! This is a port of the [Embargoed](https://github.com/rameerez/embargoed) Gem (for Ruby/Rails) by [rameerez](https://github.com/rameerez).
//! > check out their official [list of ports for other frameworks](https://github.com/rameerez/embargoed-list).
//!
//! to use simply attach the fairing to your Rocket build, like in this example (using Rocket version 0.5.0-rc.1):
//! ```
//! #[macro_use] extern crate rocket;
//!
//! #[get("/test")]
//! fn test() -> &'static str {
//!     "not embargoed"
//! }
//!
//! #[launch]
//! fn rocket() -> _ {
//!     rocket::build()
//!         .attach(embargoed::fairing())
//!         .mount("/", routes![test])
//! }
//! ```

use maxminddb;
use std::net::IpAddr;
use rocket::{
    Rocket, Build, Request, Response,
    http::ContentType,
    fairing::{self, Fairing, Info, Kind},
};
use std::io;

static DATABASE: &'static [u8] = include_bytes!("Geoacumen-Country.mmdb");

static HTML: &'static str = include_str!("embargoed.html");

pub struct Embargoed;

pub fn fairing() -> Embargoed {
    Embargoed {}
}

fn get_reader() -> maxminddb::Reader<&'static [u8]> {
    maxminddb::Reader::from_source(DATABASE)
        .expect("Database error")
}

fn get_country(ip: IpAddr) -> String {
    let reader = get_reader();

    let country: maxminddb::geoip2::Country = match reader
        .lookup(ip) {
            Ok(country) => country,
            Err(_) => return "None".to_string(),
        };

    if let Some(c) = country.country {
        if let Some(code) = c.iso_code {
            code.to_string()
        } else {
            "None".to_string()
        }
    } else {
        "None".to_string()
    }
}

#[rocket::async_trait]
impl Fairing for Embargoed {
    fn info(&self) -> Info {
        Info {
            name: "Embargo for Russian IPs",
            kind: Kind::Ignite | Kind::Response,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let _ = get_reader();
        Ok(rocket)
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        match request.client_ip() {
            Some(ip) => {
                let country = get_country(ip);
                if country == "RU" {
                    response.set_sized_body(HTML.len(), io::Cursor::new(HTML));

                    response.remove_header("set-cookie");
                    response.set_header(ContentType::HTML);
                }
            },
            None => (),
        }
    }
}
