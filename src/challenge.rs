use super::android::*;
use super::db::*;
use rand::{thread_rng, Rng};
use std::thread::sleep;
use std::time::Duration;
pub struct Challenge {
    db: DB,
    filename: String,
    bank: Bank,
    has_bank: bool,
    banks: Vec<Bank>,
    json_questions: Vec<Bank>,
}
impl Challenge {
    pub fn new() -> Self {
        let json_questions = load(config("database_json"));
        let filename = get_config("challenge_json");
        let banks = load(&filename);
        let database_uri = config("database_uri");
        let db = DB::new(database_uri);
        Self {
            bank: Bank::new(),
            db: db,
            filename: filename,
            has_bank: false,
            banks: banks,
            json_questions: json_questions,
        }
    }

    pub fn run(&mut self) {
        let count = get_config("challenge_count");
        println!("开始挑战答题,挑战题数：{}", count);
        return_home();
        click("rule_bottom_mine");
        click("rule_quiz_entry");
        click("rule_challenge_entry");
        sleep(Duration::from_secs(2));

        // 开始
        let mut i = 0;
        while i < count {
            self.submit();
            let mut rng = thread_rng();
            let challenge_delay = rng.gen_range(1, 5);
            sleep(Duration::from_secs(challenge_delay));
            if positions("rule_judge_bounds").len() > 0 {
                self.dump();
                click("rule_close_bounds");
                click("rule_again_bounds");
                sleep(Duration::from_secs(2));
                i = 0;
                continue;
            }
            i += 1;
            // 将正确答案添加到数据库
            if !self.has_bank {
                self.db.add(&BankQuery::from(&self.bank));
            }
        }
        sleep(Duration::from_secs(30));
        println!("已经达成目标题数（{}题），退出挑战", i);
        return_home();
    }
    fn submit(&mut self) {
        let (content, options, mut ptns) = content_options_positons(
            "rule_challenge_content",
            "rule_challenge_options_content",
            "rule_challenge_options_bounds",
        );
        self.bank.clear();
        self.bank.category.push_str("单选题");
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
        let banks = load(&self.filename);
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
        while ptns.len() <= cursor {
            cursor -= 1;
        }
        // # 点击正确选项
        while (0, 0) == ptns[cursor] {
            draw();
            ptns = positions("rule_challenge_options_bounds");
        }
        // 现在可以安全点击(触摸)
        let (x, y) = ptns[cursor];
        tap(x, y);
    }
    fn dump(&mut self) {
        // 在note中追加一个错误答案，以供下次遇到排除
        let mut bank = self.bank.clone();
        match self.banks.iter_mut().find(|b| **b == bank) {
            Some(b) => b.notes.push_str(&self.bank.answer),
            None => {
                bank.notes.push_str(&self.bank.answer);
                self.banks.push(bank);
            }
        }
        dump(&self.filename, &self.banks);
        // 删除数据库中错误题目
        if self.has_bank {
            println!("删除数据库中错题");
            let bq = BankQuery::from(&self.bank);
            self.db.delete(&bq);
        }
    }
    fn search(&self) -> String {
        println!("search - {:}", &self.bank.content);
        for b in &self.json_questions {
            let mut b = b.clone();
            b.content = b.content.replace('\u{a0}', " ");
            if b == self.bank {
                println!("search success: {}", &b.answer);
                return b.answer.to_string();
            }
        }
        println!("search failed");
        return "A".to_string();
    }
}
