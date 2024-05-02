# Strong exception safe functions

An trait for functions offering strong exception safety guarantees explicitly and utility for executing them one after another in an exception safe manner.

## Motivation

While Rust does not have exceptions, as far as I can tell the terminology of exception safety (https://en.wikipedia.org/wiki/Exception_safety) is still used. It is of course entirely possible to write functions offering strong exception safety in Rust, following this rough pattern:

```rust
fn strong(&mut world) -> Result<(), Error>{
    // May fail, but only changes temporary state
    let tmp = may_fail()?;

    // Commit. The second part changes application state, but can never fail
    world.change(tmp);
    Ok(())
}
```

A problem arises, though if we want to execute two exception safe functions one after each other in an exception safe way.

```rust
// If this line fails, all is good. We did not change application state in case of an error, because
// `strong` is exception safe.
strong(&mut world)?;
// Oh no, if this line fails, we already changed `world` in the first line. The fact that `strong2`
// is execption safe does not help much.
strong2(&mut world)?;
```

Common occurrences of this problem are calling a function in a loop, or calling a list of handlers in an observer pattern. By making the `may fail` and `commit` part of a function explict in this trait you gain utility for chaining them together in manner which maintains strong exception safety.

```rust
struct Strong<'a> {
    arg: &'a mut Arg
}
impl<'a> Invocation for Strong<'a> {
    type Error = Error;
    type Output = ();
    type IntermediateState = Tmp;
    fn may_fail(&self) -> Result<Tmp, Error> {
        may_fail()
    }
    fn commit(self, tmp: Tmp) -> () {
        self.world.change(tmp)
    }
}

// ...snip...

(Strong { arg: a}, Strong { arg: b}).execute();
```