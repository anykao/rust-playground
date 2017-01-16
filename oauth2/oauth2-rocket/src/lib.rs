#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;


use std::env;
use std::sync::Mutex;
use std::collections::HashMap;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;

lazy_static! {
    pub static ref PRO: Mutex<Providers> = Mutex::new(Providers::new());
}

pub struct Provider {
    name: String,
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
}

impl Provider {
    pub fn new<T: Into<String>>(name: T,
                                id: Option<T>,
                                secret: Option<T>,
                                auth_url: T,
                                token_url: T)
                                -> Self {
        let client_id = match id {
            None => env::var("_ID").unwrap_or(String::new()),
            Some(val) => val.into(),
        };

        let client_secret = match secret {
            None => env::var("_SECRET").unwrap_or(String::new()),
            Some(val) => val.into(),
        };

        Provider {
            name: name.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            auth_url: auth_url.into(),
            token_url: token_url.into(),
        }
    }
}


pub struct Providers(HashMap<&'static str, Provider>);

impl Providers {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("GITHUB",
                 Provider::new("github",
                               Some("cf07a51bf0c95a35fff8"),
                               Some("89969eb236953ca9c0c86de8064f2d91fb69b14f"),
                               "https://github.com/login/oauth/authorize",
                               "https://github.com/login/oauth/access_token"));
        Providers(m)
    }
}

// impl Providers {
// pub fn new(providers: Option<Vec<Provider>>) -> Self {
// match providers {
// None => {
// return Providers { providers: vec![] };
//

// Some(providers) => {
// return Providers { providers: providers };
//
//
//

// pub fn add_provider(&mut self, p: Provider) {
// self.providers.push(p);
//
//

impl<'a, 'r> FromRequest<'a, 'r> for Providers {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut data = PRO.lock().unwrap();
        Success(*data)
    }
}



#[cfg(test)]
mod tests {
    use super::{Provider, Providers};
    #[test]
    fn it_works() {
        let mut providers = Providers::new(None);

        providers.add_provider(Provider::new("github",
                                             Some("cf07a51bf0c95a35fff8"),
                                             Some("89969eb236953ca9c0c86de8064f2d91fb69b14f"),
                                             "https://github.com/login/oauth/authorize",
                                             "https://github.com/login/oauth/access_token"));
        providers.add_provider(Provider::new("github",
                                             Some("cf07a51bf0c95a35fff8"),
                                             Some("89969eb236953ca9c0c86de8064f2d91fb69b14f"),
                                             "https://github.com/login/oauth/authorize",
                                             "https://github.com/login/oauth/access_token"));
    }

}
