use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::time::Instant;
use xml::reader::{EventReader, XmlEvent};

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
        }
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    //benchmark
    let start = Instant::now();
    //-------------------------//

    // let all_documents = HashMap::<Path, HashMap<String, usize>>::new();

    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(dir_path)?;

    for file in dir {
        let file_path = file?.path();
        let content = read_entire_xml_file(&file_path)?;
        println!("{file_path:?} -> {size}", size = content.len());
    }
    let duration = start.elapsed();

    //--------------------------//
    //benchmark
    println!(
        "The program took {duration} secs",
        duration = duration.as_secs()
    );
    Ok(())
}
