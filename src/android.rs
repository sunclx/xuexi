use super::db::Bank;
use amxml::dom::new_document;
use config::{Config, File};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File as StdFile;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref CFG: Config = {
        let mut config = Config::default();
        config
            .merge(File::with_name("./config-custom.ini"))
            .expect("加载config-custom.ini失败");
        dbg!(&config);
        config
    };
    static ref COMMON: HashMap<String, String> = {
        let common: HashMap<_, _> = CFG
            .get_table("common")
            .expect("获取配置失败")
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        common
    };
    static ref CONFIG: HashMap<String, String> = {
        let config: HashMap<_, _> = CFG
            .get_table(&COMMON["device"])
            .expect("获取配置失败")
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        config
    };
    static ref FILENAME: String = {
        &IME;
        set_ime("com.android.adbkeyboard/.AdbIME");
        CONFIG["xml_uri"].to_string()
    };
    pub static ref DEVICE: String = {
        let host = &CONFIG["host"];
        let port = &CONFIG["port"];
        connect(host, port);
        get_devices().expect("未连接设备")
    };
    pub static ref IME: String = { get_ime().expect("获取输入法失败") };
    static ref SIZE: (usize, usize) = { size() };
    static ref RE_POSITION: Regex = { Regex::new(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]").unwrap() };
}
static ADB: &'static str = "./resource/ADB/ADB";
pub fn config(key: &str) -> &str {
    COMMON.get(key).expect(&format!("get key({}) failed", key))
}

pub fn get_config<T: FromStr>(key: &str) -> T {
    match COMMON[key].parse::<T>() {
        Ok(ok) => ok,
        Err(_) => panic!(format!("parse key({}) failed", key)),
    }
}

pub fn swipe(x0: usize, y0: usize, x1: usize, y1: usize, duration: usize) {
    Command::new(ADB)
        .args(&[
            "shell",
            "input",
            "swipe",
            &x0.to_string(),
            &y0.to_string(),
            &x1.to_string(),
            &y1.to_string(),
            &duration.to_string(),
        ])
        .output()
        .expect(&format!(
            "faild: ADB shell input swipe {:} {:} {:} {:} {:}",
            x0, y0, x1, y1, duration
        ));
    return;
}
pub fn tap(x: usize, y: usize) {
    swipe(x, y, x, y, 50)
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
        .args(&["-s", &DEVICE, "shell", "input", "keyevent", &4.to_string()])
        .output()
        .expect("faild: input keyevent 4");
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
        .expect(&format!(
            "faild: ADB shell am broadcast -a ADB_INPUT_TEXT --es msg {:}",
            msg
        ));
}
pub fn connect(host: &str, port: &str) {
    Command::new(ADB)
        .arg("connect")
        .arg(format!("{:}:{:}", host, port))
        .output()
        .expect(&format!("ADB connect {:}:{:} failed", host, port));
}
pub fn _disconnect(host: &str, port: &str) {
    Command::new(ADB)
        .arg("disconnect")
        .arg(format!("{:}:{:}", host, port))
        .output()
        .expect(&format!("ADB disconnect {:}:{:} failed", host, port));
}
pub fn set_ime(ime: &str) {
    Command::new(ADB)
        .args(&["-s", &DEVICE, "shell", "ime", "set", ime])
        .output()
        .expect(&format!("ADB shell ime set {} failde", ime));
}

pub fn get_ime() -> Option<String> {
    let output = Command::new(ADB)
        .args(&["-s", &DEVICE, "shell", "ime", "list", "-s"])
        .output()
        .expect("获取输入法失败");
    let output = String::from_utf8_lossy(&output.stdout);
    output.split_whitespace().nth(0).map(|x| x.to_string())
}

pub fn get_devices() -> Option<String> {
    let output = Command::new(ADB)
        .arg("devices")
        .output()
        .expect("ADB devices 失败");
    let output = String::from_utf8_lossy(&output.stdout);
    let devices: Vec<_> = output
        .lines()
        .filter(|&line| line.ends_with("\tdevice"))
        .map(|line| line.trim_end_matches("\tdevice"))
        .collect();
    devices.get(0).map(|x| x.to_string())
}
pub fn size() -> (usize, usize) {
    let output = Command::new(ADB)
        .args(&["-s", &DEVICE, "shell", "wm", "size"])
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
        .args(&[
            "-s",
            &DEVICE,
            "shell",
            "uiautomator",
            "dump",
            "/sdcard/ui.xml",
        ])
        .output()
        .expect(&format!(
            "ADB failed: ADB -s {:} shell uiautomator dump /sdcard/ui.xml",
            DEVICE.as_str()
        ));
    Command::new(ADB)
        .args(&["pull", "/sdcard/ui.xml", &FILENAME])
        .output()
        .expect(&format!(
            "ADB failed: ADB pull /scdcard/ui.xml {}",
            FILENAME.as_str()
        ));
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
pub fn texts(rule: &str) -> Vec<String> {
    uiautomator();
    // dbg!(rule);
    let xpath_rule = &CONFIG[rule];
    // dbg!(xpath_rule);
    return xpath(xpath_rule);
}

pub fn positions(rule: &str) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = vec![];
    for s in texts(rule) {
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

pub fn content_options_positons(
    content: &str,
    options: &str,
    positions: &str,
) -> (String, String, Vec<(usize, usize)>) {
    let content = &CONFIG[content];
    let options = &CONFIG[options];
    let positions = &CONFIG[positions];
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
pub fn click(rule: &str) {
    let mut ptns = vec![];
    for _ in 0..20 {
        ptns = positions(rule);
        if ptns.len() == 1 {
            break;
        }
    }
    let (x, y) = ptns[0];
    tap(x, y);
}

pub fn return_home() {
    let mut ptns = positions("rule_bottom_work");
    while ptns.len() < 1 {
        back();
        ptns = positions("rule_bottom_work");
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
