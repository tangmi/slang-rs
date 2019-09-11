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
            url: "https://github.com/shader-slang/slang/releases/download/v0.12.13/slang-0.12.13-win32.zip",
            relative_path_to_binaries: "bin/windows-x86/release/",
            static_libs: &["slang"],
        };
    } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
        const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
            url: "https://github.com/shader-slang/slang/releases/download/v0.12.13/slang-0.12.13-win64.zip",
            relative_path_to_binaries: "bin/windows-x64/release/",
            static_libs: &["slang"],
        };
    } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        const SLANG_RELEASE: SlangReleaseInfo = SlangReleaseInfo {
            url: "https://github.com/shader-slang/slang/releases/download/v0.12.13/slang-0.12.13-linux-x86_64.zip",
            relative_path_to_binaries: "bin/linux-x64/release/",
            static_libs: &[],
        };
    } else {
        compile_error!("No official release for the current platform! See: https://github.com/shader-slang/slang/releases/tag/v0.12.13");
    }
}

const SLANG_REPO_URL: &str = "https://github.com/shader-slang/slang.git";
// NOTE: This commit is one after the release, but includes `slang-binaries` as a submodule.
const SLANG_RELEASE_REV: &str = "4fc07614d6407e49a0c34e7483d410153c0b269a";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // The directory with the header files (slang.h, etc.)
    let slang_dir = if cfg!(all(
        not(feature = "from-source"),
        any(
            all(target_os = "windows", target_arch = "x86"),
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "x86_64")
        )
    )) {
        // This branch will try to download and unzip the official release.

        // TODO: use `target_family`?
        let release_zip_url = Url::parse(SLANG_RELEASE.url).expect("hard-coded url is invalid");

        let download_dir = &out_dir;

        // download zip file
        let release_file_name = release_zip_url.path_segments().unwrap().last().unwrap();
        let download_destination_path = download_dir.join(release_file_name);
        if !download_destination_path.exists() {
            // todo delete everything and retry if slang.h/binaries don't exist?
            download(&release_zip_url, &download_destination_path).unwrap();
        }

        // extract zip file
        let extract_dir = download_dir.join(release_file_name.replace(".zip", ""));
        if !extract_dir.exists() {
            let file = fs::File::open(&download_destination_path).unwrap();
            let archive = zip::ZipArchive::new(file).unwrap();
            extract(archive, &extract_dir).unwrap();
        }

        // emit cargo metadata
        {
            let slang_binaries_path = extract_dir.join(SLANG_RELEASE.relative_path_to_binaries);
            println!("cargo:rustc-link-search={}", slang_binaries_path.display());

            for static_lib in SLANG_RELEASE.static_libs {
                println!("cargo:rustc-link-lib=static={}", static_lib);
            }
        }

        extract_dir
    } else {
        // This branch will try to download and build source revision associated with the official release.

        // todo feature="git2" doesn't work, figure out a better way to gate this feature
        if cfg!(feature = "git2") {
            use git2::Error;
            use git2::Oid;
            use git2::Repository;
            use git2::ResetType;

            fn clone_repo_and_tools(repo_target_dir: &Path) -> Result<(), Error> {
                fn update_submodules_recusive(repo: &Repository) -> Result<(), Error> {
                    // From git2-rs's private Repository::update_submodules.
                    fn add_subrepos(
                        repo: &Repository,
                        list: &mut Vec<Repository>,
                    ) -> Result<(), Error> {
                        for mut subm in repo.submodules()? {
                            subm.update(true, None)?;
                            list.push(subm.open()?);
                        }
                        Ok(())
                    }

                    let mut repos = Vec::new();
                    add_subrepos(repo, &mut repos)?;
                    while let Some(sub_repo) = repos.pop() {
                        add_subrepos(&sub_repo, &mut repos)?;
                    }

                    Ok(())
                }

                if repo_target_dir.exists() {
                    if repo_target_dir
                        .join("external/slang-binaries/premake")
                        .exists()
                    {
                        // We're assuming if the target dir exists, it is
                        return Ok(());
                    } else {
                        // delete it and try cloning again!
                        std::fs::remove_dir_all(repo_target_dir).unwrap();
                    }
                }

                let repo = Repository::clone_recurse(SLANG_REPO_URL, repo_target_dir)?;
                let rev = repo.find_commit(Oid::from_str(SLANG_RELEASE_REV)?)?;
                repo.reset(rev.as_object(), ResetType::Hard, None)?;
                update_submodules_recusive(&repo)?;

                Ok(())
            }

            let slang_dir = out_dir.join("slang");
            clone_repo_and_tools(&slang_dir).unwrap();

            if cfg!(target_os = "windows") {
                // todo get actualy output/success
                std::process::Command::new("msbuild")
                    .arg("/p:Configuration=Release")
                    .arg("/p:PlatformToolset=v142")
                    .current_dir(&slang_dir)
                    .output()
                    .expect("failed to run MSBuild");
            } else {
                // Assume we're in a unix-like system with `make`.

                cfg_if! {
                    if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
                        // Note: we don't actually need this! We're just using MSBuild on Windows.
                        const PREMAKE5_RELATIVE_PATH: &str = "external/slang-binaries/premake/premake-5.0.0-alpha13/bin/windows-x64/premake5.exe";
                    } else if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                        const PREMAKE5_RELATIVE_PATH: &str = "external/slang-binaries/premake/premake-5.0.0-alpha13/bin/linux-64/premake5";
                    } else if #[cfg(target_os = "macos")] {
                        const PREMAKE5_RELATIVE_PATH: &str = "external/slang-binaries/premake/premake-5.0.0-alpha13/bin/osx/premake5";
                    } else {
                        compile_error!("No build tools available for the current platform!");
                    }
                }

                let premake = slang_dir.join(PREMAKE5_RELATIVE_PATH);

                // todo get actualy output/success
                std::process::Command::new(premake)
                    .arg("gmake")
                    .current_dir(&slang_dir)
                    .output()
                    .expect("failed to run premake");

                // todo get actualy output/success
                std::process::Command::new("make")
                    .arg("config=release_x64")
                    .current_dir(&slang_dir)
                    .output()
                    .expect("failed to run make");
            }

            // todo get output files

            // todo emit cargo metadata
            {
                let slang_binaries_path = slang_dir.join("bin/windows-x64/release");
                println!("cargo:rustc-link-search={}", slang_binaries_path.display());

                let static_libs = &["slang"];
                for static_lib in static_libs {
                    println!("cargo:rustc-link-lib=static={}", static_lib);
                }
            }

            slang_dir
        } else {
            unreachable!("Something in the Cargo.toml is misconfigured?");
        }
    };

    generate_bindings(&slang_dir);
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
