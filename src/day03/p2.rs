pub fn proc(mut val_list: Vec<i64>, len: usize, ascd: bool) -> i64 {
    let mut base = 0;
    for i in 0..len {
        base |= 1 << (len - i - 1);
        println!("{:?} {}", val_list, base);
        if val_list.len() < 2 {
            break;
        }
        let loc = val_list
            .iter()
            .position(|x| x >= &base)
            .expect("Data corrupt");
        // XNOR
        if !((2 * loc <= val_list.len()) ^ ascd) {
            println!("[{:?}]", 1);
            val_list = val_list.split_off(loc);
        } else {
            println!("[{:?}]", 0);
            let off = 1 << (len - i - 1);
            base &= -(off + 1);
            val_list.truncate(loc);
        }
    }
    val_list[0]
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let len = inp[0].len();
    let mut val_list = inp
        .into_iter()
        .map(|x| i64::from_str_radix(x, 2).unwrap())
        .collect::<Vec<i64>>();
    val_list.sort();

    let gamma = proc(val_list.clone(), len, true);
    let epsilon = proc(val_list, len, false);
    Ok(gamma * epsilon)
}
