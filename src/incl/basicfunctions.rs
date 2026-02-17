use colored::{ColoredString, CustomColor};

use crate::*;
use std::{char, time::{SystemTime, UNIX_EPOCH}};
pub struct Nstring {
}
pub fn nscriptfn_split(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    //let mut thisvar = NscriptVar::new("var");
        let mut delim = "".to_string();
        if  args.len() > 1{
            delim = storage.getargstring(&args[1], block).to_string();
        }
        let mut var = storage.getvar(&args[0], block);
        var.stringvec = Nstring::split(&var.stringdata,&delim);
        var.stringdata = "".to_string();
        return var;
}

pub fn nscriptfn_replace(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut nvar = storage.getvar(&args[0], block);
    if args.len() > 2{
        nvar.stringdata = Nstring::replace(&nvar.stringdata, &storage.getargstring(&args[1], block), &storage.getargstring(&args[2], block));
    }else{
        print("string::replace arguments missing, returing nothing","r");
    }
    return nvar;
}
pub fn nscriptfn_newvar(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) ->NscriptVar{
    return NscriptVar::new("v");
}
pub fn nscriptfn_replacebyref(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    if args.len() > 2{
        let mut refvar = storage.getvar(args[0], block);
        let value = Nstring::replace(&storage.getargstring(&args[0], block), &storage.getargstring(&args[1], block), &storage.getargstring(&args[2], block));
        refvar.stringdata = value.to_string();
        storage.setdefiningword(args[0], refvar, block);
    }else{
        print("string::replace arguments missing, returing nothing","r");
    }

    return NscriptVar::new("res");
}
pub fn nscriptfn_stringtoeval(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    NscriptVar::newstring("res",Nstring::stringtoeval(&storage.getargstring(&args[0], block)))
}
pub fn nscriptfn_len(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    NscriptVar::newstring("len", storage.getargstringvec(&args[0],block).len().to_string())
}
pub fn nscriptfn_instring(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() < 2 {
        neovar.stringdata = "false".to_string();
    }
    else{
        if Nstring::instring(&storage.getargstring(&args[0], block), &storage.getargstring(&args[1], block)) {
            neovar.stringdata = "true".to_string();
        }
        else{
            neovar.stringdata = "false".to_string();
        }
    }
    neovar
}
pub fn nscriptfn_stringbetween(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = storage.getvar(&args[0], block);
    if args.len() < 2 {
        neovar.stringdata = "".to_string();
    }
    else{
        neovar.stringdata = Nstring::stringbetween(&neovar.stringdata, &storage.getargstring(&args[1], block), &storage.getargstring(&args[2], block))

    }
    neovar
}

pub fn nscriptfn_trim(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    NscriptVar::newstring("trim",storage.getargstring(&args[0],block).trim().to_string())
}
pub fn nscriptfn_contains(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 1{
        let value = Nstring::instring(&storage.getargstring(&args[0],block), & storage.getargstring(&args[1],block) );
        neovar.stringdata = value.to_string();
    }else{
        print("string::contains arguments missing, returing nothing","r");
    }
    return neovar;
}

pub fn nscriptfn_join(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 0{
        let mut tojoin = "".to_string();
        if args.len() > 1 {
            tojoin = storage.getargstring(&args[1], block);
        }
        neovar.stringdata = storage.getargstringvec(&args[0], block).join(&tojoin);
    }
    else{
        print("join arguments missing, returing nothing","r");
    }
    return neovar;
}

pub fn nscriptfn_fromleft(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut nvar = storage.getvar(&args[0], block);
    if args.len() > 1{
         nvar.stringdata = Nstring::fromleft(&nvar.stringdata, storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0));
    }else{
        print("string::fromleft arguments missing, returing nothing","r");
    }
    return nvar;
}

pub fn nscriptfn_fromright(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut nvar = storage.getvar(&args[0], block);
    if args.len() > 1{
        nvar.stringdata = Nstring::fromright(&nvar.stringdata, storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0));
    }
    else{
        print("string::fromleft arguments missing, returing nothing","r");
    }
    return nvar;
}

pub fn nscriptfn_trimright(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    if args.len() > 1{
        return NscriptVar::newstring("res",Nstring::trimright(&storage.getargstring(&args[0], block), storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0)).to_string());
    }
    print("string::fromleft arguments missing, returing nothing","r");
    NscriptVar::new("result")
}
pub fn nscriptfn_trimleft(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    if args.len() > 1{
        return NscriptVar::newstring("res",Nstring::trimleft(&storage.getargstring(&args[0], block), storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0)).to_string());
    }
    print("string::fromleft arguments missing, returing nothing","r");
    NscriptVar::new("result")
}
pub fn nscriptfn_toupper(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    var.stringdata = var.stringdata.to_uppercase();
    var
}
pub fn nscriptfn_tolower(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    var.stringdata = var.stringdata.to_lowercase();
    var
}
impl Nstring {

    pub fn replace(s: &str, f: &str, r: &str) -> String {
        if f == "" || s == ""{
            return s.to_string();
        }
        // i know slaat nergens op.. :P
        return s.replace(f, r);
    }

