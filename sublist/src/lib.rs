#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// if first list is contained within second list
// if second list is contained within the first list
// if both lists are contained within each other or if none of these are true

// a is sublist of b 
//    if by dropping 0 or more elements from the front of B 
//       AND 0 or more elements from the back of B 
//     
//        -> you get a list that's completely equal to A

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.is_empty() || b.windows(a.len()).any(|x| x == a)
}


pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if is_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
