pub trait Func {
    type Parameters;
    type Error;
    type Output;
    type TempState;

    fn may_fail(parameters: &Self::Parameters) -> Result<Self::TempState, Self::Error>;

    fn commit(parameters: Self::Parameters, tmp: Self::TempState) -> Self::Output;

    fn execute(parameters: Self::Parameters)-> Result<Self::Output, Self::Error> {
        let tmp = Self::may_fail(&parameters)?;
        let output = Self::commit(parameters, tmp);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Not a realistic use case. For a function which would always succeed, we would use a
    /// infalliable built-in Rust function. Still a nice test. More so for usability however, how
    /// much bulk does it take to implement the simplest function?
    #[test]
    fn trivial_succeeding_function() {

        /// Implements `Func` and always succeeds with `42`.
        struct Answer;

        impl Func for Answer {
            type Parameters = &'static str;
            type Error = ();
            type Output = i32;
            type TempState = ();
        
            fn may_fail(_question: &&'static str) -> Result<(), Self::Error> {
                Ok(())
            }
        
            fn commit(_question: &'static str, _tmp: ()) -> i32 {
                42
            }
        }

        let answer = Answer::execute("The life, the Universe and all the rest");
        assert_eq!(Ok(42), answer)
    }
}
