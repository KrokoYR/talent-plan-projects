/// The empty private Sealed supertrait cannot be named by downstream crates,
/// so we are guaranteed that implementations of Sealed (and therefore TheTrait)
/// only exist in the current crate. We are free to add methods to TheTrait in a non-breaking release
/// even though that would ordinarily be a breaking change for traits that are not sealed.
/// Also we are free to change the signature of methods that are not publicly documented.
///
/// Note that removing a public method or changing the signature
/// of a public method in a sealed trait are still breaking changes.

/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait TheTrait: private::Sealed {
    // Zero or more methods that the user is allowed to call.
    fn some_pub_function(some_value: usize);

    // Zero or more private methods, not allowed for user to call.
    #[doc(hidden)]
    fn some_hidden_function();
}

// Implement for some types.
impl TheTrait for usize {
    fn some_pub_function(some_value: usize) {
        println!("Some value: {}", some_value);
        ()
    }

    fn some_hidden_function() {
        print!("Hidden command was called");
        ()
    }
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl Sealed for usize {}
}
