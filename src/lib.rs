use maxminddb;
use std::net::IpAddr;
use rocket::{
    Rocket, Build, Request, Response,
    http::ContentType,
    fairing::{self, Fairing, Info, Kind},
};
use std::io;
