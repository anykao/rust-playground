#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate oauth2_rocket::{PROVIDERS, Provider, Providers};

use rocket::response::Redirect;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct Code<'r> {
    code: &'r str,
}

#[get("/<provider>/begin")]
fn begin(p: Providers, provider: &str) -> Redirect {
    println!("{:?}", p);

    // let url = config.authorize_url(String::new());
    // println!("{}", url);
    Redirect::to("/")
}

#[get("/<provider>/callback?<code>")]
fn callback(p: Providers, provider: &str, code: Code) -> &'static str {
    // println!("{}/callback?{}", p, code.code);

    // config.exchange(code.code.to_string());
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/connect", routes![begin, callback])
        .launch();
}
