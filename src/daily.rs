use super::base::Base;
use super::db::*;
use config::{Config, File};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;
pub struct Daily {
    base: Base,
    config: HashMap<String, String>,
    db: DB,
    bank: Bank,
    has_bank: bool,
}
impl Daily {
    pub fn new() -> Self {
        let mut cfg = Config::default();
        cfg.merge(File::with_name("./config-custom.ini")).unwrap();

        let common: HashMap<_, _> = cfg
            .get_table("common")
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        let database_uri = &common["database_uri"];
        let db = DB::new(database_uri);
        let base = Base::new();
        Self {
            base: base,
            bank: Bank::new(),
            config: common,
            db: db,
            has_bank: false,
        }
    }

    pub fn enter(&self) {
        self.base.return_home();
        self.base.click("rule_bottom_mine");
        self.base.click("rule_quiz_entry");
        self.base.click("rule_daily_entry");
    }
    pub fn run(&mut self) {
        // # 每日答题，每组题数
        let count = 10;
        // # 是否永远答题
        let forever: bool = self.config["daily_forever"].parse().unwrap();
        let daily_delay: u64 = self.config["daily_delay"].parse().unwrap();
        let mut rng = thread_rng();
        let daily_delay = rng.gen_range(1, daily_delay);
        println!("开始每日答题");
        self.enter();
        let mut group = 1;
        loop {
            println!("\n<----正在答题,第 {} 组---->", group);
            for _ in 0..count {
                self.submit();
            }
            if !forever && "领取奖励已达今日上限" == self.config["rule_score_reached"] {
                println!("大战{}回合，终于分数达标咯，告辞！", group);
                self.base.return_home();
                return;
            }
            println!("再来一组");
            sleep(Duration::from_secs(daily_delay));
            self.base.click("rule_next");
            group += 1
        }
    }
    fn submit(&mut self) {
        self.has_bank = false;
        self.bank.clear();
        self.bank
            .category
            .push_str(&self.base.texts("rule_type")[0]);
        match self.bank.category.as_str() {
            "填空题" => self.blank(),
            "单选题" => self.radio(),
            "多选题" => self.check(),
            _ => {
                println!("category: {}", &self.bank.category);
                panic!("未知题目类型")
            }
        }
        // # 填好空格或选中选项后
        self.base.click("rule_submit");

        // 提交答案后，获取答案解析，若为空，则回答正确，否则，返回正确答案
        match &*self.base.texts("rule_desc") {
            [des, ..] => {
                self.bank.answer = des.replace(r"正确答案：", "");
                println!("正确答案：{}", &self.bank.answer);
                self.bank.notes.push_str(&self.base.texts("rule_note")[0]);
                self.base.click("rule_submit");
            }
            [] => {
                println!("回答正确");
            }
        }

        // #保存数据
        if !self.has_bank {
            self.db.add(&(&self.bank).into());
        }
    }
    fn blank(&mut self) {
        let contents = self.base.texts("rule_blank_content");
        self.bank.content = contents.join("");
        let edits = self.base.positions("rule_edits");
        let count_blank = edits.len();
        self.bank.options = count_blank.to_string();
        match &*self.db.query(&self.bank) {
            [b, ..] => {
                self.has_bank = true;
                self.bank.answer.push_str(&b.answer);
                println!("{}", &self.bank);
                println!("自动提交答案 {}", &self.bank.answer);
                for (answer, (x, y)) in self.bank.answer.split(" ").zip(edits.iter()) {
                    self.base.tap(*x, *y);
                    self.base.input(answer);
                }
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: 不忘初心牢记使命");
                for ((x, y), answer) in edits.iter().zip(repeat("不忘初心牢记使命")) {
                    self.base.tap(*x, *y);
                    self.base.input(answer);
                }
            }
        }
    }
    fn radio(&mut self) {
        let (content, options, positions) = self.base.content_options_positons(
            "rule_content",
            "rule_radio_options_content",
            "rule_options",
        );
        self.bank.content = content;
        self.bank.options = options;
        match &*self.db.query(&self.bank) {
            [b, ..] => {
                self.has_bank = true;
                self.bank.answer.push_str(&b.answer);
                let cursor = self.bank.answer.chars().nth(0).unwrap() as usize - 65;
                println!("{}", &self.bank);
                println!("自动提交答案 {}", &self.bank.answer);
                let (x, y) = positions[cursor];
                self.base.tap(x, y);
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: A");
                self.bank.answer.push('A');
                let (x, y) = positions[0];
                self.base.tap(x, y);
            }
        }
    }
    fn check(&mut self) {
        let (content, options, positions) = self.base.content_options_positons(
            "rule_content",
            "rule_radio_options_content",
            "rule_options",
        );
        self.bank.content = content;
        self.bank.options = options;
        match &*self.db.query(&self.bank) {
            [b, ..] => {
                self.has_bank = true;
                self.bank.answer.push_str(&b.answer);
                println!("{}", &self.bank);
                println!("自动提交答案 {}", &self.bank.answer);
                for c in self.bank.answer.chars() {
                    let cursor = c as usize - 65;
                    let (x, y) = positions[cursor];
                    self.base.tap(x, y);
                }
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: 全选");
                let answers = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                for ((x, y), answer) in positions.iter().zip(answers.chars()) {
                    self.bank.answer.push(answer);
                    self.base.tap(*x, *y);
                }
            }
        }
    }
}
