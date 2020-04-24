use super::android::{content_options_positons, draw, dump, load, return_home, sleep, tap, Xpath};
use super::config::{CFG, DCFG as d};
use super::db::{Bank, DB};

pub struct Challenge {
    db: DB,
    filename: String,
    bank: Bank,
    has_bank: bool,
    banks: Vec<Bank>,
}
impl Challenge {
    pub fn new() -> Self {
        let filename = CFG.challenge_json.clone();
        let banks = load(&filename);
        Self {
            bank: Bank::new(),
            db: DB::new(&CFG.database_uri),
            filename: filename,
            has_bank: false,
            banks: banks,
        }
    }

    pub fn run(&mut self) {
        println!("开始挑战答题,挑战题数：{}", CFG.challenge_count);
        return_home();
        d.rule_bottom_mine.click();
        d.rule_quiz_entry.click();
        d.rule_challenge_entry.click();
        sleep(2);

        // 开始
        let mut i = 0;
        while i < CFG.challenge_count {
            print!("第{}题", i);
            self.submit();
            sleep(3);
            if d.rule_judge_bounds.positions().len() > 0 {
                self.dump();
                d.rule_close_bounds.click();
                d.rule_again_bounds.click();
                i = 0;
                continue;
            }
            i += 1;
            // 将正确答案添加到数据库
            if !self.has_bank {
                self.db.add(&self.bank);
            }
        }
        println!("已经达成目标题数（{}题），退出挑战", i);
        sleep(30);
        return_home();
    }
    fn submit(&mut self) {
        let (content, options, mut ptns) = content_options_positons(
            &d.rule_challenge_content,
            &d.rule_challenge_options_content,
            &d.rule_challenge_options_bounds,
        );
        self.bank.clear();
        self.bank.category.push_str("单选题");
        self.bank.content = content;
        self.bank.options = options;
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
                self.bank.answer.push('A');
                println!("试探性提交答案 {}", &self.bank.answer);
            }
        }
        let mut bank = self.bank.clone();
        if let Some(b) = self.banks.iter_mut().find(|b| **b == bank) {
            bank.answer.push_str("ABCDEFGHIJKLMN");
            for c in bank.answer.chars() {
                if !b.notes.contains(c) {
                    self.bank.answer.clear();
                    self.bank.answer.push(c);
                    break;
                }
            }
        };
        let mut cursor = self.bank.answer.chars().nth(0).unwrap() as usize - 65;
        while ptns.len() <= cursor {
            cursor -= 1;
        }
        // # 点击正确选项
        while (0, 0) == ptns[cursor] {
            draw();
            ptns = d.rule_challenge_options_bounds.positions();
        }
        // 现在可以安全点击(触摸)
        let (x, y) = ptns[cursor];
        tap(x, y);
    }
    fn dump(&mut self) {
        // 在note中追加一个错误答案，以供下次遇到排除
        let mut bank = self.bank.clone();
        if let Some(b) = self.banks.iter_mut().find(|b| **b == bank) {
            b.notes.push_str(&self.bank.answer)
        } else {
            bank.notes.push_str(&self.bank.answer);
            self.banks.push(bank);
        }
        dump(&self.filename, &self.banks);
        // 删除数据库中错误题目
        if self.has_bank {
            println!("删除数据库中错题");
            self.db.delete(&self.bank);
        }
    }
}
