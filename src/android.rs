use super::db::Bank;
use regex::Regex;
use std::process::Command;
lazy_static! {
    pub static ref DEVICE: String = { get_devices().expect("未连接设备") };
    pub static ref IME: String = { get_ime().expect("获取输入法失败") };
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
        .args(&["pull", "/sdcard/ui.xml", "./resource/ui.xml"])
        .output()
        .expect("failed");
}
pub fn xpath(xpath_rule: &str) -> Vec<String> {
    let xml = std::fs::read_to_string("./resource/ui.xml").expect("读取xml文件失败");
    let res = amxml::dom::new_document(&xml)
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
            let caps = RE_POSITION.captures(&s).unwrap();
            let s: Vec<usize> = caps
                .iter()
                .filter_map(|x| x)
                .filter_map(|x| x.as_str().parse().ok())
                .collect();
            ((s[0] + s[2]) / 2, (s[1] + s[3]) / 2)
        })
        .collect();

    return (content, options, positions);
}

pub fn load<P: AsRef<std::path::Path>>(path: P) -> Vec<Bank> {
    let s = std::fs::read_to_string(path).unwrap();
    let v: Vec<Bank> = serde_json::from_str(&s).unwrap();
    return v;
}
pub fn dump<P: AsRef<std::path::Path>>(path: P, banks: &Vec<Bank>) {
    let f = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(f, banks).unwrap();
}

pub fn sleep(second: u64) {
    std::thread::sleep(std::time::Duration::from_secs(second));
}
pub trait Xpath {
    fn click(&self);
    fn texts(&self) -> Vec<String>;
    fn positions(&self) -> Vec<(usize, usize)>;
}
impl Xpath for String {
    fn texts(&self) -> Vec<String> {
        uiautomator();
        xpath(&self)
    }
    fn positions(&self) -> Vec<(usize, usize)> {
        self.texts()
            .iter()
            .map(|s| {
                let caps = RE_POSITION.captures(&s).unwrap();
                let s: Vec<usize> = caps
                    .iter()
                    .filter_map(|x| x)
                    .filter_map(|x| x.as_str().parse().ok())
                    .collect();
                ((s[0] + s[2]) / 2, (s[1] + s[3]) / 2)
            })
            .collect()
    }
    fn click(&self) {
        for _ in 0..10 {
            if let [(x, y)] = &*self.positions() {
                tap(*x, *y);
                return;
            };
        }
        dbg!(&self);
        panic!("click failed");
    }
}