    pub fn instring(s: &str, f: &str) -> bool {
        let  r: bool;
        match s.find(f) {
            Some(_) => r = true,
            None => r = false,
        }
        return r;
    }
    pub fn trimleft(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len+1 {
            return String::from(&s[f..len]);
        }
        else {
            return s.to_string();
        }
    }
    pub fn trimprefix(s: &str) -> &str {
        let len = s.len();
        if len > 0 {
            return &s[1..len];
        }
        else {
            return s;
        }
    }
    pub fn trimsuffix(s: &str) -> &str {
        let len = s.len();
        if len > 0 {
            return &s[0..len - 1];
        }
        else {

            return s;
        }
    }
    pub fn trimright(s: &str, f: usize) -> String {
        let len = s.len();
        if s.len() > f {
            return String::from(&s[0..len - f]);
        }
        else {

            return s.to_string();
        }
    }
    pub fn prefix(s: &str) -> &str {
        let len = s.len();
        if 0 < len {
            return &s[0..1];
        } else {
            return &"";
        }
    }
    pub fn postfix(s: &str) -> &str {
        let len = s.len();
        if 0 < len {
            return &s[len-1..len];
        } else {
            return &"";
        }
    }
    pub fn fromleft(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[0..f]);
        } else {
            return String::new();
        }
    }
    pub fn fromright(s: &str, f: usize) -> String {
        let len = s.len();
        if f < len {
            return String::from(&s[len - f..len]);
        } else {
            return String::new();
        }
    }
    pub fn split(string:&str,delim:&str)->Vec<String>{
        string.split(delim).map(str::to_string).collect()
    }
    /// string to evaluatable string
    pub fn stringtoeval(s: &str) -> String {
        let checkstring = "abcdefghijklmnopqrstuvwxyz0123456789".to_string();
        let fromstring = s.to_lowercase();
        let mut newstring = "".to_string();
        for xchr in split(&fromstring,""){
            if Nstring::instring(&checkstring, &xchr){
                newstring += &xchr;
            }
            else{
                newstring += "_";
            }
        }
        newstring

    }
    /// returns the value between 2 search strings
    pub fn stringbetween<'a>(str: &'a str, a: &str, b: &str) -> String {
        if let Some(start_pos) = str.find(a) {
            let rest = &str[start_pos + a.len()..];
            if let Some(end_pos) = rest.find(b) {
                let extracted = &rest[..end_pos];
                return extracted.to_string();
            }
        }
        "".to_owned()
    }

    pub fn stringbetweenincludeempty<'a>(str: &'a str, a: &str, b: &str) -> String {
        // used for interal usage to extraxt scopes, if a scope is empty its still a scope.
        // iteratrs shoulnd exit then so this funtion retuns something else
        // to let the iterator know to continue instead of a empty string.
        // ---------------------------------------
        if let Some(start_pos) = str.find(a) {
        let rest = &str[start_pos + a.len()..];
        if let Some(end_pos) = rest.find(b) {
            let extracted = &rest[..end_pos];
                return extracted.to_string();
        }
    }
    "<nonefound!>".to_owned()
    }
    pub fn f32(string: &str) ->f32{
        string.parse::<f32>().unwrap_or(0.0)
    }
    pub fn f64(string: &str) ->f64{
        string.parse::<f64>().unwrap_or(0.0)
    }
    pub fn i32(string: &str) ->i32{
        string.parse::<i32>().unwrap_or(0)
    }
    pub fn i64(string: &str) ->i64{
        string.parse::<i64>().unwrap_or(0)
    }
    pub fn u8(string: &str) ->u8{
        string.parse::<u8>().unwrap_or(0)
    }
    pub fn u16(string: &str) ->u16{
        string.parse::<u16>().unwrap_or(0)
    }
    pub fn u32(string: &str) ->u32{
        string.parse::<u32>().unwrap_or(0)
    }
    pub fn u64(string: &str) ->u64{
        string.parse::<u64>().unwrap_or(0)
    }
    pub fn usize(string: &str) ->usize{
        string.parse::<usize>().unwrap_or(0)
    }
}

pub struct Ntimer {

}

