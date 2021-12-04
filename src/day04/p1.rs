struct BingoRes {
    board: usize,
    idx: usize,
    iscol: bool,
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let number_list = inp[0]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut bingo_list = vec![];
    let mut board = vec![vec![0i64; 5]; 5];

    let mut local_i = 0;
    for line in inp.into_iter().skip(2) {
        if line == "" {
            bingo_list.push(board.clone());
            local_i = 0;
            for row in board.iter_mut() {
                row.fill(0);
            }
        } else {
            let mut local_j = 0;
            for raw_num in line.split_whitespace() {
                let num = raw_num.parse::<i64>()?;
                board[local_i][local_j] = num;
                local_j += 1;
            }
            local_i += 1;
        }
    }
    bingo_list.push(board);
    let original_board = bingo_list.clone();

    for rec_board in bingo_list.iter_mut() {
        let mut col = vec![0i64; 5];
        for i in 0..5 {
            for j in 0..5 {
                col[j] = rec_board[j][i];
            }
            rec_board.push(col.clone());
        }
    }

    let idx_bingo = bingo_list
        .into_iter()
        .map(|board| {
            board
                .into_iter()
                .map(|arr| {
                    let pos = arr
                        .into_iter()
                        .map(|num| number_list.iter().position(|&x| num == x))
                        .filter(|x| x.is_some())
                        .collect::<Vec<Option<usize>>>();
                    if pos.len() != 5 {
                        0
                    } else {
                        pos.into_iter().map(|x| x.unwrap()).max().unwrap()
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    println!("{:?}", idx_bingo);
    let mut ans = BingoRes {
        board: 0,
        idx: 0,
        iscol: false,
    };
    let mut idx_brd = 0;
    let mut global_min = usize::MAX;
    for board in idx_bingo {
        let mut idx_typ = 0;
        for bingo_time in board {
            if global_min > bingo_time {
                global_min = bingo_time;
                ans.board = idx_brd;
                ans.idx = idx_typ % 5;
                ans.iscol = idx_typ >= 5;
            }
            idx_typ += 1;
        }
        idx_brd += 1;
    }

    let bingo_past = number_list
        .into_iter()
        .take(global_min + 1)
        .collect::<Vec<i64>>();
    let rest: i64 = (&original_board[ans.board])
        .into_iter()
        .flatten()
        .filter(|x| !bingo_past.contains(x))
        .sum();
    println!("{:?}", bingo_past);
    Ok(rest * bingo_past.last().unwrap())
}
