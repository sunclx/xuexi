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

fn main() {
    let mut r = challenge::Challenge::new();
    r.run();
    let mut r = daily::Daily::new();
    r.run();
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
