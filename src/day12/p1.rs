use std::collections::HashMap;

fn is_upper(s: &str) -> bool {
    s.chars().all(|x| x >= 'A' && x <= 'Z')
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut path_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in inp {
        if let Some((from, to)) = line.split_once('-') {
            if !path_map.contains_key(from) {
                path_map.insert(from, vec![]);
            }
            path_map.get_mut(from).expect("Corrupted map").push(to);
            if !path_map.contains_key(to) {
                path_map.insert(to, vec![]);
            }
            path_map.get_mut(to).expect("Corrupted map").push(from);
        }
    }

    println!("{:?}", path_map);
    let mut multi_node: HashMap<&str, bool> = HashMap::new();
    for v in path_map.keys() {
        multi_node.insert(v, is_upper(v));
    }
    println!("{:?}", multi_node);
    let mut visited = vec![];
    let mut stack = vec!["start"];
    let mut count = 0;
    while !stack.is_empty() {
        let idx = stack.pop().unwrap();
        // println!("{:?} {:?} {}", visited, stack, idx);
        if let Some(val) = visited.last() {
            if val == &idx {
                visited.pop();
                continue;
            }
        }
        if multi_node[idx] || !visited.contains(&idx) {
            visited.push(idx);
            if idx == "end" {
                // println!("Sol: {:?}", visited);
                count += 1;
                visited.pop();
            } else {
                stack.push(idx);
                for e in path_map[idx].iter() {
                    if multi_node[e] || !visited.contains(e) {
                        stack.push(e);
                    }
                }
            }
        }
    }
    Ok(count)
}
