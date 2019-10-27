// extern crate rayon;

// use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
// use std::io::BufReader;
// use rayon::prelude::*;

fn main() {
    // outpath = place to output the extracted files
    let outpath = Path::new("E:/Rust/Projects/Output Files/Cat_Extractions");
    // catpath = XR main directory
    let catpath = Path::new("E:/Games/Steam/steamapps/common/X4 Foundations");

    // read the files in the catpath folder and capture the ones with .cat extensions, ignoring sig files
    for entry in fs::read_dir(catpath).expect("read_dir") {
        let file = entry.expect("unwrapping entry");
        let catfile_path = file.path();
        if !catfile_path.is_dir()
            && catfile_path.extension().expect("unwrapping extension") == "cat"
            && !catfile_path
                .file_stem()
                .expect("unwrapping stem")
                .to_str()
                .expect("checking for sig")
                .contains("sig")
        {
            println!("{:#?}", catfile_path);
            let datfile_path = catfile_path.with_extension("dat");
            let datfile = &File::open(datfile_path).unwrap();
            // let mut reader = BufReader::new(datfile);

            let catfile_string =
                fs::read_to_string(&catfile_path).expect("unwrapping catfile_string");
            for line in catfile_string.lines() {
                let mut iter = line.split_whitespace().rev();
                let hash = iter.next().expect("hash");
                let modified_epoch = iter.next().expect("epoch");
                let size = iter.next().expect("size");
                let pattern = format!(" {} {} {}", size, modified_epoch, hash);
                let path = line.replace(pattern.as_str(), "");

                let filepath = outpath.join((Path::new(&path)).parent().expect("no folder listed"));
                fs::create_dir_all(filepath).expect("making folders");
                // let mut file_string = vec![];

                // let reader = BufReader::with_capacity(reference.size.parse::<usize>().expect("parsing size"), datfile);
                let bytes_to_read = size.parse::<u64>().expect("parsing buffer");
                let mut buf = vec![];
                let mut chunk = datfile.take(bytes_to_read);

                let mut reader: &[u8] = &buf;
                let mut writer: Vec<u8> = vec![];

                io::copy(&mut reader, &mut writer).expect("shut up");

                chunk.read_to_end(&mut buf).expect("Didn't read enough");
                // for byte in &mut reader.bytes() {
                //     file_string.push(byte.unwrap());
                //     if file_string.len() == reference.size.parse::<usize>().expect("parsing size") {
                //         break;
                //     }
                // }
                // read up to 10 bytes
                // let n = datfile.read(&mut buffer[..]);
                // println!("{:#?}", reader);
                let out_path = outpath.join(&path);
                let mut outputfile = File::create(out_path).unwrap();
                outputfile.write_all(&writer).unwrap();
                // outputfile.write_all(&file_string).unwrap();
                // break;
            }
            // break;
        }
    }
}
