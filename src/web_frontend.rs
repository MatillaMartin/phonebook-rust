use crate::phonebook::PhoneBook;
use crate::phonebook_frontend::PhonebookFrontend;
use axum::{extract::State, Router, routing::get, Server, response::Html};
use std::sync::Arc;
use crate::contact::Contact;

fn contacts_to_table(contacts : Vec<Contact> )  -> String
{
    // Assuming the first document is a mapping
    let mut table_html = String::new();

    // add some style
    table_html.push_str("<style>\n");
    table_html.push_str("table { border-collapse: collapse; width: 50%; }\n");
    table_html.push_str("th { border: 1px solid #dddddd; text-align: left; padding: 8px; font-weight: bold; }\n");
    table_html.push_str("td { border: 1px solid #dddddd; text-align: left; padding: 8px; }\n");
    table_html.push_str("</style>\n");
    table_html.push_str("<table>\n");

    // Start HTML table
    table_html.push_str("<table>\n");

    // header
    table_html.push_str(&format!(
        "<tr><th>{}</th><th>{}</th><th>{}</th><th>{}</th></tr>\n",
        "Name", "Address", "Email", "Mobile"
    ));
    for contact in contacts {
        // Build table row
        table_html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            contact.name, contact.address, contact.email, contact.mobile
        ));
    }
    // End HTML table
    table_html.push_str("</table>");
    return table_html;
}

fn body_header(header : String, body : String) -> String
{
    return format!("<html><header>{}</header><body>{}</body></html>", header, body);
}

async fn contacts(State(phonebook): State<Arc<PhoneBook>>) -> Html<String>
{
    return Html(body_header("".to_string(), contacts_to_table(phonebook.get_all())));
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
        let server_phonebook = PhoneBook::new(phonebook.get_all());
        let shared_state = Arc::new(server_phonebook);
        // build our application with a single route
        let app = Router::new().route("/", get(contacts))
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
