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
// value(&self, char opt) -> &str
// options(&self) -> &HashMap<char, String>
// - defaults are created with default_options():
//   - /P "."    - root search path is current directory
//   - /s "true" - recurse
//   - /r "."    - match all text
//   - /H "true" - hide unused directories
//   - no patterns are equivalent to all files
// path(&self) -> String
// abs_path(&self) -> String
// set_path(&self, p:&str)
// patterns(&self) -> &Vec<String>
// add_pattern(&mut self, p:&str) -> &mut self
// set_regex(&mut self, re:&str)
// get_regex(&self) -> &str
// help(&self) -> String
// replace_help(&mut self, &str) -> String

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
            opt_map: Options::default(),
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
    /*-- return path --*/
    pub fn path(&self) -> String {
        if self.contains_option('P') {
            self.opt_map[&'P'].clone()
        }
        else {
            ".".to_string()
        }
    }

    /*-- replace Win directory separator with Linux separator --*/
    fn replace_sep(path: &str) -> String {
        let rtn = path.to_string();
        rtn.replace("\\", "/")
    }
    /*-- convert relative to absolute path --*/
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
    /*-- set new root path --*/
    pub fn set_path(&mut self, p:&str) {
        self.opt_map.insert('P', p.to_string());
    }
    /*-- set new regex string for matching --*/
    pub fn set_regex(&mut self, re:&str) {
        self.opt_map.insert('r', re.to_string());
    }
    /*-- return current regex string --*/
    pub fn get_regex(&self) -> &str {
        let re_opt = self.opt_map.get(&'r');
        match re_opt {
            Some(value) => value,
            None => ".",
        }
    }
    /*-- commonly used default options --*/
    pub fn default_options(&mut self) {
        self.opt_map.insert('P', ".".to_string());     // root is curr dir
        self.opt_map.insert('s', "true".to_string());  // recurse
        self.opt_map.insert('r', ".".to_string());     // regex always matches
        self.opt_map.insert('H', "true".to_string());  // hide unused dirs
    }
    /*-- does options contain opt? --*/
    pub fn contains_option(&self, opt:char) -> bool {
        self.opt_map.contains_key(&opt)
    }
    /*-- insert if doesn't exist, else overwrite value --*/
    pub fn add_option(&mut self, o:char, v:&str) {
        self.opt_map.insert(o, v.to_string());
    }
    /*-- return option value --*/
    pub fn value(&self, opt:char) -> &str {
        &self.opt_map[&opt]
    }
    /*-- add file ext (with no *.) --*/
    pub fn add_pattern(&mut self, p:&str) -> &mut Self {
        let s = String::from(p);
        if !self.patterns.contains(&s) {
            self.patterns.push(s);
        }
        self
    }
    /*-- non-mutable reference to patterns --*/
    pub fn patterns(&self) -> &CmdLinePatterns {
        &self.patterns
    }
    /*-- non-mutable ref to options --*/
    pub fn options(& self) -> &Options {
        &self.opt_map
    }
    /*-- return help string --*/
    pub fn help(&self) -> &str {
        &self.help_str
    }
    /*-- replace help string --*/
    pub fn replace_help(&mut self, s:&str) {
        self.help_str = s.to_string();
    }
    /*-- parse command line arguments provided by env() --*/
    pub fn parse(&mut self) {
    
        let cl_args:Vec<String> = args().collect();
        let end = cl_args.len();
        for i in 1..end {
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
