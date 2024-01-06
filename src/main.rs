mod database;
use database::{Database, ManageDatabase};

fn main() {
    let mut db = Database::new(String::from("user"), 1024, String::from(".dbloc_data"));
    db.manage_database();
    match db.create_database() {
        Ok(_) => println!("Database create with success"),
        Err(err) => panic!("Error when created database: {}", err),
    }
}
