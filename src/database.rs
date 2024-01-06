use std::fs;
use std::path::Path;

pub struct Database {
    pub name: String,
    pub size: i32,
    pub db_loc: String,
}

impl Database {
    pub fn new(name: String, size: i32, db_loc: String) -> Self {
        Self { name, size, db_loc }
    }
}

pub trait ManageDatabase {
    fn manage_database(&mut self);
    fn create_database(&self) -> Result<(), std::io::Error>;
}

impl ManageDatabase for Database {
    fn manage_database(&mut self) {
        self.size = 1024;
        self.db_loc = String::from(".dbloc_data");
    }

    fn create_database(&self) -> Result<(), std::io::Error> {
        if let Err(_metadata) = fs::metadata(&self.db_loc) {
            let _ = fs::create_dir(&self.db_loc);
        }
        let db_path = Path::new(&self.db_loc).join(&self.name);
        let _ = fs::create_dir(db_path);

        Ok(())
    }
}
