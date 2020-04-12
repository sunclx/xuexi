use super::android::{
    back, draw, get_ime, input, return_home, set_ime, sleep, swipe, tap, Xpath, IME,
};
use super::config::{CFG, DCFG as d};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Instant;
lazy_static! {
    static ref COMMENTS: Vec<Comment> = {
        let comments_str = include_str!("../resource/comments.json");
        let comments: Vec<Comment> = serde_json::from_str(comments_str).unwrap();
        comments
    };
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Comment {
    id: u64,
    tags: Vec<String>,
    content: Vec<String>,
}
fn get_comment(name: &str) -> String {
    //let msg = "不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！";
    let mut rng = rand::thread_rng();
    for comment in COMMENTS.iter() {
        for tag in &comment.tags {
            if name.contains(tag) {
                let i = rng.gen_range(0, comment.content.len());
                let i2 = rng.gen_range(0, comment.content.len());
                return comment.content[i].clone() + &comment.content[i2];
            }
        }
    }
    let i = rng.gen_range(0, COMMENTS[0].content.len());
    let i2 = rng.gen_range(0, COMMENTS[0].content.len());
    return COMMENTS[0].content[i].clone() + &COMMENTS[0].content[i2];
}

pub struct Reader;
impl Drop for Reader {
    fn drop(&mut self) {
        set_ime(&IME);
    }
}

impl Reader {
    pub fn new() -> Self {
        &IME;
        set_ime("com.android.adbkeyboard/.AdbIME");
        get_ime();
        Self
    }
    pub fn run(&self) {
        println!("开始新闻学习");
        self.enter();
        let mut ssc = CFG.star_share_comment;

        let mut i = 1;
        let mut article_list = Vec::<String>::new();
        while i < CFG.article_count {
            let titles = d.rule_news_content.texts();
            let positions = d.rule_news_bounds.positions();
            for (title, (x, y)) in titles.iter().zip(positions.iter()) {
                if article_list.iter().any(|x| x == title) {
                    continue;
                }
                println!("新闻[{}]\t{}", i, title);
                tap(*x, *y);
                let now = Instant::now();
                article_list.push(title.to_string());
                self.read_new(CFG.article_delay);
                if ssc > 0 {
                    ssc -= self.star_share_comment(title);
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
        for _ in 0..10 {
            let texts = d.rule_columns_content.texts();
            let positions = d.rule_columns_bounds.positions();
            for (name, (x, y)) in texts.iter().zip(positions.iter()) {
                if &CFG.article_column_name == name {
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
        sleep(delay / 3);
        draw();
        sleep(delay / 3);
        draw();
        sleep(delay / 3);
    }
    fn star_share_comment(&self, title: &str) -> u64 {
        let p = d.rule_comment_bounds.texts();
        if p.len() != 1 {
            return 0;
        }
        //  分享
        d.rule_share_bounds.click();
        d.rule_share2xuexi_bounds.click();
        println!("分享一篇文章!");
        back();

        //let msg = "不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！";
        let msg = get_comment(title);

        // 留言
        d.rule_comment_bounds.click();
        d.rule_comment2_bounds.click();
        input(&msg);
        println!("留言一篇文章: {}", &msg);

        d.rule_publish_bounds.click();

        // pos_publish = self.positions('rule_publish_bounds')
        // if len(pos_publish) == 1:
        //     print(f'# {pos_publish}没点着，按偏移量再点一次')
        //     offset = round(0.0203 * max(Base.WM_SIZE) + 0.7595)
        //     print(f'发布按钮偏移量 {offset} 屏幕大小 {Base.WM_SIZE}')
        //     x, y = pos_publish[0]
        //     # 由于下面有一栏输入法提示，导致这里pos或出现offset位置偏差，多点一次
        //     self.tap(x, y - offset)

        // 收藏
        d.rule_star_bounds.click();
        println!("收藏一篇文章!");

        // 保留评论与收藏
        if !CFG.keep_star_comment {
            for (x, y) in d.rule_delete_bounds.positions() {
                tap(x, y);
            }
            d.rule_delete_confirm_bounds.click();
            println!("删除评论");
            d.rule_star_bounds.click();
            println!("取消收藏");
        }
        return 1;
    }
}
