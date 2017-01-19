#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::collections::HashMap;
use reqwest::header::{Authorization, Bearer};
use reqwest::{Client, Response, StatusCode};
use std::borrow::Cow;

const V3_API_URL: &'static str = "https://api.sendgrid.com/v3/mail/send";

pub type SGMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SGMailV3 {
    from_email: Email,
    subject: String,
    personalizations: Vec<Personalization>,
    contents: Vec<Content>,
    attachments: Vec<Attachment>,
    template_id: String,
    sections: Option<SGMap>,
    headers: Option<SGMap>,
    categories: Vec<String>,
    customArgs: Option<SGMap>,
    send_at: Option<u64>,
    batch_id: Option<String>,
    ip_pool_name: Option<String>,
    reply_to: Email,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Personalization {
    tos: Vec<Email>,
    ccs: Vec<Email>,
    bccs: Vec<Email>,
    subject: String,
    headers: SGMap,
    substitutions: SGMap,
    custom_args: SGMap,
    send_at: i64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Email {
    name: Option<String>,
    email: String,
}

impl Email {
    pub fn new<S>(name: S, email: S) -> Self
        where S: Into<String>
    {
        Email {
            name: Some(name.into()),
            email: email.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Content {
    typ: String,
    value: String,
}

impl Content {
    pub fn new<S: Into<String>>(typ: S, value: S) -> Self {
        Content {
            typ: typ.into(),
            value: value.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Attachment {
    content: String,
    typ: String,
    name: String,
    filename: String,
    disposition: String,
    content_id: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Asm {
    group_id: i64,
    groups_to_display: Vec<i64>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct MailSettings {
    bcc: BccSetting,
    bypass_list_management: Setting,
    footer: FooterSetting,
    sandbox_mode: Setting,
    spam_checkSetting: SpamCheckSetting,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct TrackingSettings {
    click_tracking: ClickTrackingSetting,
    open_tracking: OpenTrackingSetting,
    subscription_tracking: SubscriptionTrackingSetting,
    google_analytics: GaSetting,
    bcc: BccSetting,
    bypass_list_management: Setting,
    footer: FooterSetting,
    sandbox_mode: SandboxModeSetting,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct BccSetting {
    enable: bool,
    email: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct FooterSetting {
    enable: bool,
    text: String,
    html: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct ClickTrackingSetting {
    enable: bool,
    enable_text: bool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct OpenTrackingSetting {
    enable: bool,
    substitution_tag: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SandboxModeSetting {
    enable: bool,
    forward_spam: bool,
    spam_check: SpamCheckSetting,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SpamCheckSetting {
    enable: bool,
    spam_threshold: i64,
    post_to_url: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SubscriptionTrackingSetting {
    enable: bool,
    text: String,
    html: String,
    substitution_tag: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct GaSetting {
    enable: bool,
    campaign_source: String,
    campaign_term: String,
    campaign_content: String,
    campaign_name: String,
    campaign_medium: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Setting {
    enable: bool,
}

pub struct EmailBuilder(SGMailV3);

impl EmailBuilder {
    pub fn new() -> Self {
        let inner: SGMailV3 = Default::default();
        EmailBuilder(inner)
    }

    pub fn add_from(&mut self, email: Email) -> &mut Self {
        self.0.from_email = email;
        self
    }

    pub fn add_content(&mut self, content: Content) -> &mut Self {
        self.0.contents.push(content);
        self
    }

    pub fn add_personalization(&mut self,
                               name: Option<String>,
                               address: String,
                               subject: String)
                               -> &mut Self {
        self.0.subject = subject;
        self
    }
}

pub struct MailClient {
    client: Client,
    key: String,
}

impl MailClient {
    pub fn new<T>(api_key: T) -> Self
        where T: Into<String>
    {

        let client = Client::new().unwrap();
        MailClient {
            client: client,
            key: api_key.into(),
        }

    }

    pub fn send_mail(self, mail: SGMailV3) -> Response {
        self.client
            .post(V3_API_URL)
            .header(Authorization(Bearer { token: self.key }))
            .json(&mail)
            .send()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        EmailBuilder::new().add_from(Email::new("1", "2")).add_content(Content::new("3", "4"));
    }
}
