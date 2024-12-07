// https://stackoverflow.com/a/74763922
pub fn borrow_mut_elementwise<'a, T>(v:&'a mut Vec<T>) -> Vec<&'a mut T> {
    let mut result:Vec<&mut T> = Vec::new();
    let mut current: &mut [T];
    let mut rest = &mut v[..];
    while rest.len() > 0 {
        (current, rest) = rest.split_at_mut(1);
        result.push(&mut current[0]);
    }
    result
}