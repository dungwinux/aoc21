fn calc_score(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

struct Scope {
    typ: char,
    cor: char,
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut score_cul = vec![];
    for line in inp {
        let mut stack = vec![];
        let mut err = 0;
        for ch in line.chars() {
            match ch {
                '(' => stack.push(Scope { typ: '(', cor: ')' }),
                '[' => stack.push(Scope { typ: '[', cor: ']' }),
                '{' => stack.push(Scope { typ: '{', cor: '}' }),
                '<' => stack.push(Scope { typ: '<', cor: '>' }),
                ')' | ']' | '}' | '>' => {
                    if stack.pop().expect("stack shortage").cor != ch {
                        err += 1
                    }
                }
                _ => (),
            };
        }
        let mut score = 0;
        if err == 0 {
            while !stack.is_empty() {
                score = score * 5 + calc_score(stack.pop().expect("stack run out before calc").cor);
            }
            score_cul.push(score);
        }
    }
    let idx = score_cul.len() / 2;
    score_cul.sort();
    println!("{:?}", score_cul);
    Ok(score_cul[idx])
}
