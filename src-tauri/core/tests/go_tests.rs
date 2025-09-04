#[cfg(feature = "with-go")]
use spacebar_core::call_go_add_two;

#[cfg(feature = "with-go")]
#[test]
fn test_go_add_two() {
    assert_eq!(call_go_add_two(40), 42);
}
