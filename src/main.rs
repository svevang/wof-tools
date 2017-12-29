/*
 * libgit2 "log" example - shows how to walk history and get commit info
 *
 * Written by the libgit2 contributors
 *
 * To the extent possible under law, the author(s) have dedicated all copyright
 * and related and neighboring rights to this software to the public domain
 * worldwide. This software is distributed without any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication along
 * with this software. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate git2;
extern crate time;

use std::str;
use std::path::Path;

use docopt::Docopt;
use git2::{Repository, ObjectType};
use git2::{Error};

#[derive(Deserialize)]
struct Args {
    flag_git_dir: Option<String>,
}

fn run(args: &Args) -> Result<(), Error> {
    let path = args.flag_git_dir.as_ref().map(|s| &s[..]).unwrap_or(".");
    let repo = Repository::open(path)?;

    println!("looking for wof geojson in {}", repo.path().to_string_lossy());

    let index = repo.index().unwrap();

    for index_entry in index.iter() {
        let path_name = String::from_utf8(index_entry.path).unwrap();
        println!("path name {}", path_name);
        println!("looking!");
        let object = match repo.find_object(index_entry.id, Some(ObjectType::Blob)) {
            Ok(i) => i,
            Err(err) => continue,
        };
        let blob = object.as_blob();
        //println!("path: {} OID: {}", String::from_utf8(index_entry.path).unwrap(), index_entry.id);
        let path = Path::new(&path_name);
        //path.display();
    }

    Ok(())
}


fn main() {
    const USAGE: &'static str = "
usage: log [options] [<commit>..] [--] [<spec>..]

Options:
    --git-dir <dir>         alternative git directory to use
    -h, --help              show this message
";

    let args = Docopt::new(USAGE).and_then(|d| d.deserialize())
                                 .unwrap_or_else(|e| e.exit());
    match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}
