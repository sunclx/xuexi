use super::config::{XpathString, DCFG as d};
use super::db::Bank;
use amxml::dom::new_document;
use regex::Regex;
use std::fs::read_to_string;
use std::fs::File as StdFile;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref FILENAME: String = { d.xml_uri.clone() };
    pub static ref DEVICE: String = {
        let host = &d.host;
        let port = &d.port;
        connect(host, port);
        get_devices().expect("未连接设备")
    };
    pub static ref IME: String = { get_ime().expect("获取输入法失败") };
    static ref SIZE: (usize, usize) = { size() };
    static ref RE_POSITION: Regex = { Regex::new(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]").unwrap() };
}

#[cfg(target_os = "windows")]
static ADB: &'static str = "./resource/ADB/adb";

#[cfg(not(target_os = "windows"))]
static ADB: &'static str = "adb";

pub fn swipe(x0: usize, y0: usize, x1: usize, y1: usize, duration: usize) {
    Command::new(ADB)
        .args(
            format!(
                "shell input swipe {:} {:} {:} {:} {:}",
                x0, y0, x1, y1, duration
            )
            .split_whitespace(),
        )
        .output()
        .expect("failed");
}
pub fn tap(x: usize, y: usize) {
    swipe(x, y, x, y, 50);
}
pub fn draw() {
    let xml = xpath("//hierarchy/node/@bounds");
    let s = xml.get(0).expect("draw failed: xpath 错误");
    let caps = RE_POSITION.captures(s).unwrap();
    let x: usize = caps[3].parse().unwrap();
    let y: usize = caps[4].parse().unwrap();

    let (height, width) = (x, y);
    // 中点 三分之一点 三分之二点
    let (x0, x1) = (width / 2, width / 2);
    let (y0, y1) = (height / 4, height / 4 * 3);
    swipe(x1, y1, x0, y0, 500);
}
pub fn back() {
    Command::new(ADB)
        .args(format!("-s {:} shell input keyevent 4", DEVICE.as_str()).split_whitespace())
        .output()
        .expect("failed");
}
pub fn input(msg: &str) {
    Command::new(ADB)
        .args(&[
            "-s",
            &DEVICE,
            "shell",
            "am",
            "broadcast",
            "-a",
            "ADB_INPUT_TEXT",
            "--es",
            "msg",
            msg,
        ])
        .output()
        .expect("failed");
}
pub fn connect(host: &str, port: &str) {
    Command::new(ADB)
        .args(format!("connect {:}:{:}", host, port).split_whitespace())
        .output()
        .expect("failed");
}
pub fn _disconnect(host: &str, port: &str) {
    Command::new(ADB)
        .args(format!("disconnect {:}:{:}", host, port).split_whitespace())
        .output()
        .expect("failed");
}
pub fn set_ime(ime: &str) {
    Command::new(ADB)
        .args(format!("-s {} shell ime set {}", DEVICE.as_str(), ime).split_whitespace())
        .output()
        .expect("failed");
}

pub fn get_ime() -> Option<String> {
    let output = Command::new(ADB)
        .args(format!("-s {} shell ime list -s", DEVICE.as_str()).split_whitespace())
        .output()
        .expect("获取输入法失败");
    let output = String::from_utf8_lossy(&output.stdout);
    let imes: Vec<_> = output.split_whitespace().collect();
    dbg!(&imes); //.map(|x| x.to_string())
    Some(imes[0].to_string())
}

