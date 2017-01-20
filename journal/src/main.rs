extern crate chrono;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;
use chrono::*;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})").unwrap();
}

fn main() {
    pretty_env_logger::init().ok();
    let dt = Local::now();
    let mut start_at = Local::now();
    let mut end_at = Local::now();

    let mut p = PathBuf::new();
    p.push("/home/innolab/dotfiles");
    p.push(format!("{}.log", dt.format("%Y%m%d")));

    let f = OpenOptions::new()
        .read(true)
        .open(p.to_str().unwrap())
        .unwrap();
    let file = BufReader::new(&f);
    let mut lines = file.lines();
    match lines.next() {
        Some(Ok(start)) => {
            let caps = RE.captures(&start).unwrap();
            start_at = Local.datetime_from_str(caps.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap();
            info!("{:?}", start_at);
            println!("{:?}", start_at);
        }
        _ => println!("cannot get start time!"),

    }
    match lines.next() {
        Some(Ok(end)) => {
            let caps = RE.captures(&end).unwrap();
            end_at = Local.datetime_from_str(caps.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap();
            info!("{:?}", end_at);
            println!("{:?}", end_at);
        }
        _ => println!("cannot get end time!"),
    }

    let d = end_at - start_at;
    let round_up_15 = (d.num_minutes() / 15 + 1) * 15;
    let h = round_up_15 / 60 - 1;
    let m = round_up_15 % 60;

    let hour = match m {
        15 => format!("{}.25", h),
        30 => format!("{}.5", h),
        45 => format!("{}.75", h),
        _ => format!("{}", h),
    };

    let display = match dt.weekday() {
        Weekday::Mon => "月",
        Weekday::Tue => "火",
        Weekday::Wed => "水",
        Weekday::Thu => "木",
        Weekday::Fri => "金",
        Weekday::Sat => "土",
        Weekday::Sun => "日",
    };
    println!("");
    println!("{}",
             format!("{}({})-{}", dt.format("%m/%d"), display, "実績"));
    println!("");
    println!("{}", " 10:00~12:00");
    println!("{}", "  午前作業");
    println!("{}", " 12:00~13:00");
    println!("{}", "  昼");
    println!("{}", " 13:00~19:00");
    println!("{}", "  午後作業");
    println!("");
    println!("工数:{}h", hour);
    println!("");
}
