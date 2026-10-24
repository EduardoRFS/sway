//! Socket discovery with the original sway / i3 fallback order.
#![forbid(unsafe_code)]

use std::{
    ffi::OsString,
    io::{BufRead, BufReader},
    os::unix::ffi::OsStringExt,
    process::{Command, Stdio},
};

pub fn discover() -> Option<OsString> {
    discover_with(|name| std::env::var_os(name), query)
}

fn discover_with(
    environment: impl Fn(&str) -> Option<OsString>,
    command: impl Fn(&str) -> Option<Vec<u8>>,
) -> Option<OsString> {
    environment("SWAYSOCK")
        .or_else(|| command("sway").map(OsString::from_vec))
        .or_else(|| environment("I3SOCK"))
        .or_else(|| command("i3").map(OsString::from_vec))
}

fn query(program: &str) -> Option<Vec<u8>> {
    let mut child = Command::new("sh")
        .args(["-c", &format!("{program} --get-socketpath 2>/dev/null")])
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    let mut line = Vec::new();
    let result = BufReader::new(child.stdout.take()?).read_until(b'\n', &mut line);
    // Like popen/getline/pclose, consume one line, close the pipe, and wait.
    let _ = child.wait();
    if !matches!(result, Ok(size) if size > 0) {
        return None;
    }
    if line.last() == Some(&b'\n') {
        line.pop();
    }
    Some(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn empty_environment_value_is_authoritative() {
        assert_eq!(
            discover_with(|_| Some(OsString::new()), |_| panic!("must not run")),
            Some(OsString::new())
        );
    }

    #[test]
    fn follows_sway_then_i3_order_and_preserves_non_utf8_paths() {
        let calls = RefCell::new(Vec::new());
        let result = discover_with(
            |name| {
                calls.borrow_mut().push(name.to_string());
                None
            },
            |program| {
                calls.borrow_mut().push(program.to_string());
                (program == "i3").then(|| b"/tmp/\xff".to_vec())
            },
        );
        assert_eq!(*calls.borrow(), ["SWAYSOCK", "sway", "I3SOCK", "i3"]);
        assert_eq!(result, Some(OsString::from_vec(b"/tmp/\xff".to_vec())));
    }
}
