//! `spongemock` provides a convenient way to MoCk ThInGs in the style of
//! [Mocking SpongeBob](https://knowyourmeme.com/memes/mocking-spongebob).

/// Returns a MoCkInG version of the given input
///
/// # Example
///
/// ```
/// let mocked = spongemock::mock("a sensible example");
///
/// assert_eq!("A SeNsIbLe eXaMpLe", mocked);
/// ```
pub fn mock<S: Into<String>>(input: S) -> String {
    let input = input.into();
    let mut out = String::with_capacity(input.capacity());
    let mut make_uppercase = true;

    for c in input.chars() {
        if make_uppercase {
            for upper in c.to_uppercase() {
                out.push(upper);
            }
        } else {
            out.push(c);
        }
        make_uppercase = !make_uppercase;
    }

    out
}

#[cfg(test)]
mod test {
    use super::mock;

    #[test]
    fn basic_mocking() {
        assert_eq!("FoO".to_string(), mock("foo"));
        assert_eq!("1234567890!@#$%^&*()-=_+".to_string(), mock("1234567890!@#$%^&*()-=_+"));
        assert_eq!("ÄöÜßSS".to_string(), mock("äöüßß"));
    }
}