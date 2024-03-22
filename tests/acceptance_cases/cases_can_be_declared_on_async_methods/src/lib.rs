#![cfg(test)]
use test_case::{test_case, test_matrix};

#[test_case(100i32 => 100usize)]
#[tokio::test]
async fn works_seamlessly_with_tokio(arg: i32) -> usize {
    arg as usize
}

mod conf {
    use std::sync::OnceLock;
    pub static SOME_SETTING: OnceLock<bool> = OnceLock::new();
}

#[test_matrix([true, false], [1, 2, 3, 5, 8])]
#[tokio::test]
async fn test_matrix_with_tokio(b: bool, arg: usize) {
    conf::SOME_SETTING.set(b).expect("Setting SOME_SETTING");
    assert_eq!(arg, arg);
    ()
}

#[test_case(100i32 => 100usize)]
#[async_std::test]
async fn works_seamlessly_with_async_std(arg: i32) -> usize {
    arg as usize
}
