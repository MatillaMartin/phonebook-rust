mod contact;
mod phonebook;
mod console_frontend;

use crate::phonebook::PhoneBook;
use crate::console_frontend::ConsoleFrontend;

fn main() {
    let mut phonebook = PhoneBook::default();
    let console = ConsoleFrontend::default();
    console.run(&mut phonebook);
}
