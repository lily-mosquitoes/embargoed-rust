# Embargoed-Rust
`embargoed` is a Rust library for [Rocket](https://rocket.rs/), which implements a fairing that can be used to block all requests from Russian IP addresses and display a pro-Ukraine message instead.

This is a port of the [Embargoed](https://github.com/rameerez/embargoed) Gem (for Ruby/Rails) by [rameerez](https://github.com/rameerez).
> check out their official [list of ports for other frameworks](https://github.com/rameerez/embargoed-list).

This is the message which will replace all pages of your application:

<p align="center">
  <img src="https://github.com/rameerez/embargoed/blob/main/public/embargoed-message.jpg?raw=true" alt="Embargoed message displayed to Russian visitors" width="500"/>
</p>

## How to use
Add `embargoed` to your dependencies on `Cargo.toml`:
```
// --snip--

[dependencies]
embargoed = "0.1.0"
```

Then in your `rocket::build()` attach `embargoed::fairing()`, as in this example:
```
// --snip--

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(embargoed::fairing())
        .mount("/", routes![my_route])
}
```

You're done! Now all requests coming from Russian IP addresses will receive a response containing only the pro-Ukraine message depicted above!

## Collaborate
Please check out the [original project by rameerez](https://github.com/rameerez/embargoed) for porting this to other languages/frameworks.

Feel free to contact me or open a PR for contributing to this repository!
