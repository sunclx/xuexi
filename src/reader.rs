use super::base::Base;
use config::{Config, File};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
pub struct Reader {
    base: Base,
    config: HashMap<String, String>,
}
impl Reader {
    pub fn new() -> Self {
        let mut cfg = Config::default();
        cfg.merge(File::with_name("./config-custom.ini")).unwrap();

        let common: HashMap<_, _> = cfg
            .get_table("common")
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into_str().unwrap()))
            .collect();
        Self {
            base: Base::new(),
            config: common,
        }
    }

    pub fn run(&self) {
        println!("开始新闻学习");
        self.enter();
        let count = self.config["article_count"].parse().unwrap();
        let delay = self.config["article_delay"].parse().unwrap();
        let mut ssc: usize = self.config["star_share_comment"].parse().unwrap();

        let mut i = 1;
        let mut article_list = Vec::<String>::new();
        while i < count {
            let titles = self.base.texts("rule_news_content");
            let positions = self.base.positions("rule_news_bounds");
            for (title, (x, y)) in titles.iter().zip(positions.iter()) {
                if article_list.iter().any(|x| x == title) {
                    continue;
                }
                println!("新闻[{}]\t{}", i, title);
                self.base.tap(*x, *y);
                let now = Instant::now();
                article_list.push(title.to_string());
                sleep(Duration::from_secs(1));
                self.read_new(delay);
                if ssc > 0 {
                    ssc -= self.star_share_comment();
                }
                self.base.back();
                println!("新闻[{}]已阅，耗时{:?}", i, now.elapsed());
                i += 1;
            }
            self.base.draw()
        }
        self.base.return_home();
    }
    fn enter(&self) {
        self.base.return_home();
        let article_column_name = &self.config["article_column_name"];
        for _ in 0..10 {
            let texts = self.base.texts("rule_columns_content");
            let positions = self.base.positions("rule_columns_bounds");
            for (name, (x, y)) in texts.iter().zip(positions.iter()) {
                if article_column_name == name {
                    self.base.tap(*x, *y);
                    return;
                }
            }
            let (x0, y0) = positions[0];
            let (x1, y1) = positions[positions.len() - 2];
            self.base.swipe(x1, y1, x0, y0, 500);
        }
    }
    fn read_new(&self, delay: u64) {
        let slide_times = 2;
        for _ in 0..slide_times {
            sleep(Duration::from_secs(delay / slide_times));
            self.base.draw();
        }
        sleep(Duration::from_secs(1));
    }
    fn star_share_comment(&self) -> usize {
        let keep_star_comment: bool = self.config["keep_star_comment"].parse().unwrap();
        let p = self.base.texts("rule_comment_bounds");
        if p.len() != 1 {
            return 0;
        }
        //  分享
        self.base.click("rule_share_bounds");
        self.base.click("rule_share2xuexi_bounds");
        println!("分享一篇文章!");
        self.base.back();

        let msg = "不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！";

        // 留言
        self.base.click("rule_comment_bounds");
        self.base.click("rule_comment2_bounds");
        self.base.input(msg);
        println!("留言一篇文章: {}", &msg);

        self.base.click("rule_publish_bounds");

        // pos_publish = self.positions('rule_publish_bounds')
        // if len(pos_publish) == 1:
        //     print(f'# {pos_publish}没点着，按偏移量再点一次')
        //     offset = round(0.0203 * max(Base.WM_SIZE) + 0.7595)
        //     print(f'发布按钮偏移量 {offset} 屏幕大小 {Base.WM_SIZE}')
        //     x, y = pos_publish[0]
        //     # 由于下面有一栏输入法提示，导致这里pos或出现offset位置偏差，多点一次
        //     self.tap(x, y - offset)

        // 收藏
        self.base.click("rule_star_bounds");
        println!("收藏一篇文章!");

        // 保留评论与收藏
        if !keep_star_comment {
            for (x, y) in self.base.positions("rule_delete_bounds") {
                self.base.tap(x, y);
            }
            self.base.click("rule_delete_confirm_bounds");
            println!("删除评论");
            self.base.click("rule_star_bounds");
            println!("取消收藏");
        }
        return 1;
    }
    // def _star_share_comment(self, title):
    //     keep_star_comment = self.config.getboolean(
    //         'common', 'keep_star_comment')

    //     if 1 == len(self.positions('rule_comment_bounds')):
    //         # 分享
    //         self.click('rule_share_bounds')
    //         self.click('rule_share2xuexi_bounds')
    //         print(f'分享一篇文章!')
    //         self.back()
    //         sleep(1)

    //         # 随机取一条留言
    //         msg = '不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！'
    //         has_comment = False
    //         self.json_comments = self.json_comments or self.load(
    //             Path(self.config.get('common', 'comments_json')))
    //         for comment in self.json_comments:
    //             for tag in comment["tags"]:
    //                 if tag in title:
    //                     msg = choice(
    //                         comment["content"]) or f'{title} 不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！'
    //                     has_comment = True
    //                     break
    //                 else:
    //                     continue
    //             if has_comment:
    //                 break
    //         else:
    //             # 没有一个关键词匹配，双随机：随机关键词中的随机评论
    //             comment = self.json_comments[0]
    //             msg = choice(
    //                 comment["content"]) or f'{title} 不忘初心牢记使命！为实现中华民族伟大复兴的中国梦不懈奋斗！'

    //         # 留言
    //         self.click('rule_comment_bounds')
    //         self.click('rule_comment2_bounds')
    //         self.input(msg)
    //         print(f'留言一篇文章: {msg}')
    //         sleep(3)
    //         self.click('rule_publish_bounds')
    //         sleep(3)

    //         pos_publish = self.positions('rule_publish_bounds')
    //         if len(pos_publish) == 1:
    //             print(f'# {pos_publish}没点着，按偏移量再点一次')
    //             offset = round(0.0203 * max(Base.WM_SIZE) + 0.7595)
    //             print(f'发布按钮偏移量 {offset} 屏幕大小 {Base.WM_SIZE}')
    //             x, y = pos_publish[0]
    //             # 由于下面有一栏输入法提示，导致这里pos或出现offset位置偏差，多点一次
    //             self.tap(x, y - offset)

    //         # 收藏
    //         self.click('rule_star_bounds')
    //         print(f'收藏一篇文章!')
    //         sleep(1)

    //         # 保留评论与收藏
    //         if not keep_star_comment:
    //             for (x, y) in self.positions('rule_delete_bounds'):
    //                 self.tap(x, y)
    //             self.click('rule_delete_confirm_bounds')
    //             print(f'删除评论 ')
    //             self.click('rule_star_bounds')
    //             print(f'取消收藏')

    //         return 1
    //     else:
    //         print(f'这是一篇关闭评论的文章，老子不留言了，告辞！')
    //         return 0
}
