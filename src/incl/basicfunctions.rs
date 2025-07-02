use crate::*;
use std::{char, time::{SystemTime, UNIX_EPOCH}};
pub struct Nstring {
}
pub fn nscriptfn_split(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    //let mut thisvar = NscriptVar::new("var");
    if args.len() > 1 {
        let mut delim = "".to_string();
        if  args.len() > 1{
            delim = storage.getargstring(&args[1], block).to_string();
        }
        return NscriptVar::newvec("split",Nstring::split(&storage.getargstring(&args[0], block),&delim));
        // for xitem in split(&storage.getargstring(&args[0], block),&delim){
        //     thisvar.stringvec.push(xitem.to_string());
        // }
    }
    else{
        println!("Error on givenarguments for func split");
    }
    NscriptVar::new("var")
}

pub fn nscriptfn_replace(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 2{
        let value = Nstring::replace(&storage.getargstring(&args[0], block), &storage.getargstring(&args[1], block), &storage.getargstring(&args[2], block));
        neovar.stringdata = value.to_string();
    }else{
        print("string::replace arguments missing, returing nothing","r");
    }
    return neovar;
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
    let mut neovar = NscriptVar::new("result");
    let mut return_val = storage.getargstring(&args[0], block);

    let replacements = [
        ("#", ""), ("%", ""), ("-", "_"), (" ", "_"), (":", "_"), ("\\", "_"), ("/", "_"),
        (".", "_"), ("@", "_"), ("&", "_"), ("!", ""), ("'", ""), ("[", "_"), ("]", "_"),
        ("(", "_"), (",", "_"), ("^", "_"), (")", "_"), ("|", "_")
    ];

    for (search, replace) in replacements {
        return_val = return_val.replace(search, replace);
    }
    neovar.stringdata = return_val;
    return neovar;
}
pub fn nscriptfn_len(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    neovar.stringdata = storage.getargstringvec(&args[0],block).len().to_string();
    neovar
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
    let mut neovar = NscriptVar::new("result");
    if args.len() < 2 {
        neovar.stringdata = "".to_string();
    }
    else{
        neovar.stringdata = Nstring::stringbetween(&storage.getargstring(&args[0], block), &storage.getargstring(&args[1], block), &storage.getargstring(&args[2], block))

    }
    neovar
}
pub fn nscriptfn_stringbetweennested(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() < 2 {
        neovar.stringdata = "".to_string();
    }
    else{
        neovar.stringdata = Nstring::stringbetween(&storage.getargstring(&args[0], block), &storage.getargstring(&args[1], block), &storage.getargstring(&args[2], block))

    }
    neovar
}
pub fn nscriptfn_trim(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    neovar.stringdata = storage.getargstring(&args[0],block);
    neovar
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
    }else{
        print("join arguments missing, returing nothing","r");
    }
    return neovar;
}

pub fn nscriptfn_fromleft(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 1{
        let string = Nstring::fromleft(&storage.getargstring(&args[0], block), storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0));
        neovar.stringdata = string;
    }else{
        print("string::fromleft arguments missing, returing nothing","r");
    }
    return neovar;
}

pub fn nscriptfn_fromright(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 1{
        let string = Nstring::fromright(&storage.getargstring(&args[0], block), storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0));
        neovar.stringdata = string;
    }else{
        print("string::fromleft arguments missing, returing nothing","r");
    }
    return neovar;
}

