/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    if code == "0" {
        return false;
    }
    let chars = code.chars();
    let mut is_even = false;
    let mut sum = 0;
    for c in chars.rev() {
        if let Some(val) = c.to_digit(10) {
            let mut tmp = val;
            if is_even {
                tmp *= 2;
                if tmp > 9 {
                    tmp -= 9;
                }
            }
            is_even = !is_even;
            sum += tmp;
        } else if c != ' ' {
            return false;
        }
    }
    sum % 10 == 0
}
