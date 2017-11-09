#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Labware {
  pub id: u32,
  pub name: String,
  pub timestamp: String, 
  pub labware_type: String,
}

impl Labware {
  pub fn new(id: u32, name: &str, timestamp: &str, labware_type: &str) -> Labware {
    Labware { id: id, 
              name: name.to_string(), 
              timestamp: timestamp.to_string(),
              labware_type: labware_type.to_string()
            }
  }
}

#[cfg(test)]
mod tests {

  use labware::Labware;

  #[test]
  fn labware_should_have_the_correct_attributes() {
    let labware = Labware::new(1, "Labware 1", "2016-01-01", "Plate");
    
    assert_eq!(labware.id, 1);
    assert_eq!(labware.name, "Labware 1");
    assert_eq!(labware.timestamp, "2016-01-01");
    assert_eq!(labware.labware_type, "Plate");
  }
}