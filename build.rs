fn main() {
    for_minifb();
}

fn for_minifb() {}
// fn for_sdl2() {
//     let target = env::var("TARGET").unwrap();
//
//     // for windows
//     if target.contains("pc-windows") {
//         let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
//         let mut lib_dir = manifest_dir.clone();
//         let mut dll_dir = manifest_dir.clone();
//
//         lib_dir.push("ci");
//         dll_dir.push("ci");
//
//         if target.contains("msvc") {
//             lib_dir.push("msvc");
//             dll_dir.push("msvc");
//         } else {
//             lib_dir.push("gnu-mingw");
//             dll_dir.push("gnu-mingw");
//         }
//         lib_dir.push("lib");
//         dll_dir.push("dll");
//         if target.contains("x86_64") {
//             lib_dir.push("64");
//             dll_dir.push("64");
//         } else {
//             lib_dir.push("32");
//             dll_dir.push("32");
//         }
//         println!("cargo:rustc-link-search=all={}", lib_dir.display());
//
//         if let Ok(dll) = std::fs::read_dir(dll_dir) {
//             for entry in dll {
//                 let entry_path = entry.expect("Invalid fs entry").path();
//                 let file_name_result = entry_path.file_name();
//                 let mut new_file_path = manifest_dir.clone();
//                 if let Some(file_name) = file_name_result {
//                     let file_name = file_name.to_str().unwrap();
//                     if file_name.ends_with(".dll") || file_name.ends_with(".lib") {
//                         new_file_path.push(file_name);
//                         std::fs::copy(&entry_path, new_file_path.as_path())
//                             .expect("Can't copy from DLL dir");
//                     } else {
//                     }
//                 } else {
//                 }
//             }
//         } else {
//         }
//     } else {
//         // for linux && mac
//     }
// }
