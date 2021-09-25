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
    //    model::add_memo("ciao".to_string());
    //    model::list_memos();

    match Opt::from_args() {
        Opt::Add { body } => println!("{:?}", fmt(model::add_memo(body).unwrap())),
        Opt::Del { id } => println!("{:?}", fmt(model::delete_memo(id).unwrap())),
        Opt::List {} => println!("{:?}", fmt(model::list_memos().unwrap())),
        Opt::Find { body } => println!("{:?}", fmt(model::find_memo(body).unwrap())),
    };
}

fn fmt(memos: Memos) {
    for (id, memo) in memos.iter() {
        println!("{} => {:?}", id, memo);
    }
}
