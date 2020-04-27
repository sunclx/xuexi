use super::android::{return_home, sleep, swipe, tap, Xpath};
use super::config::{CFG, DCFG as d};
pub struct Local;
impl Local {
    pub fn new() -> Self {
        Self
    }
    pub fn run(&self) {
        println!("开始本地频道");
        return_home();
        for _ in 0..10 {
            let txts = d.rule_columns_content.texts();
            let ptns = d.rule_columns_bounds.positions();
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
        d.rule_local_bounds.click();
        sleep(10);
        return_home();
        println!("本地频道结束");
    }
}
