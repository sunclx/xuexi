use super::android::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
pub struct Reader;
impl Reader {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) {
        println!("开始新闻学习");
        self.enter();
        let count = get_config("article_count");
        let delay = get_config("article_delay");
        let mut ssc: usize = get_config("star_share_comment");

        let mut i = 1;
        let mut article_list = Vec::<String>::new();
        while i < count {
            let titles = texts("rule_news_content");
            let positions = positions("rule_news_bounds");
            for (title, (x, y)) in titles.iter().zip(positions.iter()) {
                if article_list.iter().any(|x| x == title) {
                    continue;
                }
                println!("新闻[{}]\t{}", i, title);
                tap(*x, *y);
                let now = Instant::now();
                article_list.push(title.to_string());
                sleep(Duration::from_secs(1));
                self.read_new(delay);
                if ssc > 0 {
                    ssc -= self.star_share_comment();
                }
                back();
                println!("新闻[{}]已阅，耗时{:?}", i, now.elapsed());
                i += 1;
            }
            draw()
        }
        return_home();
    }
    fn enter(&self) {
        return_home();
        let article_column_name = config("article_column_name");
        for _ in 0..10 {
            let texts = texts("rule_columns_content");
            let positions = positions("rule_columns_bounds");
            for (name, (x, y)) in texts.iter().zip(positions.iter()) {
                if article_column_name == name {
                    tap(*x, *y);
                    return;
                }
            }
            let (x0, y0) = positions[0];
            let (x1, y1) = positions[positions.len() - 2];
            swipe(x1, y1, x0, y0, 500);
        }
    }
    fn read_new(&self, delay: u64) {
        let slide_times = 2;
        for _ in 0..slide_times {
            sleep(Duration::from_secs(delay / slide_times));
            draw();
        }
        sleep(Duration::from_secs(1));
    }
    fn star_share_comment(&self) -> usize {
        let keep_star_comment: bool = get_config("keep_star_comment");
        let p = texts("rule_comment_bounds");
        if p.len() != 1 {
            return 0;
        }
        //  分享
        click("rule_share_bounds");
        sleep(Duration::from_secs(2));
        click("rule_share2xuexi_bounds");
        sleep(Duration::from_secs(2));
        println!("分享一篇文章!");
        back();

        let msg = "不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！";

        // 留言
        click("rule_comment_bounds");
        click("rule_comment2_bounds");
        input(msg);
        println!("留言一篇文章: {}", &msg);

        click("rule_publish_bounds");

        // pos_publish = self.positions('rule_publish_bounds')
        // if len(pos_publish) == 1:
        //     print(f'# {pos_publish}没点着，按偏移量再点一次')
        //     offset = round(0.0203 * max(Base.WM_SIZE) + 0.7595)
        //     print(f'发布按钮偏移量 {offset} 屏幕大小 {Base.WM_SIZE}')
        //     x, y = pos_publish[0]
        //     # 由于下面有一栏输入法提示，导致这里pos或出现offset位置偏差，多点一次
        //     self.tap(x, y - offset)

        // 收藏
        click("rule_star_bounds");
        println!("收藏一篇文章!");

        // 保留评论与收藏
        if !keep_star_comment {
            for (x, y) in positions("rule_delete_bounds") {
                tap(x, y);
            }
            click("rule_delete_confirm_bounds");
            println!("删除评论");
            click("rule_star_bounds");
            println!("取消收藏");
        }
        return 1;
    }
}
