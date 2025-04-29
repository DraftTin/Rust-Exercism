pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let lyrics = |x| {
        match x {
        1 => {
            "One green bottle hanging on the wall,\nOne green bottle hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be no green bottles hanging on the wall.\n"
        }
        2 => {
            "Two green bottles hanging on the wall,\nTwo green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be one green bottle hanging on the wall.\n"
        }
        3 => {
            "Three green bottles hanging on the wall,\nThree green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be two green bottles hanging on the wall.\n"
        }
        4 => {
            "Four green bottles hanging on the wall,\nFour green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be three green bottles hanging on the wall.\n"
        }
        5 => {
            "Five green bottles hanging on the wall,\nFive green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be four green bottles hanging on the wall.\n"
        }
        6 => {
            "Six green bottles hanging on the wall,\nSix green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be five green bottles hanging on the wall.\n"
        }
        7 => {
            "Seven green bottles hanging on the wall,\nSeven green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be six green bottles hanging on the wall.\n"
        }
        8 => {
            "Eight green bottles hanging on the wall,\nEight green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be seven green bottles hanging on the wall.\n"
        }
        9 => {
            "Nine green bottles hanging on the wall,\nNine green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be eight green bottles hanging on the wall.\n"
        }
        10 => {
            "Ten green bottles hanging on the wall,\nTen green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be nine green bottles hanging on the wall.\n"
        }
        _ => "",
    }
    };
    let tmp: String = (start_bottles - take_down + 1..=start_bottles)
        .rev()
        .map(|x| lyrics(x).to_string() + "\n")
        .collect();
    tmp
}
