use playground_rs::data_structure::trie::ACAutomata;

fn main() {
    let mut root = ACAutomata::new_boxed_root();
    let dict = ["Hello", "World", "Rustacean"];
    let sentence = "Hi World! Rustaceans!";

    for (i, word) in dict.iter().enumerate() {
        root.insert(word.chars(), i);
    }

    let walker = root.walk(sentence.chars());
    for node in walker {
        if let Some(id) = node.attached_info {
            println!("Found {id}th word '{}' in '{sentence}'",dict[id]);
        }
    }
}
