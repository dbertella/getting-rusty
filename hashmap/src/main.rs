use hashmap::my_hash::MyHash;

fn main() {
    let mut bla = MyHash::new();
    bla.insert("bla".to_string(), "Blabla");
    dbg!(bla.remove("bla".to_string()));

    println!("{}", bla.len());
    // bla.clear();
    // dbg!(bla.get("bla".to_string()));
}
