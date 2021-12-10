fn calc_penalty(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

struct Scope {
    typ: char,
    cor: char,
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut err = 0;
    for line in inp {
        let mut stack = vec![];
        for ch in line.chars() {
            match ch {
                '(' => stack.push(Scope { typ: '(', cor: ')' }),
                '[' => stack.push(Scope { typ: '[', cor: ']' }),
                '{' => stack.push(Scope { typ: '{', cor: '}' }),
                '<' => stack.push(Scope { typ: '<', cor: '>' }),
                ')' | ']' | '}' | '>' => {
                    if stack.pop().expect("stack shortage").cor != ch {
                        err += calc_penalty(ch)
                    }
                }
                _ => (),
            };
        }
    }
    Ok(err)
}
