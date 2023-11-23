use crate::phonebook::PhoneBook;
use crate::phonebook_frontend::PhonebookFrontend;
use axum::{
    extract::State,
    Router,
    routing::{get, post},
    Server,
    Form,
    response::{Redirect}
};

use std::sync::{Arc, Mutex};
use crate::contact::Contact;
use build_html::{Container, ContainerType, Html, HtmlContainer, HtmlPage, Table};

#[derive(Debug)]
struct Link {
    href: String,
    rel: String,
    integrity: String,
    crossorigin: String,
}

impl Link {
    pub fn new(href: impl ToString, rel: impl ToString, integrity: impl ToString, crossorigin: impl ToString) -> Self {
        Link { href: href.to_string(), rel: rel.to_string(), integrity: integrity.to_string(), crossorigin: crossorigin.to_string() }
    }
}

impl Html for Link {
    fn to_html_string(&self) -> String {
        return format!("<link href=\"{}\", rel=\"{}\", integrity=\"{}\", crossorigin=\"{}\">", self.href, self.rel, self.integrity, self.crossorigin);
    }
}

fn add_style(page: &mut HtmlPage)
{
    page.add_container(Container::new(ContainerType::Header));
    page.add_html(Link::new("https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css", "stylesheet", "sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN", "anonymous"));
}

fn add_contact_view(page: &mut HtmlPage, contacts: Vec<Contact>)
{
    let source_table = contacts.iter().map(|contact|
        { return vec![contact.name.clone(), contact.address.clone(), contact.email.clone(), contact.mobile.clone()]; }
    );

    let table = Table::from(source_table)
        .with_attributes(vec![("class", "table table-striped table-hover")])
        .with_header_row(vec!["Name", "Address", "Email", "Mobile"]);

    page.add_table(table);
}

fn add_contact_form(page: &mut HtmlPage)
{
    page.add_raw(r#"
    <form action="/add_contact" method=post>
    <label for="name">Name:</label><br>
    <input type="text" id="name" name="name"><br>
    <label for="address">Address:</label><br>
    <input type="text" id="address" name="address"><br>
    <label for="email">Email:</label><br>
    <input type="text" id="email" name="email"><br>
    <label for="mobile">Mobile:</label><br>
    <input type="text" id="mobile" name="mobile"><br><br>
    <input type="submit" value="Add contact">
    </form>"#);
}

async fn contacts(State(phonebook): State<Arc<Mutex<PhoneBook>>>) -> axum::response::Html<String>
{
    let mut page: HtmlPage = HtmlPage::new();
    add_style(&mut page);
    let data = phonebook.lock().expect("mutex was poisoned");
    add_contact_view(&mut page, data.get_all());
    add_contact_form(&mut page);
    return axum::response::Html(page.to_html_string());
}

async fn add_contact(State(phonebook): State<Arc<Mutex<PhoneBook>>>, Form(sign_up): Form<Contact>)  -> Redirect {
    let mut data = phonebook.lock().expect("mutex was poisoned");
    let ok_added = data.add(sign_up);
    return Redirect::to("/");
}

pub struct WebFrontend
{}

impl Default for WebFrontend
{
    fn default() -> WebFrontend { WebFrontend {} }
}

impl PhonebookFrontend for WebFrontend
{
    fn run(phonebook: &mut PhoneBook)
    {
        // temporarily copy phonebook, we will copy it back when server ends
        let shared_state = Arc::new(Mutex::new(PhoneBook::new(phonebook.get_all())));
        // build our application with a single route
        let app = Router::new()
            .route("/", get(contacts))
            .route("/add_contact", post(add_contact))
            .with_state(shared_state);
        run(app);
    }
}

#[tokio::main]
async fn run(app: Router)
{
    // run it with hyper on localhost:8000
    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
