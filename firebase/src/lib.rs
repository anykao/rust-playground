// http://stackoverflow.com/questions/37322747/using-mail-and-password-to-authenticate-via-the-rest-api-firebase
// https://developers.google.com/identity/toolkit/web/reference/
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::collections::HashMap;
use reqwest::header::{Authorization, Bearer};

pub struct config {
    apiKey: String,
    authDomain: String,
    databaseURL: String,
    storageBucket: String,
}

impl config {
    pub fn new(apiKey: String, authDomain: String) -> Self {
        config {
            apiKey: apiKey,
            authDomain: authDomain,
            databaseURL: String::new(),
            storageBucket: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct regist {
    email: String,
    password: String,
    returnSecureToken: bool,
}

impl regist {
    fn new(email: String, password: String) -> Self {
        regist {
            email: email,
            password: password,
            returnSecureToken: true,
        }

    }
}

struct id_token {
    idToken: String,
}


pub struct firebase {
    cfg: config,
}

impl firebase {
    pub fn new(cfg: config) -> Self {
        firebase { cfg: cfg }
    }

    pub fn login(&self, email: String, password: String) -> Result<(), String> {

        let client = reqwest::Client::new().unwrap();
        let api_key = self.cfg.apiKey.to_owned();
        let end_point = &format!("https://www.googleapis.\
                          com/identitytoolkit/v3/relyingparty/verifyPassword?key={}",
                                 api_key);
        println!("{}", end_point.clone());
        let mut payload = HashMap::new();
        payload.insert("email", email);
        payload.insert("password", password);
        payload.insert("returnSecureToken", "true".to_string());
        let mut res = client.post(end_point)
            .json(&payload)
            .send()
            .unwrap();
        ::std::io::copy(&mut res, &mut ::std::io::stdout()).unwrap();

        Ok(())
    }

    pub fn regist(&self, email: String, password: String) -> Result<(), String> {
        let client = reqwest::Client::new().unwrap();
        let api_key = self.cfg.apiKey.to_owned();
        let end_point = &format!("https://www.googleapis.\
                          com/identitytoolkit/v3/relyingparty/signupNewUser?key={}",
                                 api_key);
        let mut payload = HashMap::new();
        payload.insert("email", email);
        payload.insert("password", password);
        payload.insert("returnSecureToken", "true".to_string());
        let mut res = client.post(end_point)
            .json(&payload)
            .send()
            .unwrap();
        ::std::io::copy(&mut res, &mut ::std::io::stdout()).unwrap();

        Ok(())
    }

    pub fn get_account(&self, id_token: String) -> Result<(), String> {
        let client = reqwest::Client::new().unwrap();
        let api_key = self.cfg.apiKey.to_owned();
        let end_point = &format!("https://www.googleapis.\
                          com/identitytoolkit/v3/relyingparty/getAccountInfo?key={}",
                                 api_key);
        let mut payload = HashMap::new();
        payload.insert("idToken", id_token);
        let mut res = client.post(end_point)
            .json(&payload)
            .send()
            .unwrap();
        ::std::io::copy(&mut res, &mut ::std::io::stdout()).unwrap();

        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use dotenv::dotenv;
    use super::*;
    use std::env;

    const ID_TOKEN: &'static str = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjRmMmRiMjA5ZDMzNzNlMTU4MzA4Y2M0NTU0YTRkN2NhMGI1OTk3ZTkifQ.\
                                    eyJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vYW55a2FvLTE4NWFkIiwiYXVkIjoiYW55a2FvLTE4NWFkIiwiYXV0aF90aW1lIjoxNDg1NzU5MTAyLCJ1c2VyX2lkIjoiWURYaGo2UGhQbGdyYVZSUjRQZUt4OEpZMGtFMiIsInN1YiI6IllEWGhqNlBoUGxncmFWUlI0UGVLeDhKWTBrRTIiLCJpYXQiOjE0ODU3NTkxMDIsImV4cCI6MTQ4NTc2MjcwMiwiZW1haWwiOiI2MDA5MTNAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImZpcmViYXNlIjp7ImlkZW50aXRpZXMiOnsiZW1haWwiOlsiNjAwOTEzQGdtYWlsLmNvbSJdfSwic2lnbl9pbl9wcm92aWRlciI6InBhc3N3b3JkIn19.\
                                    XemihXwL4FSZbuVn73wjZaoqoGQyScKq9XnH3f7w2hsyTFbP3jkqYhQYH534KtIzyz4JBtBK8_6_4JYEu8IFgdgo2Ws6zY48jNJh1JEMuCpuYbzsH1K_zyuu0qzz91kiSxIVxX5M0420GRkg69TjXrktNZawjFTNhD28gBYqqff6gFrVv22y5zjSfG8jOHrOhV1Vpl6DGkNYPhD5lxq_7h73LAqpK5DVfUW3nXBDsuDG6LnstwFWHWIB7QbMsW9MiK3TNR8rnXdrz3p7YGI9_KFGTvzDSZ4ScEb64Nrp6om40y4WZyfWiC32jdwWb-1jXJUXs56309KKVPJG0RZzHg";

    #[test]
    fn it_works() {
        dotenv().ok();
        pretty_env_logger::init().unwrap();
        let api_key = env::var("apiKey").unwrap();
        let auth_domain = env::var("authDomain").unwrap();
        let fb = firebase::new(config::new(api_key, auth_domain));
        fb.login("600913@gmail.com".to_owned(), "1234567".to_owned());
        fb.regist("900913@gmail.com".to_owned(), "321321".to_owned());
        fb.get_account(ID_TOKEN.to_string());
    }
}
