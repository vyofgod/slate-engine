//! Incremental HTML parser. Accepts UTF-8 chunks and emits
//! [`OwnedWebCall`]s for the pieces it has fully consumed. Anything
//! straddling a chunk boundary is held in the internal buffer until
//! the next `feed`.
//!
//! Same subset as `slate_kernel::parse`: `<tag attr="value">…</tag>`
//! plus text. No entity decoding, no error recovery — this is a
//! demonstration that the streaming pipeline is *shaped* correctly,
//! not a spec-compliant parser.

use slate_ais::NodeId;
use slate_dispatcher::OwnedWebCall;

#[derive(Debug)]
pub struct IncrementalParser {
    buf:     String,
    stack:   Vec<(NodeId, u32)>,
    next_id: u32,
}

impl Default for IncrementalParser {
    fn default() -> Self { Self::new() }
}

impl IncrementalParser {
    pub fn new() -> Self {
        Self { buf: String::new(), stack: Vec::new(), next_id: 1 }
    }

    /// Feed a chunk of bytes. Valid UTF-8 is assumed; invalid bytes
    /// are replaced with U+FFFD so the parser never panics mid-stream.
    pub fn feed(&mut self, chunk: &[u8]) -> Vec<OwnedWebCall> {
        // Lossy append keeps us streamable even if the chunk boundary
        // splits a UTF-8 sequence — a real parser would buffer the
        // partial bytes; for Phase 2 this is good enough.
        self.buf.push_str(&String::from_utf8_lossy(chunk));
        self.drive()
    }

    /// Signal EOF. Flushes any residual text.
    pub fn finish(&mut self) -> Vec<OwnedWebCall> {
        let out = self.drive();
        self.buf.clear();
        self.stack.clear();
        out
    }

    fn drive(&mut self) -> Vec<OwnedWebCall> {
        let mut out = Vec::new();
        loop {
            // If the buffer is empty, we're done.
            if self.buf.is_empty() { break; }

            // Is the next token a tag?
            let first = self.buf.as_bytes()[0];
            if first == b'<' {
                // Wait until we've seen the closing '>'.
                let Some(gt) = self.buf.find('>') else { break };
                let inner = self.buf[1..gt].trim().to_string();
                self.buf.drain(..=gt);

                if let Some(tag) = inner.strip_prefix('/') {
                    let _ = tag; // name is informational only
                    self.stack.pop();
                } else {
                    let (tag, rest) = match inner.find(char::is_whitespace) {
                        Some(i) => (inner[..i].to_string(), inner[i..].trim().to_string()),
                        None    => (inner, String::new()),
                    };

                    let id = NodeId(self.next_id);
                    self.next_id += 1;

                    let (parent, index) = match self.stack.last_mut() {
                        Some(last) => {
                            let idx = last.1;
                            last.1 += 1;
                            (Some(last.0), idx)
                        }
                        None => (None, 0),
                    };

                    out.push(OwnedWebCall::CreateElement { node: id, tag });
                    if let Some(p) = parent {
                        out.push(OwnedWebCall::AppendChild {
                            parent: p,
                            child:  id,
                            index,
                        });
                    }

                    for (name, value) in parse_attrs(&rest) {
                        if name == "style" {
                            out.push(OwnedWebCall::SetInlineStyle {
                                node: id,
                                css:  value,
                            });
                        } else {
                            out.push(OwnedWebCall::SetAttribute {
                                node: id,
                                name,
                                value,
                            });
                        }
                    }

                    self.stack.push((id, 0));
                }
            } else {
                // Text until the next '<' *or* buffer end.
                let end = self.buf.find('<').unwrap_or(self.buf.len());
                // If we consumed to the very end and didn't see a '<',
                // we might be holding a partial text run — keep it in
                // the buffer until the next chunk arrives.
                if end == self.buf.len() { break; }
                let _text = self.buf[..end].trim().to_string();
                self.buf.drain(..end);
                // Text primitives not yet in AIS — drop silently.
            }
        }
        out
    }
}

fn parse_attrs(mut s: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    while !s.is_empty() {
        let eq = match s.find('=') { Some(i) => i, None => break };
        let name = s[..eq].trim().to_string();
        s = s[eq + 1..].trim_start();
        let Some(rest) = s.strip_prefix('"') else { break };
        let Some(close) = rest.find('"') else { break };
        out.push((name, rest[..close].to_string()));
        s = rest[close + 1..].trim_start();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streaming_splits_cleanly_across_chunks() {
        let mut p = IncrementalParser::new();
        let calls: Vec<_> = [
            &b"<div styl"[..],
            &b"e=\"background:red"[..],
            &b"\"><span></span></div>"[..],
        ]
        .into_iter()
        .flat_map(|c| p.feed(c))
        .collect();

        assert!(matches!(calls[0], OwnedWebCall::CreateElement { .. }));
        assert!(matches!(calls[1], OwnedWebCall::SetInlineStyle { .. }));
        assert!(matches!(calls[2], OwnedWebCall::CreateElement { .. }));
        assert!(matches!(calls[3], OwnedWebCall::AppendChild   { .. }));
    }
}
