/**

Derives the [`PanicFmt`](trait@crate::PanicFmt) trait.

This requires the `"derive"` feature, disabled by default.

This generates a generic [`PanicFmt`](trait@crate::PanicFmt) impl,
as well as one or more inherent `to_panicvals` method definitions 
[as described in the trait docs](trait@crate::PanicFmt#implementor).

You can also use [`impl_panicfmt`] as an alternative that requires less time to compile 
from scratch, but requires repeating the type definition.

[Jump straight to examples](#examples)

# Limitations


### Type parameters

Types with type parameters can't be generically formatted,
to work around this you can use either or both of these attributes:
- `#[pfmt(ignore(T))]`:
if the type parameter(s) are only used in marker types (eg: `PhantomData`).
- `#[pfmt(impl Foo<Bar, Baz>)]`:
to implement panic formatting with concrete type arguments
(this attribute can be used multiple times to add impls).

This limitation is caused by:
- the lack of trait bound support in stable const fns.
- the need to [have a concrete type argument](#concrete-pv-count)

[example below](#type-parameter-example)

### Const parameters

Const parameters must not affect the value of the `PanicFmt::PV_COUNT`  of this type,
since the const parameter [must be replaceable with a concrete value](#concrete-pv-count).
<br>Note that arrays have a `PV_COUNT` of `1` for all lengths.

<a id = "concrete-pv-count"></a>
### Concrete `Self` type for `PanicFmt::PV_COUNT`

The `to_panicvals` method that this macro generates roughly returns a
```text
[PanicVal<'_>; <Self as PanicFmt>::PV_COUNT]
```

Because of limitations in stable const generics, 
the generic arguments of `Self` in the above code must be replaced with concrete arguments,
requiring:
- Lifetime arguments to be replaced with `'_`
- Type arguments to be replaced with concrete types 
(usually `()` or the concrete types used in [`#[pfmt(impl ....)]`](#pfmt-impl-attr) attributes)
- Const arguments to be replaced with concrete values (usually the default value for the type)


# Attributes

### Container attributes

Attributes used above the type definition.

### `#[pfmt(crate = foo::bar)]`

Replaces the path to `const_panic` with `foo::bar`

[example](#crate-example)

### `#[pfmt(debug_print)]`: <br>

For diagnostics, causes the derive macro to panic with the code generated by it.

<a id = "pfmt-ignored-attr"></a>
##### `#[pfmt(ignored(T, C))]`

Accepts the names of type and const parameters,
replacing the generic arguments in [`here`](#concrete-pv-count) with a concrete value.

For type parameters, this replaces the type parameter with `()` unless overriden,
and also tells the derive not to require `T: PanicFmt` in
the `PanicFmt` implementation for the deriving type
(since the type parameter is not formatted).

Const parameters are ignored by default,
replacing them with the default value for that type [^1]

The concrete value for each generic parameter can be overriden with `T = value`

examples:
- `#[pfmt(ignored(T))]`
- `#[pfmt(ignored(T = u16))]`
- `#[pfmt(ignored(T = u32, C))]`
- `#[pfmt(ignored(T, C = 100))]`
- `#[pfmt(ignored(U = str, A = false))]`

([more conplete example](#phantom-type-parameter-example))

[^1]: a private trait is used to get the default value for const parameters.

<a id = "pfmt-impl-attr"></a>
##### `#[pfmt(impl Foo<Bar, BAZ>)]`

Tells the derive to generate an inherent `to_panicvals` method for the type in the attribute
(it must be the deriving type with concrete enough generic arguments).

examples:
- `#[pfmt(impl Foo<u32, 10>)]`
- `#[pfmt(impl<T> Foo<T, 'A'>)]`:
this also requires a [`#[pfmt(ignored(T))]`](#pfmt-ignored-attr) attribute

([more conplete example](#type-parameter-example))

# Examples

### Basic struct

```rust
use const_panic::{ArrayString, FmtArg, PanicFmt};

assert_eq!(
    const_panic::concat_!(Foo { x: 3, y: &[3, 5, 8] }),
    "Foo { x: 3, y: [3, 5, 8] }",
);

#[derive(PanicFmt)]
struct Foo<'a> {
    x: u32,
    y: &'a [u8],
}
```

### Basic enum

```rust
use const_panic::{ArrayString, FmtArg, PanicFmt};

assert_eq!(const_panic::concat_!(Foo::Bar), "Bar");

assert_eq!(
    const_panic::concat_!(Foo::Baz("hello", true)),
    "Baz(\"hello\", true)",
);

#[derive(PanicFmt)]
enum Foo {
    Bar,
    Baz(&'static str, bool),
}

```

<a id = "type-parameter-example"></a>
### Type parameters

This example demonstrates support for types with type parameters.

```rust
use const_panic::{ArrayString, FmtArg, PanicFmt};

use std::marker::PhantomData;

{
    const WITH_INT: Foo<&str, u8> = Foo {
        value: 100u8,
        _marker: PhantomData,
    };
    assert_eq!(
        const_panic::concat_!(WITH_INT),
        "Foo { value: 100, _marker: PhantomData }",
    );
}
{
    const WITH_STR: Foo<bool, &str> = Foo {
        value: "hello",
        _marker: PhantomData,
    };
    assert_eq!(
        const_panic::concat_!(WITH_STR),
        r#"Foo { value: "hello", _marker: PhantomData }"#,
    );
}

#[derive(Debug, PanicFmt)]
// Tells the derive that the `A` type parameter is not formatted,
// removing the `A: PanicFmt` bound in `impl<A, B> PanicFmt for Foo<A, B>`,
// and using `()` as the `A` type parmeter for
// `<Foo<....> as PanicFmt>::PV_COUNT` in the generated `to_panicvals` method.
#[pfmt(ignore(A))]
// Defines a `to_panicvals` method for `Foo<A, u8>`
#[pfmt(impl<A> Foo<A, u8>)]
// Defines a `to_panicvals` method for `Foo<A, &str>`
#[pfmt(impl<A> Foo<A, &str>)]
pub struct Foo<A, B> {
    value: B,
    _marker: PhantomData<A>,
}

```

<a id = "phantom-type-parameter-example"></a>
### Phantom Type parameters

This example demonstrates how type parameters can be ignored with
`#[pfmt(ignore(...))]`.

```rust
use const_panic::{ArrayString, FmtArg, PanicFmt};

use std::marker::PhantomData;

{
    const WITH_INT: Foo<u8, bool, 100> = Foo{
        value: 5,
        _marker: PhantomData,
    };
    assert_eq!(
        const_panic::concat_!(WITH_INT),
        "Foo { value: 5, _marker: PhantomData }",
    );
}
{
    const WITH_STR: Foo<str, char, 200> = Foo {
        value: 8,
        _marker: PhantomData,
    };
    assert_eq!(
        const_panic::concat_!(WITH_STR),
        r#"Foo { value: 8, _marker: PhantomData }"#,
    );
}

#[derive(Debug, PanicFmt)]
// Tells the derive that the `A` and `B` type parameters are not formatted,
// removing the `A: PanicFmt` and `B: PanicFmt` bounds in the `PanicFmt` impl for `Foo`,
// and using `()` and `u8` as the `A` and `B` type parameters for
// `<Foo<(), u8> as PanicFmt>::PV_COUNT` in the generated `to_panicvals` method.
#[pfmt(ignore(A, B = u8))]
pub struct Foo<A: ?Sized, B, const X: u32> {
    value: u32,
    _marker: PhantomData<(PhantomData<A>, B)>,
}

```

### Const-generic struct

```rust
use const_panic::{ArrayString, FmtArg, PanicFmt};

assert_eq!(const_panic::concat_!(Foo([])), "Foo([])");

assert_eq!(const_panic::concat_!(Foo([3, 5, 8])), "Foo([3, 5, 8])");

#[derive(PanicFmt)]
struct Foo<const LEN: usize>([u8; LEN]);

```


<a id = "crate-example"></a>
### Crate renaming

This example demonstrates how the `const_panic` crate can be renamed,
passing the new name to the derive macro.

```rust
# extern crate const_panic as cpanic;
# extern crate std as const_panic;
#
use cpanic::{ArrayString, FmtArg, PanicFmt};;

assert_eq!(cpanic::concat_!(Foo(Some(13))), "Foo(Some(13))");

#[derive(PanicFmt)]
#[pfmt(crate = cpanic)]
struct Foo(Option<u32>);
```

*/
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "derive")))]
pub use const_panic_proc_macros::PanicFmt;