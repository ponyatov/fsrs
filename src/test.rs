#![cfg(test)]

#[test]
fn test_check_me() {
    assert!(true) //false)
}

mod libre {
    #[test]
    fn test_yes() {
        assert!(fsrs::yes())
    }
}
