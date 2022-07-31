use std::env;
use std::fs;
mod html;

fn process_file(file_as_string: String) -> std::io::Result<()> {
    let data_vector : Vec<html::Data> = html::parsing::parse_list(file_as_string);
    html::write_index("index.html".to_string(), data_vector)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: optview-to-highcharts index.html");
        std::process::exit(-128);
    }

    let html_index = fs::read_to_string(&args[1]);

    match html_index {
        Ok(index_as_string) => {
            match process_file(index_as_string) {
                Ok(_) => std::process::exit(0),
                Err(_) => std::process::exit(-128)
            }
        },
        Err(e) =>{
            println!("There was a problem while reading the file {}", e);
            std::process::exit(-128)
        },
    }

}
