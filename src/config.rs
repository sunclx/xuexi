use serde::{Deserialize, Serialize};
// use std::cell::Cell;
use std::collections::HashMap;

use std::fs::read_to_string;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref CFG: Config = {
        let s = read_to_string("./config.toml").unwrap();
        let c: Config = toml::from_str(&s).unwrap();
        c
    };
    pub static ref KEY: Arc<Mutex<bool>> = { Arc::new(Mutex::new(false)) };
    pub static ref OUT: Arc<Mutex<String>> = Arc::new(Mutex::new(String::with_capacity(1024)));
    pub static ref DCFG: DeviceConfig = {
        let key;
        match KEY.clone().lock().unwrap().deref() {
            &true => key = "mumu",
            &false => key = "huawei",
        }
        let config = CFG.device_configs[key].clone();
        config
    };
}
#[macro_export]
macro_rules! xprintln {
    ($($value:expr),*) =>
        {{
            use std::fmt::Write;
            let clone = OUT.clone();
            let mut io = clone.lock().unwrap();
            writeln!(*io,$($value),*).unwrap();

        }
        };

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub device: String,
    pub database_uri: String,
    pub database_json: String,
    pub db_wrong_json: String,
    pub daily_json: String,
    pub challenge_json: String,
    pub comments_json: String,
    pub is_user: bool,
    pub daily_forever: bool,
    pub daily_delay: u64,
    pub challenge_count: u64,
    pub challenge_delay: u64,
    pub video_column_name: String,
    pub video_count: u64,
    pub video_delay: u64,
    pub enable_article_list: bool,
    pub article_column_name: String,
    pub local_column_name: String,
    pub article_count: u64,
    pub article_delay: u64,
    pub star_share_comment: u64,
    pub keep_star_comment: bool,
    #[serde(flatten)]
    pub device_configs: HashMap<String, DeviceConfig>,
}
pub type XpathString = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceConfig {
    pub is_virtual_machine: String,
    pub xml_uri: String,
    pub host: String,
    pub port: String,
    pub rule_local_bounds: XpathString,
    pub rule_bottom_message: XpathString,
    pub rule_bottom_ding: XpathString,
    pub rule_bottom_work: XpathString,
    pub rule_bottom_contact: XpathString,
    pub rule_bottom_mine: XpathString,
    pub rule_bonus_entry: XpathString,
    pub rule_quiz_entry: XpathString,
    pub rule_quiz_exit: XpathString,
    pub rule_daily_entry: XpathString,
    pub rule_challenge_entry: XpathString,
    pub rule_bonus_title: XpathString,
    pub rule_bonus_score: XpathString,
    pub rule_type: XpathString,
    pub rule_content: XpathString,
    pub rule_blank_content: XpathString,
    pub rule_options: XpathString,
    pub rule_radio_options_content: XpathString,
    pub rule_edits: XpathString,
    pub rule_score: XpathString,
    pub rule_score_reached: XpathString,
    pub rule_desc: XpathString,
    pub rule_note: XpathString,
    pub rule_back: XpathString,
    pub rule_submit: XpathString,
    pub rule_return: XpathString,
    pub rule_next: XpathString,
    pub rule_challenge_content: XpathString,
    pub rule_challenge_options_content: XpathString,
    pub rule_challenge_options_bounds: XpathString,
    pub rule_judge_bounds: XpathString,
    pub rule_revive_bounds: XpathString,
    pub rule_again_bounds: XpathString,
    pub rule_close_bounds: XpathString,
    pub rule_first_video: XpathString,
    pub rule_columns_content: XpathString,
    pub rule_columns_bounds: XpathString,
    pub rule_fixed_top_bounds: XpathString,
    pub rule_fixed_bottom_bounds: XpathString,
    pub rule_news_bounds: XpathString,
    pub rule_news_content: XpathString,
    pub rule_news3pic_bounds: XpathString,
    pub rule_news3pic_content: XpathString,
    pub rule_star_bounds: XpathString,
    pub rule_share_bounds: XpathString,
    pub rule_comment_bounds: XpathString,
    pub rule_share2xuexi_bounds: XpathString,
    pub rule_publish_bounds: XpathString,
    pub rule_delete_bounds: XpathString,
    pub rule_delete_confirm_bounds: XpathString,
    pub rule_comment2_bounds: XpathString,
    pub rule_exit: XpathString,
}
