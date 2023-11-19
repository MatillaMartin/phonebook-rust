mod contact;
mod phonebook;
mod console_frontend;
mod filesystem_backend;

use crate::phonebook::PhoneBook;
use crate::console_frontend::ConsoleFrontend;
use crate::filesystem_backend::FilesystemBackend;
use directories::UserDirs;

fn main()
{
    let filesystem_backend = create_filesystem_backend();
    match filesystem_backend
    {
        None =>
            {
                println!("Could not create filesystem backend");
                return;
            }
        Some(backend) =>
            {
                let mut phonebook = backend.load().unwrap_or(PhoneBook::default());
                // create a console frontend and run it
                let console = ConsoleFrontend::default();
                console.run(&mut phonebook);
                // save phonebook before closing
                let save_res = backend.save(&phonebook);
                match save_res
                {
                    Ok(..) => { },
                    Err(error) => { println!("Could not save phonebook: {}", error)}
                }
            }
    }

}

// create a backend filesystem for the phonebook
fn create_filesystem_backend() -> Option<FilesystemBackend>
{
    let user_dirs = UserDirs::new()?;
    let path: std::path::PathBuf = user_dirs.home_dir().join("phonebook");
    return Some(FilesystemBackend::new(path.as_path()));
}
