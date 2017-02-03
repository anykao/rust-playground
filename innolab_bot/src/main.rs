#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate tg_botapi;
extern crate dotenv;

use dotenv::dotenv;
use tg_botapi::args;
use tg_botapi::BotApi;

use std::sync::Arc;
use std::thread;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tg_botapi::Client;
use std::process::Command;

mod errors {
    error_chain!{}
}

use errors::*;

quick_main!(run);

fn file_download(token: &str, file_path: &str) -> Result<()> {
    let client = Client::new();

    let url = format!("https://api.telegram.org/file/bot{}/{}", token, file_path);
    println!("url {}", url);
    let mut res = client.get(&url)
        .send()
        .chain_err(|| "cannot get file")?;
    println!("{:?}", res);
    let mut body: Vec<u8> = vec![];
    res.read_to_end(&mut body).chain_err(|| "cannot read body")?;
    let mut f = File::create(&file_path).chain_err(|| "cannot create file")?;
    println!("{:?}", f);
    f.write_all(&body);

    Ok(())
}

fn run_speech_recog(file_path: &str) -> String {

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let url = format!("https://api.telegram.org/file/bot{}/{}", token, file_path);
    let output = Command::new("echo")
        .arg(&url)
        .output()
        .expect("failed to execute process");
    String::from_utf8(output.stdout).unwrap()

}

fn run() -> Result<()> {
    dotenv().ok();
    // Create bot, test simple API call and print bot information
    let token = env::var("TELEGRAM_BOT_TOKEN").chain_err(|| "cannot get token")?;
    println!("{}", token);

    let bot_arc = Arc::new(BotApi::new(&token));

    let mut update_args = args::GetUpdates::new().timeout(600).offset(0);


    'update_loop: loop {
        let updates = bot_arc.get_updates(&update_args).unwrap();

        for update in updates {
            update_args.offset = Some(update.update_id + 1);

            if let Some(message) = update.message {
                let bot = bot_arc.clone();
                if let Some(voice) = message.voice {

                    let file = bot.get_file(&args::GetFile::new(&voice.file_id)).unwrap();
                    println!("{:?}", file);
                    let file_path = &file.file_path.unwrap();
                    file_download(&token, file_path).chain_err(|| "cannot download file")?;

                    let chat_id = message.chat.id;
                    let msg_id = message.message_id;

                    let message_text = run_speech_recog(file_path);

                    let _ = bot.send_message(&args::SendMessage::new(&message_text)
                        .chat_id(chat_id)
                        .parse_mode("HTML"));
                } else {
                    let _ = bot.send_message(&args::SendMessage::new("no voice")
                        .chat_id(message.chat.id)
                        .parse_mode("HTML"));
                }

            }
        }
    }
    update_args.limit = Some(0);
    update_args.timeout = Some(0);
    let _ = bot_arc.get_updates(&update_args);
    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate opus;
    #[test]
    fn it_works() {
        let decoder = opus::Decoder::new(16000, opus::Channels::Mono);

        assert!(false);
    }
}
