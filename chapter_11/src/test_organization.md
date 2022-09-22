# Test Organization

Rust thinks about tests in terms of _unit tests_ or _integration tests_.
_Unit tests_ are small and focused, testing one module in isolation, and can test private interfaces.
_Integration Tests_ are entirely external to your library and use you code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

## Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected. You'll put unit tests directly in the _src_ directory in each file with the code they are testing. The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

#### The Tests Module and #[cfg(test)]

the #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`. This saves compile time whe you only want to build the library and saves space in the resulting compiled artifact because the tests are not included. We'll see in a bit that because _Integration tests_ go in a separate directory they don't need the `cfg(test)` annotation.

Recall that when we generated a `--lib` project Cargo generated this code for us by default:

```rust
#[cfg(test)]
mod tests {
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
```

This code is the automatically generated test module. the attribute `cfg` stands for _configuration_ and tells Rust that the following item should only be included given a certain configuration option. In this case, the configuration option is `test` which is provided by Rust for compiling tests. This includes any helper functions that are defined within the `tests` module in addition to our test functions.

##### Testing Private Functions

There's debate within the testing community about whether or not private functions should be tested directly, other languages make it difficult or impossible to test private functions. Regardless of which ideology you subscribe to, Rust's privacy rules do allow you to test private functions. Consider the following code with the private function `internal_adder`:

```rust
pub fn add_two(a: i32) -> i32 {
  internal_adder(a, 2);
}

fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*

  #[test]
  fn internal() {
    assert_eq!(4, internal_adder(2, 2));
  }
}
```

Note that the `internal_adder` function is not marked as `pub`. Tests are just Rust code, and the `tests` module is just another module. As previously discussed, items in a child module can use items in their ancestor modules. In this test, we bring all of the `test` module's parent's items into scope with `use super::*`, then our test can call `internal_adder`. If you don't want to test private functions there is nothing that requires you to do so.

## Integration Tests

In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library's public API. Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. To create integration tests, you first need a _tests_ directory.

##### The _tests_ Directory

We create a _tests_ directory at the top level of our project directory, next to _src_. Cargo knows to look for integration test files in this directory. We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.

Let's create an integration test. With the code used above existing in _src/lib.rs_, make a _tests_ directory, and create a new file named _tests/integration_test.rs_. The folder structure should look like:

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

We'll place the following code in _integration_tests.rs_

```rust
use adder;

#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}
```

Each file in the _tests_ directory is a separate crate, so we need to bring our library into each test crate's scope. For that reason we add `use adder;` at the top of the code, which we didn't need in the unit tests.

We don't need to annotate the test configuration either. The _tests_ directory will only ever compile when we use `cargo test`.

When we run `cargo test` now we will see 3 distinct sections in the output:

1. unit tests
2. integration tests
3. doc tests

Note: if any tests fail in one of the sections, the following sections will not be run.

Each integration test file gets its own output in the integration test section.

We can still run specific integration tests, like unit tests, by supplying the function name as an arg to `cargo test`, or if we want to run a whole integration test file, we can use something like `cargo test --test integration_test`.

#### Submodules in Integration Tests

As we add more integration tests, we might want to make more files in the _tests_ directory to help organize them; for example, you can group the test functions by the functionality they're testing. As mentioned earlier, each file in the _tests_ directory is compiled as its own separate crate, which is useful for creating separate scopes to more closely imitate the way end users will be using your crate. However, this means files in the _tests_ directory don;t share the same behavior as files in _src_ do.
The different behavior of _tests_ directory files is most noticeable when you have a set of helper functions to use in multiple integration test files and you try to extract them into a common module. For example, if we create _test/common.rs_ and place a function named `setup` in it, we can add some code to `setup` that we want to call from multiple test functions in multiple test files:

_tests/common.rs_

```rust
pub fn setup() {
  // setup code specific to your library's tests would go here
}
```

When we run the tests again, we'll see a section in the test output for the _common.rs_ file, even though this file doesn't contain any actual tests, nor did we call `setup` anywhere yet.
This isn't exactly what we want to happen.
To avoid having _common.rs_ appear in the test output, instead of creating _tests/common.rs_, we'll create _tests/common/mod.rs_.
The project directory now looks like:

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

```

This is the older naming convention that Rust also understands. Naming the file this way tells Rust not to treat the _common_ module as an integration test file. Files in the subdirectories of the _tests_ directory don't get compiled as separate crates or have sections in the test output.

Now that we have our _common_ module, we can use it from any of the integration test files as a module. Here is an example calling the `setup` function from our earlier _tests/integration_test.rs_ file:

_test/integration_test.rs_

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}
```

#### Integration Tests for Binary Crates

If our project is a binary crate that only contains a _src/main.rs_ file and doesn't have a _src/lib.rs_ file, we can't create integration tests in the _tests_ directory and bring functions defined in the _src/main.rs_ file into scope with a `use` statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward _src/main.rs_ file that calls logic that lives in the _src/lib.rs_ file. Using this structure, integration tests _can_ test the library crate with `use` to make the important functionality available. If the important functionality works, the small amount of code in the _src/main.rs_ file will work as well, and that small amount of code doesn't need to be tested.
