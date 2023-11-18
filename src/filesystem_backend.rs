use crate::contact::Contact;
use crate::phonebook::PhoneBook;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

pub struct FilesystemBackend
{
    pub file_path: PathBuf
}

impl FilesystemBackend
{
    pub fn new(folder_path : &Path)->Self
    {
        return FilesystemBackend{file_path : folder_path.join("phonebook.yaml")};
    }

    pub fn load(&self) -> std::io::Result<PhoneBook>
    {
        // read yaml file

        let yaml = fs::read_to_string(&self.file_path)?;

        // load from yaml format
        let contacts: Vec<Contact> = serde_yaml::from_str::<Vec<Contact>>(&yaml).unwrap();
        return Ok(PhoneBook::new(contacts));
    }

    pub fn save(&self, phonebook: &PhoneBook) -> std::io::Result<()>
    {
        // convert to yaml
        let yaml = serde_yaml::to_string(&phonebook.get_all()).expect("Unable to serialize contacts");
        // save to file
        return fs::write(&self.file_path, yaml);
    }
}