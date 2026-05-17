//! A *minimal* HTML-ish tokenizer for the Phase 1 demo.
//!
//! This is not an HTML5 parser. It recognizes `<tag attr="value">…
//! </tag>` and nothing else: no self-closing tags, no doctype, no
//! CDATA, no entities, no error recovery. A real parser will live in
//! a separate crate later — the point here is to show the pipeline
//! end-to-end without pulling in a million lines of spec compliance.

#[derive(Debug, Clone, PartialEq)]
pub enum Event<'a> {
    Open(&'a str, Vec<(&'a str, &'a str)>),
    Close(&'a str),
    Text(&'a str),
}

pub fn events(src: &str) -> EventIter<'_> {
    EventIter { src, pos: 0 }
}

pub struct EventIter<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Event<'a>> {
        let bytes = self.src.as_bytes();
        while self.pos < bytes.len() {
            if bytes[self.pos] == b'<' {
                let start = self.pos + 1;
                let rel = match self.src[start..].find('>') {
                    Some(r) => r,
                    None => { self.pos = bytes.len(); return None; }
                };
                let end = start + rel;
                let inner = self.src[start..end].trim();
                self.pos = end + 1;

                if let Some(tag) = inner.strip_prefix('/') {
                    return Some(Event::Close(tag.trim()));
                }

                let (tag, rest) = match inner.find(char::is_whitespace) {
                    Some(i) => (&inner[..i], inner[i..].trim()),
                    None    => (inner, ""),
                };
                return Some(Event::Open(tag, parse_attrs(rest)));
            } else {
                let start = self.pos;
                let end = self.src[start..]
                    .find('<')
                    .map(|i| start + i)
                    .unwrap_or(bytes.len());
                self.pos = end;
                let text = self.src[start..end].trim();
                if !text.is_empty() {
                    return Some(Event::Text(text));
                }
            }
        }
        None
    }
}

fn parse_attrs(mut s: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    while !s.is_empty() {
        let eq = match s.find('=') { Some(i) => i, None => break };
        let name = s[..eq].trim();
        s = s[eq + 1..].trim_start();
        let Some(rest) = s.strip_prefix('"') else { break };
        let Some(close) = rest.find('"') else { break };
        out.push((name, &rest[..close]));
        s = rest[close + 1..].trim_start();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element_roundtrip() {
        let src = r#"<div style="background:red">hi</div>"#;
        let evs: Vec<_> = events(src).collect();
        assert_eq!(evs.len(), 3);
        match &evs[0] {
            Event::Open(t, a) => {
                assert_eq!(*t, "div");
                assert_eq!(a, &vec![("style", "background:red")]);
            }
            _ => panic!(),
        }
        assert_eq!(evs[1], Event::Text("hi"));
        assert_eq!(evs[2], Event::Close("div"));
    }

    #[test]
    fn nested() {
        let src = r#"<a><b></b></a>"#;
        let evs: Vec<_> = events(src).collect();
        assert_eq!(evs.len(), 4);
    }
}
