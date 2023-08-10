pub fn main() {
  loop {
    crate::helper::create_menu("Journal", &vec![
      "Create Entry",
      "Read Entries",
      "Update Entry",
      "Delete Entry",
    ]);
  }
}