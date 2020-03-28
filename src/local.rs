use super::android;
use config::{Config, File};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
pub struct Local {
    config: HashMap<String, String>,
}

impl Local {
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
            // base: Base::new(),
            config: common,
        }
    }

    fn enter(&self) {
        android::return_home();
        let local_column_name = &self.config["local_column_name"];
        for _ in 0..10 {
            let texts = android::texts("rule_columns_content");
            let positions = android::positions("rule_columns_bounds");
            for (name, (x, y)) in texts.iter().zip(positions.iter()) {
                if local_column_name == name {
                    android::tap(*x, *y);
                    return;
                }
            }
            let (x0, y0) = positions[0];
            let (x1, y1) = positions[positions.len() - 2];
            android::swipe(x1, y1, x0, y0, 500);
        }
    }
    pub fn run(&self) {
        println!("开始本地频道");
        self.enter();
        android::click("rule_local_bounds");
        sleep(Duration::from_secs(15));
        println!("本地频道结束");
        android::return_home();
    }
}
