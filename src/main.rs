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
use db::{BankQuery, DB};
use std::env;

fn main() {
    // let mut current = env::current_exe().unwrap();
    // current.pop();
    // env::set_current_dir(current).unwrap();
    println!("{:?}", env::current_dir());
    println!("{:}", android::DEVICE.to_string());
    if false {
        xuexi()
    }
}
// fn run<T: android::Widget>(mut w: T) {
//     w.start();
// }
fn xuexi() {
    // run(local::Local::new());
    local::Local::new().run();
    viewer::Viewer::new().run();
    reader::Reader::new().run();
    challenge::Challenge::new().run();
    daily::Daily::new().run();
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
    let mut banks2 = android::load("./resource/all1.json");
    for bank in &mut banks2 {
        bank.content = bank.content.replace('\u{a0}', " ");
        if !banks1.contains(bank) {
            let bank = &bank.clone();
            db1.add(&bank.into());
        }
    }
}
