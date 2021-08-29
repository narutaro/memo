use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, Seek, SeekFrom};

#[derive(Serialize, Deserialize, Debug)]
struct Memo {
    id: isize,
    body: String,
    star: bool,
}

type Memos = HashMap<String, Memo>;

fn main() {
    //write_file("hello".to_string());
    //read_file();
    //add_memo("hello".to_string());
    list_memos();
    //let hm: Memos =
    //    serde_json::from_str("{\"300\":{\"id\":300,\"body\":\"hello\",\"star\":true},\"400\":{\"id\":400,\"body\":\"hey\",\"star\":true}}").unwrap();
    //new_id(&hm);
}

fn write_file(body: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memo.json")?;
    println!("file: {:#?}", file);

    let memo = Memo {
        id: 100,
        body: body,
        star: true,
    };

    let mut memos = HashMap::new();
    memos.insert(memo.id.to_string(), memo);

    // write as text
    let memos_json = serde_json::to_string(&memos).unwrap();
    println!("memos: {:?}", &memos_json);
    //serde_json::to_writer(&file, &memos_text)?; // escaped
    //write!(&file, "{:?}", memos_text)?; // escaped
    write!(&file, "{}", memos_json)?;

    // write as is
    //write!(&file, "{:?}", memos)?;

    Ok(())
}

fn read_file() -> std::io::Result<()> {
    let memos_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memo.json")?;
    //let memos_file = File::open("foo.json").unwrap();
    let reader = BufReader::new(memos_file);
    let memos: Memos = serde_json::from_reader(reader)?;
    println!("memo_text: {:?}", memos);
    Ok(())
}

pub fn add_memo(body: String) -> std::io::Result<()> {
    // read current memos
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memo.json")?;

    let reader = BufReader::new(&file);
    let mut memos: Memos = serde_json::from_reader(reader)?;
    // TODO - what if the file is empty
    println!("before - memos: {:#?}", &memos);

    // write new memos
    let memo = Memo {
        id: new_id(&memos),
        body: body,
        star: true,
    };

    memos.insert(memo.id.to_string(), memo);

    let memos_json = serde_json::to_string(&memos).unwrap();

    file.seek(SeekFrom::Start(0))?;
    write!(&file, "{}", memos_json)?;
    println!("after - memos: {:#?}", &memos);

    Ok(())
}

fn new_id(memos: &HashMap<String, Memo>) -> isize {
    let max_id = memos.keys().max().unwrap();
    println!("max_id: {:#?}", max_id);
    let new_id = max_id.parse::<isize>().unwrap() + 1;
    println!("new_id: {}", new_id);
    new_id
}

pub fn list_memos() -> std::io::Result<()> {
    let file = File::open("memo.json")?;
    let mut buf = BufReader::new(file);
    let deserialized: HashMap<isize, Memo> = serde_json::from_reader(&mut buf).unwrap();
    for (id, memo) in deserialized.iter() {
        println!("{} => {:?}", id, memo);
    }
    Ok(())
}
