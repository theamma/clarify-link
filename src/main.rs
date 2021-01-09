use structopt::StructOpt;
use std::borrow::Cow;
use regex::Regex;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Cli {
    obfuscated_link: String,

    #[structopt(short, long, 
                help = "Don't put schema part in the url")]
    skip_schema: bool,

    #[structopt(short, long, default_value = "https", 
                help = "Schema part of the url (http, https, ftp, ...)")]
    proto: String,
}

fn clarify(s: &str) -> Cow<str> {
    let re = Regex::new(r"(\[|\]|BLOCKED)").unwrap();
    re.replace_all(s, "")
}

fn main() {
    let args = Cli::from_args();

    if args.skip_schema {
        println!("{}", clarify(&args.obfuscated_link));
    } else {
        println!("{}://{}", args.proto, clarify(&args.obfuscated_link))
    }
}
