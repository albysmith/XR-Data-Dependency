use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;
#[macro_use]
extern crate serde;

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    output_path: String,
    input_path: String,
}

#[derive(Debug, Default)]
struct Folder {
    outpath: String,
    catpath: String,
    mod_id: String,
    mod_name: String,
}

fn main() {

// Make the tool create separate mods for each dlc folder

    // for non-release testing ONLY
    let config: Config = toml::from_str(
        &fs::read_to_string("E:/Rust/Projects/cat_extractor/target/release files/config.toml").expect("read config")).expect("toml parsing");
    
    // for release version
    // read config.toml in current directory
    // let config: Config = toml::from_str(
    //     &fs::read_to_string(&env::current_dir().expect("current dir").join("config.toml"))
    //         .expect("read config")
    // )
    // .expect("toml parsing");

    // for ship mod extraction only
    // let x_rebirth: Folder = Folder {outpath: format!("{}/xr_data_dependency", &config.output_path), catpath: (&config.input_path).to_string(), mod_id: "xr_data_dependency".to_string(), mod_name: "XR Data Dependency".to_string()} ;
    // let dlc_home: Folder = Folder {outpath: format!("{}/xr_data_dependency_teladioutpost", &config.output_path), catpath: format!("{}/extensions/ego_dlc_teladi_outpost", &config.input_path), mod_id: "xr_data_dependency_teladioutpost".to_string(), mod_name: "XR Data Dependency - Teladi Outpost".to_string()} ;
    // let dlc_teladi: Folder = Folder {outpath: format!("{}/xr_data_dependency_homeoflight", &config.output_path), catpath: format!("{}/extensions/ego_dlc_2", &config.input_path), mod_id: "xr_data_dependency_homeoflight".to_string(), mod_name: "XR Data Dependency - Home of Light".to_string()} ;
    // end

    // for total extraction 
    let x_rebirth: Folder = Folder {outpath: format!("{}/xrebirth", &config.output_path), catpath: (&config.input_path).to_string(), mod_id: "xr_data_dependency".to_string(), mod_name: "XR Data Dependency".to_string()} ;
    let dlc_home: Folder = Folder {outpath: format!("{}/teladioutpost", &config.output_path), catpath: format!("{}/extensions/ego_dlc_teladi_outpost", &config.input_path), mod_id: "xr_data_dependency_teladioutpost".to_string(), mod_name: "XR Data Dependency - Teladi Outpost".to_string()} ;
    let dlc_teladi: Folder = Folder {outpath: format!("{}/homeoflight", &config.output_path), catpath: format!("{}/extensions/ego_dlc_2", &config.input_path), mod_id: "xr_data_dependency_homeoflight".to_string(), mod_name: "XR Data Dependency - Home of Light".to_string()} ;
    // end

    let folder_vec = vec![x_rebirth, dlc_home, dlc_teladi];
    for folder in folder_vec {
        if Path::new(&folder.catpath).exists() {
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

                        // for ship mod extraction only
                        // if path.contains("assets/units/")
                        //     && (path.contains("data") || path.contains("DATA"))
                        // {
                        //     println!("{}", path);
                        // end

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
                            let mut outputfile = File::create(out_file_path).unwrap();
                            outputfile.write_all(&writer).unwrap();

                        // for ship mod extraction only
                        // } else {
                        //     let bytes_to_read = size.parse::<u64>().expect("parsing buffer");
                        //     datfile.seek(SeekFrom::Current(bytes_to_read as i64)).expect("seek");
                        // }
                        // end

                        // break;
                    }
                    // break;
                }
            }
            // for ship mod extraction only
//             let content_string = format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>
// <content id=\"{}\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" version=\"1\" date=\"2019-10-31\" save=\"0\" enabled=\"1\">
//  <text language=\"7\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"33\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"34\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"39\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"44\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"49\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"55\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"81\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"82\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"86\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
//  <text language=\"88\" name=\"{}\" description=\"This mod will provide no in-game content without the use of other mods\" author=\"albysmith\" />
// </content>
// ", &folder.mod_id, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name, &folder.mod_name);
//             let mut content_file =
//                 File::create(outpath.join("content.xml")).expect("creating content file");
//             content_file
//                 .write_all(&content_string.as_bytes())
//                 .expect("writing content file");
            // end
            println!("Extraction Complete");
        }
    }
}
