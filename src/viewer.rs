use super::android::{back, click, draw, return_home, sleep};
use super::config::CFG;
use std::time::Instant;
pub struct Viewer;
impl Viewer {
    pub fn new() -> Self {
        Self
    }
    pub fn run(&self) {
        println!("开始试听学习");
        return_home();
        click("rule_bottom_work");
        click("rule_bottom_ding");
        click("rule_first_video");

        for i in 0..CFG.video_count {
            let now = Instant::now();
            println!("观看视频第{}则{}秒", i + 1, CFG.video_delay);
            sleep(CFG.video_delay);
            draw();
            println!("完成试听学习第{}则，耗时{:?}秒", i + 1, now.elapsed());
            println!("进入下一个视频");
        }
        println!("本地频道结束");
        back();
        return_home();
    }
}
