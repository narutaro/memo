use structopt::StructOpt;

#[derive(Debug)]
struct Memo {
    id: isize,
    name: String,
    pri: isize,
}

#[derive(StructOpt, Debug)]
/// Help you take notes
enum Opt {
    /// add a memo
    Add { name: String },
    /// delete a memo
    Del { id: String },
    /// list memos
    List {},
}

fn main() {
    let memo1 = Memo {
        id: 1,
        name: String::from("my first memo"),
        pri: 1,
    };
    let memo2 = Memo {
        id: 2,
        name: String::from("my second memo"),
        pri: 1,
    };
    let memo3 = Memo {
        id: 3,
        name: String::from("my third memo"),
        pri: 1,
    };

    let mut memos: Vec<Memo> = Vec::new();
    memos.push(memo1);
    memos.push(memo2);
    memos.push(memo3);

    match Opt::from_args() {
        Opt::Add { name } => println!("{} added", name),
        Opt::Del { id } => println!("{} deleted", id),
        Opt::List {} => println!("{:#?}", memos),
    };
}
