
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum LabwareType {
  Plate,
  Tube,
  Other
}

impl fmt::Display for LabwareType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

impl From<&'static str> for LabwareType {
    fn from(val: &'static str) -> Self {
        match val {
            "Plate" => LabwareType::Plate,
            "Tube" => LabwareType::Tube,
            _ => LabwareType::Other,
        }
    }
}

#[cfg(test)]
mod tests {

  use labware_type::LabwareType;

  #[test]
  fn should_have_all_the_standard_types() {
    assert_eq!(LabwareType::Plate.to_string(), "Plate");
    assert_eq!(LabwareType::Tube.to_string(), "Tube");
  }

  #[test]
  fn should_convert_string_to_labware_type() {
    assert_eq!(LabwareType::from("Plate"), LabwareType::Plate);
    assert_eq!(LabwareType::from("Tube"), LabwareType::Tube);
  }

}