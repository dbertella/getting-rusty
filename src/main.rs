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

    fn search_person(&self, query: &str) -> Vec<&ContactsRecord> {
        let mut people_matched = Vec::new();
        for (_, v) in self.people.iter() {
            let string_contact = match v {
                ContactsRecord::Person(v) => format!("{}, {}, {}", v.name, v.short_name, v.email),
                _ => "".to_string(),
            };
            if string_contact.contains(query) {
                people_matched.push(v);
            }
        }
        people_matched
    }

    fn search_company(&self, query: &str) -> Vec<&ContactsRecord> {
        let mut companies_matched = Vec::new();
        for (_, v) in self.companies.iter() {
            let string_contact = match v {
                ContactsRecord::Company(v) => format!("{}, {}, {}", v.name, v.short_name, v.email),
                _ => "".to_string(),
            };
            if string_contact.contains(query) {
                companies_matched.push(v);
            }
        }
        companies_matched
    }
}

enum ContactsRecord {
    Person(Person),
    Company(Company),
}

fn print_results(results: &Vec<&ContactsRecord>) {
    if results.len() == 0 {
        println!("No result")
    } else {
        for res in results {
            match res {
                ContactsRecord::Person(res) => {
                    println!("Person: {}, {}, {}", res.name, res.short_name, res.email)
                }
                ContactsRecord::Company(res) => {
                    println!("Company: {}, {}, {}", res.name, res.short_name, res.email)
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
    print_results(&contacts.search_company("Bla"));
    print_results(&contacts.search_company("ADSK"));
    print_results(&contacts.search_person("Susan"));
    print_results(&contacts.search_person("S"));
}
