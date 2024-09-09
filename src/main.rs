extern crate glob;
use argh::FromArgs;
use glob::glob_with;
use glob::MatchOptions;
use std::usize;

mod rscript;
use crate::rscript::run_rscript;

#[derive(FromArgs, PartialEq, Debug)]
/// CLI tool for running tests in R packages
struct Rt {
    #[argh(subcommand)]
    subcommand: Subcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    One(TestThatDir),
    Two(TestThatFile),
    Three(ListTestFiles),
    Four(ListVersions),
}

#[derive(PartialEq, Clone, Debug, FromArgs)]
#[argh(subcommand, name = "dir")]
/// Test an R package using testthat
struct TestThatDir {
    /// path to a package's directory
    #[argh(positional, default = r#"String::from(".")"#)]
    dir: String,
}

#[derive(PartialEq, Clone, Debug, FromArgs)]
#[argh(subcommand, name = "file")]
/// Test a single file using testthat
struct TestThatFile {
    /// path to a test file
    #[argh(positional, default = r#"String::from(".")"#)]
    file: String,
}

#[derive(PartialEq, Clone, Debug, FromArgs)]
#[argh(subcommand, name = "list")]
/// List test files in an R package
struct ListTestFiles {
    /// path to a test file
    #[argh(positional, default = r#"String::from(".")"#)]
    dir: String,
}

#[derive(PartialEq, Clone, Debug, FromArgs)]
#[argh(subcommand, name = "r-vers")]
/// List available versions of R
struct ListVersions {}

fn main() -> anyhow::Result<()> {
    let args: Rt = argh::from_env();
    match args.subcommand {
        Subcommands::One(cmd) => {
            let devtools_call = format!("devtools::test('{}')", cmd.dir);
            run_rscript(&devtools_call)?;
        }
        Subcommands::Two(cmd) => {
            let testthat_call =
                format!("devtools::load_all(); testthat::test_file('{}')", cmd.file);
            run_rscript(&testthat_call)?;
        }
        Subcommands::Three(cmd) => {
            let mut owned_string: String = "/tests/testthat/test-*.R".to_owned();
            owned_string.insert_str(0, &cmd.dir);
            // println!("{}", owned_string);

            let options = MatchOptions {
                case_sensitive: true,
                require_literal_separator: false,
                require_literal_leading_dot: false,
            };
            for entry in glob_with(&owned_string, options).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Subcommands::Four(_) => {
            let vers = rt_lib::RVersions::discover();
            match vers {
                Ok(v) => {
                    let mut vv = v
                        .versions
                        .into_iter()
                        .map(|v| v.version)
                        .collect::<Vec<_>>();
                    vv.sort();
                    // check which one is the default

                    let is_default = match v.default {
                        Some(v) => vv.iter().position(|vv| vv == &v.version),
                        None => None,
                        // no one will ever have this many R installations...
                        // simplifies the printing in the CLI
                    }
                    .unwrap_or(usize::MAX);

                    vv.into_iter().enumerate().for_each(|(i, v)| {
                        let def_msg = if i == is_default { "(default)" } else { "" };
                        println!("  R {v} {}", def_msg)
                    })
                }
                Err(_) => eprintln!("Unable to find any R installation in common locations."),
            }
        }
    }
    Ok(())
}
