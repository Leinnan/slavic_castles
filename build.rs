use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=card_project.svg");
    {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={}", &git_hash[..7]);
    }
    {
        let output = Command::new("git")
            .args(["log", "-1", "--date=format:%Y/%m/%d %T", "--format=%ad"])
            .output()
            .unwrap();
        let git_hash = String::from_utf8(output.stdout).unwrap();
        println!("cargo:rustc-env=GIT_DATE={}", git_hash);
    }
}
