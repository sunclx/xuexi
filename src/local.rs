use super::android::{click, positions, return_home, sleep, swipe, tap, texts};
use super::config::CFG;
pub struct Local;

impl Local {
    pub fn new() -> Self {
        Self
    }
    fn enter(&self) {
        return_home();
        for _ in 0..10 {
            let txts = texts("rule_columns_content");
            let ptns = positions("rule_columns_bounds");
            for (name, (x, y)) in txts.iter().zip(ptns.iter()) {
                if &CFG.local_column_name == name {
                    tap(*x, *y);
                    return;
                }
            }
            let (x0, y0) = ptns[0];
            let (x1, y1) = ptns[ptns.len() - 2];
            swipe(x1, y1, x0, y0, 500);
        }
    }
    pub fn run(&self) {
        println!("开始本地频道");
        self.enter();
        click("rule_local_bounds");
        sleep(10);
        println!("本地频道结束");
        return_home();
    }
}
