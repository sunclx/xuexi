use super::android::{back, draw, get_ime, input, set_ime, sleep, swipe, tap, Xpath};
use super::config::Rules;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Comment {
    tags: Vec<String>,
    content: Vec<String>,
}
fn get_comment(name: &str) -> String {
    lazy_static! {
        static ref COMMENTS: Vec<Comment> = {
            let comments_str = include_str!("comments.json");
            let comments: Vec<Comment> = serde_json::from_str(comments_str).unwrap();
            comments
        };
    }
    let comment = COMMENTS
        .iter()
        .find(|comment| comment.tags.iter().any(|tag| name.contains(tag)))
        .unwrap_or(&COMMENTS[0]);
    let mut rng = rand::thread_rng();
    let mut _comment = String::with_capacity(8 * 4 * 50);
    while _comment.chars().count() < 32 {
        let i = rng.gen_range(0, comment.content.len());
        _comment.push_str(&comment.content[i]);
    }
    return _comment;
}

pub struct Reader {
    ime: String,
    article_column_name: String,
    article_count: u64,
    article_delay: u64,
    star_share_comment: u64,
    keep_star_comment: bool,
    rules: Rules,
}
impl std::ops::Deref for Reader {
    type Target = Rules;
    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}
impl Drop for Reader {
    fn drop(&mut self) {
        set_ime(&self.ime);
    }
}

impl Reader {
    pub fn new(
        article_column_name: String,
        article_count: u64,
        article_delay: u64,
        star_share_comment: u64,
        keep_star_comment: bool,
        rules: Rules,
    ) -> Self {
        let ime = get_ime().unwrap();
        set_ime("com.android.adbkeyboard/.AdbIME");
        Self {
            ime,
            article_column_name,
            article_count,
            article_delay,
            star_share_comment,
            keep_star_comment,
            rules,
        }
    }
    fn enter(&self) {
        self.return_home();
        for _ in 0..10 {
            let texts = self.rule_columns_content.texts();
            let positions = self.rule_columns_bounds.positions();
            let (x0, y0) = positions[0];
            let (x1, y1) = positions[positions.len() - 2];
            for (name, (x, y)) in texts.iter().zip(positions) {
                if self.article_column_name == *name {
                    tap(x, y);
                    return;
                }
            }
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
        println!("开始新闻学习");
        self.enter();
        let mut ssc = self.star_share_comment;
        let mut i = 1;
        let mut article_list = vec![];
        while i < self.article_count {
            let titles = self.rule_news_content.texts();
            let positions = self.rule_news_bounds.positions();
            for (title, (x, y)) in titles.iter().zip(positions) {
                if article_list.contains(title) {
                    continue;
                }
                println!("新闻[{}]: {}", i, title);
                tap(x, y);
                let now = std::time::Instant::now();
                article_list.push(title.clone());
                sleep(self.article_delay / 3);
                draw();
                sleep(self.article_delay / 3);
                draw();
                sleep(self.article_delay / 3);
                if ssc > 0 {
                    ssc -= self.star_share_comment(title);
                }
                back();
                println!("新闻已阅，耗时{:}秒", now.elapsed().as_secs());
                i += 1;
            }
            draw()
        }
        self.return_home();
        println!("新闻学习结束");
    }

    fn star_share_comment(&self, title: &str) -> u64 {
        let p = self.rule_comment_bounds.texts();
        if p.len() != 1 {
            return 0;
        }
        //  分享
        self.rule_share_bounds.click();
        self.rule_share2xuexi_bounds.click();
        println!("分享一篇文章!");
        back();

        // 留言
        self.rule_comment_bounds.click();
        while let [(x, y)] = &*self.rule_comment2_bounds.positions() {
            tap(*x, *y);
            let msg = get_comment(title);
            set_ime("com.android.adbkeyboard/.AdbIME");
            input(&msg);
            println!("留言一篇文章: {}", &msg);
        }
        self.rule_publish_bounds.click();

        // 收藏
        self.rule_star_bounds.click();
        println!("收藏一篇文章!");

        // 保留评论与收藏
        if !self.keep_star_comment {
            for (x, y) in self.rule_delete_bounds.positions() {
                tap(x, y);
                self.rule_delete_confirm_bounds.click();
                println!("删除评论");
            }
            self.rule_star_bounds.click();
            println!("取消收藏");
        }
        return 1;
    }
}
