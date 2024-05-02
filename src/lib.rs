/// An instance is associated with a specific invocation of a function offering storng execption
/// safety guarantees. Implmenters of this function are encouraged to hold the arguments of the
/// function invocation as members.
pub trait Invocation: Sized {
    type Error;
    type Output;
    type IntermediateState;

    fn may_fail(&self) -> Result<Self::IntermediateState, Self::Error>;

    fn commit(self, tmp: Self::IntermediateState) -> Self::Output;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let tmp = Self::may_fail(&self)?;
        let output = Self::commit(self, tmp);
        Ok(output)
    }
}

/// Use this to chain two invocation, while maintaining strong exception safety guarantee. This
/// only works if the error types of both functions are identical.
impl<F1, F2> Invocation for (F1, F2)
where
    F1: Invocation,
    F2: Invocation<Error = F1::Error>,
{
    type Error = F1::Error;
    type Output = (F1::Output, F2::Output);
    type IntermediateState = (F1::IntermediateState, F2::IntermediateState);

    fn may_fail(&self) -> Result<Self::IntermediateState, Self::Error> {
        Ok((self.0.may_fail()?, self.1.may_fail()?))
    }

    fn commit(self, tmp: Self::IntermediateState) -> Self::Output {
        (self.0.commit(tmp.0), self.1.commit(tmp.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Implements `Func` and always succeeds with `42`.
    struct Constant;
    struct DummyState;

    impl Invocation for Constant {
        type Error = ();
        type Output = i32;
        type IntermediateState = DummyState;

        fn may_fail(&self) -> Result<DummyState, Self::Error> {
            Ok(DummyState)
        }

        fn commit(self, _tmp: DummyState) -> i32 {
            42
        }
    }

    /// Passes through argument unchanged. Move semantics.
    struct Identity<A>(A);
    impl<A> Identity<A> {
        pub fn new(arg: A) -> Self {
            Identity(arg)
        }
    }

    impl<A> Invocation for Identity<A> {
        type Error = ();
        type Output = A;
        type IntermediateState = ();

        fn may_fail(&self) -> Result<(), ()> {
            Ok(())
        }

        fn commit(self, _: ()) -> A {
            self.0
        }
    }

    /// Not a realistic use case. For a function which would always succeed, we would use a
    /// infalliable built-in Rust function. Still a nice test. More so for usability however, how
    /// much bulk does it take to implement the simplest function?
    #[test]
    fn trivial_succeeding_function() {
        let invocation = Constant;
        let answer = invocation.execute();
        assert_eq!(Ok(42), answer)
    }

    /// Execute two `Func`s both in a way so the combined operation is still exception safe
    #[test]
    fn combine_two_invocations() {
        let first_invocation = Constant;
        let second_invocation = Constant;
        // Both of these operations may fail, but do not change application state.
        let (first_result, second_result) =
            (first_invocation, second_invocation).execute().unwrap();

        assert_eq!((42, 42), (first_result, second_result))
    }

    #[test]
    fn identity_local_argument() {
        // Given an argument with a non-static lifetime
        let local_argument = "Hello, World!".to_owned();

        let invocation = Identity::new(local_argument);
        let result = invocation.execute().unwrap();

        assert_eq!("Hello, World!", result)
    }
}
