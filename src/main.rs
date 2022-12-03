use clap::{arg, command, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .next_line_help(true)
        .arg(arg!(-t --two <VALUE>).required(true).action(ArgAction::Set))
        .arg(arg!(-o --one <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );

    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );

    ba_lib::a();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        2
    }
}

