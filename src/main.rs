use std::collections::HashMap;

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
        let key = (&person.short_name).to_string();
        self.people.insert(key, ContactsRecord::Person(person));
    }

    fn register_company(&mut self, company: Company) {
        let key = (&company.short_name).to_string();
        self.companies.insert(key, ContactsRecord::Company(company));
    }

    fn search_person(&self, short_name: &str) -> Option<&ContactsRecord> {
        self.people.get(short_name)
    }

    fn search_company(&self, short_name: &str) -> Option<&ContactsRecord> {
        self.companies.get(short_name)
    }
}

enum ContactsRecord {
    Person(Person),
    Company(Company),
}

fn print_results(contact: Option<&ContactsRecord>) {
    match contact {
        Some(ContactsRecord::Person(v)) => println!("{}, {}, {}", v.name, v.short_name, v.email),
        Some(ContactsRecord::Company(v)) => println!("{}, {}, {}", v.name, v.short_name, v.email),
        None => println!("No match")
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
    print_results(contacts.search_person("Susan"));
    print_results(contacts.search_company("Bla"));
    print_results(contacts.search_person("Sophy"));
    print_results(contacts.search_company("ADSK"));
}
