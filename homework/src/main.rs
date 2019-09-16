use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
struct Person {
    name: String,
    short_name: String,
    email: String,
}

impl Person {
    fn new(name: &str, short_name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            short_name: short_name.to_string(),
            email: email.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Company {
    name: String,
    short_name: String,
    email: String,
}

impl Company {
    fn new(name: &str, short_name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            short_name: short_name.to_string(),
            email: email.to_string(),
        }
    }
}

#[derive(Clone)]
struct Contacts {
    people: HashMap<String, ContactsRecord>,
    companies: HashMap<String, ContactsRecord>,
}

impl Contacts {
    fn new() -> Self {
        Self {
            people: HashMap::new(),
            companies: HashMap::new(),
        }
    }

    fn register_person(&mut self, person: Person) {
        self.people
            .insert(person.short_name.clone(), ContactsRecord::Person(person));
    }

    fn register_company(&mut self, company: Company) {
        self.companies
            .insert(company.short_name.clone(), ContactsRecord::Company(company));
    }

    fn search_person(&self, query: &str) -> QueryResult {
        let mut result = QueryResult::new(query);
        for contact in self.people.values() {
            if let ContactsRecord::Person(Person {
                name,
                short_name,
                email,
            }) = contact
            {
                if name.contains(query) || short_name.contains(query) || email.contains(query) {
                    result.contacts.push(contact.clone());
                }
            }
        }
        result
    }

    fn search_company(&self, query: &str) -> QueryResult {
        let mut result = QueryResult::new(query);
        for contact in self.companies.values() {
            if let ContactsRecord::Company(Company {
                name,
                short_name,
                email,
            }) = contact
            {
                if name.contains(query) || short_name.contains(query) || email.contains(query) {
                    result.contacts.push(contact.clone());
                }
            }
        }
        result
    }

    fn search_all(&self, query: &str) -> QueryResult {
        let mut result = QueryResult::new(query);
        for contact in self.companies.values().chain(self.people.values()) {
            match contact {
                ContactsRecord::Company(Company {
                    name,
                    short_name,
                    email,
                }) => {
                    if name.contains(query) || short_name.contains(query) || email.contains(query) {
                        result.contacts.push(contact.clone());
                    }
                }
                ContactsRecord::Person(Person {
                    name,
                    short_name,
                    email,
                }) => {
                    if name.contains(query) || short_name.contains(query) || email.contains(query) {
                        result.contacts.push(contact.clone());
                    }
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug)]
enum ContactsRecord {
    Person(Person),
    Company(Company),
}

#[derive(Debug)]
struct QueryResult {
    query: String,
    contacts: Vec<ContactsRecord>,
}

impl QueryResult {
    fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            contacts: Vec::new(),
        }
    }
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contacts.is_empty() {
            writeln!(f, "No result with query {}", self.query)
        } else {
            let mut status = write!(f, "[");
            if let Err(_) = status {
                return status;
            }
            for res in self.contacts.clone() {
                match res {
                    ContactsRecord::Person(r) => {
                        status = write!(
                            f,
                            "Person with query {}: {}, {}, {}",
                            self.query, r.name, r.short_name, r.email
                        );
                        if let Err(_) = status {
                            return status;
                        }
                    }
                    ContactsRecord::Company(r) => {
                        status = write!(
                            f,
                            "Company with query {}: {}, {}, {}",
                            self.query, r.name, r.short_name, r.email
                        );
                        if let Err(_) = status {
                            return status;
                        }
                    }
                }
                status = write!(f, ", ");
                if let Err(_) = status {
                    return status;
                }
            }
            write!(f, "]")
        }
    }
}

// fn print_results(result: QueryResult) {


// }

fn create_contacts() -> Contacts {
    let mut contacts = Contacts::new();
    contacts.register_person(Person::new("Sophia Wood", "Sophy", "sophia.wood@gmail.com"));
    contacts.register_person(Person::new("Thomas Smith", "Tom", "thomas@smith.com"));
    contacts.register_person(Person::new("Thomas Taylor", "Tommy", "tom@taylor.co.uk"));
    contacts.register_company(Company::new("Twitter", "TWTR", "info@twitter.com"));
    contacts.register_company(Company::new("Autodesk", "ADSK", "help@autodesk.com"));
    contacts
}


fn main() {
    let contacts = create_contacts();
    println!("{}", contacts.search_company("Bla"));
    println!("{}", contacts.search_company("ADSK"));
    println!("{}", contacts.search_person("Susan"));
    println!("{}", contacts.search_all("S"));
}
