#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;
use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel::sql_types::Text;
use diesel::QueryableByName;

enum LogLevel {
    Debug   = 1,
    Info    = 2,
    Warning = 3,
    Error   = 4
}

fn str_to_log_level(log_level: &str) -> Option<LogLevel> {
    match log_level {
        "debug"   => Some(LogLevel::Debug),
        "info"    => Some(LogLevel::Info),
        "warning" => Some(LogLevel::Warning),
        "error"   => Some(LogLevel::Error),
        _         => None
    }
}

#[database("db_log")]
struct LogsDbConn(diesel::MysqlConnection);

#[derive(QueryableByName)]
struct LogRec {
#[sql_type = "Text"]
    txt: String,
#[sql_type = "Text"]
    dt: String
}

#[get("/get/<log_level>/<time_begin>/<time_end>")]
fn get_handler(conn: LogsDbConn, log_level: String, time_begin: String, time_end: String) -> String {
    let log_level = str_to_log_level(&log_level)
        .expect("invalid log level");

    let log_recs = diesel::sql_query(
            "select txt, DATE_FORMAT(dt, '%Y-%m-%dT%H:%i:%s.%f') dt 
             from log 
             where log_level = ? and dt between STR_TO_DATE(?, '%Y-%m-%dT%H:%i:%s.%f') and STR_TO_DATE(?, '%Y-%m-%dT%H:%i:%s.%f');")
        .bind::<Integer, _>(log_level as i32)
        .bind::<Text, _>(time_begin)
        .bind::<Text, _>(time_end)
        .load::<LogRec>(&*conn)
        .expect("Error selecting");

    let mut ret = String::new();
    for rec in log_recs.iter() {
        ret = ret + "dt: "  + &rec.dt  + "\n"
                  + "txt: " + &rec.txt + "\n";
    }
    ret
}

#[post("/save/<log_level>", data = "<txt>")]
fn save_handler(conn: LogsDbConn, log_level: String, txt: String) -> &'static str {
    let log_level = str_to_log_level(&log_level)
        .expect("invalid log level");

    diesel::sql_query("insert into log (log_level, txt) values (?, ?);")
        .bind::<Integer, _>(log_level as i32)
        .bind::<Text, _>(txt)
        .execute(&*conn)
        .expect("Error inserting");
    "save_handler"
}

fn main() {
    rocket::ignite()
        .attach(LogsDbConn::fairing())
        .mount("/", routes![get_handler, save_handler])
        .launch();
}

