/// Remove trailing '\n' at the end of a string.
pub fn trim_string(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        let new_len = s.len() - 1;
        s.truncate(new_len);
    }
}
//Todo error handler


