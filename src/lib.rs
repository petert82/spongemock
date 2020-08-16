//! `spongemock` provides a convenient way to MoCk ThInGs in the style of
//! [Mocking SpongeBob](https://knowyourmeme.com/memes/mocking-spongebob).

/// Returns a MoCkInG version of the given input
///
/// # Example
///
/// ```
/// let mocked = spongemock::mock("a sensible EXAMPLE.");
///
/// assert_eq!("A sEnSiBlE eXaMpLe.", mocked);
/// ```
pub fn mock<S: Into<String>>(input: S) -> String {
    let input = input.into();
    let mut out = String::with_capacity(input.capacity());
    let mut make_uppercase = true;

    for c in input.chars() {
        if make_uppercase && c.is_lowercase(){
            for upper in c.to_uppercase() {
                out.push(upper);
            }
        } else if !make_uppercase && c.is_uppercase() {
            for upper in c.to_lowercase() {
                out.push(upper);
            }
        } else {
            out.push(c);
        }

        if c.is_alphabetic() {
            make_uppercase = !make_uppercase;
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::mock;

    #[test]
    fn basic_mocking() {
        assert_eq!(mock("abcdefghijklmnopqrstuvwxyz"), "AbCdEfGhIjKlMnOpQrStUvWxYz".to_string());
        assert_eq!(mock("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), "AbCdEfGhIjKlMnOpQrStUvWxYz".to_string());
        assert_eq!(mock("a1b2c3 d4e5f6"), "A1b2C3 d4E5f6".to_string());
        assert_eq!(mock("1234567890!@#$%^&*()-=_+"), "1234567890!@#$%^&*()-=_+".to_string());
        assert_eq!(mock("äöüßßâç"), "ÄöÜßSSâÇ".to_string());
        assert_eq!(mock("✌🤞😎"), "✌🤞😎".to_string());
    }
}