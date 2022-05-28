use std::process::Command;
#[cfg(windows)] use winres::WindowsResource;

fn main() {
  if cfg!(target_os = "windows") {
    Command::new("C:/Github/sciter-js-sdk/bin/windows/packfolder.exe")
      .args(&["sciter", "target/assets.rc", "-binary"])
      .output()
      .expect("Unable to run packfolder.exe!");
      WindowsResource::new()
        .set_icon("icon.ico")
        .compile()
        .expect("Unable to set the icon!");
    }
  }