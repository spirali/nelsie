use std::fmt::Display;
use std::fmt::Write;

pub(crate) struct SimpleXmlWriter<'a> {
    buffer: String,
    is_open: bool,
    stack: Vec<&'a str>,
}

impl<'a> SimpleXmlWriter<'a> {
    pub fn new() -> Self {
        SimpleXmlWriter {
            buffer: String::new(),
            is_open: false,
            stack: Vec::new(),
        }
    }

    pub fn begin(&mut self, name: &'a str) {
        self.close();
        self.stack.push(name);
        self.is_open = true;
        self.buffer.push('<');
        self.buffer.push_str(name);
    }

    fn close(&mut self) {
        if self.is_open {
            self.buffer.push('>');
            self.is_open = false;
        }
    }

    pub fn end(&mut self, name: &str) {
        let n = self.stack.pop().unwrap();
        assert_eq!(n, name);
        if self.is_open {
            self.buffer.push_str(" />");
            self.is_open = false;
        } else {
            self.buffer.push_str("</");
            self.buffer.push_str(name);
            self.buffer.push('>');
        }
    }

    fn attr_begin(&mut self, name: &str) {
        assert!(self.is_open);
        write!(self.buffer, " {name}='").unwrap();
    }

    pub fn attr<T: Display>(&mut self, name: &str, value: T) {
        self.attr_buf(name, |s| write!(s, "{}", value).unwrap());
    }

    pub fn attr_buf<F: Fn(&mut String)>(&mut self, name: &str, f: F) {
        self.attr_begin(name);
        let n = self.buffer.len();
        f(&mut self.buffer);
        if let Some(m) = self.buffer[n..].bytes().position(|c| c == b'\'') {
            let s = &self.buffer[n + m..].to_string();
            self.buffer.truncate(n);
            for c in s.chars() {
                if c == '\'' {
                    self.buffer.push_str("\\'");
                } else {
                    self.buffer.push(c);
                }
            }
        }
        self.buffer.push('\'');
    }

    /*pub fn text(&mut self, value: &str) {
        self.close();
        for c in value.chars() {
            match c {
                // '"' => self.buffer.push_str("&quot;"),
                // '\'' => self.buffer.push_str("&apos;"),
                '<' => self.buffer.push_str("&lt;"),
                '>' => self.buffer.push_str("&gt;"),
                '&' => self.buffer.push_str("&amp;"),
                c => self.buffer.push(c),
            }
        }
    }*/

    pub fn text_raw(&mut self, value: &str) {
        self.close();
        self.buffer.push_str(value);
    }

    pub fn into_string(self) -> String {
        assert!(self.stack.is_empty());
        self.buffer
    }
}

#[cfg(test)]
mod test {
    use super::SimpleXmlWriter;

    #[test]
    pub fn test_sxml_simple_tag() {
        let mut writer = SimpleXmlWriter::new();
        writer.begin("abc");
        writer.end("abc");
        assert_eq!(writer.into_string(), "<abc />")
    }

    #[test]
    pub fn test_sxml_nested() {
        let mut writer = SimpleXmlWriter::new();
        writer.begin("abc");
        writer.begin("x");
        writer.end("x");
        writer.end("abc");
        assert_eq!(writer.into_string(), "<abc><x /></abc>")
    }

    #[test]
    pub fn test_sxml_attr() {
        let mut writer = SimpleXmlWriter::new();
        writer.begin("abc");
        writer.attr("zzz", "foo");
        writer.attr("name", "bar");
        writer.begin("x");
        writer.attr("id", "'abc'");
        writer.end("x");
        writer.end("abc");
        assert_eq!(
            writer.into_string(),
            "<abc zzz='foo' name='bar'><x id='\\'abc\\'' /></abc>"
        )
    }
}
