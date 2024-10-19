pub fn brain_fuck(str: &str) {
    let mut arr = [0; 2048];
    let mut ptr = 0;

    let chars: Vec<char> = str.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        match chars[idx] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => arr[ptr] += 1,
            '-' => arr[ptr] -= 1,
            '.' => print!("{}", arr[ptr] as u8 as char),
            '[' if arr[ptr] == 0 => while chars[idx] != ']' { idx += 1 },
            ']' if arr[ptr] != 0 => while chars[idx] != '[' { idx -= 1 },
            _ => {}
        }

        idx += 1;
    }
}
