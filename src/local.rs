use super::android::{back, sleep, swipe, tap, Xpath};
use super::config::Rules;
pub struct Local {
    local_column_name: String,
    rules: Rules,
}
impl std::ops::Deref for Local {
    type Target = Rules;
    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}
impl Local {
    pub fn new(local_column_name: String, rules: Rules) -> Self {
        Self {
            local_column_name,
            rules,
        }
    }
    fn enter(&self) {
        self.return_home();
        for _ in 0..10 {
            let txts = self.rule_columns_content.texts();
            let ptns = self.rule_columns_bounds.positions();
            for (name, (x, y)) in txts.iter().zip(ptns.iter()) {
                if &self.local_column_name == name {
                    tap(*x, *y);
                    return;
                }
            }
            let (x0, y0) = ptns[0];
            let (x1, y1) = ptns[ptns.len() - 2];
            swipe(x1, y1, x0, y0, 500);
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
    pub fn run(&self) {
        println!("开始本地频道");
        self.enter();
        self.rule_local_bounds.click();
        sleep(10);
        self.return_home();
        println!("本地频道结束");
    }
}
