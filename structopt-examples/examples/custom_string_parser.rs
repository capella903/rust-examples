use structopt::StructOpt;

#[derive(Debug)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
}

fn parse_month(src: &str) -> Result<Month, String> {
    match src {
        "1" => Ok(Month::January),
        "2" => Ok(Month::February),
        "3" => Ok(Month::March),
        _ => Err("Out of range".to_string()),
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = parse_month))]
    month: Month,
}

fn main() {
    let opt = Opt::from_args();
    dbg!(opt);
}
