/////////////////////////////////////////////////////////////
// rust_cmd_line::lib.rs                                   //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 Apr 2020  //
/////////////////////////////////////////////////////////////

use std::env::{args};
use std::collections::HashMap;
use std::fs::*;

/////////////////////////////////////////////////////////////
// sample command line with options
//-----------------------------------------------------------
// /P "." /p "rs,txt" /s [true] /r "abc" /h [true] /H [true]
//
// P - path in either absolute or relative form
// p - pattern, a file extension indicating file to process
// s - recurse directory tree rooted at P
// r - regular expression
// H - hide directories that don't contain any target files
// h - help: display this message
// custom option:
// /x [v] - x is application specific option which may
//          have value, v
// Note:
// Any attribute that has no value on command line will
// have value "true" in option map
/////////////////////////////////////////////////////////////
// CmdLineParse methods
//-----------------------------------------------------------
// new() -> CmdLineParse
// parse(&self)
// contains_option(&self, opt: char) -> bool
// value(&self, char opt) -> &String
// options(&self) -> &HashMap<char, String>
// patterns(&self) -> &Vec<String>
// add_pattern(&self, p:&str)

pub fn show_cmd_line() {
    print!("\n  {:?}\n  ", args().next());
    for arg in args().skip(1) {
        print!("{:?} ", arg)
    }
}

pub type Options = HashMap<char, String>;
pub type CmdLinePatterns = Vec<String>;

#[derive(Debug, Default)]
pub struct CmdLineParse {
    opt_map : Options,
    patterns : CmdLinePatterns,
    help_str : String,
}
impl CmdLineParse {
    pub fn new() -> Self {
        let help = CmdLineParse::help_txt();
        Self {
            opt_map: Options::new(),
            patterns: CmdLinePatterns::new(),
            help_str: help,
        }
    }
    /*-- private helper function --*/
    fn help_txt() -> String {
        let mut str =
        "\n  Help:\n  Options: /P . /p \"rs,txt\"".to_string();  
        str.push_str(" /s /r \"abc\" /H /h"); 
        str
    }
    /*-- private helper function --*/
    fn is_opt(&self, s:&str) -> bool {
        let bytes = s.as_bytes();
        bytes[0] as char == '/'
    }

    pub fn path(&self) -> String {
        if self.contains_option('P') {
            self.opt_map[&'P'].clone()
        }
        else {
            "".to_string()
        }
    }

    /*-- replace Win directory separator with Linux separator --*/
    fn replace_sep(path: &str) -> String {
        let rtn = path.to_string();
        rtn.replace("\\", "/")
    }

    pub fn abs_path(&self) -> String {
        let abs = std::path::PathBuf::from(self.path());
        let rslt = canonicalize(&abs);
        match rslt {
            Ok(path_buf) => {
                let ap = path_buf.to_string_lossy().to_string();
                let ap = CmdLineParse::replace_sep(&ap);
                let ap:String = ap.chars().skip(4).collect();
                ap
            }
            Err(error) => error.to_string()
        }
    }

    pub fn contains_option(&self, opt:char) -> bool {
        self.opt_map.contains_key(&opt)
    }

    pub fn value(&self, opt:char) -> &str {
        &self.opt_map[&opt]
    }

    pub fn add_pattern(&mut self, p:&str) {
        self.patterns.push(p.to_string());
    }

    pub fn patterns(&self) -> &CmdLinePatterns {
        &self.patterns
    }

    pub fn options(&self) -> &Options {
        &self.opt_map
    }

    pub fn help(&self) -> &str {
        &self.help_str
    }

    pub fn replace_help(&mut self, s:&str) {
        self.help_str = s.to_string();
    }
    
    pub fn parse(&mut self) {
    
        let cl_args:Vec<String> = args().collect();
        let end = cl_args.len();
        // print!("\n  number of cl args: {:?}",end);
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
                        self.opt_map.insert(key,cl_args[i+1].to_string());
                    }
                    else {
                        self.opt_map.insert(key, "true".to_string());
                    }
                }
                else {
                    self.opt_map.insert(key, "true".to_string());
                }
            }
        }
        /*-- build patterns --*/
        if self.contains_option('p') {
            let pat_str = self.value('p').to_string();
            let split_iter = pat_str.split(',');
            for patt in split_iter {
                self.add_pattern(patt);
            }
        }
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
        let mut parser = CmdLineParse::new();
        parser.parse();
        for arg in args() {
            let bytes = arg.as_bytes();
            if '/' == (bytes[0] as char) {
                assert!(parser.opt_map.contains_key(&(bytes[1] as char)));
            }
        }
    }
}
