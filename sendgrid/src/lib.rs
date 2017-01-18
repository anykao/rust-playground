#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::collections::HashMap;
use reqwest::header::{Authorization, Bearer};
use reqwest::Client;

const V3_API_URL: &'static str = "https://api.sendgrid.com/v3/mail/send";

pub type SGMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SGMailV3 {
    from_email: Email,
    subject: String,
    personalizations: Vec<Personalization>,
    constent: Vec<Content>,
    attachments: Vec<Attachment>
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Content {
    typ: String,
    value: String,
}

struct Attachment {
    content: String,
    typ: String,
    name: String,
    filename: String,
    disposition: String,
    content_id: String,
}

struct Asm {
    group_id: i64,
    groups_to_display: Vec<i64>,
}

struct MailSettings {
    bcc: BccSetting,
    bypass_list_management: Setting,
    footer: FooterSetting,
    sandbox_mode: Setting,
    spam_checkSetting: SpamCheckSetting,
}

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

struct BccSetting {
    enable: bool,
    email: String,
}

struct FooterSetting {
    enable: bool,
    text: String,
    html: String,
}

struct ClickTrackingSetting {
    enable: bool,
    enable_text: bool,
}

struct OpenTrackingSetting {
    enable: bool,
    substitution_tag: String,
}

struct SandboxModeSetting {
    enable: bool,
    forward_spam: bool,
    spam_check: SpamCheckSetting,
}

struct SpamCheckSetting {
    enable: bool,
    spam_threshold: i64,
    post_to_url: String,
}

struct SubscriptionTrackingSetting {
    enable: bool,
    text: String,
    html: String,
    substitution_tag: String,
}

struct GaSetting {
    enable: bool,
    campaign_source: String,
    campaign_term: String,
    campaign_content: String,
    campaign_name: String,
    campaign_medium: String,
}

struct Setting {
    enable: bool,
}

pub struct EmailBuilder(SGMailV3);

impl EmailBuilder {
    pub fn new() -> Self {
        let inner: SGMailV3 = Default::default();
        EmailBuilder(inner)
    }

    pub fn add_from(&mut self, name: Option<String>, address: String) -> &mut Self {
        self.0.Sender = Email {
            Name: name,
            Address: address,
        };
        self
    }

    pub fn add_personalization(&mut self,
                               name: Option<String>,
                               address: String,
                               subject: String)
                               -> &mut Self {
        self.0.Subject = subject;
        self
    }

    pub fn add_content(&mut self, typ: Option<String>, value: String) -> &mut Self {
        self.0.Subject = subject;
        self
    }

    pub fn finish(&self) -> &SGMailV3 {
        &self.0
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

    pub fn send_mail(&self, mail: SGMailV3) -> Result<(), &str> {
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
    #[test]
    fn it_works() {}
}