pub fn nscriptfn_trimright(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 1{
        let string = Nstring::trimright(&storage.getargstring(&args[0], block), storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0));
        neovar.stringdata = string;
    }else{
        print("string::fromleft arguments missing, returing nothing","r");
    }
    return neovar;
}
pub fn nscriptfn_trimleft(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    if args.len() > 1{
        let string = Nstring::trimleft(&storage.getargstring(&args[0], block), storage.getargstring(&args[1], block).parse::<usize>().unwrap_or(0));
        neovar.stringdata = string;
    }else{
        print("string::fromleft arguments missing, returing nothing","r");
    }
    return neovar;
}
pub fn nscriptfn_toupper(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    let value = storage.getargstring(&args[0], block).to_uppercase();
    neovar.stringdata = value.to_string();
    return neovar;
}
pub fn nscriptfn_tolower(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let mut neovar = NscriptVar::new("result");
    let value = storage.getargstring(&args[0], block).to_lowercase();
    neovar.stringdata = value.to_string();
    return neovar;
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
    pub fn stringtoeval(s: &str) -> String {
        // saver for hashmap keys usages
        let mut r = s.replace("-", "_");
        let all = [
            "~", "!", "#", "%", "^", "&", "*", "(", ")", "\\", "{", "}", "[", "]", ".", ",", "?",
            "'", "$", "/",
        ];
        for c in all {
            r = r.replace(c, "_");
        }
        r
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
    pub fn tohexplus(s:&str)->String{//,["er","₹"]
        let mut hexplus = string_to_hex(s);
        let torep = [
            [",","G"],[".","H"],["=","I"],["/","J"],["-","K"],["_","L"],["*","M"],["(","N"],[")","O"],["{","P"],["[","Q"],["]","R"],["<","S"],[">","T"],["!","U"],["@","V"],["#","W"],["$","X"],["%","Y"],["^","Z"],

            ["ing","!"],
            ["\nt",""],["\ni","Ι"],["\na","Λ"],["\no","Μ"],
            ["the","∑"],["and","∫"],["for","∆"],["liv","È"],["spe","É"],
            [" th","™"],[" he","©"],[" in","●"],[" an","■"],["es ","★"],[" re","Â"],[" cl","¶"],[" te","·"],[" sp","µ"],[" fi","³"],[" di","²"],[" wh","Ώ"],
            ["er ","♦"],["th ","€"],["e ","~"],["t ","+"],["g ","|"],["s ","\\"],
            [" t",";"],[" a",":"],[" s","["],[" c","]"],[" o","_"],[" b","<"],[" y",">"],[" r","Ÿ"],[" w","¡"],[" h","¦"],[" l","§"],[" m","À"],
            ["ch","¥"],["ll","£"],["ea","¢"],["ou","."],["ma",","],["th","@"],["he","#"],["in","%"],["an","&"],["es","*"],["on","("],["re","$"],["wh","Œ"],["cc","Ž"],["mb","š"],["ss","ƒ"],
            ["rt","…"],["pp","†"],["gr","‡"],["pr","‰"],["pa","¹"],["ho","º"],["iv","»"],["sp","¼"],["xt","½"],["mp","¾"],["se","¿"],["ay","Á"],["qu","Ͷ"],["ri","ͷ"],["gh","ͻ"],["by","ͼ"],["nt","ͽ"],["nc","Ϳ"],["ca","Ͱ"],["op","ͱ"],["cl","ͳ"],
            ["he","Ͳ"],["nv","Ή"],["rs","Ί"],["ti","Ό"],["ex","Ύ"],["nf","ΐ"],["hi","Α"],["nd","Ζ"],["mm","Η"], ["rm","Θ"],["to","Β"],["ta","Γ"],["yo","Ν"],["ry","Ξ"],["oo","Ο"],["nn","Π"],["gg","Ρ"],["sh","Σ"],

            ["a","z"],["b","y"],["c","x"],["d","w"],["e","v"],["f","h"],["i","k"],["k","l"],["p","o"],["\n","`"]
        ];
        for rep in torep{
            hexplus = Nstring::replace(&hexplus, &string_to_hex(rep[0]), rep[1]);
        }
        return hexplus;
    }
    pub fn fromhexplus(s:&str)->String{
        let mut hexplus = s.to_string();
        let torep = [
            [",","G"],[".","H"],["=","I"],["/","J"],["-","K"],["_","L"],["*","M"],["(","N"],[")","O"],["{","P"],["[","Q"],["]","R"],["<","S"],[">","T"],["!","U"],["@","V"],["#","W"],["$","X"],["%","Y"],["^","Z"],

            ["ing","!"],
            ["\nt",""],["\ni","Ι"],["\na","Λ"],["\no","Μ"],
            ["the","∑"],["and","∫"],["for","∆"],["liv","È"],["spe","É"],
            [" th","™"],[" he","©"],[" in","●"],[" an","■"],["es ","★"],[" re","Â"],[" cl","¶"],[" te","·"],[" sp","µ"],[" fi","³"],[" di","²"],[" wh","Ώ"],
            ["er ","♦"],["th ","€"],["e ","~"],["t ","+"],["g ","|"],["s ","\\"],
            [" t",";"],[" a",":"],[" s","["],[" c","]"],[" o","_"],[" b","<"],[" y",">"],[" r","Ÿ"],[" w","¡"],[" h","¦"],[" l","§"],[" m","À"],
            ["ch","¥"],["ll","£"],["ea","¢"],["ou","."],["ma",","],["th","@"],["he","#"],["in","%"],["an","&"],["es","*"],["on","("],["re","$"],["wh","Œ"],["cc","Ž"],["mb","š"],["ss","ƒ"],
            ["rt","…"],["pp","†"],["gr","‡"],["pr","‰"],["pa","¹"],["ho","º"],["iv","»"],["sp","¼"],["xt","½"],["mp","¾"],["se","¿"],["ay","Á"],["qu","Ͷ"],["ri","ͷ"],["gh","ͻ"],["by","ͼ"],["nt","ͽ"],["nc","Ϳ"],["ca","Ͱ"],["op","ͱ"],["cl","ͳ"],
            ["he","Ͳ"],["nv","Ή"],["rs","Ί"],["ti","Ό"],["ex","Ύ"],["nf","ΐ"],["hi","Α"],["nd","Ζ"],["mm","Η"], ["rm","Θ"],["to","Β"],["ta","Γ"],["yo","Ν"],["ry","Ξ"],["oo","Ο"],["nn","Π"],["gg","Ρ"],["sh","Σ"],

            ["a","z"],["b","y"],["c","x"],["d","w"],["e","v"],["f","h"],["i","k"],["k","l"],["p","o"],["\n","`"]
        ];
        for rep in torep{
            hexplus = Nstring::replace(&hexplus, rep[1],&string_to_hex(rep[0]));
        }
        return hex_to_string(&hexplus);

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
    pub fn usize(string: &str) ->usize{
        string.parse::<usize>().unwrap_or(0)
    }
}

pub struct Ntimer {

}

pub fn nscriptfn_timerinit(_var:&Vec<&str>,_block:&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar{
    return NscriptVar::newstring("timer", Ntimer::init().to_string());
}
pub fn nscriptfn_timerdiff(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    return NscriptVar::newstring("timer", Ntimer::diff(storage.getargstring(&args[0], block).parse::<i64>().unwrap_or(0)).to_string());
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
    //return NscriptVar::newstring("timer", Ntimer::init().to_string());
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
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let string = storage.getargstring(&args[0], block);
    let bytes = string.as_bytes();
    let mut hex_string = String::new();
    for byte in bytes {
        let high_nibble = (byte & 0xF0) >> 4;
        let low_nibble = byte & 0x0F;
        hex_string.push(hex_chars[high_nibble as usize]);
        hex_string.push(hex_chars[low_nibble as usize]);
    }
    var.stringdata = hex_string;
    var
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
    let mut color = "".to_string();
    if args.len() > 1 {
        color = storage.getargstring(&args[1], block);
    }
    let string = storage.getargstring(&args[0], block);
    print(&string,&color);
    return NscriptVar::newstring("var",string);
    //var.stringdata = string;
    //return var;

}
pub fn print(m: &str, color: &str) {
    // this is more a linux then a windows feature.
    // as for windows powershell is just lame. itl work but dont expect all colors to show!
    // --------------------------------------------
    match color {
        "bright blue" | "bb" => {
            println!("{}", m.bright_blue());
        }
        "bright green" | "bg"=> {
            println!("{}", m.bright_green());
        }
        "bright cyan" | "bc" => {
            println!("{}", m.bright_cyan());
        }
        "bright red" | "br" => {
            println!("{}", m.bright_red());
        }
        "bright magenta" | "bm" => {
            println!("{}", m.bright_magenta());
        }
        "bright yellow" | "by" => {
            println!("{}", m.bright_yellow());
        }
        "bright purple" | "bp" => {
            println!("{}", m.bright_purple());
        }
        "purple" | "p" => {
            println!("{}", m.purple());
        }
        "cyan" | "c" =>{
            println!("{}", m.cyan());
        }
        "yellow" | "y" => {
            println!("{}", m.yellow());
        }
        "red" | "r" => {
            println!("{}", m.red());
        }
        "green" | "g" => {
            println!("{}", m.green());
        }
        "blue" | "b" =>{
            println!("{}", m.blue());
        }
        "magenta" | "m" =>{
            println!("{}", m.magenta());
        }
        _ => {
            println!("{}", m);

        }
    };
}

pub struct Nfile {
}

impl NscriptStructBinding for Nfile{
    fn nscript_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>,_storage:&mut NscriptStorage) -> NscriptVar {
        let mut thisvar = NscriptVar::new("Nfile"); // Var to return.
        match tocall {
            "listdir" | "dirlist" =>{
                let mut fullp = false;
                if args.len() > 1{
                    if args[1].stringdata == "true"{
                        fullp = true;
                    }
                }
                thisvar.stringvec = Nfile::dirtolist(&args[0].stringdata, fullp);
            }
            "read" => {if args.len() > 0 {thisvar.stringdata = Nfile::read(&args[0].stringdata);}}
            "write" => {if args.len() > 1 {thisvar.stringdata = Nfile::write(&args[0].stringdata,&args[1].stringdata);}}
            "exists" => {thisvar.stringdata = Nfile::checkexists(&args[0].stringdata).to_string();}
            _ => {println!("Cant find rust function file::{}",tocall);}
        }
        return thisvar;
    }
}

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
        if realsize >= 1_000_000_000 {
            return format!("{:.2} GB", realsize as f64 / 1_000_000_000.0);
        }
        if realsize >= 1_000_000 {
            return format!("{:.2} MB", realsize as f64 / 1_000_000.0);
        }
        if realsize >= 1_000 {
            return format!("{:.2} KB", realsize as f64 / 1_000.0);
        }
        format!("{} B", realsize)
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

pub fn nscriptfn_call_program(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let var = NscriptVar::new("var");
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
    if let Ok(_) = output.spawn(){};
    var
}
pub fn nscriptfn_cat(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    //let mut var = NscriptVar::new("cat");
    let mut string = "".to_string();
    for xcat in args{
        string = string + &storage.getargstring(&xcat, block);
    }
    //var.stringdata = string;
    let var = NscriptVar::newstring("cat", string);
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
            let stderr = String::from_utf8(output.stderr);
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
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    if args.len() > 1 {
        var.stringvec.push(storage.getargstring(&args[1], block));
    }
    return var;
}

pub fn nscriptfn_arraycontains(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    if args.len() > 1 {
        var.stringdata = var.stringvec.contains(&storage.getargstring(&args[1], block)).to_string();
    }
    return var;
}

pub fn nscriptfn_arrayretain(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    if args.len() > 1 {
        var.stringvec.retain(|x| x != &storage.getargstring(&args[1], block));
    }
    return var
}
pub fn nscriptfn_arrayshuffle(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    let mut rng = rand::thread_rng();
    var.stringvec.shuffle(&mut rng);
    return var;
}
pub fn nscriptfn_arrayreverse(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    let mut newvec:Vec<String> = Vec::new();
    for x in var.stringvec.clone(){
        newvec.push("".to_string());
        newvec.insert(0,x);
    }
    var.stringvec = newvec;
    return var;
}
pub fn nscriptfn_arrayinsert(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    if args.len() > 2{
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    var.stringvec.insert(storage.getargstring(&args[1],block).parse::<usize>().unwrap_or(0),storage.getargstring(&args[2],block))
    }
    else{
        print("arrayinsert() error , unmatched arguments given arrayinsert(array,entreeid, entreedata)","r");
    }
    return var;
}
pub fn nscriptfn_arraysearch(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));

    var.stringdata = storage.getargstring(&args[0], block);

    let mut newvec = Vec::new();
    for xitem in storage.getargstringvec(&args[0], block){
        if Nstring::instring(&xitem,&storage.getargstring(&xitem, block)) {
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
    let mut var = NscriptVar::new(&storage.getargstring(&args[0], block));
    var.stringdata = storage.getargstring(&args[0], block);
    var.stringvec = storage.getargstringvec(&args[0], block);
    let mut newvec = Vec::new();
    for xitem in var.stringvec{
        if Nstring::instring(&xitem,&storage.getargstring(&args[1], block)) == false{
            newvec.push(xitem);
        }
    }
    var.stringvec =newvec;
    return var;
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

    if input.is_empty() {
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
    let var1 = storage.getargstring(&args[0], block);
    let var2 = storage.getargstring(&args[1], block);
    var.stringdata = (Nstring::f64(&var1) + Nstring::f64(&var2)).to_string();
    return var;
}
pub fn nscriptfn_subtract(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("subtract");
    if args.len() < 2 {
        print!("wrong arguments for subtract (number , tosubtract)");
        return var;
    }
    let var1 = storage.getargstring(&args[0], block);
    let var2 = storage.getargstring(&args[1], block);
    var.stringdata = (Nstring::f64(&var1) - Nstring::f64(&var2)).to_string();
    return var;
}
pub fn nscriptfn_multiply(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("multiply");
    if args.len() < 2 {
        print!("wrong arguments for multiply (number , multiplyby)");
        return var;
    }
    let var1 = storage.getargstring(&args[0], block);
    let var2 = storage.getargstring(&args[1], block);
    var.stringdata = (Nstring::f64(&var1) * Nstring::f64(&var2)).to_string();
    return var;
}
pub fn nscriptfn_devide(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let mut var = NscriptVar::new("devide");
    if args.len() < 2 {
        print!("wrong arguments for devide (number , devideby)");
        return var;
    }
    let var1 = storage.getargstring(&args[0], block);
    let var2 = storage.getargstring(&args[1], block);
    var.stringdata = (Nstring::f64(&var1) / Nstring::f64(&var2)).to_string();
    return var;
}


