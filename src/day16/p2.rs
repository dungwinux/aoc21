#[derive(Debug)]
struct Packet {
    data: i64,
    sub: Vec<Box<Packet>>,
    version: u8,
    type_id: u8,
}

fn map_hex_to_u8(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - '0' as u8,
        'A'..='F' => c as u8 - 'A' as u8 + 10,
        _ => u8::MAX,
    }
}

fn vec_to_u64(v: Vec<bool>) -> u64 {
    let mut ans = 0u64;
    for b in v.iter() {
        ans = (ans << 1) | (*b as u64);
    }
    ans
}

fn split_on<T: std::fmt::Debug + Clone>(v: &mut Vec<T>, pos: usize) -> Vec<T> {
    let mut v_next = v.split_off(pos);
    std::mem::swap(&mut v_next, v);
    // println!("Reserved: {:?}\nReturn: {:?}", v, v_next);
    v_next
}

fn packet_parsing(v: &mut Vec<bool>) -> Packet {
    let version = vec_to_u64(split_on(v, 3)) as u8;
    let type_id = vec_to_u64(split_on(v, 3)) as u8;
    // println!("{:?} {:?}", version, type_id);

    match type_id {
        4 => {
            let mut lit_data = vec![];
            loop {
                let mut chunk = split_on(v, 5);
                let is_last_chunk = !chunk[0];
                lit_data.extend(chunk.split_off(1));
                if is_last_chunk {
                    break;
                }
            }

            Packet {
                version: version,
                type_id: type_id,
                data: vec_to_u64(lit_data) as i64,
                sub: vec![],
            }
        }
        _ => {
            let length_type = v[0];
            let length_data = split_on(v, 1 + if length_type { 11 } else { 15 }).split_off(1);
            let packet_length = vec_to_u64(length_data) as usize;
            let mut subpac = vec![];
            if length_type {
                // == 11
                for _ in 0..packet_length {
                    subpac.push(Box::new(packet_parsing(v)));
                }
            } else {
                // == 15
                let mut subpac_area = split_on(v, packet_length);
                while !subpac_area.is_empty() {
                    subpac.push(Box::new(packet_parsing(&mut subpac_area)));
                }
            }
            Packet {
                version: version,
                type_id: type_id,
                data: 0,
                sub: subpac,
            }
        }
    }
}

fn calc(p: &Packet) -> i64 {
    match p.type_id {
        0 => p.sub.iter().map(|x| calc(&*x)).sum(),
        1 => p.sub.iter().map(|x| calc(&*x)).product(),
        2 => p.sub.iter().map(|x| calc(&*x)).min().unwrap(),
        3 => p.sub.iter().map(|x| calc(&*x)).max().unwrap(),
        4 => p.data,
        5 => (calc(&p.sub[0]) > calc(&p.sub[1])) as i64,
        6 => (calc(&p.sub[0]) < calc(&p.sub[1])) as i64,
        7 => (calc(&p.sub[0]) == calc(&p.sub[1])) as i64,
        _ => 0,
    }
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut latest = 0;
    for data in inp.into_iter() {
        let mut bitdata = vec![];
        for c in data.chars() {
            let mut half_byte = vec![];
            let mut val = map_hex_to_u8(c);
            if val == u8::MAX {
                panic!();
            }
            for _ in 0..4 {
                half_byte.push((val & 1) != 0);
                val >>= 1;
            }
            bitdata.extend(half_byte.into_iter().rev());
        }
        println!("{:?}", data);
        let parsed_data = packet_parsing(&mut bitdata);
        latest = calc(&parsed_data);
        // println!("{:?}", latest);
    }

    Ok(latest)
}
