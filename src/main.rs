use std::collections::HashSet;

fn main() {
    let instructions = include_str!("../doom.asm")
        .lines()
        .filter_map(|l| l.split_whitespace().nth(2));

    let mnemonics = instructions.filter_map(|i| i.split_whitespace().next());

    let unique: HashSet<_> = mnemonics.collect();

    println!("{:#?}", unique);
    println!("Unique mnemonics {:#?}", unique.len())
}
