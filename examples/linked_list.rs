use playground_rs::utils::ghost_cell::GhostToken;
use playground_rs::data_structure::linked_list::LinkedList;

fn main() {
    GhostToken::scope(|mut token| {
        let mut list = LinkedList::new();
        let times = 5;

        for i in 0..times {
            list.push_front(i, &mut token);
        }

        for i in 0..times {
            let v = list.pop_back(&mut token).unwrap();
            assert_eq!(v, i);
        }

        list.push_front(2, &mut token);
    })
}
