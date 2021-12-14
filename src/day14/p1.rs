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
                opstack.insert(rule, add);
            }
        } else {
            template = line.to_string();
        }
    }
    for _step in 0..10 {
        let mut str_construct = String::new();
        str_construct.push_str(&template[0..=0]);
        for i in 0..template.len() - 1 {
            str_construct.push_str(opstack[&template[i..=i + 1]]);
            str_construct.push_str(&template[i + 1..=i + 1]);
            // += (*given)[0] + opstack[given] + given[1];
        }
        template = str_construct;
        println!("{:?} {}", template, template.len());
    }
    let mut cat = HashMap::new();
    for c in template.chars() {
        cat.entry(c).and_modify(|x| *x += 1).or_insert(0);
    }
    let mut view = cat.values().collect::<Vec<&i64>>();
    view.sort();
    Ok((**view.last().unwrap() - **view.first().unwrap()) as i64)
}
