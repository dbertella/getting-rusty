use std::collections::HashMap;

#[derive(Clone)]
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
        self.people.insert(person.short_name.clone(), ContactsRecord::Person(person));
    }

    fn register_company(&mut self, company: Company) {
        self.companies.insert(company.short_name.clone(), ContactsRecord::Company(company));
    }

    fn search_person(&self, query: &str) -> QueryResult {
        let result = QueryResult::new(query);
        for contact in self.people.values() {
            if let ContactsRecord::Person(Person { name, short_name, email }) = contact {
                if name.contains(query) ||
                   short_name.contains(query) ||
                   email.contains(query) {
                    result.contacts.push(contact.clone());
                }
            }
        }
        result
    }

    fn search_company(&self, query: &str) -> QueryResult {
        let result = QueryResult::new(query);
        for contact in self.companies.values() {
            if let ContactsRecord::Company(Company { name, short_name, email }) = contact {
                if name.contains(query) ||
                   short_name.contains(query) ||
                   email.contains(query) {
                    result.contacts.push(contact.clone());
                }
            }
        }
        result
    }
}

enum ContactsRecord {
    Person(Person),
    Company(Company),
}

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

fn print_results(result: QueryResult) {
    if result.contacts.is_empty() {
        println!("No result with query {}", result.query)
    } else {
        for res in result.contacts {
            match res {
                ContactsRecord::Person(r) => {
                    println!("Person with query {}: {}, {}, {}", result.query, r.name, r.short_name, r.email)
                }
                ContactsRecord::Company(r) => {
                    println!("Company with query {}: {}, {}, {}", result.query, r.name, r.short_name, r.email)
                }
            }
        }
    }

}

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
    print_results(contacts.search_company("Bla"));
    print_results(contacts.search_company("ADSK"));
    print_results(contacts.search_person("Susan"));
    print_results(contacts.search_person("S"));
}
