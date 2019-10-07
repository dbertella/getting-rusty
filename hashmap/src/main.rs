use hashmap::my_hash::MyHash;

fn main() {
    let mut bla = MyHash::new();
    bla.insert("bla".to_string(), "Blabla").expect("bla");
    bla.insert("bla1".to_string(), "Blabla1").expect("bla");

    bla.insert("bla2".to_string(), "Blabla2").expect("bla");
    bla.insert("bla3".to_string(), "Blabla3").expect("bla");
    bla.insert("bla4".to_string(), "Blabla4").expect("bla");
    bla.insert("bla5".to_string(), "Blabla5").expect("bla");
    // bla.insert("bla6".to_string(), "Blabla6").expect("bla");
    println!("{:?}", bla.calc_index(&"bla5".to_string()));
    dbg!(bla.remove("bla1".to_string()));

    println!("{:?}", bla);
    println!("{:?}", bla.calc_index(&"bla5".to_string()));

    // dbg!(&my_iter.next());
    for (key, value) in bla.pairs() {
        println!("{} {}", key, value);
    }

    // bla.items().test()

    // bla.clear();
    // dbg!(bla.get("bla".to_string()));

}
