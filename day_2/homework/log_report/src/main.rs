use std::env;
mod scan_log;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 || !args[0].ends_with(".log") {
        println!("error: expected only <file name>.log");
        return;
    }

    let file_name = args.remove(0);
    let result = scan_log::scan(&file_name);

    match result {
        Ok(counts) => {
            let total: u32 = counts.values().sum();
            println!("report for {} ({} lines)", file_name, total);

            let mut levels: Vec<&String> = counts.keys().collect();
            levels.sort();

            for level in levels {
                println!("{}: {}", level, counts[level]);
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
