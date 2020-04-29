use super::android::{back, draw, sleep, tap, Xpath};
use super::config::Rules;
pub struct Viewer {
    video_count: u64,
    video_delay: u64,
    rules: Rules,
}
impl std::ops::Deref for Viewer {
    type Target = Rules;
    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}
impl Viewer {
    pub fn new(video_count: u64, video_delay: u64, rules: Rules) -> Self {
        Self {
            video_count,
            video_delay,
            rules,
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
        println!("开始试听学习");
        self.return_home();
        self.rule_bottom_work.click();
        self.rule_bottom_ding.click();
        self.rule_first_video.click();

        for i in 0..self.video_count {
            let now = std::time::Instant::now();
            println!("观看视频第{}则{}秒", i + 1, self.video_delay);
            sleep(self.video_delay);
            draw();
            println!("完成试听第{}则，耗时{}秒", i + 1, now.elapsed().as_secs());
        }
        back();
        self.return_home();
        println!("本地频道结束");
    }
}
