extern crate docopt;
extern crate git2;
extern crate rayon;
#[macro_use]
extern crate serde_derive;
extern crate time;

use std::str;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use docopt::Docopt;
use git2::Repository;
use rayon::prelude::*;

#[derive(Deserialize)]
struct Args {
    flag_wof_data_dir: Option<String>,
}

fn run(args: &Args) -> Result<(), Box<Error>> {
    let wof_path = args.flag_wof_data_dir.as_ref().map(|s| &s[..]).unwrap();
    println!(
        "loading geojson from repository located around {}",
        wof_path
    );

    let repo = Repository::open(wof_path)?;
    let index = repo.index()?;

    let paths: Vec<_> = index
        .iter()
        .map(|index_entry| String::from_utf8(index_entry.path).unwrap())
        .collect(); // collect into whatever list is

    println!("paths len:{}", paths.len());
    paths.par_iter().for_each(|path| {
        let file_path = Path::new(wof_path).join(&path);

        println!(
            "lookin at: {} ..  file? {}",
            file_path.display(),
            file_path.is_file()
        );
        if file_path.is_file() {
            let mut file = File::open(&file_path).unwrap();
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();
            println!("{} length: {}", file_path.display(), contents.len());
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

    let args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}
