// http://stackoverflow.com/questions/37322747/using-mail-and-password-to-authenticate-via-the-rest-api-firebase
extern crate reqwest;

// static END_POINT = "https://www.googleapis.com/identitytoolkit/v3/relyingparty/signupNewUser?key=<my-firebase-api-key>",

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

struct regist {
    email: String,
    password: String,
    returnSecureToken: bool,
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

    pub fn login(&self) -> Result<(), String> {
        Ok(())
    }

    pub fn get_account(&self) -> Result<(), String> {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("hello world");
    }
}
