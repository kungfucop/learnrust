use core::fmt::Alignment::Left;

mod LearnBox;
mod LearnTrait;
mod LearnOwnership;
mod Closure;
mod LearnVec;
fn main() {

    let pair = (0, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    trait Foo {
        fn foo(&self);
    }

    trait FooBar: Foo {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn foo(&self) { println!("foo"); }
    }

    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }

    let baz = Baz;

    LearnBox::store_on_help();
    LearnBox::recursive_type();
    LearnTrait::testPrinter();
    LearnTrait::test_default_trait_imp();
    LearnTrait::testTraitReference();
    Closure::run();
    LearnOwnership::run();
    LearnVec::run();
}