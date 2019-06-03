use cfg_if::cfg_if;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use url::Url;

/// Information about a Slang release zip
struct SlangReleaseInfo {
    url: &'static str,
    relative_path_to_binaries: &'static str,
    static_libs: &'static [&'static str],
}

cfg_if! {
    if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
        const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
            url: "https://github.com/shader-slang/slang/releases/download/v0.12.6/slang-0.12.6-win32.zip",
            relative_path_to_binaries: "bin/windows-x86/release/",
            static_libs: &["slang"],
        };
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
            url: "https://github.com/shader-slang/slang/releases/download/v0.12.6/slang-0.12.6-win64.zip",
            relative_path_to_binaries: "bin/windows-x64/release/",
            static_libs: &["slang"],
        };
    } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
            url: "https://github.com/shader-slang/slang/releases/download/v0.12.6/slang-0.12.6-linux-x86_64.zip",
            relative_path_to_binaries: "bin/linux-x64/release/",
            static_libs: &[],
        };
    } else {
        compile_error!("No official release for the current platform! See: https://github.com/shader-slang/slang/releases/tag/v0.12.6");
    }
}

fn main() {
    // TODO: use `target_family`?
    let release_zip_url = Url::parse(SLANG_RELEASE.url).expect("hard-coded url is invalid");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let download_dir = &out_dir;

    // download zip file
    let release_file_name = release_zip_url.path_segments().unwrap().last().unwrap();
    let download_destination_path = download_dir.join(release_file_name);
    if !download_destination_path.exists() {
        download(&release_zip_url, &download_destination_path).unwrap();
    }

    // extract zip file
    let extract_dir = download_dir.join(release_file_name.replace(".zip", ""));
    if !extract_dir.exists() {
        let file = fs::File::open(&download_destination_path).unwrap();
        let archive = zip::ZipArchive::new(file).unwrap();
        extract(archive, &extract_dir).unwrap();
    }

    generate_bindings(&extract_dir);

    // emit cargo metadata
    {
        let slang_binaries_path = extract_dir.join(SLANG_RELEASE.relative_path_to_binaries);
        println!("cargo:rustc-link-search={}", slang_binaries_path.display());

        for static_lib in SLANG_RELEASE.static_libs {
            println!("cargo:rustc-link-lib=static={}", static_lib);
        }
    }
}

fn download(url: &url::Url, download_destination_path: &Path) -> Result<(), curl::Error> {
    let f = File::create(download_destination_path).unwrap();
    let mut writer = BufWriter::new(f);
    let mut easy = curl::easy::Easy::new();
    easy.url(url.as_str())?;
    easy.follow_location(true)?;
    easy.write_function(move |data| Ok(writer.write(data).unwrap()))?;
    easy.perform()?;

    let response_code = easy.response_code()?;
    if response_code != 200 {
        panic!("Unexpected response code {} for {}", response_code, url);
    }

    Ok(())
}

fn extract<R: Read + io::Seek>(
    mut archive: zip::ZipArchive<R>,
    destination: &Path,
) -> io::Result<()> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = destination.join(file.sanitized_name());

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn generate_bindings(slang_dir: &Path) {
    let bindings = bindgen::Builder::default()
        .clang_args(vec![
            // bindgen doesn't assume *.h files are c++
            "-x",
            "c++",
            "-std=c++11",
        ])
        .header(slang_dir.join("slang.h").to_string_lossy())
        .whitelist_type("slang_.*")
        .whitelist_type("I?Slang[A-Z].*")
        .whitelist_var("SLANG_[A-Z].*")
        .whitelist_function("sp[A-Z].*")
        .generate_comments(true)
        .layout_tests(true)
        .generate()
        .expect("Could not generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write binding file");
}
