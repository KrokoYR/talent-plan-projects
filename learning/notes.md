#### Duplicating derived traits as bounds on Bad is unnecessary and a backwards-compatibiliity hazard.

1) Initial code
```rust-lang
#![allow(unused)]
fn main() {
	// Prefer this:
	#[derive(Clone, Debug, PartialEq)]
	struct Good<T> { /* ... */ }

	// Over this:
	#[derive(Clone, Debug, PartialEq)]
	struct Bad<T: Clone + Debug + PartialEq> { /* ... */ }
}
```

2) After changes:
```rust-lang
#![allow(unused)]
fn main() {
	// Non-breaking change:
	#[derive(Clone, Debug, PartialEq, PartialOrd)]
	struct Good<T> { /* ... */ }

	// Breaking change:
	#[derive(Clone, Debug, PartialEq, PartialOrd)]
	struct Bad<T: Clone + Debug + PartialEq + PartialOrd> { /* ... */ }
}
```

#### The following traits should never be used in bounds on data structures:

- Clone
- PartialEq
- PartialOrd
- Debug
- Display
- Default
- Error
- Serialize
- Deserialize
- DeserializeOwned

#### Be careful because public dependencies can sneak in at unexpected places.

```rust-lang
#![allow(unused)]
fn main() {
	pub struct Error {
		private: ErrorImpl,
	}

	enum ErrorImpl {
		Io(io::Error),
		// Should be okay even if other_crate isn't
		// stable, because ErrorImpl is private.
		Dep(other_crate::Error),
	}

	// Oh no! This puts other_crate into the public API
	// of the current crate.
	impl From<other_crate::Error> for Error {
		fn from(err: other_crate::Error) -> Self {
			Error { private: ErrorImpl::Dep(err) }
		}
	}
}
```