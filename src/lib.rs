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
