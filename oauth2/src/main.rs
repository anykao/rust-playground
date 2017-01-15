#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate oauth2;

use rocket::response::Redirect;
// use oauth2::provider;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct Code<'r> {
    code: &'r str,
}


#[get("/<provider>/start")]
fn regist(provider: &str) -> Redirect {
    println!("{}", provider);

    let config = oauth2::Config::new("cf07a51bf0c95a35fff8",
                                     "89969eb236953ca9c0c86de8064f2d91fb69b14f",
                                     "https://github.com/login/oauth/authorize",
                                     "https://github.com/login/oauth/access_token");
    let url = config.authorize_url(String::new());
    println!("{}", url);
    Redirect::to(url.into_string().as_ref())
}

#[get("/<provider>/callback?<code>")]
fn callback(provider: &str, code: Code) -> &'static str {
    println!("{}/callback?{}", provider, code.code);

    let config = oauth2::Config::new("cf07a51bf0c95a35fff8",
                                     "89969eb236953ca9c0c86de8064f2d91fb69b14f",
                                     "https://github.com/login/oauth/authorize",
                                     "https://github.com/login/oauth/access_token");
    config.exchange(code.code.to_string());
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/connect", routes![regist, callback])
        .launch();
}
