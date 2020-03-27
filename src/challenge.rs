use super::base::Base;
use super::db::*;
use config::{Config, File};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
pub struct Challenge {
    base: Base,
    config: HashMap<String, String>,
    db: DB,
    filename: String,
    bank: Bank,
    has_bank: bool,
    json_questions: Vec<Bank>,
}
impl Challenge {
    pub fn new() -> Self {
        let mut cfg = Config::default();
        cfg.merge(File::with_name("./config-custom.ini")).unwrap();

        let common: HashMap<_, _> = cfg
            .get_table("common")
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        let base = Base::new();
        let json_questions = base.load(&common["database_json"]);
        let filename = common["challenge_json"].clone();
        let database_uri = &common["challenge_json"];
        let db = DB::new(database_uri);
        Self {
            base: base,
            bank: Bank::new(),
            config: common,
            db: db,
            filename: filename,
            has_bank: false,
            json_questions: json_questions,
        }
    }

    pub fn enter(&self) {
        self.base.return_home();
        self.base.click("rule_bottom_mine");
        self.base.click("rule_quiz_entry");
        self.base.click("rule_challenge_entry");
    }
    pub fn run(&mut self) {
        let count = self.config["challenge_count"].parse().unwrap();
        //let is_user = &self.config["is_user"];
        println!("开始挑战答题,挑战题数：{}", count);
        self.enter();
        sleep(Duration::from_secs(2));
        let mut i = 0;
        while i < count {
            self.submit();

            if self.base.positions("rule_judge_bounds").len() > 0 {
                self.dump();
                self.base.click("rule_close_bounds");
                self.base.click("rule_again_bounds");
                sleep(Duration::from_secs(2));
                i = 0;
                continue;
            }
            i += 1;
            // 将正确答案添加到数据库
            if !self.has_bank {
                self.db.add(&BankQuery::from(&self.bank));
            }
            //self.is_user and self.json_blank.append(self.BankQuery.to_dict())
        }
        println!("已经达成目标题数（{}题），退出挑战", i);
        self.base.return_home();
    }
    fn submit(&mut self) {
        let mut rng = thread_rng();
        let challenge_delay = rng.gen_range(1, 5);
        sleep(Duration::from_secs(challenge_delay));

        let (content, options, mut positions) = self.base.content_options_positons(
            "rule_challenge_content",
            "rule_challenge_options_content",
            "rule_challenge_options_bounds",
        );
        self.bank.clear();
        self.bank.category.push_str("挑战题");
        self.bank.content.push_str(&content);
        self.bank.options.push_str(&options);
        match &*self.db.query(&self.bank) {
            [b, ..] => {
                self.has_bank = true;
                self.bank.answer.push_str(&b.answer);
                println!("{}", &self.bank);
                println!("自动提交答案 {}", &self.bank.answer);
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                self.bank.answer.push_str(&self.search());
                println!("试探性提交答案 {}", &self.bank.answer);
            }
        }
        let banks = self.base.load(&self.filename);
        banks.into_iter().find(|b| *b == self.bank).map(|b| {
            let mut answer = self.bank.answer.clone();
            answer.push_str("ABCDEFGHIJKLMN");
            for c in answer.chars() {
                if !b.notes.contains(c) {
                    self.bank.answer.clear();
                    self.bank.answer.push(c);
                    break;
                }
            }
        });
        let mut cursor = self.bank.answer.chars().nth(0).unwrap() as usize - 65;
        while positions.len() <= cursor {
            cursor -= 1;
        }

        // # 点击正确选项
        while (0, 0) == positions[cursor] {
            self.base.draw();
            positions = self.base.positions("rule_challenge_options_bounds");
        }
        // 现在可以安全点击(触摸)
        let (x, y) = positions[cursor];
        self.base.tap(x, y);
    }
    fn dump(&self) {
        let inner = |filename: &str| {
            let mut banks = self.base.load(filename);
            match banks.iter_mut().find(|b| **b == self.bank) {
                Some(b) => b.notes.push_str(&self.bank.answer),
                None => {
                    let mut bank_clone = self.bank.clone();
                    bank_clone.notes.push_str(&self.bank.answer);
                    banks.push(bank_clone);
                }
            }
            self.base.dump(filename, &banks);
        };
        // 在note中追加一个错误答案，以供下次遇到排除
        inner(&self.filename);

        // 标记数据库中错误题目
        if self.has_bank {
            println!("标记数据库中错题。{:?}", &self.bank);
            inner(&self.config["db_wrong_json"]);
        }
    }
    fn search(&self) -> String {
        println!("search - {:?}", &self.bank);
        for b in &self.json_questions {
            if *b == self.bank {
                return b.answer.to_string();
            }
        }
        return "A".to_string();
    }
}
