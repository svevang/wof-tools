#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate time;
extern crate walkdir;
extern crate rayon;

use std::str;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use docopt::Docopt;

use walkdir::WalkDir;

use rayon::prelude::*;

#[derive(Deserialize)]
struct Args {
    flag_wof_data_dir: Option<String>,
}

fn run(args: &Args) -> Result<(), Box<Error>> {
    let wof_path = args.flag_wof_data_dir.as_ref().map(|s| &s[..]).unwrap();
    println!("loading geojson from {}", wof_path);

    let paths: Vec<_> = WalkDir::new(wof_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|f| f.ok())       // filter out errors (silently!)
        .map(|f| f.path().to_owned()) // take the path and take ownership
        .collect();                   // collect into whatever list is

    paths.par_iter().for_each (|file_path| {

        if file_path.is_file() {
            let mut file = File::open(file_path).unwrap();
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            println!("{} length: {}", file_path.display(), contents.len() );
        }

    });

    Ok(())
}


fn main() {
    const USAGE: &'static str = "
usage: wof-tools [options]

Options:
    --wof-data-dir <dir>    wof directory to use
    -h, --help              show this message
";

    let args = Docopt::new(USAGE).and_then(|d| d.deserialize())
                                 .unwrap_or_else(|e| e.exit());
    match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}
