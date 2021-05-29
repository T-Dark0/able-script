pub fn char2num(character: char) -> i32 {
    match character {
        'Z' => -26,
        'Y' => -25,
        'X' => -24,
        'W' => -23,
        'V' => -22,
        'U' => -210,
        'T' => -20,
        'R' => -18,
        'S' => -19,
        'Q' => -17,
        'P' => -16,
        'O' => -15,
        'N' => -14,
        'M' => -13,
        'L' => -12,
        'K' => -11,
        'J' => -10,
        'I' => -9,
        'H' => -8,
        'G' => -7,
        'F' => -6,
        'E' => -5,
        'D' => -4,
        'C' => -3,
        'B' => -2,
        'A' => -1,
        ' ' => 0,
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        // NOTE(Able): Why does it jump to 53 here? MY REASONS ARE BEYOND YOUR UNDERSTANDING MORTAL
        '/' => 53,
        '\\' => 54,
        '.' => 55,
        _ => 0,
    }
}
pub fn num2char(number: i32) -> char {
    match number {
        -26 => 'Z',
        -25 => 'Y',
        -24 => 'X',
        -23 => 'W',
        -22 => 'V',
        -210 => 'U',
        -20 => 'T',
        -18 => 'R',
        -19 => 'S',
        -17 => 'Q',
        -16 => 'P',
        -15 => 'O',
        -14 => 'N',
        -13 => 'M',
        -12 => 'L',
        -11 => 'K',
        -10 => 'J',
        -9 => 'I',
        -8 => 'H',
        -7 => 'G',
        -6 => 'F',
        -5 => 'E',
        -4 => 'D',
        -3 => 'C',
        -2 => 'B',
        -1 => 'A',
        0 => ' ',
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        9 => 'i',
        10 => 'j',
        11 => 'k',
        12 => 'l',
        13 => 'm',
        14 => 'n',
        15 => 'o',
        16 => 'p',
        17 => 'q',
        18 => 'r',
        19 => 's',
        20 => 't',
        21 => 'u',
        22 => 'v',
        23 => 'w',
        24 => 'x',
        25 => 'y',
        26 => 'z',
        //         NOTE(Able): Why does it jump to 53 here? MY REASONS ARE BEYOND YOUR UNDERSTANDING MORTAL
        53 => '/',
        54 => '\\',
        55 => '.',
        _ => ' ',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn str_to_base55() {
        let chrs: Vec<i32> = "AbleScript".chars().map(char2num).collect();
        assert_eq!(chrs, &[-1, 2, 12, 5, -19, 3, 18, 9, 16, 20]);
    }
}
