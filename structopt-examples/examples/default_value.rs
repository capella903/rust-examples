use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, Clone, Copy)]
enum Month {
    January = 1,
    February = 2,
    March = 3,
}

impl FromStr for Month {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Month::January),
            "2" => Ok(Month::February),
            "3" => Ok(Month::March),
            _ => Err("Out of range".to_string()),
        }
    }
}

impl Default for Month {
    fn default() -> Self {
        Month::January
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(default_value)]
    month: Month,
}

fn main() {
    let opt = Opt::from_args();
    dbg!(opt);
}
