use super::android::{
    back, content_options_positons, draw, input, return_home, set_ime, sleep, tap, Xpath, IME,
};
use super::config::{CFG, DCFG as d};
use super::db::*;

pub struct Daily {
    db: DB,
    bank: Bank,
    has_bank: bool,
    submit_position: Option<(usize, usize)>,
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
        Self {
            bank: Bank::new(),
            db: DB::new(&CFG.database_uri),
            has_bank: false,
            submit_position: None,
        }
    }
    pub fn run(&mut self) {
        println!("开始每日答题");
        return_home();
        let count = 10;
        let daily_delay = CFG.daily_delay;
        d.rule_bottom_mine.click();
        d.rule_quiz_entry.click();
        d.rule_daily_entry.click();
        let mut group = 1;
        'out: loop {
            println!("\n<----正在答题,第 {} 组---->", group);
            for _ in 0..count {
                if let Err(_) = self.submit() {
                    back();
                    d.rule_exit.click();
                    d.rule_daily_entry.click();
                    continue 'out;
                }
            }
            if !CFG.daily_forever && d.rule_score_reached.texts().len() > 0 {
                println!("大战{}回合，终于分数达标咯，告辞！", group);
                return_home();
                return;
            }
            println!("再来一组");
            sleep(daily_delay);
            d.rule_next.click();
            group += 1
        }
    }
    fn submit(&mut self) -> Result<(), ()> {
        self.has_bank = false;
        self.bank.clear();
        self.bank.category.push_str(&d.rule_type.texts()[0]);
        match self.bank.category.as_str() {
            "填空题" => self.blank(),
            "单选题" => self.radio(),
            "多选题" => self.check(),
            _ => {
                println!("category: {}", &self.bank.category);
                panic!("未知题目类型")
            }
        }

        if let Some((x, y)) = self.submit_position {
            tap(x, y);
        } else {
            let submit_position = d.rule_submit.positions();
            if submit_position.len() != 1 {
                return Err(());
            }
            let (x, y) = submit_position[0];
            self.submit_position = Some((x, y));
            tap(x, y);
        }

        // # 填好空格或选中选项后
        // 提交答案后，获取答案解析，若为空，则回答正确，否则，返回正确答案
        if let [des, ..] = &*d.rule_desc.texts() {
            self.bank.answer = des.replace(r"正确答案：", "");
            println!("正确答案：{}", &self.bank.answer);
            self.bank.notes.push_str(&d.rule_note.texts()[0]);
            let (x, y) = self.submit_position.unwrap();
            tap(x, y);
            // 删除错误数据
            self.db.delete(&self.bank);
            self.db.add(&self.bank);
        } else {
            println!("回答正确");
            // #保存数据
            if !self.has_bank {
                self.db.add(&self.bank);
            }
        }
        Ok(())
    }
    fn blank(&mut self) {
        self.bank.content = d.rule_blank_content.texts().join("");
        let edits = d.rule_edits.positions();
        self.bank.options = edits.len().to_string();
        if let [b, ..] = &*self.db.query(&self.bank) {
            self.has_bank = true;
            self.bank.answer.push_str(&b.answer);
            println!("{}", &self.bank);
            println!("自动提交答案 {}", &self.bank.answer);
            let answer = self.bank.answer.replace(" ", "");
            for (answer, (x, y)) in answer.chars().zip(edits.iter()) {
                tap(*x, *y);
                input(&answer.to_string());
            }
        } else {
            self.has_bank = false;
            println!("{}", &self.bank);
            println!("默认提交答案: 不忘初心牢记使命");
            for ((x, y), answer) in edits.iter().zip("不忘初心牢记使命".chars()) {
                tap(*x, *y);
                input(&answer.to_string());
            }
        }
    }
    fn radio(&mut self) {
        let (content, options, mut ptns) = content_options_positons(
            &d.rule_content,
            &d.rule_radio_options_content,
            &d.rule_options,
        );
        self.bank.content = content;
        self.bank.options = options;
        if let [b, ..] = &*self.db.query(&self.bank) {
            self.has_bank = true;
            self.bank.answer.push_str(&b.answer);
            println!("{}", &self.bank);
            println!("自动提交答案 {}", &self.bank.answer);
        } else {
            self.has_bank = false;
            println!("{}", &self.bank);
            println!("默认提交答案: A");
            self.bank.answer.push('A');
        }
        let cursor = self.bank.answer.chars().nth(0).unwrap() as usize - 65;
        while (0, 0) == ptns[cursor] {
            draw();
            ptns = d.rule_challenge_options_bounds.positions();
        }
        let (x, y) = ptns[cursor];
        tap(x, y);
    }
    fn check(&mut self) {
        let (content, options, mut ptns) = content_options_positons(
            &d.rule_content,
            &d.rule_radio_options_content,
            &d.rule_options,
        );
        self.bank.content = content;
        self.bank.options = options;
        let answers: String;
        if let [b, ..] = &*self.db.query(&self.bank) {
            self.has_bank = true;
            self.bank.answer.push_str(&b.answer);
            println!("{}", &self.bank);
            println!("自动提交答案 {}", &self.bank.answer);
            answers = self.bank.answer.clone();
        } else {
            self.has_bank = false;
            println!("{}", &self.bank);
            println!("默认提交答案: 全选");
            answers = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..ptns.len()].to_string();
            self.bank.answer.push_str(&answers);
        }
        for c in answers.chars() {
            let cursor = c as usize - 65;
            while (0, 0) == ptns[cursor] {
                draw();
                ptns = d.rule_challenge_options_bounds.positions();
            }
            let (x, y) = ptns[cursor];
            tap(x, y);
        }
    }
}
