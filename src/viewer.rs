use super::base::Base;
use config::{Config, File};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
pub struct Viewer {
    base: Base,
    config: HashMap<String, String>,
}
impl Viewer {
    pub fn new() -> Self {
        let mut cfg = Config::default();
        cfg.merge(File::with_name("./config-custom.ini")).unwrap();

        let common: HashMap<_, _> = cfg
            .get_table("common")
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        Self {
            base: Base::new(),
            config: common,
        }
    }

    pub fn enter(&self) {
        self.base.return_home();
        self.base.click("rule_bottom_work");
        self.base.click("rule_bottom_ding");
        self.base.click("rule_first_video");
    }
    pub fn run(&self) {
        println!("开始试听学习");
        let count = self.config["video_count"].parse().unwrap();
        let delay = self.config["video_delay"].parse().unwrap();
        self.enter();

        for i in 0..count {
            let now = Instant::now();
            println!("观看视频第{}则{}秒", i + 1, delay);
            sleep(Duration::from_secs(delay));
            println!("进入下一个视频");
            self.base.draw();
            println!("完成试听学习第{}则，耗时{:?}秒", i + 1, now.elapsed());
        }

        println!("本地频道结束");
        self.base.back();
        self.base.return_home();
    }
}
