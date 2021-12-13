use std::collections::HashMap;

fn is_upper(s: &str) -> bool {
    s.chars().all(|x| x >= 'A' && x <= 'Z')
}

fn is_within_limit(visited: &Vec<&str>, idx: &str, limit: usize) -> bool {
    if idx == "start" || idx == "end" {
        !visited.contains(&idx)
    } else {
        visited.iter().filter(|&x| *x == idx).count() <= limit
    }
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
    let mut stack = vec![("start", None)];
    let mut count = 0;
    while !stack.is_empty() {
        // println!("{:?} {:?} {:?}", visited, stack, stack.last().unwrap());
        let (idx, used_perk) = stack.pop().unwrap();
        if let Some(val) = visited.last() {
            if val == &idx {
                // Removing frame
                visited.pop();
                continue;
            }
        }
        // Base frame
        visited.push(idx);
        if idx == "end" {
            // println!("Sol: {:?}", visited);
            count += 1;
            visited.pop();
        } else {
            stack.push((idx, used_perk));
            for e in path_map[idx].iter() {
                if multi_node[e] || is_within_limit(&visited, e, 0) {
                    stack.push((e, used_perk));
                } else if used_perk == None && is_within_limit(&visited, e, 1) {
                    stack.push((e, Some(e)));
                }
            }
        }
    }
    Ok(count)
}
