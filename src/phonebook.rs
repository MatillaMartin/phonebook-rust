use crate::contact::Contact;

pub struct PhoneBook
{
    contacts: Vec<Contact>,
}

impl Default for PhoneBook
{
    fn default() -> PhoneBook
    {
        PhoneBook
        {
            contacts: Vec::new()
        }
    }
}

impl PhoneBook
{
    pub fn get_all(&self) -> Vec<Contact>
    {
        return self.contacts.clone();
    }

    pub fn exists(&mut self, contact_name: &String) -> bool
    {
        let found = self.find_name(&contact_name);
        return found.is_some();
    }

    pub fn add(&mut self, contact: Contact) -> bool
    {
        if !self.exists(&contact.name)
        {
            self.contacts.push(contact);
            return true;
        }
        return false;
    }

    pub fn remove(&mut self, contact_name: &String) -> bool
    {
        let ret = self.find_name(&contact_name);
        if let Some(pos) = ret
        {
            self.contacts.remove(pos);
            return true;
        }
        return false;
    }

    fn find_name(&self, contact_id: &String) -> Option<usize>
    {
        return self.contacts.iter().position(|x| x.name == *contact_id);
    }
}
