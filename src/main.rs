use model::Memos;
use structopt::StructOpt;
mod model;

#[derive(StructOpt, Debug)]
/// Help you take notes
enum Opt {
    /// Add a memo
    Add { body: String },
    /// Delete a memo
    Del { id: String },
    /// List memos
    List {},
    /// Find memo
    Find { body: String },
}

fn main() {
    match Opt::from_args() {
        Opt::Add { body } => fmt(model::add_memo(body).unwrap()),
        Opt::Del { id } => fmt(model::delete_memo(id).unwrap()),
        Opt::List {} => fmt(model::list_memos().unwrap()),
        Opt::Find { body } => fmt(model::find_memo(body).unwrap()),
    };
}

fn fmt(memos: Memos) {
    println!("{0: ^5} | {1: ^150} | {2: ^5}", "#", "description", "star");
    for (id, memo) in memos.iter() {
        println!("{0: <5} | {1: <150} | {2: <5}", id, memo.body, memo.star);
        //println!("{} => {:?}", id, memo);
    }
}
