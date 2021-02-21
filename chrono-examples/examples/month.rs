use chrono::Month;
use num_traits::FromPrimitive;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = parse_month))]
    month: Month,
}

fn parse_month(src: &str) -> Result<Month, String> {
    let err_str = "out_of_range";
    let src_int = u32::from_str(src).map_err(|_| err_str.to_string())?;
    Month::from_u32(src_int).ok_or(err_str.to_string())
}

fn main() {
    let opt = Opt::from_args();
    dbg!(opt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u32() {
        assert_eq!(Month::January, Month::from_u32(1).unwrap());
    }

    #[test]
    fn test_from_u32_out_of_range() {
        assert_eq!(None, Month::from_u32(13));
    }

    #[test]
    fn test_number_from_month() {
        assert_eq!(1, Month::January.number_from_month());
        assert_eq!(8, Month::August.number_from_month());
    }
}
