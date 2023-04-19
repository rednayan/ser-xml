use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
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

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn check_index(index_path: &str) -> io::Result<()> {
    let index_file = File::open(index_path)?;
    println!("Reading {index_path} index file...");
    let tf_index: TermFreqIndex = serde_json::from_reader(index_file).expect("serde does not fail");

    println!(
        "{index_path} contains {count_files}",
        count_files = tf_index.len()
    );
    Ok(())
}

fn index_folder(dir_path: &str) -> io::Result<()> {
    let dir = fs::read_dir(dir_path)?;
    let _top_n = 20;
    let mut tf_index = TermFreqIndex::new();

    for file in dir {
        let file_path = file?.path();

        println!("Indexing {:?}...", &file_path);

        let content = read_entire_xml_file(&file_path)?
            .chars()
            .collect::<Vec<_>>();

        let mut tf = TermFreq::new();
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
        }

        let mut stats = tf.iter().collect::<Vec<_>>();
        stats.sort_by_key(|(_, f)| *f);
        stats.reverse();

        tf_index.insert(file_path, tf);
    }
    let index_path = "index.json";
    println!("saving {index_path:?}");
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, &tf_index).expect("serde works fine");
    Ok(())
}

fn main() {
    let mut args = env::args();

    let _program_path = args.next().expect("path to program exists");

    let subcommand = args.next().unwrap_or_else(|| {
        eprintln!("ERROR: no subcommand is provided");
        exit(1)
    });

    match subcommand.as_str() {
        "index" => {
            let dir_path = args.next().unwrap_or_else(|| {
                eprintln!("ERROR: directory path not provided");
                exit(1)
            });

            index_folder(&dir_path).unwrap_or_else(|err| {
                eprintln!("ERROR: could not index folder :{err}");
            })
        }

        "search" => {
            let index_path = args.next().unwrap_or_else(|| {
                eprintln!("ERROR: no file path is provided");
                exit(1)
            });
            check_index(&index_path).unwrap_or_else(|err| {
                eprintln!("ERROR: could not check index file {index_path} : {err}");
                exit(1)
            });
        }
        _ => {
            eprintln!("ERROR: unknown subcommand: {subcommand}")
        }
    }
}

fn main2() -> io::Result<()> {
    //benchmark
    let start = Instant::now();
    //-------------------------//

    //--------------------------//
    //benchmark
    let duration = start.elapsed().as_millis();
    println!("benchmark: {duration} ms");
    Ok(())
}
