use backend::add;

pub fn wrapped_add(left: usize, right: usize) -> usize {
    add(left, right)
}