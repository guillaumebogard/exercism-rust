use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut note_hashmap: HashMap<String, usize> = HashMap::new();

    for subnote in note {
        let counter_option = note_hashmap.get_mut(*subnote);

        if counter_option == None {
            note_hashmap.insert((*subnote).to_string(), 1);
        } else {
            *counter_option.unwrap() += 1;
        }
    }

    for word in magazine {
        let counter_option = note_hashmap.get_mut(*word);

        if counter_option == None {
            continue;
        }

        let counter = counter_option.unwrap();

        if *counter > 0 {
            *counter -= 1;
        }
    }

    for (_, value) in note_hashmap {
        if value > 0 {
            return false;
        }
    }
    true
}
