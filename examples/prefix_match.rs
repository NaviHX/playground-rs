use playground_rs::data_structure::trie::ACAutomata;

fn main() {
    let mut root = ACAutomata::new_boxed_root();
    let dict = ["abc", "aaaaa", "bcdef"];
    let sentence = "aabcdabc";
    for (i, word) in dict.iter().enumerate() {
        root.insert(word.chars(), i);
    }

    // For tries built by inserting words one by one, they have walk info only on their inserting
    // path and previously inserted words. e.g. you cannot jump from "abc" to "bcd ef" when "d" fails.
    let walker = root.walk(sentence.chars());
    let mut count = 0;
    for node in walker {
        count += 1;
        if std::ptr::eq(node, &*root) {
            break;
        }
    }
    println!("Count = {count}");

    // `TrieImpl::transform` helps you re-construct the whole trie, with all walk info needed. To
    // avoid redundant walk info building, you can build the new trie from a trie with no walk
    // info.
    //
    // ```rust
    // let root = Trie::new_boxed_root();
    // ...
    // let root: Box<ACAutomata<usize>> = root.transform();
    // ```
    let root: Box<ACAutomata<usize>> = root.transform();
    let walker = root.walk(sentence.chars());
    let mut count = 0;
    for node in walker {
        count += 1;
        if std::ptr::eq(node, &*root) {
            break;
        }
    }
    println!("Count = {count}");
}
