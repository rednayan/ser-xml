use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;
use std::time::Instant;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphanumeric()));
        }
        return Some(self.chop(1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn index_document(doc_content: &str) -> HashMap<String, usize> {
    todo!("not implemented")
}

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;

    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
            content.push_str(" ");
        }
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    //benchmark
    let start = Instant::now();
    //-------------------------//

    let content = read_entire_xml_file("docs.gl/gl4/glVertexAttribDivisor.xhtml")?
        .chars()
        .collect::<Vec<_>>();

    let mut tf = HashMap::<String, usize>::new();
    for token in Lexer::new(&content) {
        let term = token
            .iter()
            .map(|x| x.to_ascii_uppercase())
            .collect::<String>();
        if let Some(freq) = tf.get_mut(&term) {
            *freq += 1;
        } else {
            tf.insert(term, 1);
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();
        for (t, f) in tf.iter().take(10) {
            println!("{t} => {f}");
        }
    }

    // let all_documents = HashMap::<Path, HashMap<String, usize>>::new();
    // let dir_path = "docs.gl/gl4";
    // let dir = fs::read_dir(dir_path)?;

    // for file in dir {
    //     let file_path = file?.path();
    //     let content = read_entire_xml_file(&file_path)?;
    //     println!("{file_path:?} -> {size}", size = content.len());
    // }

    //--------------------------//
    //benchmark
    let duration = start.elapsed();
    println!("benchmark: {duration} secs ", duration = duration.as_secs());
    Ok(())
}
