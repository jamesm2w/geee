use std::io::{self, Write};

pub mod party;
pub mod result;

fn main() {
    println!("GEEE: General Election E E");

    let mut file_path = String::new();
    io::stdout().write("Enter the file path: ".as_bytes()).unwrap();
    io::stdout().flush().expect("what");
    io::stdin().read_line(&mut file_path).expect("Failed to read line");

    // let file_path = "C:/Users/james/OneDrive/Documents/Elections/geee/data/2019.json";
    let file_path_trim = file_path.trim();
    println!("{:?}", file_path_trim);

    if let Some(election) = result::parse(&file_path_trim) {

        for seat in election {
            println!("{:<3} {}", seat.winner.party, seat.name);
        }

    }
}
