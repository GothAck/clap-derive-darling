//! # An alternative implementation of the Clap Derive macros
//!
//! ## Why?
//! Mostly so that I have an excuse to play with proc macros
//!
//! ## But why?
//! Yeah I know, reinventing the wheel, etc. I needed a project.

pub mod rename;

use std::ffi::OsString;

extern crate clap;

use clap::{App, ArgMatches, Error, IntoApp};

pub use clap_derive_darling_macro::{Args, Parser};

pub use once_cell::race::OnceBox;

/// Parse command-line arguments into `Self`.
///
/// The primary one-stop-shop trait used to create an instance of a `clap`
/// [`App`], conduct the parsing, and turn the resulting [`ArgMatches`] back
/// into concrete instance of the user struct.
///
/// This trait is primarily a convenience on top of [`FromArgMatches`] +
/// [`IntoApp`] which uses those two underlying traits to build the two
/// fundamental functions `parse` which uses the `std::env::args_os` iterator,
/// and `parse_from` which allows the consumer to supply the iterator (along
/// with fallible options for each).
///
/// See also [`Subcommand`] and [`Args`].
///
/// # Examples
///
/// The following example creates a `Context` struct that would be used
/// throughout the application representing the normalized values coming from
/// the CLI.
///
/// ```rust
/// # use clap::{Clap};
/// /// My super CLI
/// #[derive(Clap)]
/// #[clap(name = "demo")]
/// struct Context {
///     /// More verbose output
///     #[clap(long)]
///     verbose: bool,
///     /// An optional name
///     #[clap(short, long)]
///     name: Option<String>,
/// }
/// ```
///
/// The equivalent [`App`] struct + `From` implementation:
///
/// ```rust
/// # use clap::{App, Arg, ArgMatches};
/// App::new("demo")
///     .about("My super CLI")
///     .arg(Arg::new("verbose")
///         .long("verbose")
///         .about("More verbose output"))
///     .arg(Arg::new("name")
///         .long("name")
///         .short('n')
///         .about("An optional name")
///         .takes_value(true));
///
/// struct Context {
///     verbose: bool,
///     name: Option<String>,
/// }
///
/// impl From<ArgMatches> for Context {
///     fn from(m: ArgMatches) -> Self {
///         Context {
///             verbose: m.is_present("verbose"),
///             name: m.value_of("name").map(|n| n.to_owned()),
///         }
///     }
/// }
/// ```
///
pub trait Clap: FromArgMatches + IntoApp + Sized {
    /// Parse from `std::env::args_os()`, exit on error
    fn parse() -> Self {
        let matches = <Self as IntoApp>::into_app().get_matches();
        <Self as FromArgMatches>::from_arg_matches(&matches, &None)
            .expect("IntoApp validated everything")
    }

    /// Parse from `std::env::args_os()`, return Err on error.
    fn try_parse() -> Result<Self, Error> {
        let matches = <Self as IntoApp>::into_app().try_get_matches()?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches, &None)
            .expect("IntoApp validated everything"))
    }

    /// Parse from iterator, exit on error
    fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().get_matches_from(itr);
        <Self as FromArgMatches>::from_arg_matches(&matches, &None)
            .expect("IntoApp validated everything")
    }

    /// Parse from iterator, return Err on error.
    fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().try_get_matches_from(itr)?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches, &None)
            .expect("IntoApp validated everything"))
    }

    /// Update from iterator, exit on error
    fn update_from<I, T>(&mut self, itr: I)
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        // TODO find a way to get partial matches
        let matches = <Self as IntoApp>::into_app_for_update().get_matches_from(itr);
        <Self as FromArgMatches>::update_from_arg_matches(self, &matches, &None).unwrap();
    }

    /// Update from iterator, return Err on error.
    fn try_update_from<I, T>(&mut self, itr: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app_for_update().try_get_matches_from(itr)?;
        <Self as FromArgMatches>::update_from_arg_matches(self, &matches, &None)?;
        Ok(())
    }
}