pub fn nscriptfn_timerinit(_var:&Vec<&str>,_block:&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar{
    NscriptVar::newstring("timer", Ntimer::init().to_string())
}
pub fn nscriptfn_timerdiff(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    NscriptVar::newstring("timer", Ntimer::diff(storage.getargstring(&args[0], block).parse::<i64>().unwrap_or(0)).to_string())
}
impl Ntimer {
    pub fn init() -> i64 {
        // sets a timestamp in a i64 (in nscript_fn_bindings converted to strings)
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        return duration.as_millis() as i64;
    }

    pub fn diff(timerhandle: i64) -> i64 {
        // given a timestamp from init() it will give the timedifference in MS
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        return duration.as_millis() as i64 - timerhandle;
    }
}
pub fn nscriptfn_isalpthabetic(var:&Vec<&str>,block:&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let string = storage.getargstring(var[0],block);
    return NscriptVar::newstring("isalpthabeth", string.chars().all(char::is_alphabetic).to_string());
}
pub fn nscriptfn_hextostring(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {
    let mut var = NscriptVar::new("var");
    match Vec::from_hex(storage.getargstring(&args[0], block)) {
        Ok(bytes) => var.stringdata = String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => {},
    }
    var
}
pub fn nscriptfn_stringtohex(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {
    let mut var = NscriptVar::new("var");
    let string = storage.getargstring(&args[0], block);
    var.stringdata = stringtohex(&string);// hex_string;
    var
}
fn stringtohex(string:&str)->String{
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let bytes = string.as_bytes();
    let mut hex_string = String::new();
    for byte in bytes {
        let high_nibble = (byte & 0xF0) >> 4;
        let low_nibble = byte & 0x0F;
        hex_string.push(hex_chars[high_nibble as usize]);
        hex_string.push(hex_chars[low_nibble as usize]);
    }
    hex_string
}
pub fn hex_to_string(hex_string: &str) -> String {
    match Vec::from_hex(hex_string) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

pub fn string_to_hex(input: &str) -> String {
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let bytes = input.as_bytes();
    let mut hex_string = String::new();
    for byte in bytes {
        let high_nibble = (byte & 0xF0) >> 4;
        let low_nibble = byte & 0x0F;
        hex_string.push(hex_chars[high_nibble as usize]);
        hex_string.push(hex_chars[low_nibble as usize]);
    }

    hex_string
}
pub fn string_to_eval(string_: &str) -> String {
    let mut return_val = string_.to_string();

    let replacements = [
        ("#", ""), ("%", ""), ("-", "_"), (" ", "_"), (":", "_"), ("\\", "_"), ("/", "_"),
        (".", "_"), ("@", "_"), ("&", "_"), ("!", ""), ("'", ""), ("[", "_"), ("]", "_"),
        ("(", "_"), (",", "_"), ("^", "_"), (")", "_"), ("|", "_")
    ];

    for (search, replace) in replacements {
        return_val = return_val.replace(search, replace);
    }

    return return_val;
}
pub fn nscriptfn_print(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let (contents,color) = createprintingstr(args, block, storage);

    print(&contents,&color);
    return NscriptVar::newstring("var",contents);
}
pub fn nscriptfn_printraw(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let (contents,color) = createprintingstr(args, block, storage);
    printraw(&contents,&color);
    return NscriptVar::newstring("var",contents);
}
//helperfn for nscriptfn print /printraw
fn createprintingstr(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage)-> (String,String){
    let mut color = "".to_string();
    let mut contents = String::new();
    let len = args.len();
    if len > 1 {
        color = storage.getargstring(&args[len-1], block);
        for x in 0..args.len()-1{
            let res = storage.getargstring(&args[x], block);
            contents = contents + &res;
        }
    }
    else{
        contents = storage.getargstring(&args[0], block);
    }
    (contents,color)
}
pub fn printraw(m: &str, color: &str) {
    print!("{}", nscriptgetprintingcolor(m,color));
}
pub struct Nfile {
}

#[cfg(not(windows))]
pub fn print(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
     println!("{}", nscriptgetprintingcolor(m,color));
}
#[cfg(windows)]
pub fn print(m: &str, _color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
     println!("{}", m);
}
pub fn nscriptgetprintingcolor(m:&str,color: &str)->ColoredString{
    let retcolor = match color {
        "bright blue" | "bb" => {
            m.bright_blue()
        }
        "bright green" | "bg"=> {
            m.bright_green()
        }
        "bright cyan" | "bc" => {
            m.bright_cyan()
        }
        "bright red" | "br" => {
            m.bright_red()
        }
        "bright magenta" | "bm" => {
            m.bright_magenta()
        }
        "bright yellow" | "by" => {
            m.bright_yellow()
        }
        "bright purple" | "bp" => {
            m.bright_purple()
        }
        "purple" | "p" => {
            m.custom_color((73, 4, 58))
        }
        "cyan" | "c" =>{
            m.cyan()
        }
        "yellow" | "y" => {
            m.yellow()
        }
        "red" | "r" => {
            m.red()
        }
        "green" | "g" => {
            m.green()
        }
        "blue" | "b" =>{
            m.blue()
        }
        "magenta" | "m" =>{
            m.magenta()
        }
        "orange" | "o" =>{
            m.custom_color((237,100,8))
        }
        "bright orange" | "bo" =>{
            m.custom_color((255,152,80))
        }
        "grey" | "gr" =>{
            m.custom_color((113,113,113))
        }
        "pink" | "pi" =>{
            m.custom_color((250, 108, 219))
        }
        "brown" | "brn" =>{
            m.custom_color((126, 81, 76))
        }
        "" =>{
            m.custom_color((255,255,255))
        }
        _ => {
            if Nstring::instring(&color, "rgb("){
                let rgb = Nstring::stringbetween(&color,"rgb(", ")");
                let rgb = split(&rgb,",");
                if rgb.len() > 2{
                    return m.custom_color(CustomColor::new(Nstring::u8(&rgb[0].trim()),Nstring::u8(&rgb[1].trim()),Nstring::u8(&rgb[2].trim())));

                }
            }
            m.custom_color((255,255,255))
        }
    };
    retcolor
}
//
//
// impl NscriptStructBinding for Nfile{
//     fn nscript_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>,_storage:&mut NscriptStorage) -> NscriptVar {
//         let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
//         match tocall {
//             "listdir" | "dirlist" =>{
//                 let mut fullp = false;
//                 if args.len() > 1{
//                     if args[1].stringdata == "true"{
//                         fullp = true;
//                     }
//                 }
//                 thisvar.stringvec = Nfile::dirtolist(&args[0].stringdata, fullp);
//             }
//             "read" => {if args.len() > 0 {thisvar.stringdata = Nfile::read(&args[0].stringdata);}}
//             "write" => {if args.len() > 1 {thisvar.stringdata = Nfile::write(&args[0].stringdata,&args[1].stringdata);}}
//             "exists" => {thisvar.stringdata = Nfile::checkexists(&args[0].stringdata).to_string();}
//             _ => {println!("Cant find rust function file::{}",tocall);}
//         }
//         return thisvar;
//     }
// }

pub fn nscriptfn_fileread(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
    if args.len() > 0 {thisvar.stringdata = Nfile::read(&storage.getargstring(&args[0], block));}
    thisvar
}

pub fn nscriptfn_filewrite(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
    if args.len() > 1 {
        thisvar.stringdata = Nfile::write(&storage.getargstring(&args[0], block),&storage.getargstring(&args[1], block));
    };
    thisvar
}
pub fn nscriptfn_filewriteasync(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let fdata = storage.getargstring(&args[1], block).to_string();
    let fname = storage.getargstring(&args[0], block);
    if args.len() > 1 {
        thread::spawn(move || {
            Nfile::write(&fname,&fdata);
        });
    };
    NscriptVar::new("Nfile")
}

pub fn nscriptfn_fileexists(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
    thisvar.stringdata = Nfile::checkexists(&storage.getargstring(&args[0], block)).to_string();
    thisvar
}
pub fn nscriptfn_listdir(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
    let mut fullp = false;
    if args.len() > 1{
        if storage.getargstring(&args[1], block) == "true"{
            fullp = true;
        }
    }
    thisvar.stringvec = Nfile::dirtolist(&storage.getargstring(&args[0], block), fullp);
    thisvar
}
pub fn nscriptfn_filesize(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
    thisvar.stringdata = Nfile::filesize(&storage.getargstring(&args[0], block));
    thisvar
}
pub fn nscriptfn_filesizebytes(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
    thisvar.stringdata = Nfile::filesizebytes(&storage.getargstring(&args[0], block)).to_string();
    thisvar
}
impl Nfile {
    pub fn dirtolist(readpath: &str, fullpathnames: bool) -> Vec<String> {
        let mut output = Vec::new();
        let paths = match fs::read_dir(readpath) {
            Ok(paths) => paths,
            Err(error) => {
                println!("<error>: Cannot read directory: {}", error);
                return Vec::new();
            }
        };
        for path in paths {
            match path {
                Ok(entry) => {
                    let unwraped = entry.path().display().to_string();
                    if !unwraped.is_empty() {
                        if !fullpathnames {
                            output.push(unwraped.replace(readpath, ""));
                        }else{
                            output.push(unwraped);
                        }
                    }
                }
                Err(error) => {
                    println!("<error>: Cannot access directory entry: {}", error);
                    return Vec::new();
                }
            }
        }
        output
    }
    pub fn filesizebytes(file: &str) -> String {
        // returns the full byte size of a file!
        let path = Path::new(file);
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(_) => return String::new(),
        };
        let realsize = metadata.len();
        realsize.to_string()
    }

    pub fn filesize(file: &str) -> String {
        // returns a fancy calculated string of the size rounded GB/MB/KB
        let path = Path::new(file);
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(_) => return String::new(),
        };
        let realsize = metadata.len();
        formatbytes(&realsize)
    }

    pub fn checkexists(fp: &str) -> bool {
        return std::path::Path::new(fp).exists();
    }
    pub fn write(path: &str, data: &str) -> String {
        if std::path::Path::new(&path).exists(){
            let  _error = match fs::remove_file(path) {
                Ok(_) => format!("File deleted successfully"),
                Err(err) =>{
                    return format!("Error writing a file ( cant delete,before write): {}", err);
                } ,
            };
        }
        let mut f = match File::create(path) {
            Ok(file) => file,
            Err(err) => return err.to_string(),
        };

        if let Err(err) = f.write_all(data.as_bytes()) {
            return err.to_string();
        }

        if let Err(err) = f.sync_all() {
            return err.to_string();
        }

        String::new()
    }
    pub fn read(floc: &str) -> String {
        let mut file = match File::open(floc) {
            Ok(file) => file,
            Err(_) => return String::new(), // Return empty string on error
        };
        let mut contents = String::new();
        if let Err(_) = file.read_to_string(&mut contents) {
            return String::new(); // Return empty string on error
        }
        contents
    }
}
pub fn nscriptfn_round(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0],block);
    let decimals = Nstring::usize(&storage.getvar(&args[1],block).stringdata);
    let mut increment = 1.0 as f64;
    for _ in  0..decimals{
        increment *= 10.0;
    }
    var.stringdata = ((Nstring::f64(&var.stringdata) *increment).round() / increment).to_string();
    var
}
pub fn nscriptfn_call_program(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new("var");
    let mut non_empty_args :Vec<String>= Vec::new();
    for xarg in args{
        non_empty_args.push(storage.getargstring(&xarg, block));
    }
    let mut output = if cfg!(target_os = "windows") {
        let mut output = Command::new("cmd");
        output.arg("/C");

        output
    } else {
        let mut output = Command::new("sh");
        output.arg("-c");

        output
    };
    for arg in non_empty_args {
        output.arg(arg);
    }
    if let Ok(handle) = output.spawn(){
        var.stringdata = format!("{:?}",handle);
    };
    var
}
pub fn nscriptfn_cat(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0],block);
    for xcat in 1..args.len(){
        var.stringdata = var.stringdata + &storage.getargstring(&args[xcat], block);
    }
    return var;
}

