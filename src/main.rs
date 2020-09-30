#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod android;
mod challenge;
mod config;
mod daily;
mod db;
mod local;
mod reader;
mod ui;
mod viewer;
use android::Xpath;
use std::collections::HashMap;
use std::env;
fn main() {
    ui::run_ui();
}
fn xuexi(args: ui::ArgsState) {
    #[cfg(feature = "release")]
    {
        let mut current = env::current_exe().unwrap();
        current.pop();
        env::set_current_dir(current).unwrap();
    }
    let mut args = args;
    println!("当前工作目录{:?}", env::current_dir());
    println!("设备名称: {}", android::DEVICE.as_str());
    if args.auto {
        println!("获取学习积分情况");
        args.rules.rule_bottom_mine.click();
        args.rules.rule_bonus_entry.click();
        let mut bonus = HashMap::new();
        while bonus.len() == 0 {
            let titles = args.rules.rule_bonus_title.texts();
            let scores = args.rules.rule_bonus_score.texts();
            bonus = titles.into_iter().zip(scores).collect();
        }
        dbg!(&bonus);
        let completed = "已完成";
        args.local = completed != bonus["本地频道"];
        args.video = completed != bonus["视听学习"] || completed != bonus["视听学习时长"];
        args.article = completed != bonus["我要选读文章"];
        args.challenge = completed != bonus["挑战答题"];
        args.daily = completed != bonus["每日答题"];
    }
    println!(
        "本地视频:{},视听学习:{},阅读文章:{},挑战答题:{},每日答题:{}.",
        args.local, args.video, args.article, args.challenge, args.daily
    );
    if args.local {
        local::Local::new(
            args.config.local_column_name.to_string(),
            args.rules.clone(),
        )
        .run();
    }
    if args.video {
        viewer::Viewer::new(
            args.config.video_count.unwrap(),
            args.config.video_delay.unwrap(),
            args.rules.clone(),
        )
        .run();
    }
    if args.article {
        reader::Reader::new(
            args.config.article_column_name.clone(),
            args.config.article_count.unwrap(),
            args.config.article_delay.unwrap(),
            args.config.star_share_comment.unwrap(),
            args.config.keep_star_comment,
            args.rules.clone(),
        )
        .run();
    }
    if args.challenge {
        challenge::Challenge::new(
            args.config.challenge_count.unwrap(),
            args.config.challenge_json.to_string(),
            args.config.database_uri.to_string(),
            args.rules.clone(),
        )
        .run();
    }
    if args.daily {
        daily::Daily::new(
            args.config.database_uri.to_string(),
            args.config.daily_delay.unwrap(),
            args.config.daily_forever,
            args.rules.clone(),
        )
        .run();
    }
}