/// Converts an instance of [`ArgMatches`] to a user-defined container.
pub trait FromArgMatches: Sized {
    /// Instantiate `Self` from [`ArgMatches`], parsing the arguments as needed.
    ///
    /// Motivation: If our application had two CLI options, `--name
    /// <STRING>` and the flag `--debug`, we may create a struct as follows:
    ///
    #[cfg_attr(not(feature = "derive"), doc = " ```ignore")]
    #[cfg_attr(feature = "derive", doc = " ```no_run")]
    /// struct Context {
    ///     name: String,
    ///     debug: bool
    /// }
    /// ```
    ///
    /// We then need to convert the `ArgMatches` that `clap` generated into our struct.
    /// `from_arg_matches` serves as the equivalent of:
    ///
    #[cfg_attr(not(feature = "derive"), doc = " ```ignore")]
    #[cfg_attr(feature = "derive", doc = " ```no_run")]
    /// # use clap::ArgMatches;
    /// # struct Context {
    /// #   name: String,
    /// #   debug: bool
    /// # }
    /// impl From<ArgMatches> for Context {
    ///    fn from(m: ArgMatches) -> Self {
    ///        Context {
    ///            name: m.value_of("name").unwrap().to_string(),
    ///            debug: m.is_present("debug"),
    ///        }
    ///    }
    /// }
    /// ```
    fn from_arg_matches(matches: &ArgMatches, prefix: &Option<String>) -> Result<Self, Error>;

    /// Assign values from `ArgMatches` to `self`.
    fn update_from_arg_matches(
        &mut self,
        matches: &ArgMatches,
        prefix: &Option<String>,
    ) -> Result<(), Error>;
}

/// Parse arguments into a user-defined container.
///
/// Implementing this trait lets a parent container delegate argument parsing behavior to `Self`.
/// with:
/// - `#[clap(flatten)] args: ChildArgs`: Attribute can only be used with struct fields that impl
///   `Args`.
/// - `Variant(ChildArgs)`: No attribute is used with enum variants that impl `Args`.
///
///
/// # Example
///
#[cfg_attr(not(feature = "derive"), doc = " ```ignore")]
#[cfg_attr(feature = "derive", doc = " ```")]
/// #[derive(clap::Parser)]
/// struct Args {
///     #[clap(flatten)]
///     logging: LogArgs,
/// }
///
/// #[derive(clap::Args)]
/// struct LogArgs {
///     #[clap(long, short = 'v', parse(from_occurrences))]
///     verbose: i8,
/// }
/// ```
pub trait Args: FromArgMatches + Sized {
    /// Append to [`App`] so it can instantiate `Self`.
    ///
    /// See also [`IntoApp`].
    fn augment_args<'a>(app: App<'a>, prefix: &Option<String>) -> App<'a>;
    /// Append to [`App`] so it can update `self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    ///
    /// See also [`IntoApp`].
    fn augment_args_for_update<'a>(app: App<'a>, prefix: &Option<String>) -> App<'a>;
}

/// Parse a sub-command into a user-defined enum.
///
/// Implementing this trait lets a parent container delegate subcommand behavior to `Self`.
/// with:
/// - `#[clap(subcommand)] field: SubCmd`: Attribute can be used with either struct fields or enum
///   variants that impl `Subcommand`.
/// - `#[clap(flatten)] Variant(SubCmd)`: Attribute can only be used with enum variants that impl
///   `Subcommand`.
///
/// # Example
///
#[cfg_attr(not(feature = "derive"), doc = " ```ignore")]
#[cfg_attr(feature = "derive", doc = " ```")]
/// #[derive(clap::Parser)]
/// struct Args {
///     #[clap(subcommand)]
///     action: Action,
/// }
///
/// #[derive(clap::Subcommand)]
/// enum Action {
///     Add,
///     Remove,
/// }
/// ```
pub trait Subcommand: FromArgMatches + Sized {
    /// Append to [`App`] so it can instantiate `Self`.
    ///
    /// See also [`IntoApp`].
    fn augment_subcommands(app: App<'_>) -> App<'_>;
    /// Append to [`App`] so it can update `self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    ///
    /// See also [`IntoApp`].
    fn augment_subcommands_for_update(app: App<'_>) -> App<'_>;
    /// Test whether `Self` can parse a specific subcommand
    fn has_subcommand(name: &str) -> bool;
}