pub fn nscriptfn_vec(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut vec = NscriptVar::new("v");
    for xcat in args{
        vec.stringvec.push(storage.getargstring(&xcat, block));
    }
    return vec;
}
pub fn nscriptfn_call_programwait(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new("var");
    let mut non_empty_args :Vec<String>= Vec::new();
    for xarg in args{
        non_empty_args.push(storage.getargstring(xarg, block));
    }
    let mut output = if cfg!(target_os = "windows") {
        let mut output = Command::new("cmd");
        output.arg("/C");

        output
    } else {
        let mut output = Command::new("sh");
        output.arg("-c");

        output
    };
    for arg in non_empty_args {
        output.arg(arg);
    }
    let output = output.output();
      let result = match output {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout);
            var.stringvec.push(stdout.clone().unwrap_or("".to_string()).to_string());
            let stderr = String::from_utf8(output.stderr);

            var.stringvec.push(stderr.clone().unwrap_or("".to_string()));
            format!(
                "Program executed successfully.\nStdout: {}\nStderr: {}",
                stdout.unwrap(),
                stderr.unwrap()
            )
        },
        Err(err) => {
            format!("Failed to execute program: {}", err)
        }
    };
    var.stringdata = result.to_string();
    var
}
pub fn nscriptfn_sleep(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage)->NscriptVar {
    let milliseconds = storage.getargstring(&args[0], block).parse::<u64>().unwrap_or(0);
    let duration = Duration::from_millis(milliseconds);
    thread::sleep(duration);
    return NscriptVar::new("sleep");
}
pub fn nscriptfn_random(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    if args.len() > 1 {
        let min = &storage.getargstring(&args[0], block);
        let max = &storage.getargstring(&args[1], block);
        let mut decimals = "".to_string();
        if args.len() > 2 {
            decimals = storage.getargstring(&args[2], block);
        }
        let min_num = match min.parse::<f64>() {
            Ok(parsed_num) => parsed_num,
            Err(_) => return NscriptVar::new("error"),
        };
        let max_num = match max.parse::<f64>() {
            Ok(parsed_num) => parsed_num,
            Err(_) => return NscriptVar::new("error"),
        };
        if min_num > max_num {
            return NscriptVar::new("error");
        }
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(min_num..=max_num);
        if decimals.is_empty() {
            let mut var = NscriptVar::new("random");
            var.stringdata = random_num.to_string();
            return var;
        }
        let rounded_num = match decimals.parse::<usize>() {
            Ok(num_decimals) => format!("{:.*}", num_decimals, random_num),
            Err(_) => return NscriptVar::new("error"),

        };
        let mut var = NscriptVar::new("random");
        var.stringdata = rounded_num.to_string();
        return var;
    }
    return NscriptVar::new("error");
}