pub fn get_devices() -> Option<String> {
    let output = Command::new(ADB).arg("devices").output().unwrap();
    let output = String::from_utf8_lossy(&output.stdout);
    output
        .lines()
        .filter(|&line| line.ends_with("\tdevice"))
        .map(|line| line.trim_end_matches("\tdevice"))
        .next()
        .map(ToString::to_string)
}
pub fn size() -> (usize, usize) {
    let output = Command::new(ADB)
        .args(format!("-s {} shell wm size", DEVICE.as_str()).split_whitespace())
        .output()
        .expect("failed: ADB shell wm size");
    let res = String::from_utf8_lossy(&output.stdout);
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let nums: Vec<_> = RE
        .captures_iter(&res)
        .map(|cap| cap[0].parse::<usize>().unwrap())
        .collect();
    let l = nums.len();

    return (nums[l - 2], nums[l - 1]);
}
pub fn uiautomator() {
    Command::new(ADB)
        .args(
            format!(
                "-s {:} shell uiautomator dump /sdcard/ui.xml",
                DEVICE.as_str()
            )
            .split_whitespace(),
        )
        .output()
        .expect("failed");
    Command::new(ADB)
        .args(&["pull", "/sdcard/ui.xml", &FILENAME])
        .output()
        .expect("failed");
}
pub fn xpath(xpath_rule: &str) -> Vec<String> {
    let xml = read_to_string(FILENAME.as_str()).expect("读取xml文件失败");
    let res = new_document(&xml)
        .expect("解析xml文件失败")
        .root_element()
        .get_nodeset(xpath_rule)
        .expect("xpath 执行失败");
    let v: Vec<String> = res
        .iter()
        .map(|x| x.value().replace('\u{a0}', " "))
        .map(|x| if x == "" { " ".to_string() } else { x })
        .collect();
    return v;
}

pub fn content_options_positons(
    content: &str,
    options: &str,
    positions: &str,
) -> (String, String, Vec<(usize, usize)>) {
    uiautomator();

    let content = xpath(content)[0].clone();
    let options = xpath(options).join("|");
    let positions = xpath(positions);
    let positions = positions
        .iter()
        .map(|s| {
            let cap = RE_POSITION.captures(s).unwrap();
            let x0: usize = cap[1].parse().unwrap();
            let y0: usize = cap[2].parse().unwrap();
            let x1: usize = cap[3].parse().unwrap();
            let y1: usize = cap[4].parse().unwrap();
            let x = (x0 + x1) / 2;
            let y = (y0 + y1) / 2;
            (x, y)
        })
        .collect();

    return (content, options, positions);
}

pub fn return_home() {
    let mut ptns = d.rule_bottom_work.positions();
    while ptns.len() < 1 {
        back();
        ptns = d.rule_bottom_work.positions();
    }
    let (x, y) = ptns[0];
    tap(x, y);
}

pub fn load<P: AsRef<Path>>(path: P) -> Vec<Bank> {
    let s = std::fs::read_to_string(path).unwrap();
    let v: Vec<Bank> = serde_json::from_str(&s).unwrap();
    return v;
}
pub fn dump<P: AsRef<Path>>(path: P, banks: &Vec<Bank>) {
    let f = StdFile::create(path).unwrap();
    serde_json::to_writer_pretty(f, banks).unwrap();
}

pub fn sleep(second: u64) {
    thread::sleep(Duration::from_secs(second));
}
pub trait Xpath {
    fn click(&self);
    fn texts(&self) -> Vec<String>;
    fn positions(&self) -> Vec<(usize, usize)>;
}
impl Xpath for XpathString {
    fn texts(&self) -> Vec<String> {
        uiautomator();
        xpath(&self)
    }
    fn positions(&self) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = vec![];
        for s in self.texts() {
            let caps = RE_POSITION.captures(&s).unwrap();
            let x0: usize = caps[1].parse().unwrap();
            let y0: usize = caps[2].parse().unwrap();
            let x1: usize = caps[3].parse().unwrap();
            let y1: usize = caps[4].parse().unwrap();
            let x = (x0 + x1) / 2;
            let y = (y0 + y1) / 2;
            v.push((x, y));
        }
        return v;
    }
    fn click(&self) {
        let mut ptns = vec![];
        for _ in 0..20 {
            ptns = self.positions();
            if ptns.len() == 1 {
                break;
            }
        }
        let (x, y) = ptns[0];
        tap(x, y);
    }
}
