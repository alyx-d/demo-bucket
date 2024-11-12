pub mod q1;
pub mod q1290;
pub mod q2058;
pub mod q2181;
pub mod q3222;
pub mod q3255;
pub mod q3285;
pub mod q540;

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_str_replace() {}
}
