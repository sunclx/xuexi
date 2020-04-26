use super::android::{back, draw, return_home, sleep, Xpath};
use super::config::{CFG, DCFG as d};
pub struct Viewer;
impl Viewer {
    pub fn new() -> Self {
        Self
    }
    pub fn run(&self) {
        println!("开始试听学习");
        return_home();
        d.rule_bottom_work.click();
        d.rule_bottom_ding.click();
        d.rule_first_video.click();

        for i in 0..CFG.video_count {
            let now = std::time::Instant::now();
            println!("观看视频第{}则{}秒", i + 1, CFG.video_delay);
            sleep(CFG.video_delay);
            draw();
            println!("完成试听第{}则，耗时{}秒", i + 1, now.elapsed().as_secs());
        }
        back();
        return_home();
        println!("本地频道结束");
    }
}
