# Chapter 14: More About Cargo and Crate.io
So far we've only used the most basic features of Cargo to build, run, and test our code. In Chapter 14 we'll discuss some of the more advanced features of Cargo.

## Customizing Builds with Release Profiles
In Rust, _release profiles_ are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.

Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo build` and the `release` profile Cargo uses when you run `cargo build -- release`. The `dev` profile is defined with good defaults for development, and `release` has good defaults for release builds. 

Cargo has default settings for each of the profiles that apply when you haven't explicitly added any `[profile.*]` sections in the project's _Cargo.toml_ file. By adding `[profile.*]` sections for any profile you want to customize, you override any subset of the default settings. For example, here are the default values for the `opt-level` setting for the `dev` and `release` profiles:

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

the `opt-level` setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. Applying more optimizations extends compiling time, so if you're in development mode and compiling your code often, you'll want fewer optimizations to compile faster even if the resulting code runs slower. 

You can override the default settings for profiles by explicitly setting it to a different value in the _Cargo.toml_  file. 

## Publishing a Crate to Crates.io
Just as we have used packages from crates.io as dependencies, we can publish our own packages. Rust and Cargo hav features that make your published package easier for people to find and use.

### Making Useful Documentation Comments
A major focus of Rust itself, and the Rust community is good documentation. We already know that we can make code comments in Rust using `//` to comment out the line. There is another kind of comment we can make though, a _documentation comment_, that will generate html documentation (and more details on hover in VSCode). These documentation comments will only be displayed for _public_ pieces of code. 

These documentation comments are made using `///` rather than `//`. Once in the three-slash comments, they support normal markdown notation for formatting the comments. These comments should be placed just above what it is they are referencing. 

```rust
///Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer)
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}
```

In the example above we give a brief description of what the `add_one` function does, start a section with the heading `Examples`, and then provide code that demonstrates how to use the `add_one` function. We can generate the HTML documentation from these comments by running `cargo doc`. This command runs the `rustdoc` tool distributed with Rust and puts the generated HTML documentation in the _target/doc_ directory. We can build then immediately open the docs by running `cargo doc --open`. This also builds the docs for all the dependencies of our crate as well.

#### Commonly Used Sections
We used `# Examples` above to create a section for our documentation. Some other common sections in Rust documentation are:
- *Panics*: The scenarios in which the function being documented could panic. Callers of the function who don't want their programs to panic should make sure they don't call the function in these situations.
- *Errors*: If the function returns a `Result`, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so they can write code to handle the different kinds of errors in different ways. 
- *Safety*: If the function is `unsafe` to call (discussed in Chapter 19), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold. 

Not every piece of documentation needs all 3 of these sections, nor is it an exhaustive list, but it is a good reference to keep in mind when writing documentation.

#### Documentation Comments as Tests
Adding example code blocks in the documentation comments can help illustrate how to use the library, and it comes with an additional bonus: running `cargo test` will run the code examples in the documentation as tests. This helps ensure our documentation is always up to date. If a function has a breaking change and we don't update the documentation, we'll catch it during our test run. 

#### Commenting Contained Items
the `//!` style doc comment adds documentation to the item that contains the comments rather than to the items following the comments. We typically use these doc comments inside the crate root file (_src/lib.rs_ by convention) or inside a module to document the crate or module as a whole. 

For example, to add documentation that describes the purpose of the `my_crate` crate that contains the `add_one` function, we add documentation comments that start with `//!` to the beginning of the lib file.

### Exporting a Convenient Public API with `pub use`
Having a convenient and accessible structure is important for people that need to use your crate. Sometimes this doesn't happen naturally. The structure of the crate for development purposes might not result in a friendly structure for users. Lucky for us, we can make use of the concept of _re-exporting_ to make for a more friendly public API, without having to completely restructure our code. 

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        unimplemented!();
    }
}
```

The documentation for the example above will show the _kinds_ and _utils_ modules in the _Art_ crate, but doesn't show any of the specifics about the `PrimaryColor`, `SecondaryColor`, or `mix` types/functions. 

An example of how to import the `PrimaryColor` and `mix` items is:

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

We can see that there is a bit of drilling down required to get to the items we're looking for, and in some cases we may not even be aware of them because they are not easy to find. 

To resolve these issues we can modify our art crate a bit

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

The documentation will now list our re-exports: `PrimaryColor`, `SecondaryColor`, and `mix`. Importing is much nicer now as well

```rust
use art::mix;
use art::PrimaryColor;

