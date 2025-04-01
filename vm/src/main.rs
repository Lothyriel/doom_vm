use indexmap::IndexSet;

fn main() {
    let instructions = include_str!("../doom.asm")
        .lines()
        .filter_map(|l| l.split_whitespace().nth(2));

    let mnemonics = instructions.filter_map(|i| i.split_whitespace().next());

    let unique: IndexSet<_> = mnemonics.collect();
    let first: Vec<_> = unique.iter().take(50).collect();

    println!("{:#?}", first);
    println!("Unique mnemonics {:#?}", unique.len())
}
