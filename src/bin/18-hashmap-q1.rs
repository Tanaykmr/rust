use std::collections::HashMap;

fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in pairs {
        hm.insert(key, value);
    }

    return hm;
}

fn main() {
    let pairs: Vec<(String, i32)> =
        vec![(String::from("tanay"), 21), (String::from("Harkirat"), 25)];

    let grouped_pairs = group_values_by_key(pairs);
    for (key, value) in &grouped_pairs {
        println!("{} : {}", key, value);
    }
    println!("-------");

    println!("{:?}", grouped_pairs);
}

// The output of the above code differs when I run it multiple times, sometimes tanay ends up on top, sometimes HarkiratThe difference in the output order is due to how Rust’s HashMap stores and retrieves elements. Rust’s HashMap doesn’t guarantee a specific order for the elements when iterating. This unordered nature is typical of hash-based data structures because they rely on hashing functions, which distribute keys across various “buckets” based on the hash values.

// Why the Order Can Vary

// 	1.	Hashing and Buckets: When you insert elements into a HashMap, each key is hashed to determine where it should be stored internally. The order in which elements are stored depends on their hash values, not on the insertion order.
// 	2.	Iteration Order in HashMap: When you iterate over a HashMap, it returns the elements in the order they’re stored internally, which may vary between runs or even between different platforms. Rust’s HashMap does not maintain insertion order, so you’ll see the elements in an arbitrary order that can change between executions.
// 	3.	Random Hash Seed: Rust's HashMap uses a random seed for its hashing function to prevent certain types of attacks. This can lead to different bucket placements for the same data on each run, especially if the program is recompiled or run on a different machine.

// If You Need a Consistent Order

// If you need to maintain a specific order (like insertion order or sorted order), you could use different data structures:

// 	1. BTreeMap: A tree-based map that maintains elements in sorted order by key.
//     2. LinkedHashMap (from the linked-hash-map crate): Maintains elements in insertion order.