extern crate simplebase;
use serde::{Deserialize, Serialize};
use simplebase::engine::*;
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct Memo {
    id: isize,
    body: String,
    star: bool,
}

#[derive(StructOpt, Debug)]
/// Help you take notes
enum Opt {
    /// add a memo
    Add { body: String },
    /// delete a memo
    Del { id: String },
    /// list memos
    List {},
}

// DB
fn list(db: &RecordData) {
    println!("----");
    for (k, v) in &db.hash_data {
        println!("k: {:?}, v: {:?}", k, v);
    }
}

fn main() {
    //
    // Serde
    //
    let memo = Memo {
        id: 1,
        body: "first memo".to_string(),
        star: true,
    };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&memo).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Memo = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    //
    // DB
    //
    let mut db = new_empty_database();
    db.add_record(serialized);
    db.save_database("memo.db");
    list(&db);

    //
    // CLI
    //
    let memo1 = Memo {
        id: 1,
        body: String::from("my first memo"),
        star: true,
    };
    let memo2 = Memo {
        id: 2,
        body: String::from("my second memo"),
        star: false,
    };
    let memo3 = Memo {
        id: 3,
        body: String::from("my third memo"),
        star: true,
    };

    let mut memos: Vec<Memo> = Vec::new();
    memos.push(memo1);
    memos.push(memo2);
    memos.push(memo3);

    match Opt::from_args() {
        Opt::Add { body } => println!("{} added", body),
        Opt::Del { id } => println!("{} deleted", id),
        Opt::List {} => println!("{:#?}", memos),
    };
}
