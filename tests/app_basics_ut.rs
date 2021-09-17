#[cfg(test)]
use demonstrate::demonstrate;

#[cfg(test)]
demonstrate! {
    describe "In a new context" {
        use app_basics::*;
        use all_asserts::*;
    
        it "works" {
            assert_eq!(4, sum(2, 2));
        }
        it "fails" {
            assert_gt!(5, sum(2, 2));
        }
    }
}
