use clap::Clap;
use rusqlite::{params, Connection, Result};

#[derive(Clap, Debug)]
#[clap(
    name = "Huuzoku name generator",
    version = "1.0.0",
    author = "Yuhei Nakasaka",
    about = "Generator of shop names like huuzoku"
)]

struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "count", short, long, default_value = "4")]
    count: String,
}

#[derive(Debug)]
struct Wakati {
    word: String,
    position: i32,
}

fn main() {
    let opts = Opts::parse();
    generate(opts.count, opts.verbose).expect("Failed to generate words.");
}

fn generate(count: String, verbose: bool) -> Result<String> {
    let count = count.parse::<i32>().expect("count must be integer.");
    let conn = Connection::open("data.db")?;
    let mut words: Vec<String> = Vec::new();
    for i in 1..count + 1 {
        let mut stmt = conn.prepare("select word, position from wakati_shopname where length(word) > 1 and position = ? group by word order by random() limit 1;")?;
        let resp = stmt.query_map(params![i], |row| {
            Ok(Wakati {
                word: row.get(0)?,
                position: row.get(1)?,
            })
        })?;
        for res in resp {
            if verbose {
                println!("{:?}", res);
            }
            let word = res?.word;
            words.push(word);
        }
    }
    let result = words.join("");
    println!("{:?}", result);
    Ok(result)
}

#[cfg(test)]
mod generate {
    use super::*;

    #[test]
    fn valid_generation() {
        let result = generate("3".to_string(), false);
        match result {
            Ok(s) => assert!(s.chars().count() > 0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn zero_count() {
        let result = generate("0".to_string(), false);
        match result {
            Ok(s) => assert_eq!(s, ""),
            Err(_) => assert!(false),
        }
    }

    #[test]
    #[should_panic]
    fn invalid_count() {
        let result = generate("abc".to_string(), false);
        match result {
            Ok(_) => assert!(false),
            Err(_) => assert!(false),
        }
    }
}
