use crate::phonebook::PhoneBook;

pub trait PhonebookFrontend {
    fn run(phonebook: &mut PhoneBook);
}