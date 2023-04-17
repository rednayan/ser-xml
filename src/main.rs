use std::collections::HashMap;
use std::fs::{self, File};
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

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_ascii_alphabetic() {
            let mut n = 0;
            while n < self.content.len() && self.content[n].is_ascii_alphanumeric() {
                n += 1;
            }

            let token = &self.content[0..n];
            self.content = &self.content[n..];
            return Some(token);
        }

        todo!()
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

    for token in Lexer::new(&content) {
        println!("{token}", token = token.iter().collect::<String>());
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
