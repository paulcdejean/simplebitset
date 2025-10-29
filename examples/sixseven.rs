use simplebitset::BitSet;

pub fn main() {
    let mut example: std::collections::HashSet<u8> = std::collections::HashSet::new();
    dbg!(&example);
    example.insert(5u8);
    example.insert(6u8);
    example.insert(7u8);
    example.insert(8u8);
    example.insert(9u8);
    dbg!(example);

    let mut example: BitSet = BitSet::new();
    example.insert(6);
    example.insert(7);
    println!("6 = {}", example.contains(6));
    println!("7 = {}", example.contains(7));
    println!("8 = {}", example.contains(8));
    println!("9 = {}", example.contains(9));
    for n in example {
        dbg!(n);
    }
    dbg!(example);
}
