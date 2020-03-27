use super::db::Bank;
use amxml::dom::new_document;
use config::{Config, File};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File as StdFile;
use std::path::Path;
use std::process::Command;

pub struct Base {
    config: HashMap<String, String>,
    filename: String,
    ime: Option<String>,
    size: (usize, usize),
}

impl Base {
    pub fn new() -> Self {
        // self.path = Path(config.get(self.rules, 'xml_uri'))
        // self.host = config.get(self.rules, 'host')
        // self.port = config.getint(self.rules, 'port')
        // self.is_virtual = config.getboolean(self.rules, 'is_virtual_machine')
        // if not Base.CONNECTED:
        //     self.connect_device()

        let mut cfg = Config::default();
        cfg.merge(File::with_name("./config-custom.ini")).unwrap();

        let common: HashMap<_, _> = cfg
            .get_table("common")
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        let config: HashMap<_, _> = cfg
            .get_table(&common["device"])
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();

        let filename = config["xml_uri"].to_string();
        let ime = Base::get_ime();
        Base::set_ime(ime.as_ref().unwrap());

        Self {
            config: config,
            filename: filename,
            ime: ime,
            size: (0, 0),
        }
    }
    // def connect_device(self):
    // if self.is_virtual:
    //     self._connect()

    // Base.DEVICE = self._getDevice()
    // if Base.DEVICE is not None:
    //     print(f'当前设备 {Base.DEVICE}')
    //     Base.IME = self._getIME()
    //     Base.WM_SIZE = self._size()
    //     self._setIME('com.android.adbkeyboard/.AdbIME')
    // else:
    //     print(f'未连接设备')
    //     raise RuntimeError(f'未连接任何设备')
    pub fn swipe(&self, x0: usize, y0: usize, x1: usize, y1: usize, duration: usize) {
        Command::new("adb")
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
                "faild: adb shell input swipe {:} {:} {:} {:} {:}",
                x0, y0, x1, y1, duration
            ));
        return;
    }
    pub fn tap(&self, x: usize, y: usize) {
        self.swipe(x, y, x, y, 50)
    }
    pub fn draw(&self) {
        let (height, width) = self.size();
        // 中点 三分之一点 三分之二点
        let (x0, x1) = (width / 2, width / 2);
        let (y0, y1) = (height / 3, height / 3 * 2);
        self.swipe(x1, y1, x0, y0, 500);
    }

    pub fn back(&self) {
        Command::new("adb")
            .args(&["shell", "input", "keyevent", &4.to_string()])
            .output()
            .expect(&format!("faild: input keyevent 4"));
    }

    pub fn input(&self, msg: &str) {
        Command::new("adb")
            .args(&[
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
                "faild: adb shell am broadcast -a ADB_INPUT_TEXT --es msg {:}",
                msg
            ));
    }

    fn _connect(&self, host: &str, port: &str) {
        Command::new("adb")
            .arg("connect")
            .arg(format!("{:}:{:}", host, port));
        return;
    }
    fn _disconnect(&self, host: &str, port: &str) {
        Command::new("adb")
            .arg("disconnect")
            .arg(format!("{:}:{:}", host, port));
        return;
    }
    pub fn set_ime(ime: &str) {
        match Command::new("adb")
            .args(&["shell", "ime", "set", ime])
            .output()
        {
            Ok(o) => {
                dbg!(o);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }

    fn get_ime() -> Option<String> {
        match Command::new("adb")
            .args(&["shell", "ime", "list", "-s"])
            .output()
        {
            Ok(output) => String::from_utf8_lossy(&output.stdout)
                .split_whitespace()
                .next()
                .map(String::from),
            Err(e) => {
                println!("获取输入发失败");
                dbg!(e);
                None
            }
        }
    }

    pub fn get_devices(&self) {
        let output = Command::new("adb").arg("devices").output();
        let output = match output {
            Ok(output) => output,
            Err(e) => {
                println!("获取设备失败！！！");
                dbg!(e);
                return;
            }
        };
        let output = String::from_utf8_lossy(&output.stdout);
        let re = Regex::new(r"(.*)\tdevice").unwrap();
        let nums: Vec<_> = re
            .captures_iter(&output)
            .map(|cap| cap[1].to_string())
            .collect();
        dbg!(&nums);
    }

    pub fn size(&self) -> (usize, usize) {
        let output = Command::new("adb")
            .args(&["shell", "wm", "size"])
            .output()
            .expect("failed: adb shell wm size");
        let res = String::from_utf8_lossy(&output.stdout);
        let re = Regex::new(r"(\d+)").unwrap();
        let nums: Vec<_> = re
            .captures_iter(&res)
            .map(|cap| cap[0].parse::<usize>().unwrap())
            .collect();

        return (nums[0], nums[1]);
    }
    pub fn uiautomator(&self) {
        Command::new("adb")
            .args(&["shell", "uiautomator", "dump", "/sdcard/ui.xml"])
            .output()
            .unwrap();
        Command::new("adb")
            .args(&["pull", "/sdcard/ui.xml", &self.filename])
            .output()
            .unwrap();
    }
    pub fn xpath(&self, xpath_rule: &str) -> Vec<String> {
        let xml = read_to_string(&self.filename).unwrap();
        let dom = new_document(&xml).unwrap();
        let root = dom.root_element();
        let res = root.get_nodeset(xpath_rule).unwrap();
        let v: Vec<String> = res
            .iter()
            .map(|x| x.value().replace('\u{a0}', " "))
            .map(|x| if x == "" { " ".to_string() } else { x })
            .collect();
        return v;
    }
    pub fn texts(&self, rule: &str) -> Vec<String> {
        self.uiautomator();
        dbg!(rule);
        let xpath_rule = &self.config[rule];
        return self.xpath(xpath_rule);
    }
    pub fn positions(&self, rule: &str) -> Vec<(usize, usize)> {
        let re = Regex::new(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]").unwrap();
        let mut v: Vec<(usize, usize)> = vec![];
        for s in self.texts(rule) {
            let caps = re.captures(&s).unwrap();
            let x0: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let y0: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let x1: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let x = (x0 + x1) / 2;
            let y = (y0 + y1) / 2;
            v.push((x, y));
        }
        return v;
    }
    pub fn content_options_positons(
        &self,
        content: &str,
        options: &str,
        positions: &str,
    ) -> (String, String, Vec<(usize, usize)>) {
        let content = &self.config[content];
        let options = &self.config[options];
        let positions = &self.config[positions];
        self.uiautomator();

        let content = self.xpath(content)[0].clone();
        let options = self.xpath(options).join("|");

        let positions = self.xpath(positions);
        let re = Regex::new(r"\[(\d+),(\d+)\]\[(\d+),(\d+)\]").unwrap();
        let positions = positions
            .iter()
            .map(|s| {
                let cap = re.captures(s).unwrap();
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

    pub fn click(&self, rule: &str) {
        let mut positons = self.positions(rule);
        for _ in 0..20 {
            if positons.len() == 1 {
                break;
            }
            positons = self.positions(rule);
        }
        match &*positons {
            [(x, y)] => self.tap(*x, *y),
            _ => {
                dbg!(positons);
            }
        }
    }

    pub fn return_home(&self) {
        let mut positions = self.positions("rule_bottom_work");
        while positions.len() < 1 {
            self.back();
            positions = self.positions("rule_bottom_work");
        }
        let (x, y) = positions[0];
        self.tap(x, y);
    }
    pub fn load<P: AsRef<Path>>(&self, path: P) -> Vec<Bank> {
        let s = std::fs::read_to_string(path).unwrap();
        let v: Vec<Bank> = serde_json::from_str(&s).unwrap();
        return v;
    }
    pub fn dump<P: AsRef<Path>>(&self, path: P, banks: &Vec<Bank>) {
        let f = StdFile::create(path).unwrap();
        serde_json::to_writer_pretty(f, banks).unwrap();
    }
}
