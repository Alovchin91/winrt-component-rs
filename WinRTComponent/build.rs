use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/component.idl");

    let crate_path: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    let tools_path = crate_path.parent().unwrap().join("tools");

    println!("cargo:warning=Installing NuGet packages. This might take a while...");

    Command::new(tools_path.join("nuget.exe"))
        .current_dir(&tools_path)
        .args([
            "install", "Microsoft.Windows.SDK.CPP",
            "-Version", "10.0.22000.196",
            "-DirectDownload",
            "-DependencyVersion", "Ignore",
            "-NoCache",
            "-NonInteractive",
            "-OutputDirectory", "packages",
        ])
        .status()
        .unwrap();

    let sdk_dir = tools_path.join(r"packages\Microsoft.Windows.SDK.CPP.10.0.22000.196");

    let midl_path = sdk_dir.join(r"c\bin\10.0.22000.0\x64\midl.exe");

    let metadata_path = sdk_dir.join(r"c\References\10.0.22000.0");
    let foundation_contract = metadata_path.join(r"Windows.Foundation.FoundationContract.winmd");
    let universal_contract = metadata_path.join(r"Windows.Foundation.UniversalApiContract.winmd");

    let cl_tool = cc::windows_registry::find_tool(&env::var("TARGET").unwrap(), "cl.exe").unwrap();

    let out_winmd = crate_path.join(r".windows\winmd\RustComponent.winmd");
    std::fs::create_dir_all(out_winmd.parent().unwrap()).unwrap();

    let status = Command::new(midl_path)
        .arg("/winrt")
        .arg("/metadata_dir").arg(metadata_path)
        .arg("/h").arg("nul")
        .arg("/nomidl")
        .arg("/reference").arg(foundation_contract)
        .arg("/reference").arg(universal_contract)
        .arg("/winmd").arg(out_winmd)
        .arg(r"src\component.idl")
        .current_dir(&crate_path)
        .env("PATH", cl_tool.path().parent().unwrap())
        .env("INCLUDE", sdk_dir.join(r"c\Include\10.0.22000.0\winrt"))
        .status()
        .unwrap();

    if !status.success() {
        panic!("midl.exe command failed with exit code {}", status);
    }

    let gen = windows_bindgen::Gen::default();

    let mut res = "#![allow(non_snake_case)] ".to_string();
    res += &windows_bindgen::gen_type("RustComponent.ISample", &gen);
    res += &windows_bindgen::gen_type("RustComponent.ISampleFactory", &gen);

    let gen_path = crate_path.join(r"src\component.rs");

    std::fs::write(&gen_path, res).unwrap();

    Command::new("rustfmt").arg(&gen_path).status().unwrap();
}