fn main() {
  // --snip--
}
```

### Setting Up a Crates.io Account
Setting up an account is easy. visit [crates.io](http://crates.io) and create an account by logging in via github, get your API key and login using it via cargo:

```sh
$ cargo login <api-key>
```

This stores our api key in _~/.cargo/credentials_.

### Adding Metadata to a New Crate
Metadata about the package can be added via the `[package]` section of our _Cargo.toml_. If publishing on crates.io, it needs a unique name. License is another required filed for publishing. Other helpful metadata fields include: description, version, editions, etc...

### Publishing to Crates.io
To publish to crates.io after following all the previous steps above run

```sh
cargo publish
```

Be sure you're ready to publish your code, once published it cannot be removed. Crates.io focuses on being a permanent archive for code, ensuring any code that uses any crate will always work.

### Publishing a New Version of an Existing Crate
When it comes time to publish a new version it is as simple as incrementing the version in the _Cargo.toml_ file as appropriate and republish. 

### Deprecating Versions from Crates.io with `cargo yank`
Although we can't fully remove a package from crates.io, we can prevent future projects from adding them as new dependencies without breaking legacy projects that need it. To do this run 

```sh
cargo yank --vers X.X.X
```

you can undo `yank`s by adding the `--undo` flag

## Cargo Workspaces
Cargo workspaces are a way to split a large library crate into many smaller related packages.

### Creating a Workspace
a _workspace_ is a set of packages that share the same _Cargo.lock_ and output directory. Inside this Chapter 14 folder we'll set up a basic workspace. We'll use one common way to structure the workspace, though there are many options. We'll have a workspace containing a binary and two libraries. The binary, which will provide the main functionality, will depend on the two libraries. One library will provide an `add_one` function, and a second library an `add_two` function. All three crates will be part of the same workspace. 

Start by creating a new directory for the workspace

```sh
mkdir add
cd add
```
(we're using _chapter\_14_ in this case)
Now, we create a _Cargo.toml_ file that will configure the entire workspace. This file won't have a `[package]` section. Instead, it will start with a `[workspace]` section that will allow us to add members to the workspace by specifying the path to the package with our binary crate; in our example the path will be _adder_

```
[workspace]

members = [
    "adder",
]
```

Next lets create the `adder` binary crate by running `cargo new` within the directory

```sh
cargo new adder
```

at this point our workspace looks like:

```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The workspace has one _target_ directory at the top level that the compiled artifacts will be placed into; the _adder_ package doesn't have its own _target_ directory. we can run `cargo run` from the _chapter\_13_ level and it will build and run our code in the _adder_ crate.

### Creating the Second PAckage in the Workspace
Next, we'll create another member package in the workspace and call it `add_one`. Change the top-level _Cargo.toml_ to specify the _add\_one_ path in the `members` list:

```
[workspace]

members = [
    "adder",
    "add_one",
]
```

Then generate another library named `add_one`

```sh
cargo new add_one --lib
```

Now our _adder_ package can depend on our _add\_one_ package. First we need to add a path dependency on `add_one` to the _Cargo.toml_ file.

```
[dependencies]
add_one = { path = "../add_one" }
```

Cargo doesn't assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships. 

Next, let's use the `add_one` function in the `adder` crate. In _adder/src/main.rs_ we add a `use` line at the top to bring the new `add_one` library crate into scope. Then change the `main` function to call the `add_one` function

Now we can build and run our code. We can specify which package in the workspace we want to run by using the `-p` argument and the package name with `cargo run`:

```sh
cargo run -p adder
```

#### Depending on an External Package in a Workspace
Notice that the workspace only has a single _Cargo.lock_ file in the workspace, rather than one in every crate. This ensures that all crates are using the same version of all dependencies. If we add the `rand` package to the _adder/Cargo.toml_ and the _add\_one/Cargo.toml_ files, Cargo will resolve both of those to one version of `rand` and record that in the one _Cargo.lock_. Making all crates in the workspace use the same dependencies means the crates will always be compatible with each other.

Adding some dependency in one of the crates results in that dependency showing up in the root _Cargo.lock_, but each individual package still needs to add it to it's _Cargo.toml_ file to make use of it.

#### Adding a Test to the Workspace
For another enhancement, let's add a test of the `add_one::add_one` function within the `add_one` crate

Then we can run `cargo test` from the top-level directory to run tests for every crate in the workspace

just like `cargo build` we can pass the `-p` flag to `cargo test` to only run the tests in a specific crate.

## Installing Binaries with `cargo install`
