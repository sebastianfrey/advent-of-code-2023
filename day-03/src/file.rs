use std::{fs::File, io::BufRead, io::BufReader};

pub trait FileProcessor {
  fn get_result(&mut self) -> i32;
  fn on_start(&mut self) -> ();
  fn on_line(&mut self, line: String) -> ();
  fn on_finish(&mut self) -> ();
}

pub struct FileReader<'r> {
  pub file: &'r str,
  pub processor: Box<dyn FileProcessor>,
}

impl FileReader<'_> {
  pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>> {
      let file = File::open(self.file)?;
      let reader = BufReader::new(file);

      self.processor.on_start();

      for _line in reader.lines() {
          let line = _line?;
          self.processor.on_line(line);
      }

      self.processor.on_finish();

      Ok(())
  }
}
