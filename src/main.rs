#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
use serde::{Deserialize, Serialize};

mod android;
mod challenge;
mod config;
mod daily;
mod db;
mod local;
mod reader;
mod ui;
mod viewer;
use android::Xpath;
use config::DCFG as d;
use db::{Bank, DB};
use std::collections::HashMap;
use std::env;
fn main() {
    ui::run_ui();
}
fn xuexi(args: ui::ArgsState) {
    #[cfg(feature = "release")]
    {
        let mut current = env::current_exe().unwrap();
        current.pop();
        env::set_current_dir(current).unwrap();
    }
    let mut args = args;
    println!("当前工作目录{:?}", env::current_dir());
    println!("设备名称: {}", android::DEVICE.as_str());
    if args.auto {
        println!("获取学习积分情况");
        android::return_home();
        d.rule_bottom_mine.click();
        d.rule_bonus_entry.click();
        let mut bonus = HashMap::new();
        while bonus.len() == 0 {
            let titles = d.rule_bonus_title.texts();
            let scores = d.rule_bonus_score.texts();
            bonus = titles.into_iter().zip(scores.into_iter()).collect();
        }
        dbg!(&bonus);
        let completed = "已完成";
        args.local = completed != bonus["本地频道"];
        args.video = completed != bonus["视听学习"] || completed != bonus["视听学习时长"];
        args.article = completed != bonus["阅读文章"] || completed != bonus["文章学习时长"];
        args.challenge = completed != bonus["挑战答题"];
        args.daily = completed != bonus["每日答题"];
    }
    println!(
        "本地视频:{},视听学习:{},阅读文章:{},挑战答题:{},每日答题:{}.",
        args.local, args.video, args.article, args.challenge, args.daily
    );
    if args.local {
        local::Local::new().run();
    }
    if args.video {
        viewer::Viewer::new().run();
    }
    if args.article {
        reader::Reader::new().run();
    }
    if args.challenge {
        challenge::Challenge::new().run();
    }
    if args.daily {
        daily::Daily::new().run();
    }
}

fn _delete() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let banks = android::load("./resource/same.json");
    for bank in &banks {
        db.delete(bank);
    }
}
fn _query() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let mut banks = db._query_content("%");
    banks = banks
        .into_iter()
        .filter(|bank| bank.category == "填空题")
        .map(|mut bank| {
            bank.answer = bank.answer.replace(" ", "");
            bank
        })
        .map(|mut bank| {
            bank.options = bank.answer.chars().count().to_string();
            bank
        })
        .collect();

    android::dump("./resource/blank.json", &banks);
}
fn _same() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let banks = db._query_content("%");
    android::dump("./resource/all.jsonn", &banks);
    let mut not_same = vec![];
    let mut same = vec![];

    for bank in &banks {
        let mut bank = bank.clone();
        bank.content = bank.content.replace('\u{a0}', " ");
        let contain = not_same.contains(&bank);
        if contain {
            println!("{}", &bank);
            same.push(bank);
        } else {
            not_same.push(bank);
        }
    }

    android::dump("./resource/same.json", &same);
}

fn _add() {
    // let db1 = DB::new("./resource/data-dev.sqlite");
    // let mut banks1 = db1._query_content("%");
    let mut banks1 = load("./resource/questions.json");
    let db2 = DB::new("./resource/data.sqlite");
    let mut banks2 = db2._query_content("%");

    for bank in &mut banks1 {
        bank.content = bank.content.replace('\u{a0}', " ");
        bank.answer = bank.answer.replace('\u{a0}', " ");
        bank.answer = bank.answer.replace(" ", "");
        if bank.category == "填空题" {
            bank.options = bank.answer.chars().count().to_string();
        }
        let bank = bank.clone();
        if bank.answer == "" || bank.content == "" || bank.options == "" {
            continue;
        }
        if !banks2.contains(&bank) {
            db2.add(&bank);
            banks2.push(bank);
        }
    }
}
fn _banks_from_db(path: &str) -> Vec<Bank> {
    let db = DB::new(path);
    db._query_content("%")
}
fn _banks_from_json(path: &str) -> Vec<Bank> {
    android::load(path)
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bank1 {
    #[serde(skip)]
    pub id: i32,
    pub category: String,
    pub content: String,
    pub options: Vec<String>,
    pub answer: String,
    pub notes: String,
}
pub fn load(path: &str) -> Vec<Bank> {
    let s = std::fs::read_to_string(path).unwrap();
    let v: Vec<Bank1> = serde_json::from_str(&s).unwrap();
    v.into_iter()
        .map(|bank| {
            let mut b = Bank::new();
            b.category = bank.category;
            b.content = bank.content;
            b.options = bank.options.join("|");
            b.answer = bank.answer;
            b.notes = bank.notes;
            b
        })
        .collect()
}
