use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Seek, SeekFrom};

#[derive(Serialize, Deserialize, Debug)]
pub struct Memo {
    pub id: isize,
    pub body: String,
    pub star: bool,
}

pub type Memos = HashMap<String, Memo>;

fn main() {}

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

    let memos_json = serde_json::to_string(&memos).unwrap();
    println!("memos: {:?}", &memos_json);
    write!(&file, "{}", memos_json)?;

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

//pub fn add_memo(body: String) -> std::io::Result<()> {
pub fn add_memo(body: String) -> std::io::Result<Memos> {
    // read current memos
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("memo.json")?;

    let reader = BufReader::new(&file);
    let mut memos: Memos = serde_json::from_reader(reader)?;
    // TODO - what if the file is empty
    //println!("before - memos: {:#?}", &memos);

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
    //println!("after - memos: {:#?}", &memos);

    //Ok(())
    Ok(memos)
}

pub fn delete_memo(id: String) -> std::io::Result<Memos> {
    // read current memos
    //let mut file = OpenOptions::new().read(true).open("memo.json")?;
    let file = OpenOptions::new().read(true).open("memo.json")?;

    let reader = BufReader::new(&file);
    let mut memos: Memos = serde_json::from_reader(reader)?;
    // TODO - what if the file is empty
    //println!("before - memos: {:#?}", &memos);
    drop(file);

    // remove
    &memos.remove(&id);

    let memos_json = serde_json::to_string(&memos).unwrap();

    //let mut file = OpenOptions::new()
    let file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open("memo.json")?;
    write!(&file, "{}", memos_json)?;
    //println!("after - memos: {:#?}", &memos);

    Ok(memos)
}

fn new_id(memos: &HashMap<String, Memo>) -> isize {
    let max_id = memos.keys().max().unwrap();
    //println!("max_id: {:#?}", max_id);
    let new_id = max_id.parse::<isize>().unwrap() + 1;
    //println!("new_id: {}", new_id);
    new_id
}

//pub fn list_memos() -> std::io::Result<()> {
pub fn list_memos() -> std::io::Result<Memos> {
    let file = File::open("memo.json")?;
    let mut buf = BufReader::new(file);
    //let memos: HashMap<isize, Memo> = serde_json::from_reader(&mut buf).unwrap();
    let memos: HashMap<String, Memo> = serde_json::from_reader(&mut buf).unwrap();
    //for (id, memo) in memos.iter() {
    //    println!("{} => {:?}", id, memo);
    //}
    //Ok(())
    Ok(memos)
}

pub fn find_memo(keyword: String) -> std::io::Result<Memos> {
    let file = File::open("memo.json")?;
    let buf = BufReader::new(file);
    let memos: HashMap<String, Memo> = serde_json::from_reader(buf).unwrap();

    let mut found = HashMap::<String, Memo>::new();

    for (key, value) in memos {
        match value.body.contains(&keyword) {
            true => {
                found.insert(key, value);
            }
            _ => (),
        }
    }

    Ok(found)
}
