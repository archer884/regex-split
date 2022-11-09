# regex-split

The regex crate doesn't provide split_inclusive, which is found in the standard library for string, etc. There's an unstable feature that allows a regex to be used as the search pattern for a split, yadda yadda, etc., but who wants to use unstable these days?

Anyway, this library adds `split_inclusive` and `split_inclusive_left`, with the difference being that `split_inclusive_left` places the delimiter at the beginning of the substring, where `split_inclusive` places it at the end.

## Usage

First, add the package.

```shell
$ cargo add regex-split
```

Then import regex_split::RegexSplit wherever you'd like to use the extra methods. Consuming the new methods is straightforward.

```rust
use regex_split::RegexSplit;

// split_inclusive
let re = Regex::new("\r?\n").unwrap();
let text = "This is just\na set of lines\r\nwith different newlines.";
let v: Vec<&str> = re.split_inclusive(text).collect();

assert_eq!(v, [
    "This is just\n",
    "a set of lines\r\n",
    "with different newlines.",
]);

// split_inclusive_left
let re = Regex::new("(?m)^-").unwrap();
let text = "List of fruits:\n-apple\n-pear\n-banana";
let v: Vec<&str> = re.split_inclusive_left(text).collect();

assert_eq!(v, [
    "List of fruits:\n",
    "-apple\n",
    "-pear\n",
    "-banana",
]);
```

That's pretty much it.
