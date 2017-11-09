use rustc_serialize::json::{self};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use labware_collection::LabwareCollection;

pub fn load_file(path: &str) -> String {

  let path = Path::new(path);
  let display = path.display();

  let mut f = match File::open(&path) {
    Err(_)    => panic!("couldn't open {}:", display),
    Ok(file)  => file,
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Err(_)  => panic!("couldn't read {}:", display),
    Ok(_)   => (),
  };

  s
}

pub fn load_json(path: &str) -> LabwareCollection {

  let s = load_file(&path);
  json::decode(&s).unwrap()
}

pub fn write_file(labwares: LabwareCollection, path: &str) -> () {
  let path = Path::new(path);
  let display = path.display();

  let mut f = match File::create(&path) {
    Err(_)    => panic!("couldn't create {}:", display),
    Ok(file)  => file,
  };

  match f.write(json::encode(&labwares).unwrap().as_bytes()) {
        Err(_) => {
            panic!("couldn't write to {}", display)
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }

}

#[cfg(test)]
mod tests {

  use loader::*;
  use labware_collection::LabwareCollection;

  #[test]
  fn should_load_json() {
    let labwares: LabwareCollection = load_json("data/labwares.json");
    assert_eq!(labwares.count(), 3);
  }

  #[test]
  fn should_load_file() {
    let s = load_file("data/foo.txt");
    assert_eq!(s, "Hello world!");
  }

  #[test]
  fn should_write_to_file() {
    let mut labwares: LabwareCollection = load_json("data/labwares.json");
    labwares.add("Labware 4", "2016-12-20", "Tube");
    write_file(labwares, "data/labwares_output.json");
    let labwares = load_json("data/labwares_output.json");
    assert_eq!(labwares.count(), 4);
  }
}

