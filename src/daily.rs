use super::android::*;
use super::db::*;
use rand::{thread_rng, Rng};
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;
pub struct Daily {
    db: DB,
    bank: Bank,
    has_bank: bool,
}
impl Daily {
    pub fn new() -> Self {
        let database_uri = config("database_uri");
        let db = DB::new(database_uri);
        Self {
            bank: Bank::new(),
            db: db,
            has_bank: false,
        }
    }

    pub fn enter(&self) {
        return_home();
        click("rule_bottom_mine");
        click("rule_quiz_entry");
        click("rule_daily_entry");
    }
    pub fn run(&mut self) {
        // # 每日答题，每组题数
        let count = 10;
        // # 是否永远答题
        let forever: bool = get_config("daily_forever");
        let daily_delay: u64 = get_config("daily_delay");
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
            if !forever && "领取奖励已达今日上限" == config("rule_score_reached") {
                println!("大战{}回合，终于分数达标咯，告辞！", group);
                return_home();
                return;
            }
            println!("再来一组");
            sleep(Duration::from_secs(daily_delay));
            click("rule_next");
            group += 1
        }
    }
    fn submit(&mut self) {
        self.has_bank = false;
        self.bank.clear();
        self.bank.category.push_str(&texts("rule_type")[0]);
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
        click("rule_submit");
        // 提交答案后，获取答案解析，若为空，则回答正确，否则，返回正确答案
        match &*texts("rule_desc") {
            [des, ..] => {
                self.bank.answer = des.replace(r"正确答案：", "");
                println!("正确答案：{}", &self.bank.answer);
                self.bank.notes.push_str(&texts("rule_note")[0]);
                click("rule_submit");
                // 删除错误数据
                if self.has_bank {
                    self.db.delete(&(&self.bank).into());
                    self.db.add(&(&self.bank).into());
                }
            }
            [] => {
                println!("回答正确");
                // #保存数据
                if !self.has_bank {
                    self.db.add(&(&self.bank).into());
                }
            }
        }
    }
    fn blank(&mut self) {
        let contents = texts("rule_blank_content");
        self.bank.content = contents.join("");
        let edits = positions("rule_edits");
        let count_blank = edits.len();
        self.bank.options = count_blank.to_string();
        match &*self.db.query(&self.bank) {
            [b, ..] => {
                self.has_bank = true;
                self.bank.answer.push_str(&b.answer);
                println!("{}", &self.bank);
                println!("自动提交答案 {}", &self.bank.answer);
                for (answer, (x, y)) in self.bank.answer.split(" ").zip(edits.iter()) {
                    tap(*x, *y);
                    input(answer);
                }
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: 不忘初心牢记使命");
                for ((x, y), answer) in edits.iter().zip(repeat("不忘初心牢记使命")) {
                    tap(*x, *y);
                    input(answer);
                }
            }
        }
    }
    fn radio(&mut self) {
        let (content, options, positions) =
            content_options_positons("rule_content", "rule_radio_options_content", "rule_options");
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
                tap(x, y);
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: A");
                self.bank.answer.push('A');
                let (x, y) = positions[0];
                tap(x, y);
            }
        }
    }
    fn check(&mut self) {
        let (content, options, positions) =
            content_options_positons("rule_content", "rule_radio_options_content", "rule_options");
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
                    tap(x, y);
                }
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: 全选");
                let answers = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                for ((x, y), answer) in positions.iter().zip(answers.chars()) {
                    self.bank.answer.push(answer);
                    tap(*x, *y);
                }
            }
        }
    }
}
