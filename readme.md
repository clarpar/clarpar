A command line parser for Rust

The primary purpose of this library is to parse a full command line. That is, a string containing the complete command line.
It can also parse rust environment command line arguments as supplied by Rust (`std::env::Args`) however there are other libraries which
specialise in this and may be a better choice.

While the Rust standard library does not give access to the full environment command line, this library could be used to
parse internal commands entered within an application.

# Features

* Command line parsing
* Environment arguments parsing
* Command Line Style
    * Specify which character(s) can be used to quote parameters and option values
    * Specify which character(s) can be used to announce an option
    * Specify which character(s) can be used to announce an option value (space character can be included)
    * Specify which character(s) will terminate parsing of a command line
    * Case sensitivity when matching parameters, option codes and option values
    * Whether options with code that have more than one character, require 2 announcer characters (eg --anOpt)
    * Use double quotes to embed quote characters within quoted parameters and option values
    * Use escaping to include characters with special meaning
    * Whether first argument in command line is the binary's name/path
* Argument Matching
    * Parameter or Option
    * Argument indices
    * Parameter indices
    * Parameter text (string or Regex)
    * Option indices
    * Option codes (string or Regex)
    * Whether option has value (None, IfPossible, Always)
    * Option value text (string or Regex)
    * Whether option value can start with an option announcer character
* Tag parameters and options arguments with with any enum (or any other type) from matcher for easy identification
* Parse error result has properties detailing the type of error and where it occurred.

# Usage

The main items in this library are:
* `Parser` - The object which parses a command line. It has properties which are set according to the style of a
command line and a list of matchers which are used to identify command line arguments.
* `Matcher` - Each argument must be matched against a matcher. Typically one matcher is created for each argument however
matchers can also be used to match multiple arguments.
* `Arg` - An enum with 3 variants: Binary, Param, Option. The Parser’s parse functions return an array of these
variants - each of which identify an argument the parser found in the command line.

Follow the steps below to parse a command line:
1. Create an enum with one variant for each type of option argument expected in the command line.
1. Create an enum with one variant for each type of parameter argument expected in the command line.
1. Create an instance of a `Parser`.
1. If necessary, set relevant properties of the Parser instance to reflect the style of the command line.
1. Add a `Matcher` for all possible arguments to the parser. Tag each matcher with the appropriate enum.
1. Call `Parser::parse_line(command_line)` which will parse the command line and return a result containing
either a vector of `Arg` or an error.
1. Loop through the returned vector and process each `Arg`.
