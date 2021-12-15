use std::collections::HashMap;

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut opmode = false;
    let mut opstack = HashMap::new();
    let mut template = String::new();
    for line in inp {
        if line == "" {
            opmode = true;
        } else if opmode {
            if let Some((rule, add)) = line.split_once(" -> ") {
                opstack.insert(
                    (rule.chars().nth(0).unwrap(), rule.chars().nth(1).unwrap()),
                    add.chars().nth(0).unwrap(),
                );
            }
        } else {
            template = line.to_string();
        }
    }
    let mut proc_stack: HashMap<(char, char), i64> = HashMap::new();
    // Initialize proc_stack
    for i in 0..template.len() - 1 {
        let given = &template[i..=i + 1];
        let idx = (given.chars().nth(0).unwrap(), given.chars().nth(1).unwrap());
        proc_stack.entry(idx).and_modify(|x| *x += 1).or_insert(1);
    }
    for _step in 0..40 {
        let mut next = HashMap::new();
        for e in proc_stack {
            let new_item = opstack[&e.0];
            let lt = (e.0 .0, new_item);
            let rt = (new_item, e.0 .1);
            next.entry(lt).and_modify(|x| *x += e.1).or_insert(e.1);
            next.entry(rt).and_modify(|x| *x += e.1).or_insert(e.1);
        }
        proc_stack = next;
    }
    // Character collector (each character is duplicated once)
    let mut cat = HashMap::new();
    for e in proc_stack {
        cat.entry(e.0 .0).and_modify(|x| *x += e.1).or_insert(e.1);
        cat.entry(e.0 .1).and_modify(|x| *x += e.1).or_insert(e.1);
    }
    cat.entry(template.chars().last().unwrap())
        .and_modify(|x| *x += 1)
        .or_insert(1);
    cat.entry(template.chars().nth(0).unwrap())
        .and_modify(|x| *x += 1)
        .or_insert(1);
    let mut view = cat.values().collect::<Vec<&i64>>();
    view.sort();
    println!("{:?}", view);
    let res_double = **view.last().unwrap() - **view.first().unwrap();
    Ok(res_double / 2 as i64)
}
