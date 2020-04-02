#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
mod android;
mod challenge;
mod daily;
mod db;
mod local;
mod reader;
mod viewer;
use db::{Bank, BankQuery, DB};
use std::collections::HashMap;
use std::env;

fn main() {
    let mut current = env::current_exe().unwrap();
    current.pop();
    env::set_current_dir(current).unwrap();

    println!("当前工作目录{:?}", env::current_dir());
    println!("设备名称: {}", android::DEVICE.as_str());

    println!("获取学习积分情况");
    android::return_home();
    android::click("rule_bottom_mine");
    android::click("rule_bonus_entry");

    let mut bonus = HashMap::new();
    android::texts("rule_bonus_title");
    while bonus.len() == 0 {
        let titles = android::texts("rule_bonus_title");
        let scores = android::texts("rule_bonus_score");
        bonus = titles.into_iter().zip(scores.into_iter()).collect();
    }
    dbg!(&bonus);
    let completed = "已完成";
    if completed != bonus["本地频道"] {
        local::Local::new().run();
    }
    if completed != bonus["视听学习"] || completed != bonus["视听学习时长"] {
        viewer::Viewer::new().run();
    }
    if completed != bonus["阅读文章"] || completed != bonus["文章学习时长"] {
        reader::Reader::new().run();
    }
    if completed != bonus["挑战答题"] {
        challenge::Challenge::new().run();
    }
    if completed != bonus["每日答题"] {
        daily::Daily::new().run();
    }
    android::set_ime(&android::IME);
}

fn _delete() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let banks = android::load("./resource/same.json");
    for bank in &banks {
        let bq = BankQuery::from(bank);
        db.delete(&bq);
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
    let db1 = DB::new("./resource/data-dev.sqlite");
    let banks1 = db1._query_content("%");

    let mut banks2 = _banks_from_json("./resource/same.json");
    for bank in &mut banks2 {
        bank.content = bank.content.replace('\u{a0}', " ");
        if !banks1.contains(bank) {
            let bank = &bank.clone();
            db1.add(&bank.into());
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
