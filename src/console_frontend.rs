use std::io::Write;
use crate::phonebook::PhoneBook;
use crate::contact::Contact;
use crate::phonebook_frontend::PhonebookFrontend;

pub struct ConsoleFrontend;

enum ConsoleAction
{
    Add(),
    Remove(),
    Print(),
    Close(),
}

impl PhonebookFrontend for ConsoleFrontend
{
    fn run(phonebook: &mut PhoneBook)
    {
        Self::print_intro(&phonebook);
        let mut run = true;
        while run
        {
            run = Self::run_actions(phonebook);
        }
    }
}


impl ConsoleFrontend
{
    // runs application, action loop
    fn run_actions(phonebook: &mut PhoneBook) -> bool
    {
        Self::print_menu();
        let input = Self::get_input();
        let action = Self::parse_action(&input);
        return match action
        {
            Some(x) => Self::run_action(x, phonebook),
            None => {
                println!("Could not process action");
                true
            }
        };
    }

    fn print_intro(phonebook: &PhoneBook)
    {
        println!("PhoneBook Application");
        println!("---------------------");
        println!("Contacts: {}", phonebook.get_all().len());
    }

    fn print_menu()
    {
        println!("---------------------");
        println!("[0] Print contacts");
        println!("[1] Add contact");
        println!("[2] Remove contact");
        println!("[3] Close");
        println!("Select action: ");
        let _ = std::io::stdout().flush();
    }

    fn get_input() -> String
    {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Did not enter a correct string");
        return input.trim().to_string();
    }

    fn parse_action(str: &String) -> Option<ConsoleAction>
    {
        return match str.as_str()
        {
            "0" => Some(ConsoleAction::Print()),
            "1" => Some(ConsoleAction::Add()),
            "2" => Some(ConsoleAction::Remove()),
            "3" => Some(ConsoleAction::Close()),
            _ => None,
        };
    }

    fn run_action(action: ConsoleAction, phonebook: &mut PhoneBook) -> bool
    {
        return match action
        {
            ConsoleAction::Print() => {
                Self::phonebook_print(phonebook);
                true
            }
            ConsoleAction::Add() => {
                Self::phonebook_add(phonebook);
                true
            }
            ConsoleAction::Remove() => {
                Self::phonebook_remove(phonebook);
                true
            }
            ConsoleAction::Close() => false,
        };
    }

    fn phonebook_print_contact(contact: &Contact)
    {
        println!("Name:\t\t{}", contact.name);
        println!("Mobile:\t\t{}", contact.mobile);
        println!("Address:\t{}", contact.address);
        println!("Email:\t\t{}", contact.email);
        println!("------");
    }

    fn phonebook_print(phonebook: &PhoneBook)
    {
        let contacts = phonebook.get_all();
        println!("PhoneBook");
        println!("---------");
        contacts.iter().for_each(Self::phonebook_print_contact);

        if contacts.is_empty()
        {
            println!("No contacts");
        }
    }

    fn phonebook_add(phonebook: &mut PhoneBook)
    {
        // get contact info
        let name: String;
        let email: String;
        let address: String;
        let mobile: String;
        println!("Name: ");
        name = Self::get_input();
        // already check for name here
        if phonebook.exists(&name)
        {
            println!("Could not add contact, name already in use");
            return;
        }

        println!("Mobile: ");
        mobile = Self::get_input();
        println!("Address: ");
        address = Self::get_input();
        println!("Email: ");
        email = Self::get_input();
        let ok = phonebook.add(Contact { name, email, address, mobile });
        if !ok
        {
            println!("Could not add contact");
            return;
        }
    }

    fn phonebook_remove(phonebook: &mut PhoneBook)
    {
        let name: String;
        println!("Name: ");
        name = Self::get_input();
        let ok = phonebook.remove(&name);
        if !ok
        {
            println!("Could not remove contact: {}", &name);
        }
    }
}