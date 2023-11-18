mod contact;
mod phonebook;
mod console_frontend;
mod filesystem_backend;

use crate::phonebook::PhoneBook;
use crate::console_frontend::ConsoleFrontend;
use crate::filesystem_backend::FilesystemBackend;

fn main() {
    // create a backend filesystem for the phonebook
    let filesystem_backend = FilesystemBackend::new(std::path::Path::new("C:/Users/Hennio/Projects/Rust/phone_book/data"));
    // load phone book or default empty
    let mut phonebook = filesystem_backend.load().unwrap_or(PhoneBook::default());
    // create a console frontend and run it
    let console = ConsoleFrontend::default();
    console.run(&mut phonebook);
    // save phonebook before closing
    filesystem_backend.save(&phonebook).expect("Could not save phonebook");
}
