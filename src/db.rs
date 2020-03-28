use diesel::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::fmt;
lazy_static! {
    static ref RE: Regex = Regex::new(r"\s+").unwrap();
}

#[derive(Queryable, Debug, Serialize, Deserialize, Clone, Eq)]
pub struct Bank {
    #[serde(skip)]
    pub id: i32,
    pub category: String,
    pub content: String,
    pub options: String,
    pub answer: String,
    pub notes: String,
    #[serde(skip)]
    pub bounds: String,
}
impl Bank {
    pub fn new() -> Self {
        Bank {
            id: 0,
            category: "".to_string(),
            content: "".to_string(),
            options: "".to_string(),
            answer: "".to_string(),
            notes: "".to_string(),
            bounds: "".to_string(),
        }
    }
    pub fn clear(&mut self) {
        self.id = 0;
        self.category.clear();
        self.content.clear();
        self.options.clear();
        self.answer.clear();
        self.notes.clear();
        self.bounds.clear();
    }
}
impl PartialEq for Bank {
    fn eq(&self, other: &Self) -> bool {
        return self.content == other.content && self.options == other.options;
    }
}
impl<'a> From<&'a Bank> for BankQuery<'a> {
    fn from(bank: &'a Bank) -> Self {
        BankQuery {
            category: &bank.category,
            content: &bank.content,
            options: &bank.options,
            answer: &bank.answer,
            notes: &bank.notes,
            bounds: "",
        }
    }
}

#[derive(Insertable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "banks"]
pub struct BankQuery<'a> {
    pub category: &'a str,
    pub content: &'a str,
    pub options: &'a str,
    pub answer: &'a str,
    pub notes: &'a str,
    pub bounds: &'a str,
}

impl<'a> fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opts: Vec<_> = self.options.split("|").collect();
        let opts = match opts.len() {
            1 => "".to_string(),
            _ => opts
                .iter()
                .enumerate()
                .map(|(index, s)| format!("{}. {}\n", ('A' as usize + index) as u8 as char, s))
                .collect(),
        };
        write!(
            f,
            "[{:}]{}\n{}答案：{}",
            self.category, self.content, opts, self.answer
        )
    }
}

table! {
    banks (id) {
        id->Integer,
        category-> Text,
        content-> Text,
        options-> Text,
        answer-> Text,
        notes-> Text,
        bounds-> Text,
    }
}
use self::banks::dsl::*;

pub struct DB {
    connection: SqliteConnection,
}

impl DB {
    pub fn new(database_uri: &str) -> Self {
        let database_uri = match database_uri {
            "" => "./resource/data-dev.sqlite",
            _ => database_uri,
        };
        let connection = SqliteConnection::establish(database_uri)
            .expect(&format!("链接数据库失败： {}", &database_uri));
        Self {
            connection: connection,
        }
    }
    pub fn query(&self, bank: &Bank) -> Vec<Bank> {
        let cnt = RE.replace_all(&bank.content, "%");
        banks
            .filter(category.eq(&bank.category))
            .filter(content.like(&cnt))
            .filter(options.eq(&bank.options))
            .load::<Bank>(&self.connection)
            .expect(&format!("查询答题失败。Bank:{:?}", &bank))
    }
    pub fn query_content(&self, cnt: &str) -> Vec<Bank> {
        let cnt = RE.replace_all(cnt, "%");
        banks
            .filter(content.like(&cnt))
            .load::<Bank>(&self.connection)
            .expect(&format!("查询答题失败。Bank:{:?}", &cnt))
    }
    pub fn add(&self, bankq: &BankQuery) {
        diesel::insert_into(banks)
            .values(bankq)
            .execute(&self.connection)
            .expect(&format!("添加答题失败。{:?}", &bankq));
        println!("添加的数据库成功");
    }
    pub fn _delete(&self, bankq: &BankQuery) {
        let c = RE.replace_all(bankq.content, "%");
        let target = banks
            .filter(category.eq(bankq.category))
            .filter(content.like(c))
            .filter(options.eq(bankq.options));
        let out = diesel::delete(target).execute(&self.connection);
        match out {
            Ok(t) => {
                println!("删除答题成功。BankQuery:{:?}", &bankq);
                dbg!(t);
            }
            Err(e) => {
                println!("删除答题失败。BankQuery:{:?}", &bankq);
                dbg!(e);
            }
        }
        // .expect(&format!("删除答题失败。BankQuery:{:?}", &bankq));
    }
    pub fn _update(&self, bankq: &BankQuery) {
        let c = RE.replace_all(bankq.content, "%");
        let target = banks
            .filter(category.eq(bankq.category))
            .filter(content.like(c))
            .filter(options.eq(bankq.options));
        diesel::update(target)
            .set(answer.eq(bankq.answer))
            .execute(&self.connection)
            .expect(&format!("更新答题失败。BankQuery:{:?}", &bankq));
    }
}
