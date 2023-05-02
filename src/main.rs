use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn usage(pname: &str) {
    println!("Usage: {} -f <file.json> -r <record>", pname);
}

fn get_id<'a>(option: &'a str, value: &'a str) -> Id<'a> {
    match option {
        "-r" => Id::Record(value),
        "-e" => Id::Event(value),
        _ => Id::Unknown,
    }
}

enum Id<'a> {
    Record(&'a str),
    Event(&'a str),
    Unknown,
}

fn parse_record(filename: &str, records_id: &str) {
    let Ok(file) = File::open(filename) else {
        println!("Error: can't open the file '{}'", filename);
        return;
    };
    let reader = BufReader::new(file);
    let mut pflag: bool = false;
    for line in reader.lines() {
        let mut records_id = records_id.split(',');
        let line = line.unwrap();
        if line.starts_with("Record") {
            pflag = false;
            if records_id.any(|record_id| line.ends_with(record_id)) {
                pflag = true;
            }
        }
        if pflag {
            println!("{}", line);
        }
    }
}

fn parse_event(_filename: &str, _event_id: &str) {
    // let Ok(file) = File::open(filename) else {
    // println!("Error: can't open the file '{}'", filename);
    // return;
    // };
    // let reader = BufReader::new(file);
    println!("Work in progress 🛠️");
    // TODO
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        usage(&args[0]);
        return;
    }

    let filename: &str;
    let id: Id;
    if args[1] == "-f" {
        filename = &args[2];
        id = get_id(&args[3], &args[4]);
    } else {
        filename = &args[4];
        id = get_id(&args[1], &args[2]);
    }

    match id {
        Id::Record(rid) => parse_record(filename, rid),
        Id::Event(eid) => parse_event(filename, eid),
        Id::Unknown => usage(&args[0]),
    };
}
