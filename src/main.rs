use structopt::StructOpt;

#[derive(Debug)]
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

fn main() {
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
