use super::android::*;
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

        let count = get_config("video_count");
        let delay = get_config("video_delay");
        for i in 0..count {
            let now = Instant::now();
            println!("观看视频第{}则{}秒", i + 1, delay);
            sleep(delay);
            draw();
            println!("完成试听学习第{}则，耗时{:?}秒", i + 1, now.elapsed());
            println!("进入下一个视频");
        }
        println!("本地频道结束");
        back();
        return_home();
    }
}
