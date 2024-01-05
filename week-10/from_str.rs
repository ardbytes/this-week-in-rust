use std::str::FromStr;

struct Square {
    size: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSquareError {
    error: String,
}

impl FromStr for Square {
    type Err = ParseSquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sz = s.trim();

        let sz_fromstr = sz.parse::<u64>().map_err(|_| ParseSquareError {
            error: format!("Square cannot have size {}", sz),
        })?;

        Ok(Square { size: sz_fromstr })
    }
}

fn main() {
    println!(
        "Size of square is {}",
        " 2 ".parse::<Square>().unwrap().size
    );
    let sq = " a ".parse::<Square>();
    match sq {
        Err(psq) => println!("Invalid square: {}", psq.error),
        _ => (),
    }
}
