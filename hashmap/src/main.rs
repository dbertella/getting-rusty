use hashmap::my_hash::MyHash;

fn main() {
    let mut bla = MyHash::new();
    bla.insert("bla".to_string(), "Blabla");
    bla.insert("bla1".to_string(), "Blabla");
    bla.insert("bla2".to_string(), "Blabla");
    bla.insert("bla3".to_string(), "Blabla");
    bla.insert("bla4".to_string(), "Blabla");
    bla.insert("bla5".to_string(), "Blabla");
    bla.insert("bla6".to_string(), "Blabla");
    // dbg!(bla.remove("bla".to_string()));

    println!("{}", bla.len());

    for key in bla.keys()
    {
        println!("{}", key);
    }

    // bla.clear();
    // dbg!(bla.get("bla".to_string()));


}
