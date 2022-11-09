use chrono::{self, NaiveDate, NaiveTime};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub total: i32,
    pub history: Vec<History>,
}

#[derive(Serialize, Deserialize)]
pub struct History {
    pub value: i32,
    pub date: NaiveDate,
    pub time: NaiveTime,
}

pub fn get_test_values() -> Arc<RwLock<Vec<Category>>> {
    Arc::new(RwLock::new(vec![
        Category {
            name: String::from("A"),
            total: 2,
            history: vec![
                History {
                    value: 1,
                    date: NaiveDate::from_weekday_of_month(2022, 10, chrono::Weekday::Mon, 2),
                    time: NaiveTime::from_hms(15, 0, 0),
                },
                History {
                    value: 1,
                    date: NaiveDate::from_weekday_of_month(2022, 10, chrono::Weekday::Fri, 2),
                    time: NaiveTime::from_hms(17, 15, 0),
                },
            ],
        },
        Category {
            name: String::from("B"),
            total: 3,
            history: vec![
                History {
                    value: 4,
                    date: NaiveDate::from_weekday_of_month(2022, 9, chrono::Weekday::Thu, 2),
                    time: NaiveTime::from_hms(16, 10, 0),
                },
                History {
                    value: -1,
                    date: NaiveDate::from_weekday_of_month(2022, 10, chrono::Weekday::Sun, 3),
                    time: NaiveTime::from_hms(12, 0, 0),
                },
            ],
        },
    ]))
}

pub fn get_initial_values() -> Arc<RwLock<Vec<Category>>> {
    let files = get_csv_files();
    let mut cat_vec: Vec<Category> = vec![];
    for file in files {
        let f = file.replace("_", " ");
        let mut name = f.as_str();
        name = &name[0..name.len() - 4];
        let mut cat = Category {
            name: name.to_string(),
            total: 0,
            history: vec![],
        };
        let content = fs::read_to_string(file).expect("Please run");
        let mut reader = Reader::from_reader(content.as_bytes());
        for record in reader.deserialize() {
            let record: History = record.unwrap();
            cat.total = cat.total + record.value;
            cat.history.push(record);
        }
        cat_vec.push(cat);
    }
    Arc::new(RwLock::new(cat_vec))
}

pub fn insert_new_category(name: &str, val: i32) {
    let mut name = name.replace(" ", "_");
    name.extend(['.', 'c', 's', 'v'].iter());
    let mut writer = Writer::from_writer(vec![]);
    match File::open(&name) {
        Ok(_) => {
            return ();
        }
        _ => {}
    };
    let date_time = chrono::Local::now();
    let date = date_time.naive_local().date();
    let time = date_time.time();
    writer
        .serialize(History {
            value: val,
            time,
            date,
        })
        .unwrap();
    let contents = String::from_utf8(writer.into_inner().unwrap()).unwrap();
    fs::write(&name, contents).unwrap();
}

pub fn add_value(name: &str, val: i32) {
    let mut name = name.replace(" ", "_");
    name.extend(['.', 'c', 's', 'v'].iter());
    match File::open(&name) {
        Ok(_) => {}
        _ => {
            return ();
        }
    };
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(name)
        .unwrap();
    // let mut writer = WriterBuilder.has_headers(yes).from_writer(file);
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
    let date_time = chrono::Local::now();
    let date = date_time.naive_local().date();
    let time = date_time.time();
    writer
        .serialize(History {
            value: val,
            time,
            date,
        })
        .unwrap();
    writer.flush().unwrap();
}

pub fn get_csv_files() -> Vec<String> {
    let paths = fs::read_dir("./").unwrap();
    let mut ret = vec![];

    for path in paths {
        let x = path.unwrap().file_name();
        let t = x.to_str().unwrap();
        if t.ends_with(".csv") {
            ret.push(String::from(t));
        }
    }
    ret
}
