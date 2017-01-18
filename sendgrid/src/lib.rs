#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;


use std::collections::HashMap;

pub type SGMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SGMailV3 {
    Sender: Email,
    Subject: String,
    Personalizations: Vec<Personalization>,
    Content: Vec<Content>,
    Sections: Option<SGMap>,
    Headers: Option<SGMap>,
    Categories: Vec<String>,
    CustomArgs: Option<SGMap>,
    SendAt: Option<u64>,
    BatchID: Option<String>,
    IPPoolID: Option<String>,
    ReplyTo: Email,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Personalization {
    To: Vec<Email>,
    CC: Vec<Email>,
    BCC: Vec<Email>,
    Subject: String,
    Headers: SGMap,
    Substitutions: SGMap,
    CustomArgs: SGMap,
    Categories: Vec<String>,
    SendAt: i64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Email {
    Name: String,
    Address: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Content {
    Type: String,
    Value: String,
}

pub struct EmailBuilder(SGMailV3);

impl EmailBuilder {
    pub fn new() -> Self {
        let inner: SGMailV3 = Default::default();
        EmailBuilder(inner)
    }

    pub fn add_from(&mut self, email: Email) -> &mut Self {
        self.0.Sender = email;
        self
    }

    pub fn add_subject(&mut self, subject: String) -> &mut Self {
        self.0.Subject = subject;
        self
    }


    pub fn finish(&self) -> &SGMailV3 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
