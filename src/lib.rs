/////////////////////////////////////////////////////////////
// rust_cmd_line::lib.rs                                   //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 Apr 2020  //
/////////////////////////////////////////////////////////////

use std::env::{args};
use std::collections::HashMap;

/////////////////////////////////////////////////////////////
// sample command line
//-----------------------------------------------------------
// /P "." /p "rs,txt" /s [true] /r "abc" /h [true] /H [true]
/////////////////////////////////////////////////////////////

pub fn show_cmd_line() {
    print!("\n  ");
    for arg in args() {
        print!("{:?} ", arg)
    }
}

#[derive(Debug)]
pub struct CmdLineParse {
    opt_map : HashMap<char, String>,
    patterns : Vec<String>,
}
impl CmdLineParse {
    pub fn new() -> Self {
        Self {
            opt_map: HashMap::<char, String>::new(),
            patterns: Vec::<String>::new(),
        }
    }

    fn is_opt(&self, s:&str) -> bool {
        let bytes = s.as_bytes();
        bytes[0] as char == '/'
    }
    
    pub fn parse(&self) {
        let mut _opt_map:HashMap<char, String> = HashMap::new();
    
        let cl_args:Vec<String> = args().collect();
        let end = cl_args.len();
        print!("\n  number of cl args: {:?}",end);
        for i in 1..end {
            // print!("\n  line number: {}", i);
            // print!("\n  {:?} is opt {:?}", cl_args[i], self.is_opt(&cl_args[i]));
            if i < end - 2 {
                // print!(", next arg is {:?}", cl_args[i+1]);
            }
            if self.is_opt(&cl_args[i]) {
                let bytes = cl_args[i].as_bytes();
                let key = bytes[1] as char;
                if i < end - 1 {
                    if !self.is_opt(&cl_args[i+1]) {
                        _opt_map.insert(key,cl_args[i+1].to_string());
                    }
                    else {
                        _opt_map.insert(key, "true".to_string());
                    }
                }
                else {
                    _opt_map.insert(key, "true".to_string());
                }
            }
        }
        print!("\n\n  {:?}", _opt_map);
        let mut _patterns = Vec::<&str>::new();
        let patts = _opt_map.get(&'p');
        match patts {
            Some(v) => {
                print!("\n  p -> {:?}", v);
                let split_iter = v.split(",");
                _patterns = split_iter.collect::<Vec<&str>>();
                print!("\n  {:?}", _patterns);
            },
            None => print!("\n  key {:?} not in map", &'p'),
        }
        // print!("\n  p -> {:?}", patts);
    
        println!("\n\n  That's all Folks!");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cl_args() {
        let _mock_args = vec!["/P", ".", "/p", "rs,txt", "/s"];
        print!("\n  cl args: ");
        for arg in args() {
            print!("{:?} ", arg);
        }
        let parser = CmdLineParse::new();
        parser.parse();
        for arg in args() {
            let bytes = arg.as_bytes();
            if '/' == (bytes[0] as char) {
                assert!(parser.opt_map.contains_key(&(bytes[1] as char)));
            }
        }
    }
}
