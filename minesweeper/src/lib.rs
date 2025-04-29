pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let h = minefield.len();
    let w = minefield.first().map_or(0, |row| row.len());

    let mut result = Vec::with_capacity(h);

    for (i, &row) in minefield.iter().enumerate() {
        let bytes = row.as_bytes();
        let mut new_row = String::with_capacity(w);
        for (j, &cell) in bytes.iter().enumerate() {
            if cell == b'*' {
                new_row.push('*');
            } else {
                let mut count = 0;
                for di in [-1isize, 0, 1] {
                    for dj in [-1isize, 0, 1] {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && nj >= 0
                            && ni < h as isize
                            && nj < w as isize
                            && minefield[ni as usize].as_bytes()[nj as usize] == b'*'
                        {
                            count += 1;
                        }
                    }
                }
                if count == 0 {
                    new_row.push(' ');
                } else {
                    new_row.push((count + b'0') as char);
                }
            }
        }
        result.push(new_row);
    }
    result
}
