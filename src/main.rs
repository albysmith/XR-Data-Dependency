use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
            // find and open matching .dat file
            let datfile_path = catfile_path.with_extension("dat");
            let datfile = &File::open(datfile_path).unwrap();
            // read .cat file
            let catfile_string =
                fs::read_to_string(&catfile_path).expect("unwrapping catfile_string");
            for line in catfile_string.lines() {
                // split at whitespace from the right edge (reverse) to capture size and other non-path values
                let mut iter = line.split_whitespace().rev();
                let hash = iter.next().expect("hash");
                let modified_epoch = iter.next().expect("epoch");
                let size = iter.next().expect("size");
                // remove non-path values to get path
                let pattern = format!(" {} {} {}", size, modified_epoch, hash);
                let path = line.replace(pattern.as_str(), "");
                // create missing directory folders (will skip automatically if they exist)
                let filepath = outpath.join((Path::new(&path)).parent().expect("no folder listed"));
                fs::create_dir_all(filepath).expect("making folders");
                // read correct amt of bytes from .dat file and copy to writer
                let bytes_to_read = size.parse::<u64>().expect("parsing buffer");
                let mut buf = vec![];
                let mut chunk = datfile.take(bytes_to_read);
                chunk.read_to_end(&mut buf).expect("Didn't read enough");
                let mut reader: &[u8] = &buf;
                let mut writer: Vec<u8> = vec![];
                io::copy(&mut reader, &mut writer).expect("shut up");
                // write the new files
                let out_path = outpath.join(&path);
                let mut outputfile = File::create(out_path).unwrap();
                outputfile.write_all(&writer).unwrap();
                // break;
            }
            // break;
        }
    }
}
