use execute::Execute;
use std::process::{Command, Stdio};

// macro_rules is a macro that helps to display output in cargo build.
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

// main adds `run npm build` to the cargo build to build the web application.
fn main() {
    println!("cargo:rerun-if-changed=app/package.json");
    println!("cargo:rerun-if-changed=app/src");

    println!("cargo:warning=run `npm run build` from `app` before building the artifact.");
    println!("cargo:warning=if it fails, make sure you can run the command from manually!!!");
    println!("cargo:warning=...");

    let mut npm_command = Command::new("npm");
    npm_command.current_dir("./app");
    npm_command.arg("run").arg("build");

    npm_command.stdout(Stdio::piped());
    npm_command.stderr(Stdio::piped());

    let output = npm_command.execute_output().unwrap();

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            let x = String::from_utf8(output.stdout).unwrap();
            x.lines().for_each(|line| {
                p!("{line}");
            });
            return;
        }
        std::process::exit(exitcode::CONFIG);
    }
    std::process::exit(exitcode::CONFIG);
}
