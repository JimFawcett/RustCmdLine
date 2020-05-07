/////////////////////////////////////////////////////////////
// rust_cmd_line::test1.rs                                 //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 Apr 2020  //
/////////////////////////////////////////////////////////////

use rust_cmd_line::*;

fn main() {

    print!("\n  Command line arguments:");
    show_cmd_line();

    let mut parser = CmdLineParse::new();
    // let mut parser = CmdLineParse::default();
    print!("\n  {}\n",parser.help());
    parser.parse();
    print!("\n  path     = {:?}", parser.path());
    print!("\n  abspath  = {:?}", parser.abs_path());
    let new_path = "C:/github/foo";
    parser.set_path(new_path);
    print!("\n  setting path to {:?}", new_path);
    print!("\n  path     = {:?}", parser.path());
    let patts = parser.patterns();
    print!("\n  patts    = {:?}", patts);
    let opts = parser.options();
    print!("\n  opts     = {:?}", opts);

    print!("\n\n  That's all Folks!\n\n")
}