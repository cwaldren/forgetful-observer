use forgetful::Observer;
use std::collections::HashMap;

fn find_leaf<'a>(
    map: &HashMap<&str, &'a str>,
    node: &'a str,
    seen: &Observer<'a, str>,
) -> Result<&'a str, &'static str> {
    let _guard = seen.notice(node).ok_or("cycle detected!")?;
    match map.get(node) {
        Some(next) => find_leaf(map, next, seen),
        None => Ok(node),
    }
}

fn main() {
    let graph = HashMap::from([("A", "B"), ("B", "C"), ("C", "A"), ("D", "E")]);

    let seen = Observer::new();

    // Should print: "error: cycle detected!""
    match find_leaf(&graph, "A", &seen) {
        Ok(node) => println!("found {}!", node),
        Err(err) => println!("error: {}", err),
    }

    // Should print: "found E!"
    match find_leaf(&graph, "D", &seen) {
        Ok(node) => println!("found {}!", node),
        Err(err) => println!("error: {}", err),
    }
}
