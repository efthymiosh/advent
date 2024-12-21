#![allow(dead_code)]

pub mod datastructs;
pub mod algorithms;
pub mod debug;
pub mod grid;
pub mod math;
pub mod parse;

#[macro_export]
macro_rules! main {
    ( $pt1:expr, $pt2:expr ) => {
        use clap::Parser;

        #[derive(Parser, Debug)]
        #[command(version)]
        struct Args {
            /// Part to run
            #[arg(short, long)]
            part: u8,

            /// Datafile to use
            #[arg(short, long)]
            data: Option<String>,
        }

        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let args = Args::parse();
            let file = args.data.unwrap_or(
                std::file!()
                    .to_string()
                    .replace("src/bin/", "data/")
                    .replace(".rs", ".txt"),
            );

            match args.part {
                1 => $pt1(file),
                2 => $pt2(file),
                _ => {
                    panic!("AoC days only have two parts!")
                }
            }
        }
    };
}
