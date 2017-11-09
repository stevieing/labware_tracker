use labware::Labware;

#[derive(RustcDecodable,RustcEncodable)]
pub struct LabwareCollection {

  pub items: Vec<Labware>,
}

impl LabwareCollection {
  pub fn new() -> Self {
    LabwareCollection{ items: Vec::<Labware>::new() }
  }

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }

  pub fn count(&self) -> usize {
    self.items.len()
  }

  pub fn add(&mut self, name: &str, timestamp: &str, labware_type: &str) -> () {
    let id = self.count() as u32;
    self.items.push(Labware::new(id+1, name, timestamp, labware_type));
  }

  pub fn find(&self, id: u32) -> Option<&Labware> {
    self.items.iter().find(|labware| labware.id == id)
  }

  pub fn remove(&mut self, id: u32) -> () {
    let index = self.items.iter().enumerate().find(|&labware| labware.1.id == id).unwrap().0;
    self.items.remove(index);
  }
}

#[cfg(test)]
mod tests {

  use labware_collection::LabwareCollection;

  #[test]
  fn new_labware_collection_should_be_empty() {
    let labware_collection = LabwareCollection::new();

    assert!(labware_collection.is_empty());
    assert_eq!(labware_collection.count(), 0);

  }

  #[test]
  fn should_add_labware() {
    let mut labware_collection = LabwareCollection::new();
    labware_collection.add("Labware 1", "2016-11-14", "Plate");

    assert_eq!(labware_collection.count(), 1);

    match labware_collection.items.last() {
        Some(labware) => assert_eq!(labware.id, 1),
        None => panic!("No labware added"),
    }

   labware_collection.add("Labware 2", "2016-11-15", "Tube");

    match labware_collection.items.last() {
        Some(labware) => assert_eq!(labware.id, 2),
        None => panic!("No labware added"),
    }

  }

  #[test]
  fn should_find_labware() {
    let mut labware_collection = LabwareCollection::new();
    labware_collection.add("Labware 1", "2016-11-14", "Plate");
    labware_collection.add("Labware 2", "2016-11-15", "Tube");

    assert_eq!(labware_collection.find(2).unwrap().id, 2);
    assert!(labware_collection.find(4).is_none());
  }

  #[test]
  fn should_remove_labware() {
    let mut labware_collection = LabwareCollection::new();
    labware_collection.add("Labware 1", "2016-11-14", "Plate");
    labware_collection.add("Labware 1", "2016-11-14", "Tube");

    assert_eq!(labware_collection.count(), 2);

    labware_collection.remove(2);
    assert_eq!(labware_collection.count(), 1);

    labware_collection.remove(1);
    assert!(labware_collection.is_empty());

    labware_collection.remove(10);
    assert!(labware_collection.is_empty());

  }
}