use super::android::{
    back, content_options_positons, draw, get_ime, input, set_ime, sleep, tap, Xpath,
};
use super::config::Rules;
use super::db::*;

pub struct Daily {
    ime: String,
    daily_delay: u64,
    daily_forever: bool,
    rules: Rules,
    db: DB,
    bank: Bank,
    has_bank: bool,
    submit_position: Option<(usize, usize)>,
}
impl std::ops::Deref for Daily {
    type Target = Rules;
    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}
impl Drop for Daily {
    fn drop(&mut self) {
        set_ime(&self.ime);
    }
}
impl Daily {
    pub fn new(database_uri: String, daily_delay: u64, daily_forever: bool, rules: Rules) -> Self {
        let ime = get_ime().unwrap();
        set_ime("com.android.adbkeyboard/.AdbIME");
        Self {
            ime,
            daily_delay: daily_delay,
            daily_forever: daily_forever,
            rules: rules,
            bank: Bank::new(),
            db: DB::new(&database_uri),
            has_bank: false,
            submit_position: None,
        }
    }
    fn return_home(&self) {
        let mut ptns = self.rule_bottom_work.positions();
        while ptns.len() < 1 {
            back();
            ptns = self.rule_bottom_work.positions();
        }
        let (x, y) = ptns[0];
        tap(x, y);
    }
    pub fn run(&mut self) {
        println!("开始每日答题");
        self.return_home();
        let count = 10;
        let daily_delay = self.daily_delay;
        self.rule_bottom_mine.click();
        self.rule_quiz_entry.click();
        self.rule_daily_entry.click();
        let mut group = 1;
        'out: loop {
            println!("\n<----正在答题,第 {} 组---->", group);
            for _ in 0..count {
                if let Err(_) = self.submit() {
                    back();
                    self.rule_exit.click();
                    self.rule_daily_entry.click();
                    continue 'out;
                }
            }
            if self.daily_forever && self.rule_score_reached.texts().len() > 0 {
                println!("大战{}回合，终于分数达标咯，告辞！", group);
                self.return_home();
                return;
            }
            println!("再来一组");
            sleep(daily_delay);
            self.rule_next.click();
            group += 1
        }
    }
    fn submit(&mut self) -> Result<(), ()> {
        self.has_bank = false;
        self.bank.clear();
        self.bank.category.push_str(&self.rule_type.texts()[0]);
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
            let submit_position = self.rule_submit.positions();
            if submit_position.len() != 1 {
                return Err(());
            }
            let (x, y) = submit_position[0];
            self.submit_position = Some((x, y));
            tap(x, y);
        }

        // # 填好空格或选中选项后
        // 提交答案后，获取答案解析，若为空，则回答正确，否则，返回正确答案
        if let [des, ..] = &*self.rule_desc.texts() {
            self.bank.answer = des.replace(r"正确答案：", "");
            println!("正确答案：{}", &self.bank.answer);
            self.bank.notes.push_str(&self.rule_note.texts()[0]);
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
        self.bank.content = self.rule_blank_content.texts().join("");
        let edits = self.rule_edits.positions();
        self.bank.options = edits.len().to_string();
        if let [b, ..] = &*self.db.query(&self.bank) {
            self.has_bank = true;
            self.bank.answer.push_str(&b.answer);
            println!("{}", &self.bank);
            println!("自动提交答案 {}", &self.bank.answer);
            let mut answer = self.bank.answer.replace(" ", "");
            if answer == "" {
                answer = "不忘初心牢记使命".to_string();
            }
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
            &self.rule_content,
            &self.rule_radio_options_content,
            &self.rule_options,
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
            ptns = self.rule_challenge_options_bounds.positions();
        }
        let (x, y) = ptns[cursor];
        tap(x, y);
    }
    fn check(&mut self) {
        let (content, options, mut ptns) = content_options_positons(
            &self.rule_content,
            &self.rule_radio_options_content,
            &self.rule_options,
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
                ptns = self.rule_challenge_options_bounds.positions();
            }
            let (x, y) = ptns[cursor];
            tap(x, y);
        }
    }
}
