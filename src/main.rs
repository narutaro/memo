//extern crate simplebase;
//use serde::{Deserialize, Serialize};
//use simplebase::engine::*;
use model::Memos;
use structopt::StructOpt;
mod model;

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

fn main() {
    //    model::add_memo("ciao".to_string());
    //    model::list_memos();

    match Opt::from_args() {
        Opt::Add { body } => println!("added"),
        Opt::Del { id } => println!("deleted"),
        Opt::List {} => println!("{:?}", fmt(model::list_memos().unwrap())),
    };
}

fn fmt(memos: Memos) {
    for (id, memo) in memos.iter() {
        println!("{} => {:?}", id, memo);
    }
}
