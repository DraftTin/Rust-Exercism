#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    } else if first_list.is_empty() {
        return Comparison::Sublist;
    } else if second_list.is_empty() {
        return Comparison::Superlist;
    }
    let first_list: Vec<String> = first_list.iter().map(|x| x.to_string()).collect();
    let second_list: Vec<String> = second_list.iter().map(|x| x.to_string()).collect();
    let new_first_list = format!("|{}|", first_list.join("|"));
    let new_second_list = format!("|{}|", second_list.join("|"));
    if new_first_list == new_second_list {
        Comparison::Equal
    } else if new_first_list.contains(&new_second_list) {
        Comparison::Superlist
    } else if new_second_list.contains(&new_first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
