#[cfg(test)]
use demonstrate::demonstrate;

#[cfg(test)]
demonstrate! {
    describe "In a new context" {
        use app_basics::*;
        use hamcrest2::prelude::*;
    
        it "works" {
            assert_that!(sum(2, 2), eq(4));
        }
        it "fails" {
            assert_that!(sum(2, 2), lt(5));
        }
    }
}
