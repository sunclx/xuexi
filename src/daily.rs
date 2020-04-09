use super::android::{
    click, content_options_positons, draw, input, positions, return_home, set_ime, sleep, tap,
    texts, Xpath, IME,
};
use super::config::{CFG, DCFG};
use super::db::*;
use rand::{thread_rng, Rng};
pub struct Daily {
    db: DB,
    bank: Bank,
    has_bank: bool,
}
impl Drop for Daily {
    fn drop(&mut self) {
        set_ime(&IME);
    }
}
impl Daily {
    pub fn new() -> Self {
        &IME;
        set_ime("com.android.adbkeyboard/.AdbIME");
        let database_uri = &CFG.database_uri;

        //  config("database_uri");
        let db = DB::new(database_uri);
        Self {
            bank: Bank::new(),
            db: db,
            has_bank: false,
        }
    }

    pub fn enter(&self) {
        return_home();
        DCFG.rule_bottom_mine.click();
        DCFG.rule_quiz_entry.click();
        DCFG.rule_daily_entry.click();
        //click("rule_bottom_mine");
        // click("rule_quiz_entry");
        // click("rule_daily_entry");
    }
    pub fn run(&mut self) {
        // # 每日答题，每组题数
        let count = 10;
        let mut rng = thread_rng();
        let daily_delay = rng.gen_range(1, CFG.daily_delay);
        println!("开始每日答题");
        self.enter();
        let mut group = 1;
        loop {
            println!("\n<----正在答题,第 {} 组---->", group);
            for _ in 0..count {
                self.submit();
            }
            if !CFG.daily_forever && texts("rule_score_reached").len() > 0 {
                println!("大战{}回合，终于分数达标咯，告辞！", group);
                return_home();
                return;
            }
            println!("再来一组");
            sleep(daily_delay);
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
        lazy_static! {
            static ref SPOSITION: (usize, usize) = {
                let mut submit_position = vec![];

                for _ in 0..10 {
                    if submit_position.len() > 0 {
                        break;
                    }
                    submit_position = positions("rule_submit");
                }
                if submit_position.len() < 1 {
                    (0, 0)
                } else {
                    submit_position[0]
                }
            };
        }

        match (SPOSITION.0, SPOSITION.1) {
            (0, 0) => click("rule_submit"),
            (x, y) => tap(x, y),
        }
        // # 填好空格或选中选项后
        // 提交答案后，获取答案解析，若为空，则回答正确，否则，返回正确答案
        match &*texts("rule_desc") {
            [des, ..] => {
                self.bank.answer = des.replace(r"正确答案：", "");
                println!("正确答案：{}", &self.bank.answer);
                self.bank.notes.push_str(&texts("rule_note")[0]);
                match (SPOSITION.0, SPOSITION.1) {
                    (0, 0) => click("rule_submit"),
                    (x, y) => tap(x, y),
                }
                // 删除错误数据
                if self.has_bank {
                    self.db.delete(&(&self.bank).into());
                    self.db.add(&(&self.bank).into());
                } else {
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
                let answer = self.bank.answer.replace(" ", "");
                for (answer, (x, y)) in answer.chars().zip(edits.iter()) {
                    tap(*x, *y);
                    input(&answer.to_string());
                }
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: 不忘初心牢记使命");
                for ((x, y), answer) in edits.iter().zip("不忘初心牢记使命".chars()) {
                    tap(*x, *y);
                    input(&answer.to_string());
                }
            }
        }
    }
    fn radio(&mut self) {
        let (content, options, mut ptns) =
            content_options_positons("rule_content", "rule_radio_options_content", "rule_options");
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
                println!("默认提交答案: A");
                self.bank.answer.push('A');
            }
        }
        let cursor = self.bank.answer.chars().nth(0).unwrap() as usize - 65;
        match ptns[cursor] {
            (0, 0) => {
                draw();
                ptns = positions("rule_options");
                let (x, y) = ptns[cursor];
                tap(x, y);
            }
            (x, y) => tap(x, y),
        }
    }
    fn check(&mut self) {
        let (content, options, mut ptns) =
            content_options_positons("rule_content", "rule_radio_options_content", "rule_options");
        self.bank.content = content;
        self.bank.options = options;
        let answers: String;
        match &*self.db.query(&self.bank) {
            [b, ..] => {
                self.has_bank = true;
                self.bank.answer.push_str(&b.answer);
                println!("{}", &self.bank);
                println!("自动提交答案 {}", &self.bank.answer);
                answers = self.bank.answer.clone();
            }
            [] => {
                self.has_bank = false;
                println!("{}", &self.bank);
                println!("默认提交答案: 全选");
                answers = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..ptns.len()].to_string();
                self.bank.answer.push_str(&answers);
            }
        }
        for c in answers.chars() {
            let cursor = c as usize - 65;
            match ptns[cursor] {
                (0, 0) => {
                    draw();
                    ptns = positions("rule_options");
                    let (x, y) = ptns[cursor];
                    tap(x, y);
                }
                (x, y) => tap(x, y),
            }
        }
    }
}
