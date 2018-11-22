use std::path::Path;
use std::process::Command;

pub fn md2pdf(input_file: &Path, home_dir: &Path) -> i32 {
    let mut tmp_tex_file = input_file.to_path_buf();
    tmp_tex_file.set_extension("tex");

    let mut config_home = home_dir.to_path_buf();
    config_home.push(".config");
    config_home.push("pandoc");

    let status = Command::new("pandoc")
        .args(&[
            input_file.to_str().unwrap(),
            format!("--output={}", tmp_tex_file.to_str().unwrap()).as_str(),
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
                "--filter={}/pandoc-minted.py",
                config_home.to_str().unwrap()
            )
            .as_str(),
            "--toc",
            "--number-sections",
        ])
        .status()
        .expect("failed to pandoc");
    if !status.success() {
        return status.code().unwrap();
    }

    let _ = Command::new("latexmk")
        .args(&[
            "-xelatex",
            "-shell-escape",
            "-interaction=nonstopmode",
            "-f",
            tmp_tex_file.to_str().unwrap(),
        ])
        .status()
        .expect("failed to latexmk -xelatex");

    let status = Command::new("latexmk")
        .arg("-c")
        .status()
        .expect("failed to latexmk -c");
    status.code().unwrap()
}
