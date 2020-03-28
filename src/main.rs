#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
mod android;
mod base;
mod challenge;
mod daily;
mod db;
mod local;
mod reader;
mod viewer;
use db::*;
use regex::Regex;

fn main() {
    // let mut r = challenge::Challenge::new();
    // r.run();
    // let mut r = daily::Daily::new();
    // r.run();
    // same();
    delete();
    let x = false;
    if x {
        xuexi()
    }
}
fn xuexi() {
    let l = local::Local::new();
    l.run();
    let v = viewer::Viewer::new();
    v.run();
    let r = reader::Reader::new();
    r.run();
    let mut r = challenge::Challenge::new();
    r.run();
    let mut r = daily::Daily::new();
    r.run();
}
fn delete() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let banks = android::load("./resource/db_wrong.json");
    for bank in &banks {
        let bq = BankQuery::from(bank);
        db.delete(&bq);
    }
}
fn same() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let banks = db.query_content(" ");
    let mut not_same = vec![];
    let mut same = vec![];

    for bank in &banks {
        let mut bank = bank.clone();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s+").unwrap();
        }
        bank.content = RE.replace_all(&bank.content, "%").to_string();
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
fn query() {
    let database_uri = "./resource/data-dev.sqlite";
    let db = DB::new(database_uri);
    let banks = db.query_content("%");
    android::dump("./resource/a0.json", &banks);
}
