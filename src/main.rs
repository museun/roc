use clap::Clap;
use roc::locate;

/// roc -- command lines rust documentation that rocks
#[derive(Clap, Debug)]
struct Options {
    /// list out all known modules
    #[clap(short = "l", long = "list")]
    list: bool,

    /// show any example code that is provided in the full docs
    #[clap(short = "e", long = "show-examples")]
    show_examples: bool,

    /// open the selected doc page in the browser using full rustdoc
    #[clap(short = "o", long = "open")]
    open_in_browser: bool,

    /// grep through the documentation for partial matches (ignores case)
    #[clap(short = "s", long = "string")]
    grep: Option<String>,

    /// <mod>[::<symbol>[::<method>]]
    target: String,
}

fn main() {
    let opts: Options = Options::parse();
    let locator = locate::Locator::new();

    println!("{:?}", opts);
    println!("{:?}", locator);
}