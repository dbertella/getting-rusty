use hashmap::my_hash::MyHash;

fn main() {
    let mut bla = MyHash::new();
    bla.insert("bla".to_string(), "Blabla");
    dbg!(bla.get("bla".to_string()));
}
