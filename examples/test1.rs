// rust_cmd_line::test1.rs

#[allow(unused_imports)]
use rust_cmd_line::*;

fn main() {

    show_cmd_line();
    let parser = rust_cmd_line::CmdLineParse::new();
    parser.parse();
}