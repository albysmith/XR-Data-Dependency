use include_dir::{include_dir, Dir};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
#[macro_use]
extern crate serde;

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    output_path: String,
    input_path: String,
    extract: bool,
}

#[derive(Debug, Default)]
struct Folder {
    outpath: String,
    catpath: String,
    // mod_id: String,
    // mod_name: String,
    error: String,
}

fn main() {
    const MODS_DIR: Dir = include_dir!("mod_files");

    let config_file =
        &fs::read_to_string(&env::current_dir().expect("current dir").join("config.toml"));
    if let Ok(i) = config_file {
        let test: std::result::Result<Config, toml::de::Error> = toml::from_str(i);
        if let Ok(j) = test {
            let config: Config = j;
            let extract_flag = &config.extract;

            if &config.output_path.chars().count() > &75 {
                println!("Error: X4 Extensions path too long; must be less than 75 characters.  Path: {}", &config.output_path);
                pause();
                panic!();
            }

            if Path::new(&config.output_path).exists() {
                for folder in MODS_DIR.dirs() {
                    println!("Writing module files: {:?}", folder.path());
                    comp_mod_files(folder, Path::new(&config.output_path));
                }

                if extract_flag == &true {
                    let folder_vec = xr_data_values(&config);
                    for folder in folder_vec {
                        if Path::new(&folder.catpath).exists() {
                            extract_cats(folder)
                        } else {
                            println!("Cannot find folder: {}.  Check config.toml to ensure correct paths, and double-check that you have both DLCs installed.", &folder.error);
                            pause();
                            break
                        }
                    }
                    println!("Extraction Complete!");
                    pause();
                }
            } else {
                println!("Cannot find folder: X4 Extensions.  Check config.toml to ensure correct paths!");
                pause();
            }
        } else {
            println!("Error: config.toml is not valid toml, or your path includes \\ slashes. Check file contents; only / slashes are supported.");
            pause();
        }
    } else {
        println!("Error: config.toml not found. Make sure it is in the same folder as the xreborn_ship_converter.exe");
        pause();
    }
}

fn comp_mod_files(folder: &Dir, out_path: &Path) {
    let filepath = Path::new(out_path).join(folder.path());
    if !filepath.exists() {
        fs::create_dir_all(filepath).expect("making mod folders");
    }
    for file in folder.files() {
        let path = Path::new(out_path).join(file.path());
        let mut content_file = File::create(path).expect("creating mod file");
        content_file
            .write_all(&file.contents())
            .expect("writing mod file");
    }
    if folder.dirs().len() >= 1 {
        for dir in folder.dirs() {
            comp_mod_files(dir, out_path)
        }
    }
}

fn pause() {
    // creates pause in terminal
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn extract_cats(folder: Folder) {
    let outpath = Path::new(&folder.outpath);
    for entry in fs::read_dir(&folder.catpath).expect("read_dir") {
        let file = entry.expect("unwrapping entry");
        let catfile_path = file.path();
        // only open .cat files that are not sig files
        if !catfile_path.is_dir()
            && catfile_path.extension().expect("unwrapping extension") == "cat"
            && !catfile_path
                .file_stem()
                .expect("unwrapping stem")
                .to_str()
                .expect("checking for sig")
                .contains("sig")
        {
            println!("{:?}", catfile_path);
            // find and open matching .dat file
            let datfile_path = catfile_path.with_extension("dat");
            let mut datfile = &File::open(datfile_path).unwrap();
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
                // only copying ship data folders and ANI files

                // extract only what we want
                if (path.contains("assets/units/")
                    && (path.contains("data") || path.contains("DATA")))
                    || (path.contains("EngineSystems")
                        && (path.contains("data") || path.contains("DATA")))
                    || (path.contains("StorageModules")
                        && (path.contains("data") || path.contains("DATA")))
                {
                    // println!("{}", path);
                    // create missing directory folders (will skip automatically if they exist)
                    let filepath =
                        outpath.join((Path::new(&path)).parent().expect("opening folder"));
                    if !filepath.exists() {
                        fs::create_dir_all(filepath).expect("making folders");
                    }
                    // read correct amt of bytes from .dat file and copy to writer
                    let bytes_to_read = size.parse::<u64>().expect("parsing buffer");
                    let mut buf = vec![];
                    let mut chunk = datfile.take(bytes_to_read);
                    chunk.read_to_end(&mut buf).expect("Didn't read enough");
                    let mut reader: &[u8] = &buf;
                    let mut writer: Vec<u8> = vec![];
                    io::copy(&mut reader, &mut writer).expect("shut up");
                    // write the new files
                    let out_file_path = outpath.join(&path);
                    let mut outputfile = File::create(out_file_path).expect("something");
                    outputfile.write_all(&writer).expect("else");

                // skip everything else
                } else {
                    let bytes_to_read = size.parse::<u64>().expect("parsing buffer");
                    datfile
                        .seek(SeekFrom::Current(bytes_to_read as i64))
                        .expect("seek");
                }
            }
        }
    }
}

fn xr_data_values(config: &Config) -> Vec<Folder> {
    // set values needed for mod folders and error names
    let x_rebirth: Folder = Folder {
        outpath: format!("{}/xre", &config.output_path),
        catpath: (&config.input_path).to_string(),
        // mod_id: "xr_data_dependency".to_string(),
        // mod_name: "XR Data Dependency".to_string(),
        error: "X:Rebirth".to_string(),
    };
    let dlc_home: Folder = Folder {
        outpath: format!("{}/xrt", &config.output_path),
        catpath: format!("{}/extensions/ego_dlc_teladi_outpost", &config.input_path),
        // mod_id: "xr_data_dependency_teladioutpost".to_string(),
        // mod_name: "XR Data Dependency - Teladi Outpost".to_string(),
        error: "Teladi Outpost (DLC)".to_string(),
    };
    let dlc_teladi: Folder = Folder {
        outpath: format!("{}/xrh", &config.output_path),
        catpath: format!("{}/extensions/ego_dlc_2", &config.input_path),
        // mod_id: "xr_data_dependency_homeoflight".to_string(),
        // mod_name: "XR Data Dependency - Home of Light".to_string(),
        error: "Home of Light (DLC)".to_string(),
    };
    let folder_vec = vec![x_rebirth, dlc_home, dlc_teladi];
    folder_vec
}

