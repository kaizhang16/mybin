use std::path::Path;
use std::process::Command;

pub fn md2pdf(input_file: &Path, home_dir: &Path) -> i32 {
    let mut output_file = input_file.to_path_buf();
    output_file.set_extension("pdf");

    let mut config_home = home_dir.to_path_buf();
    config_home.push(".config");
    config_home.push("pandoc");

    let status = Command::new("pandoc")
        .args(&[
            "--from=markdown",
            "--to=latex",
            "--pdf-engine=xelatex",
            format!("--data-dir={}", config_home.to_str().unwrap()).as_str(),
            "--template=default-zh.latex",
            "--variable=lang:zh",
            "--variable=papersize:a4",
            "--variable=documentclass:article",
            "--variable=linestretch=1.4",
            "--variable=CJKmainfont:Source Han Serif CN",
            "--variable=CJKoptions:BoldFont=Source Han Sans CN",
            "--variable=geometry:total={16cm, 24cm}",
            "--variable=toc-title:目录",
            "--variable=toccolor:Magenta",
            "--variable=urlcolor:Magenta",
            "--variable=citecolor:YellowOrange",
            "--filter=pandoc-crossref",
            format!(
                "--metadata=crossrefYaml:{}/crossref-zh.yaml",
                config_home.to_str().unwrap()
            )
            .as_str(),
            "--filter=pandoc-citeproc",
            format!(
                "--highlight-style={}/pygments.theme",
                config_home.to_str().unwrap()
            )
            .as_str(),
            "--toc",
            format!("--output={}", output_file.to_str().unwrap()).as_str(),
            input_file.to_str().unwrap(),
        ])
        .status()
        .expect("failed to pandoc");
    status.code().unwrap()
}