pub fn nscriptfn_arraypush(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    if args.len() > 1 {
        var.stringvec.push(storage.getargstring(&args[1], block));
    }
    return var;
}
pub fn nscriptfn_arraymerge(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    for x in args{
        for xvec in storage.getargstringvec(&x, block){
            var.stringvec.push(xvec);
        }
    }
    return var;
}
pub fn nscriptfn_arraycontains(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    if args.len() > 1 {
        var.stringdata = var.stringvec.contains(&storage.getargstring(&args[1], block)).to_string();
    }
    return var;
}

pub fn nscriptfn_arrayretain(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    if args.len() > 1 {
        var.stringvec.retain(|x| x != &storage.getargstring(&args[1], block));
    }
    return var
}
pub fn nscriptfn_arrayshuffle(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    let mut rng = rand::thread_rng();
    var.stringvec.shuffle(&mut rng);
    return var;
}
pub fn nscriptfn_arrayreverse(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    let mut newvec:Vec<String> = Vec::new();
    for x in var.stringvec.clone(){
        //newvec.push("".to_string());
        newvec.insert(0,x);
    }
    var.stringvec = newvec;
    return var;
}
pub fn nscriptfn_arrayinsert(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    if args.len() > 2{
        var.stringvec.insert(storage.getargstring(&args[1],block).parse::<usize>().unwrap_or(0),storage.getargstring(&args[2],block))
    }
    else{
        print("arrayinsert() error , unmatched arguments given arrayinsert(array,entreeid, entreedata)","r");
    }
    return var;
}
pub fn nscriptfn_arraysearch(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    let tosearch = storage.getargstring(&args[1], block);
    let mut newvec = Vec::new();
    for xitem in var.stringvec{
        if Nstring::instring(&xitem,&tosearch) {
            newvec.push(xitem);
        }
    }
    var.stringvec = newvec;
    return var;
}
pub fn nscriptfn_arraysort(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new("ret");
    var.stringvec =  storage.getargstringvec(&args[0],block);
    var.stringvec.sort();
    var
}
pub fn nscriptfn_arrayfilter(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    let tosearch = storage.getargstring(&args[1], block);
    let mut newvec = Vec::new();
    for xitem in var.stringvec{
        if Nstring::instring(&xitem,&tosearch) == false{
            newvec.push(xitem);
        }
    }
    var.stringvec =newvec;
    return var;
}
pub fn nscriptfn_arrayroll(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let var = storage.getvar(&args[0], block);
    let mut newvar = NscriptVar::newvar(&var.name, var.stringdata.to_string(),Vec::new());
    newvar.stringvec.push(storage.getargstring(&args[1], block));
    if args.len() > 1 {
        for xitem in 0..var.stringvec.len() -1{
            newvar.stringvec.push(var.stringvec[xitem].to_string());
        }
    }
    return newvar
}
pub fn nscriptfn_dircreate(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
 let dir = storage.getargstring(&args[0], block);
    let mut var = NscriptVar::new(&dir);
    var.stringdata = create_directory(&dir);
    return var
}
pub fn create_directory(dir_path: &str) -> String {
    match fs::create_dir(dir_path) {
        Ok(_) => format!("Directory '{}' created successfully", dir_path),
        Err(err) => format!("Error creating directory: {:?}", err),
    }
}
pub fn copy_directory(dir_path: &str, todir_path: &str) -> String {
    match fs::copy(dir_path,todir_path) {
        Ok(_) => format!("Directory '{}' created successfully", dir_path),
        Err(err) => format!("Error creating directory: {:?}", err),
    }
}
pub fn nscriptfn_dirsize(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
 let dir = storage.getargstring(&args[0], block);
    let mut var = NscriptVar::new(&dir);
    var.stringdata = formatbytes(&get_size(&dir).unwrap_or(0));
    return var
}
pub fn nscriptfn_formatbytes(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
 let dir = storage.getargstring(&args[0], block);
    let mut var = NscriptVar::new(&dir);
    var.stringdata = formatbytes(&get_size(&dir).unwrap_or(0));
    return var
}
fn formatbytes(bytesize:&u64)->String{
    let mut unit = "B";
    let mut size = bytesize.clone();
    if size > 1000000000000{
        unit = "TB";
        size = size / 10000000000;
    }
    if size > 1000000000{
        unit = "GB";
        size = size / 10000000;
    }
    else if size > 1000000{
        unit = "MB";
        size = size / 10000;
    }
    else  if size > 1000{
        unit = "KB";
        size = size / 10;
    }

    let sizestr = size.to_string();
    let fractial = Nstring::fromright(&sizestr,2);
    Nstring::trimright(size.to_string().as_str(), 2).to_string() + "." + &fractial + &unit
}
pub fn nscriptfn_int_or(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let isor = Nstring::i64(&storage.getargstring(&args[1], block));
    let mut var = storage.getvar(&args[0], block);
    if let Ok(isint) =  storage.getargstring(&args[0], block).parse::<i64>(){
        var.stringdata = isint.to_string();
    }else{
        var.stringdata = isor.to_string();
    };
    return var
}
pub fn nscriptfn_float_or(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let isor = Nstring::f64(&storage.getargstring(&args[1], block));
    let mut var = storage.getvar(&args[0], block);
    if let Ok(isint) =  storage.getargstring(&args[0], block).parse::<f64>(){
        var.stringdata = isint.to_string();
    }else{
        var.stringdata = isor.to_string();
    };
    return var
}
pub fn nscriptfn_string_or(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = storage.getvar(&args[0], block);
    if var.stringdata == ""{
        let isor = storage.getargstring(&args[1], block);
        var.stringdata = isor;
    };
    return var
}
pub fn nscriptfn_dirsizebytes(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
 let dir = storage.getargstring(&args[0], block);
    let mut var = NscriptVar::new(&dir);
    var.stringdata = get_size(&dir).unwrap_or(0).to_string();
    return var
}
// Move a file from the source path to the destination path
pub fn nscriptfn_filemove(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {

    let mut var = NscriptVar::new("filemove");
    if args.len() < 2 {
        print!("wrong arguments for filemove (source , destination)");
        return var;
    }
    let source = storage.getargstring(&args[0], block);
    let destination = storage.getargstring(&args[1], block);
    if source == "" || destination == ""{
        var.stringdata = "Aruments cannot be empty!!".to_string();
    }
    match fs::rename(source, destination) {
        Ok(_) => var.stringdata = format!("File moved successfully"),
        Err(err) => var.stringdata = format!("Error moving file: {}", err),
    };
    return var;
}

// Copy a file from the source path to the destination path
pub fn nscriptfn_filecopy(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {
    let mut var = NscriptVar::new("filemove");
    if args.len() < 2 {
        print!("wrong arguments for filemove (source , destination)");
        return var;
    }
    let source = storage.getargstring(&args[0], block);
    let destination = storage.getargstring(&args[1], block);
    match fs::copy(source, destination) {
        Ok(_) => var.stringdata = format!("File copied successfully"),
        Err(err) => var.stringdata = format!("Error copying file: {}", err),
    };
    return var;
}

// Delete a file at the specified path
pub fn nscriptfn_filedelete(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {
    let mut var = NscriptVar::new("filedelete");
    match fs::remove_file(storage.getargstring(&args[0], block)) {
        Ok(_) => var.stringdata = format!("File deleted successfully"),
        Err(err) => var.stringdata = format!("Error deleting file: {}", err),
    };
    return var;
}

// Delete a directory and all its contents
pub fn nscriptfn_directory_delete(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {

    let mut var = NscriptVar::new("filedelete");
    let directory = storage.getargstring(&args[0], block);
    match fs::remove_dir_all(directory) {
        Ok(_) => var.stringdata = format!("Directory deleted successfully"),
        Err(err) => var.stringdata = format!("Error deleting directory: {}", err),
    };
    return var;
}

// Move a directory from the source path to the destination path
pub fn nscriptfn_directory_move(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("filemove");
    if args.len() < 2 {
        print!("wrong arguments for dirmove (source , destination)");
        return var;
    }

    match fs::rename(storage.getargstring(&args[0], block).as_str(), storage.getargstring(&args[1], block).as_str()) {
        Ok(_) => var.stringdata = format!("Directory moved successfully"),
        Err(err) => var.stringdata = format!("Error moving directory: {}", err),
    };
    return var;
}

pub fn nscriptfn_terminalinput(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = NscriptVar::new("terminalinput");
    let  message = storage.getargstring(&args[0], block);
    let mut default = "".to_string();
    if args.len() > 1 {
        default =  storage.getargstring(&args[1], block);
    }
    println!("{} [default:{}] ", message,default);
    io::stdout().flush().unwrap(); // Flushes the output to ensure the message is displayed immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Remove trailing newline character
    input = input.trim_end().to_string();

    if input.is_empty() == true|| &input == ""{
        default.to_string()
    } else {
        input.to_string()
    };
    var.stringdata = input.to_string();
    var
}

pub fn nscriptfn_splitselect(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = NscriptVar::new("splitselect");
    if args.len() > 2 {
        let id = storage.getargstring(&args[2], block).parse::<usize>().unwrap_or(0);
        let a1 = storage.getargstring(&args[1],block);
        let a0 = storage.getargstring(&args[0],block);
        let splitvec = split(&a0,&a1);
        if splitvec.len() > id {
            var.stringdata = splitvec[id].to_string();
        }
    else{
            print(&format!("array[{}] len exceeds",&args[0]),"r");
        }
    }
    var
}
pub fn nscriptfn_base64tofile(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = NscriptVar::new("base64tostring");
    if let Ok(basethis) = BASE64_STANDARD.decode(&storage.getargstring(&args[0], block)){

        //Nfile::write(&storage.getargstring(&args[1],block),&String::from_utf8(basethis).unwrap());
        let path = storage.getargstring(&args[1],block);
        if std::path::Path::new(&path).exists(){
            let  _error = match fs::remove_file(&path) {
                Ok(_) => format!("File deleted successfully"),
                Err(err) =>{
                    var.stringdata = err.to_string();
                    return var;//format!("Error writing a file ( cant delete,before write): {}", err);
                } ,
            };
        }
        let mut f = match File::create(&path) {
            Ok(file) => file,
            Err(err) =>{
                var.stringdata = err.to_string();
                return var;
            }
        };

        if let Err(err) = f.write_all(&basethis) {
                var.stringdata = err.to_string();
                return var;
        }

        if let Err(err) = f.sync_all() {
                var.stringdata = err.to_string();
                return var;
        }
        var.stringdata = "succes".to_string();
        return var;
    }
    var.stringdata = "error".to_string();
    return var;
}
pub fn nscriptfn_filetobase64(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = NscriptVar::new("base64tostring");
    let floc = storage.getargstring(&args[0], block);

        let mut file = match File::open(floc) {
            Ok(file) => file,
            Err(_) => {
print("cant find file","r");
              return var // Return empty string on error
            }
        };
        let mut contents: Vec<u8> = Vec::new();
        if let Err(_) = file.read_to_end(&mut contents) {

        print("cant read file","r");
            return var; // Return empty string on error
        }
        var.stringdata = BASE64_STANDARD.encode(contents);
        return var;

}
pub fn nscriptfn_base64tostring(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = NscriptVar::new("base64tostring");
    if let Ok(basethis) = BASE64_STANDARD.decode(&storage.getargstring(&args[0], block)){
        var.stringdata = String::from_utf8(basethis).unwrap();
        return var;
    }
    var.stringdata = "error".to_string();
        return var;
}
pub fn nscriptfn_stringtobase64(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut var = NscriptVar::new("base64tostring");
    var.stringdata = BASE64_STANDARD.encode(&storage.getargstring(&args[0], block));
    return var;

}

//
pub fn nscriptfn_tcplistener(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcplisten");
    if args.len() < 2 {
        print!("wrong arguments for  tcplistener (ip , port)");
        return var;
    }
    let ip = storage.getargstring(&args[0], block);
    let port = storage.getargstring(&args[1], block);
    var.stringdata = storage.tcp.listener(&ip,&port);
    return var;
}

pub fn nscriptfn_tcpconnect(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcplisten");
    if args.len() < 2 {
        print!("wrong arguments for tcpconnect (ip , port)");
        return var;
    }
    let ip = storage.getargstring(&args[0], block);
    let port = storage.getargstring(&args[1], block);
    var.stringdata = storage.tcp.connect(&ip,&port);
    return var;
}

pub fn nscriptfn_tcpaccept(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcpaccept");
    let socket = storage.getargstring(&args[0], block);
    var.stringdata = storage.tcp.accept(&socket);
    return var;
}

pub fn nscriptfn_tcpdisconnect(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcpdisconnect");
    let socket = storage.getargstring(&args[0], block);
    var.stringdata = storage.tcp.disconnect(&socket);
    return var;
}


pub fn nscriptfn_tcpreceive(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcpreceive");
    let socket = storage.getargstring(&args[0], block);
    var.stringdata = storage.tcp.receive(&socket);
    return var;
}
pub fn nscriptfn_tcpsend(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcpsend");
    if args.len() < 2 {
        print!("wrong arguments for tcpsend (socket , msg)");
        return var;
    }
    let socket = storage.getargstring(&args[0], block);
    let msg = storage.getargstring(&args[1], block);
    var.stringdata = storage.tcp.send(&socket,&msg);
    return var;
}
pub fn nscriptfn_add(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("tcpsend");
    if args.len() < 2 {
        print!("wrong arguments for add(number , toadd)");
        return var;
    }
    var.stringdata = (Nstring::f64(&storage.getargstring(&args[0], block)) + Nstring::f64(&storage.getargstring(&args[1], block))).to_string();
    return var;
}
pub fn nscriptfn_subtract(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("subtract");
    if args.len() < 2 {
        print!("wrong arguments for subtract (number , tosubtract)");
        return var;
    }
    var.stringdata = (Nstring::f64(&storage.getargstring(&args[0], block)) - Nstring::f64(&storage.getargstring(&args[1], block))).to_string();
    return var;
}
pub fn nscriptfn_multiply(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("multiply");
    if args.len() < 2 {
        print!("wrong arguments for multiply (number , multiplyby)");
        return var;
    }
    var.stringdata = (Nstring::f64(&storage.getargstring(&args[0], block)) * Nstring::f64(&storage.getargstring(&args[1], block))).to_string();
    return var;
}
pub fn nscriptfn_devide(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("devide");
    if args.len() < 2 {
        print!("wrong arguments for devide (number , devideby)");
        return var;
    }
    var.stringdata = (Nstring::f64(&storage.getargstring(&args[0], block)) / Nstring::f64(&storage.getargstring(&args[1], block))).to_string();
    return var;
}
pub fn nscriptfn_cos(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var1 = storage.getvar(&args[0], block);
    var1.stringdata = Nstring::f32(&var1.stringdata).cos().to_string();
    return var1;
}
pub fn nscriptfn_sin(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var1 = storage.getvar(&args[0], block);
    var1.stringdata = Nstring::f32(&var1.stringdata).sin().to_string();
    return var1;
}
fn nscript_encryption(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage,encrypting:bool) -> NscriptVar {
    let mut data = NscriptVar::newstring("r",storage.getargstring(&args[0], block));
    let password = storage.getvar(&args[1], block).stringdata;
    let  passwordvec = split(&password,"");
    let chrvec:Vec<&str> = [ "!", "@", "#", "$", "%", "^", "&", "*", "(", ")",
        "-", "=", "+", "[", "]", "{", "}", "~", "`",
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t",
        "u", "v", "w", "x", "y", "z",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "{", "}", "[", "]", "(", ")",
        "|", ";", ":", ",", ".", "/", "?", "~", "`",
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
        "U", "V", "W", "X", "Y", "Z"," ","","\"","\n","\r"].to_vec();
    let hexvec:Vec<&str> =["A","B","C","D","E","F","0","1","2","3","4","5","6","7","8","9","A"].to_vec();
    let passlen = password.len();
    let mut cur_pass = 0;
    let mut encstringbuff = String::new();
    if encrypting {
        data.stringdata = string_to_hex(&data.stringdata);
        for xchr in split(&data.stringdata,""){
            if xchr != ""{
                let curchari = 1600 + vecfindstringpos(&hexvec,&xchr) + passlen + vecfindstringpos(&chrvec,passwordvec[cur_pass]);
                let hexindex = modsize(curchari as f32,16.0);
                encstringbuff += &hexvec[hexindex as usize].to_string();
                cur_pass +=1;
                cur_pass = modsize(cur_pass as f32,passlen as f32) as usize;
            }
        }

        data.stringdata = encstringbuff;
    }else{
        for xchr in split(&data.stringdata,""){
            if xchr != ""{
                let curchari = 1600 + vecfindstringpos(&hexvec,xchr) - passlen - vecfindstringpos(&chrvec,passwordvec[cur_pass]);
                let hexindex = modsize(curchari as f32,16.0);
                encstringbuff += &hexvec[hexindex as usize].to_string();
                cur_pass +=1;
                cur_pass = modsize(cur_pass as f32,passlen as f32) as usize;
            }
        }
        data.stringdata = hex_to_string(&encstringbuff);
    }

    data
}

pub fn nscriptfn_encrypt(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    return nscript_encryption(args, block, storage, true);
}

pub fn nscriptfn_decrypt(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    return nscript_encryption(args, block, storage, false);
}

fn vecfindstringpos(invec:&Vec<&str>,chr:&str) -> usize{
    for x in 0..invec.len()-1{
        if chr == invec[x] {
            return x;
        }
    }
    println!("cant find chr-asci [{}] in vec [{}]",chr,invec.join(","));
    0
}
pub fn nscriptfn_mod(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    NscriptVar::newstring("r",
        modsize(Nstring::f32(&storage.getargstring(args[0], block)), Nstring::f32(&storage.getargstring(args[1], block))).to_string()
    )
}

fn modsize(input:f32,max:f32)->f32{
    if input <= max {
        input
    }
    else{
       input % max
    }
}
pub fn nscriptfn_arraynew(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar  {
    NscriptVar::newvec("r",
        Vec::new()
    )
}
pub fn nscriptfn_arraynewsized(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut vect = Vec::new();
    for _ in 0..storage.getvar(args[0], block).getnumber() as usize{
        vect.push("".to_string());
    }
    NscriptVar::newvec("r",
        vect
    )
}
pub fn nscriptfn_panic(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let blockname = block.name.to_string();
    let blocksub = block.insubblock.to_string();
    panic!("[NscriptRuntimePanic]Panix in block[{} @ subblock{}] error msg: {}",blockname, blocksub,&storage.getargstring(args[0], block));
}
// pub fn nscriptfn_stingtoalphanummeric(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
//     let checkstring = "abcdefghijklmnopqrstuvwxyz0123456789".to_string();
//     let fromstring = storage.getargstring(args[0], block).to_lowercase();
//     let mut newstring = "".to_string();
//     for xchr in split(&fromstring,""){
//         if Nstring::instring(&checkstring, &xchr){
//             newstring += &xchr;
//         }
//         else{
//             newstring += "_";
//         }
//     }
//
//     NscriptVar::newstring("r",
//         newstring
//     )
// }
// pub fn nscriptfn_createqrcode(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
//     let code = QrCode::new(storage.getargstring(args[0], block).as_bytes()).unwrap();
//     let image = code.render::<Luma<u8>>().build();
//     // Save the image.
//     image.save(storage.getargstring(args[1], block)).unwrap();
//     NscriptVar::new("qr")
// }

pub fn nscriptfn_prefix(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let  var = storage.getargstring(args[0], block);
    NscriptVar::newstring("str",Nstring::prefix(&var).to_string())
}
pub fn nscriptfn_suffix(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let  var = storage.getargstring(args[0], block);
    NscriptVar::newstring("str",Nstring::postfix(&var).to_string())
}
pub fn nscriptfn_percentage(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    //let  given = storage.getargstring(args[0], block);
    //let  total = storage.getargstring(args[1], block);
    let result = (Nstring::f32(&storage.getargstring(args[0], block)) * 100.0) / Nstring::f32(&storage.getargstring(args[1], block));
    NscriptVar::newstring("str",result.to_string())
}
pub fn nscriptfn_url(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let  host = storage.getargstring(args[0], block);
    let mut result = host.to_string() + "?";
    for xarg in &args[1..]{
        let  param = storage.getargstring(xarg, block);
        result = result + &param +"&";
    }

    NscriptVar::newstring("url",result.to_string())
}

