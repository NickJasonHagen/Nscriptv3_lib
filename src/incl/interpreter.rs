
use crate::*;
//pub type NscriptSimpleFunctions = fn(&Vec<NscriptVar>) -> NscriptVar;
pub type NscriptSimpleFunctions = fn(&Vec<String>,block:&mut NscriptCodeBlock, &mut NscriptStorage) -> NscriptVar;
// pub fn emptyfnbuffer(_vec: Vec<NscriptVar>) -> NscriptVar {
//     // Default behavior
//     None
// }
/// NscriptScript main struct
pub struct Nscript<'a>{
    // for user created structs
    pub ruststructs: HashMap<&'a str, &'a mut dyn NscriptStructBinding>, // map for all the rust fn bindings.
    pub ruststructsindex: Vec<String>, // map for all the rust fn bindings.
    pub rustfunctions: HashMap<String, NscriptSimpleFunctions>, // map for all the rust fn bindings.
    pub rustfunctionsindex: Vec<String>, // map for all the rust fn bindings.
    pub coroutines: Vec<String>,// all nonclass functions
    pub emptyblock: NscriptCodeBlock,// all nonclass functions
    pub threadsreceiver: HashMap<String, mpsc::Receiver<NscriptVar>>,
    pub threadssenders: HashMap<String, mpsc::Sender<NscriptVar>>,
    pub tcplisteners: HashMap<String,TcpListener>,
    pub storage: NscriptStorage,
    pub formattedblocks: HashMap<String,NscriptFormattedCodeBlock>,
    //pub compiledblocks:  Vec<NscriptCompiledCodeBlock<'a>>,
    //pub tcp: NscriptTcp,
}

impl <'a> Nscript<'a>{
    fn setclean() ->Nscript<'a>{
        Nscript {
            ruststructs: HashMap::new(),
            ruststructsindex: Vec::new(),
            rustfunctions: HashMap::new(),
            rustfunctionsindex: Vec::new(),
            coroutines: Vec::new(),
            emptyblock: NscriptCodeBlock::new("emptyblock"),
            threadsreceiver:HashMap::new(),
            threadssenders:HashMap::new(),
            tcplisteners:HashMap::new(),
            storage:NscriptStorage::new(),
            formattedblocks:HashMap::new(),
            //compiledblocks:Vec::new(),
            //tcp:NscriptTcp::new(),
        }
    }
    pub fn new() -> Nscript<'a> {
        let mut this = Nscript::setclean();
        this.setbasicfunctions();
        this.setcmdarguments();
        this
    }
    pub fn thread() -> Nscript<'a> {
        let mut this = Nscript::setclean();
        this.setcmdarguments();
        this
    }
    pub fn newthread(&mut self) -> Nscript<'a> {
        let mut this = Nscript::setclean();
        for xfn in self.rustfunctionsindex.clone(){
            if let Some(fnr) = self.rustfunctions.get(&xfn){
                this.insertfn(&xfn, fnr.clone());
            }
        }
        this.setcmdarguments();
        this
    }
    /// inserts a Rust function into the fnmap users can create their own function bindings using
    /// this. functions are required to have the NscriptFunc Trait implemented.
    pub fn insertstruct(&mut self, key: &'a str, value: &'a mut dyn  NscriptStructBinding)   {
        self.ruststructsindex.push(key.to_string());
        self.ruststructs.insert(key, value);
    }
    /// inserts function bindings from rust to Nscript these functions must be of public type NscriptSimpleFunctions
    pub fn insertfn(&mut self,name:&str,func: NscriptSimpleFunctions){
        self.rustfunctionsindex.push(name.to_string());
        self.rustfunctions.insert(name.to_string(),func);
    }
    fn setbasicfunctions(&mut self){
        // only done once , the whole Vec is copied during threading so it wont push twice
        if self.rustfunctionsindex.len() < 2 {
            self.insertfn("timerdiff", nscriptfn_timerdiff);
            self.insertfn("timerinit", nscriptfn_timerinit);
            self.insertfn("trim", nscriptfn_trim);
            self.insertfn("len", nscriptfn_len);
            self.insertfn("stringbetween", nscriptfn_stringbetween);
            self.insertfn("split", nscriptfn_split);
            self.insertfn("contains", nscriptfn_contains);
            self.insertfn("stringtoeval", nscriptfn_stringtoeval);
            self.insertfn("replace", nscriptfn_replace);
            self.insertfn("join", nscriptfn_join);
            self.insertfn("instring", nscriptfn_instring);
            self.insertfn("fromleft", nscriptfn_fromleft);
            self.insertfn("fromright", nscriptfn_fromright);
            self.insertfn("trimright", nscriptfn_trimright);
            self.insertfn("trimleft", nscriptfn_trimleft);
            self.insertfn("stringtohex", nscriptfn_stringtohex);
            self.insertfn("hextostring", nscriptfn_hextostring);
            self.insertfn("print", nscriptfn_print);
            self.insertfn("fileread", nscriptfn_fileread);
            self.insertfn("filewrite", nscriptfn_filewrite);
            self.insertfn("fileexists", nscriptfn_fileexists);
            self.insertfn("filedelete", nscriptfn_filedelete);
            self.insertfn("filemove", nscriptfn_filemove);
            self.insertfn("filecopy", nscriptfn_filecopy);
            self.insertfn("dirmove", nscriptfn_directory_move);
            self.insertfn("dirdelete", nscriptfn_directory_delete);
            self.insertfn("listdir", nscriptfn_listdir);
            self.insertfn("filesize", nscriptfn_filesize);
            self.insertfn("filesizebytes", nscriptfn_filesizebytes);
            self.insertfn("runwait", nscriptfn_call_programwait);
            self.insertfn("run", nscriptfn_call_program);
            self.insertfn("sleep", nscriptfn_sleep);
            self.insertfn("cat", nscriptfn_cat);
            self.insertfn("random", nscriptfn_random);
            self.insertfn("arraycontains", nscriptfn_arraycontains);
            self.insertfn("arraypush", nscriptfn_arraypush);
            self.insertfn("arrayretain", nscriptfn_arrayretain);
            self.insertfn("arrayshuffle", nscriptfn_arrayshuffle);
            self.insertfn("arrayreverse", nscriptfn_arrayreverse);
            self.insertfn("arraysearch", nscriptfn_arraysearch);
            self.insertfn("arrayfilter", nscriptfn_arrayfilter);
            self.insertfn("httpgetcontent", nscriptfn_get_http_content);
            self.insertfn("terminalinput", nscriptfn_terminalinput);
            self.insertfn("splitselect", nscriptfn_splitselect);
            self.insertfn("base64tostring", nscriptfn_base64tostring);
            self.insertfn("stringtobase64", nscriptfn_stringtobase64);
            self.insertfn("tcplistener", nscriptfn_tcplistener);
            self.insertfn("tcpaccept", nscriptfn_tcpaccept);
            self.insertfn("tcpconnect", nscriptfn_tcpconnect);
            self.insertfn("tcpdisconnect", nscriptfn_tcpdisconnect);
            self.insertfn("tcpreceive", nscriptfn_tcpreceive);
            self.insertfn("tcpsend", nscriptfn_tcpsend);
            self.insertfn("aabb_newbox", nscriptfn_aabb_newbox);
            self.insertfn("aabb_sizedbox", nscriptfn_aabb_sizedbox);
            self.insertfn("aabb_setposition", nscriptfn_aabb_setposition);
            self.insertfn("aabb_setrotation", nscriptfn_aabb_setrotation);
            self.insertfn("aabb_setscale", nscriptfn_aabb_setscale);
            self.insertfn("aabb_addtogroup", nscriptfn_aabb_addtogroup);
            self.insertfn("aabb_getgroup", nscriptfn_aabb_getgroup);
            self.insertfn("aabb_removefromgroup", nscriptfn_aabb_getgroup);
            self.insertfn("aabb_getcollisions", nscriptfn_aabb_getcollisions);
            self.insertfn("aabb_removegroup", nscriptfn_aabb_getcollisions);
            self.insertfn("decode_html_url", nscriptfn_decode_html_url);

        }
    }
    pub fn setcmdarguments(&mut self){
        let args: Vec<String> = env::args().collect();
        let mut i = 0;
        for givenarg in args.clone() {

            //println!("env:{}",&givenarg);
            let v = "".to_owned() + &givenarg.to_owned();
            let key ="$cmdarg".to_owned() + &i.to_string();
            //vmap.setvar(key, &v);
            let mut argvar = NscriptVar::new(&key);
            argvar.stringdata = v.to_string();
            self.storage.setglobal(&key, argvar);
            i +=1;
        }
    }
    pub fn threadsend(&mut self,threadname:&str,vartosend:NscriptVar) ->NscriptVar{
        let tothread = "thread_".to_string() + &threadname;
             match self.threadssenders.get(&tothread){
                Some(sender) => {
                match sender.send(vartosend){
                    Ok(_)=>{
                            //println!("main send succes!");
                            match self.threadsreceiver.get(&tothread){
                                Some(receiver) =>{
                                   let msg: NscriptVar = match receiver.try_recv(){
                                        Ok(m) =>m,
                                        Err(_) =>NscriptVar::new("error"),
                                    };
                                    match msg.stringdata.as_str(){
                                        _ =>{
                                            if msg.stringdata.as_str() != ""{
                                                //println!("main sent{} received:{}",&tothread,msg.stringdata);
                                                return msg;
                                            }
                                        }
                                    }
                                },
                                None => {
                                    println!("no thread [{}] receiver channel found!",&tothread);
                                }
                            }
                        },
                    Err(_)=>{
                            //println!("main[{}] send error! msg({})",&param1,&param2);
                            return NscriptVar::new("error");
                    }
                };
                    return NscriptVar::new("ok");
                }
                None => {
                    println!("no threads found");
                    return NscriptVar::new("ok");

                }
            };
    }
    pub fn removecoroutine(&mut self,routine:&str){
        self.coroutines.retain(|x| x != routine);
    }
    pub fn addcoroutine(&mut self,routine:&str){
        let string = routine.to_string();
        if self.coroutines.contains(&string) != true {
            self.coroutines.push(string);
        }
    }
    ///used internally to perform binded rust fn calls during interpretation
    // fn getfn(&mut self, key: &str) -> Option<&'a mut dyn NscriptFnBinding> {
    //    self.fnmap.get(key)
    // }

    pub fn insertclass(&mut self,name:&str,class:NscriptClass){
        self.storage.classes.insert(name.trim().to_string(),class);
    }
    pub fn getclass(&mut self,name:&str)->NscriptClass{
        if let Some(thisclass) = self.storage.classes.get_mut(name){
            thisclass.clone()
        }
        else{
            NscriptClass::new("")
        }
    }
    pub fn getfunc(&mut self,name:&str)->NscriptFunc{
        if let Some(thisclass) = self.storage.functions.get_mut(name){
            thisclass.clone()
        }
        else{
            NscriptFunc::new("notfounderror".to_string(),Vec::new())
        }
    }
    pub fn getclassref(&mut self,name:&str)->Option<&mut NscriptClass>{
        if let Some(thisclass) = self.storage.classes.get_mut(name.trim()){
            Some(thisclass)
        }
        else{
            None
        }
    }
    pub fn getblock(&mut self,blockref:&str)->NscriptCodeBlock{

        if let Some(this) = self.storage.codeblocks.get_mut(blockref){
            return this.copy();
        }
        else{
            print(&format!("returning a emptyblock for [{}]",&blockref),"r");
            return self.emptyblock.copy();
        }
    }
    pub fn getblockref(&mut self,blockref:&str)->Option<&mut NscriptCodeBlock>{
        if let Some(this) = self.storage.codeblocks.get_mut(blockref){
            return Some(this);
        }
        else{
            return None;
        }
    }
    pub fn njh_fromobject(&mut self, objectname:&str) -> String{
        let mut class = self.getclass(&objectname);
        let mut njhoutput = String::new();
        for xprop in class.index.clone(){
            njhoutput = njhoutput.to_string() + "#" + &xprop + "\n" + &class.getprop(&xprop).stringdata + "\n";
        }
        njhoutput.to_string()
    }
    pub fn object_to_json(&mut self,objectname:&str) ->NscriptVar{
        let mut var = NscriptVar::new("json");
        let mut jsonstring = String::from("{");
        if let Some(class) = self.storage.getclassref(&objectname){
            for propname in class.index.clone() {

                jsonstring = jsonstring + "\"" + &propname + "\": \"" + &class.getprop(&propname).stringdata + "\",";
            }
            if Nstring::fromright(&jsonstring, 1) == "," {
                jsonstring = Nstring::trimright(&jsonstring, 1);
            }
        }

        jsonstring = jsonstring + "}";
        var.stringdata = jsonstring;
        var
    }
    pub fn object_from_json(&mut self,objectname:&str,json:&str){
        let json = Nstring::trimright(&Nstring::trimleft(&json, 1), 1); // strip {}
        // if it exists, extent it
        if let Some(class) = self.storage.getclassref(&objectname){
            for each in split(&json, "\",") {
                let splitprop = split(&each, "\": \"");
                if splitprop.len() > 1 {
                    let mut var = NscriptVar::new("prop");
                    if Nstring::fromright(splitprop[1],1) == "\"" {
                        var.name = Nstring::trimleft(&splitprop[0],1);
                        var.stringdata =Nstring::trimright(&splitprop[1],1);
                        class.setprop(&Nstring::trimleft(&splitprop[0],1),var)
                    }
                    else{
                        var.name = Nstring::trimleft(&splitprop[0],1);
                        var.stringdata =splitprop[1].to_string();
                        class.setprop(&Nstring::trimleft(&splitprop[0],1),var)
                    }
                }
            }
        }
        // if it doesnt exist , create a new class
        else{
            let mut class = NscriptClass::new(&objectname);
            for each in split(&json, "\",") {
                let splitprop = split(&each, "\": \"");
                if splitprop.len() > 1 {
                    let mut var = NscriptVar::new("prop");
                    if Nstring::fromright(splitprop[1],1) == "\"" {
                        var.name = Nstring::trimleft(&splitprop[0],1);
                        var.stringdata =Nstring::trimright(&splitprop[1],1);
                       class.setprop(&Nstring::trimleft(&splitprop[0],1),var)
                    }
                    else{
                        var.name = Nstring::trimleft(&splitprop[0],1);
                        var.stringdata =splitprop[1].to_string();
                       class.setprop(&Nstring::trimleft(&splitprop[0],1),var)
                    }
                }
            }
            self.storage.classes.insert(objectname.to_string(),class);
        }
    }
    pub fn njh_objecttofile(&mut self,objectname:&str,filepath:&str){
        Nfile::write(&filepath,&self.njh_fromobject(objectname));
    }
    pub fn njh_filetoobject(&mut self, filepath:&str,objectname:&str){
        self.njh_stringtoobject(&Nfile::read(&filepath),&objectname);

    }
    pub fn njh_stringtoobject(&mut self, string:&str,objectname:&str){
        let mut filedata = string.to_string();
        filedata = Nstring::replace(&filedata, "\n#", "||==>");
        filedata = Nstring::trimleft(&Nstring::replace(&filedata, "\n", "|=>"),1);//strip off #;
        //
        let lines = split(&filedata,"||==>");
        if let Some(class) = self.getclassref(objectname){
            for xline in lines{
                let propdata = split(&xline,"|=>");
                if propdata.len() > 1 {
                    let mut var = NscriptVar::new(&propdata[0]);
                    var.stringdata = propdata[1].to_string();
                    class.setprop(&propdata[0], var);
                }
            }
        }else{
            let mut newclass = NscriptClass::new(&objectname);
            for xline in lines{
                let propdata = split(&xline,"|=>");
                if propdata.len() > 1 {
                    let mut var = NscriptVar::new(&propdata[0]);
                    var.stringdata = propdata[1].to_string();
                    newclass.setprop(&propdata[0], var);
                }
            }
            self.insertclass(&objectname, newclass);
        };
    }
    // pub fn getvar(&mut self,name:&str)->NscriptVar{
    //
    // }

}
pub enum NscriptDefinableTypes{
    Variable,
    Property,
    Array,
    Reflection,
    Global,
}
pub enum NscriptWordTypes {
    Variable,
    Property,
    Macro,
    Array,
    Reflection,
    Global,
    Function,
    Structfn,
    Number,
    Bool,
    Static,
    //Hexstring,
    Classfunc,
    Nestedfunc,
    Arraydeclaration,
}
pub struct NscriptStorage{
    pub globalvars:HashMap<String,NscriptVar>,
    pub codeblocks:HashMap<String,NscriptCodeBlock>,
    pub classes:HashMap<String,NscriptClass>,
    pub functions:HashMap<String,NscriptFunc>,
    pub tcp: NscriptTcp,
    pub nscript3d: Nscript3d,
}
impl NscriptStorage{
    pub fn new() ->NscriptStorage{
        NscriptStorage{
            globalvars:HashMap::new(),
            codeblocks:HashMap::new(),
            classes:HashMap::new(),
            functions:HashMap::new(),
            tcp:NscriptTcp::new(),
            nscript3d:Nscript3d::new(),
        }
    }
    pub fn getglobal(&mut self,name:&str) ->NscriptVar{
        if let Some(res) = self.globalvars.get_mut(name){
            return res.clone();
        }
        NscriptVar::new(name)//<-not found,ret new
    }
    pub fn setglobal(&mut self,name:&str,data:NscriptVar){
        self.globalvars.insert(name.to_string(), data);
    }
    /// duplicate also in neocatvar, this one is for rustfnbinds
    pub fn getclassref(&mut self,name:&str)->Option<&mut NscriptClass>{
        if let Some(thisclass) = self.classes.get_mut(name.trim()){
            Some(thisclass)
        }
        else{
            None
        }
    }
    /// used for simplerustfn for retrieving strings
    pub fn getargstring(&mut self,word:&str,block: &mut NscriptCodeBlock) -> String{
        match self.argtype(word){
            NscriptWordTypes::Static =>{
                return block.staticstrings[Nstring::trimleft(word, 1).parse::<usize>().unwrap_or(0)].to_string();
            }
            NscriptWordTypes::Global => {
                return self.getglobal(&word).stringdata.to_string();
            }
            NscriptWordTypes::Variable=>{
                return block.getstring(word);
            }
            NscriptWordTypes::Macro => {
                return self.getmacrostring(word);
            }
            NscriptWordTypes::Property=>{
                let wordsplit = split(&word,".");
                if wordsplit.len() > 1{
                    let mut cname = wordsplit[0].trim().to_string();
                    let mut pname = wordsplit[1].trim().to_string();
                    if Nstring::fromleft(&wordsplit[0], 1) ==  "*" {
                        cname = self.getargstring(&Nstring::trimleft(&wordsplit[0],1), block);
                    }
                    if Nstring::fromleft(&cname, 1) ==  "$" {
                        cname = self.getargstring(&wordsplit[0], block);
                    }
                    if Nstring::fromleft(&wordsplit[1], 1) ==  "*" {
                        pname = self.getargstring(&Nstring::trimleft(&wordsplit[1], 1),block) ;
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname).stringdata.to_string();
                    }else{
                        print(&format!("word is a prop but theres no class on cname [{}] pname[{}]",&cname,&pname),"r");
                        return "".to_owned();
                    }
                }
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimleft(word, 1);
                let evaluated = self.getargstring(&toreflect, block).to_string();
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let arrays = split(word,"[");
                    let thisvar = self.getargstringvec(arrays[0], block);
                    let index = self.getargstring(&split(&arrays[1], "]")[0],block).parse::<usize>().unwrap_or(0);
                    if thisvar.len() > index{
                        return thisvar[index].to_string();
                    }else{
                    print(&format!("array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&arrays[0],&index,&thisvar.len()),"r");
                }
                return "".to_owned();
            }
            _ => {
            }
        }
        word.to_string()
    }


    //used to get the Vec<String> from injected simplerustfn
    pub fn getargstringvec(&mut self,word:&str,block: &mut NscriptCodeBlock) -> Vec<String>{
        match self.argtype(word){
            NscriptWordTypes::Global => {
                return self.getglobal(&word).stringvec;
            }
            NscriptWordTypes::Variable=>{
                return block.getstringvec(word);
            }

            NscriptWordTypes::Property=>{
                let wordsplit = split(&word,".");
                if wordsplit.len() > 1{
                    let mut cname = wordsplit[0].trim().to_string();
                    let mut pname = wordsplit[1].trim().to_string();
                    if Nstring::fromleft(&wordsplit[0], 1) ==  "*" {
                        cname = self.getargstring(&Nstring::trimleft(&wordsplit[0],1), block);
                    }
                    if Nstring::fromleft(&cname, 1) ==  "$" {
                        cname = self.getargstring(&wordsplit[0], block);
                    }
                    if Nstring::fromleft(&wordsplit[1], 1) ==  "*" {
                        pname = self.getargstring(&Nstring::trimleft(&wordsplit[1], 1),block) ;
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname).stringvec;
                    }else{
                        print(&format!("word is a prop but theres no class on cname [{}] pname[{}]",&cname,&pname),"r");
                    }
                }
            }
            _ => {
            }
        }

        Vec::new()
    }

    /// used to get the type of argument thats been given to a simplerustfn
    pub fn argtype(&mut self,word:&str) ->NscriptWordTypes{
        if word.parse::<f64>().is_ok(){
            return NscriptWordTypes::Number;//"number".to_string();
        }
        if Nstring::instring(&word, ".") && Nstring::instring(&word, "(") == false{
            return NscriptWordTypes::Property;//"property".to_string();
        }
        if Nstring::fromleft(word, 1) != "[" && Nstring::instring(&word, "[") &&  Nstring::instring(&word, "]") {
            return NscriptWordTypes::Array;//"array".to_string();
        }

        if word == "true" || word == "false"{
            return NscriptWordTypes::Bool;//"bool".to_string();
        }
        let prefix = Nstring::fromleft(word, 1);
        match prefix.as_str(){
            "$" => {
                return NscriptWordTypes::Global;//"global".to_string();
            }
            "~" => {
                return NscriptWordTypes::Static;//"static".to_string();
            }
            "@" => {
                return NscriptWordTypes::Macro;//"macro".to_string();
            }
            "*" => {
                return NscriptWordTypes::Reflection;//"reflection".to_string();
            }
            _ => {
                return NscriptWordTypes::Variable;//"variable".to_string();
            }
        }
    }
    pub fn getmacrostring(&mut self,word:&str)->String{

        let time = chrono::Utc::now();
        let macrostring  = match word{
            "@nscriptversion" => String::from(NSCRIPT_VERSION),
            "@arrowright" => "→".to_string(),
            "@arrowleft" => "←".to_string(),
            "@arrowup" => "↑".to_string(),
            "@arrowdown" => "↓".to_string(),
            "@boxhorizontalline" => "─".to_string(),
            "@boxverticalline" => "│".to_string(),
            "@boxcorner1" => "┌".to_string(),
            "@boxcorner2" => "┐".to_string(),
            "@boxcorner3" => "└".to_string(),
            "@boxcorner4" => "┘".to_string(),

            "@nscriptpath" => {
                let  string = "~/.neocat".to_string();
                //var.stringdata
                if let Ok(value) = env::var("NEOCATPATH") {
                    value
                }else{
                    string
                }
            }
            "@webpublic" => {
                NC_SCRIPT_DIR.to_owned()
                + "domains/"
                + &split(&self.getglobal("$domainname").stringdata, ":")[0]
                + "/public/"

            }
            "@webprivate" => {
                NC_SCRIPT_DIR.to_owned()
                + "domains/"
                + &split(&self.getglobal("$domainname").stringdata, ":")[0]
                + "/private/"
            }
            "@webroot" => {
                NC_SCRIPT_DIR.to_owned()
                + "domains/"
                + &split(&self.getglobal("$domainname").stringdata, ":")[0]
                + "/"
            }
            "@quote" => {
                "\"".to_string()
            }
            "@year" => {
                time.year().to_string()
            }
            "@month" => time.month().to_string(),
            "@day" => time.day().to_string(),
            "@hour" => time.hour().to_string(),
            "@min" => time.minute().to_string(),
            "@now" => time.day().to_string() + "/" + &time.month().to_string() + "/" + &time.year().to_string() + " "+ &time.hour().to_string() + ":" + &time.minute().to_string() + ":" + &time.second().to_string() + "." + &time.timestamp_millis().to_string(),
            "@date" => time.day().to_string() + "/" + &time.month().to_string() + "/" + &time.year().to_string(),
            "@exacttime" => time.hour().to_string() + ":" + &time.minute().to_string() + ":" + &time.second().to_string() + "." + &time.timestamp_millis().to_string(),
            "@time" => time.hour().to_string() + ":" + &time.minute().to_string(),
            "@OS" => MACRO_OS.to_string(),
            "@scriptdir" => NC_SCRIPT_DIR.to_string(),
            "@programdir" => NC_PROGRAM_DIR.to_string(),
            "@sec" => time.second().to_string(),
            "@msec" => time.timestamp_millis().to_string(),
            "@socketip" => self.getglobal("$socketip").stringdata,
            "@crlf" => String::from("\r\n"),
            "@lf" => String::from("\n"),
            "@error" => self.getglobal("$error").stringdata,
            "@emptystring" | _ =>{
                "".to_string()
            }
        };
        macrostring

    }

}
/// in construction !!!
// pub struct NscriptCompiledCodeBlock <'a>{
//     pub codeblock: Vec<Vec<&'a str>>,
//     pub subblockmap: Vec<Vec<Vec<&'a str>>>,
// }
// impl <'a> NscriptCompiledCodeBlock <'a> {
//     pub fn new() -> NscriptCompiledCodeBlock<'a>{
//         NscriptCompiledCodeBlock{
//             codeblock:Vec::new(),
//             subblockmap:Vec::new(),
//         }
//     }
//     pub fn compileblock<'b>(codeblockvector:&'b Vec<Vec<String>>,subblockmap: &'b Vec<Vec<Vec<String>>>) -> NscriptCompiledCodeBlock<'a>{
//         let mut compiledblock = NscriptCompiledCodeBlock::new();
//         let max = codeblockvector.len();
//         for xline in 0..max-1{
//             compiledblock.codeblock[0] = codeblockvector[xline].iter().map(String::as_str).collect();
//         }
//         // let mut index = 0;
//         // for _ in 0..subblockmap.len(){
//         //     compiledblock.subblockmap.push(Vec::new());
//         //     let max = subblockmap[index].len();
//         //     let mut i2 =0;
//         //     for _ in 0..max-1{
//         //         compiledblock.subblockmap[index].push(subblockmap[index][i2].iter().map(String::as_str).collect());
//         //         i2 +=1;
//         //     }
//         //     index +=1;
//         // }
//         return compiledblock;
//     }
// }

/// Stores the codevector so they can be used a inmutable for iterations while keeping codeblock
/// mutable for storage, at the end of the formatting the vectors from codeblock will copy ones
/// here.
pub struct NscriptFormattedCodeBlock{

    pub codeblock: String,
    pub code: Vec<Vec<Vec<String>>>,// all the subscopes, if else loop coroutines
}
impl NscriptFormattedCodeBlock{
    pub fn new()->NscriptFormattedCodeBlock{
        NscriptFormattedCodeBlock{
            codeblock: String::new(),
            code: Vec::new(),
        }
    }
    pub fn clone(&mut self)->NscriptFormattedCodeBlock{
        NscriptFormattedCodeBlock{
            codeblock: String::new(),
            code: self.code.clone(),
        }
    }
    // pub fn setfromblock(&mut  self, block:&mut NscriptCodeBlock){
    //         self.code = block.subblockmap.clone()
    // }
}

/// this struct contains the vectors with code
pub struct NscriptCodeBlock{
    pub name: String,
    pub insubblock: usize,// all the subscopes, if else loop coroutines
    pub strings: HashMap<String,String>,// scope variables
    pub stringsvec: HashMap<String,Vec<String>>,// scope variables
    pub staticstrings: Vec<String>,// scope variables
    pub ifscopedepth: usize,//used for parsing nested ifscopes
    pub ifscopes: Vec<bool>,// used for nested elseif else scopes
    pub inloop: usize, // used for nested loops
    pub breakloop: Vec<bool>, // used to break the right nested loop.
    pub formattedcode: NscriptFormattedCodeBlock, // used to break the right nested loop.
}

impl NscriptCodeBlock{
    pub fn new(nameref:&str) -> NscriptCodeBlock{
        let mut this = NscriptCodeBlock{
            name: nameref.to_string(),
            insubblock: 0,
            strings: HashMap::new(),
            stringsvec: HashMap::new(),
            staticstrings: Vec::new(),
            ifscopedepth: 0,
            ifscopes: Vec::new(),
            inloop: 0,
            breakloop: Vec::new(),
            formattedcode: NscriptFormattedCodeBlock::new(),
        };
        this.ifscopes.push(false);
        this.breakloop.push(false);
        this
    }
    pub fn copy(&mut self) ->NscriptCodeBlock{
        let mut this = NscriptCodeBlock{
            name: self.name.to_string(),
            insubblock: self.insubblock.clone(),
            strings: self.strings.clone(),
            stringsvec: self.stringsvec.clone(),
            staticstrings: self.staticstrings.clone(),
            ifscopedepth: self.ifscopedepth.clone(),
            ifscopes: self.ifscopes.clone(),
            inloop: self.inloop.clone(),
            breakloop: self.breakloop.clone(),
            formattedcode: self.formattedcode.clone(),

        };
        this.setstring("self",self.getstring("self"));
        this
    }

    pub fn setstring(&mut self,namref:&str,string:String){
        self.strings.insert(namref.to_string(),string);
    }
    pub fn getstring(&mut self,namref:&str) ->String{
        if let Some(data) = self.strings.get(&namref.to_string()){
            return data.to_string();
        }
        else{
            return "".to_string();
        }
    }
    /// stored Vec<String> for NscriptVar types.
    pub fn setstringvec(&mut self,namref:&str,stringvec:Vec<String>){
        self.stringsvec.insert(namref.to_string(),stringvec);
    }

    /// stored Vec<String> for NscriptVar types.
    pub fn getstringvec(&mut self,namref:&str) ->Vec<String>{
        if let Some(data) = self.stringsvec.get(&namref.to_string()){
            return data.to_owned();
        }
        else{
            return Vec::new();
        }
    }
    pub fn subblocktostring(&mut self,subblock:usize) -> String{
        let mut outputstring = "".to_string();
        for xline in self.formattedcode.code[subblock-1].clone(){
            outputstring = outputstring + &xline.join(" ") + "\n";
        }
        return outputstring
    }
    pub fn blocktostring(&mut self) -> String{
        let mut outputstring = "".to_string();
        for xline in self.formattedcode.code[0].clone(){
            outputstring = outputstring + &xline.join(" ") + "\n";
        }
        return outputstring
    }
    pub fn ifset(&mut self,set:bool){
        self.ifscopes[self.ifscopedepth] = set;
    }
    pub fn ifdown(&mut self){
        self.ifscopedepth -=1;
        self.ifscopes =self.ifscopes[0..self.ifscopes.len()-1].to_vec();
    }
    pub fn ifup(&mut self){
        self.ifscopes.push(false);
        self.ifscopedepth +=1;
    }
    pub fn formatargumentspaces(&mut self, code: &str) -> String {
        let mut line: String; // buffer used for changes
        let mut fixed = String::new(); // used to return
        let mut linebuf: String;
        let fixemptyargscode = Nstring::replace(&code, "()", "(\"\")");
        for each in split(&fixemptyargscode, "\n") {
            // loop lines
            line = each.to_string(); // set default
            linebuf = line.clone(); // create a buffer we can strip
            loop {
                let getbetween = Nstring::stringbetween(&linebuf, "(", ")");
                //check if "" means its done.
                if getbetween == "" {
                    break;
                }
                // create a fixed string
                let fixbetween = Nstring::replace(&getbetween, " ", "");
                line = Nstring::replace(&line, &getbetween, &fixbetween);
                // strip the buf from what its done, and loop on.
                let bufstrip = split(&linebuf, &getbetween);
                let tostrip = bufstrip[0].to_owned() + &getbetween;
                linebuf = Nstring::replace(&linebuf, &tostrip, "");
            }
            fixed = fixed + &line + "\n";
        }
        fixed
    }
    /// pre-formatting: all the converted hexed static strings will be assinged a variable
    fn convertstaticstrings(&mut self){
        let mut parsingtext = Nstring::replace(&self.formattedcode.codeblock.to_string(),"\"{","\" {");
        let mut staticstringcount = 0;
        let chars = ["\n"," ",",",")","]"];
        for xchar in chars{
            loop {
                let mut hexstring = Nstring::stringbetween(&parsingtext, "^", &xchar);
                if hexstring == ""{ // break loop when block is done
                    break;
                }
                for xchar2 in chars{// get first on the line.
                    hexstring = split(&hexstring,&xchar2)[0].to_string();
                }
                if hexstring != "" {
                    let stringvarname = "~".to_string() + &staticstringcount.to_string();
                    staticstringcount +=1;
                    for xchar2 in chars{
                        let torep = "^".to_string() + &hexstring + &xchar2;
                        let repwith = stringvarname.to_string() + &xchar2;
                        parsingtext = Nstring::replace(&parsingtext, &torep, &repwith);
                    }
                    let stringdata = hex_to_string(&hexstring);
                    self.staticstrings.push(stringdata.to_string());
                }
            }
        }
        self.staticstrings.push("".to_string());
        self.formattedcode.codeblock = parsingtext.to_string();
    }
    /// tokenizing and assinging subscopes
    pub fn formatblock(&mut self) {
        //self.subblockmap = Vec::new();
        self.convertstaticstrings();
        let mut parsingtext = self.formattedcode.codeblock.to_string();
        let mut toreturn: String;
        let mut scopecounter = 1;
        self.breakloop.push(false);
        self.formattedcode.code.push(Vec::new());
        loop {
            let splitstr = split(&parsingtext, "{");
            if splitstr.len() > 1 {
                let isscope = split(&splitstr[splitstr.len() - 1], "}")[0];
                scopecounter +=1;
                let scopeid = scopecounter.to_string();
                let scopekey =" SCOPE ".to_string() + &scopeid;
                self.breakloop.push(false);
                let formattedscope = self.formatargumentspaces(&isscope);
                let codevec = self.codetovector(&formattedscope);
                self.formattedcode.code.push(codevec);
                let toreplace = "{".to_owned() + &isscope + "}";
                parsingtext = Nstring::replace(&parsingtext, &toreplace, &scopekey);
                parsingtext = Nstring::replace(&parsingtext,"  ", " ");
            } else {
                toreturn = split(&splitstr[0], "}")[0].to_string();
                break;
            }
        }
        toreturn = self.formatargumentspaces(&toreturn);
        let codevec = self.codetovector(&toreturn);
        self.formattedcode.code[0] = codevec.clone();
        //self.codeblockvector = codevec;
        self.formattedcode.codeblock = toreturn.to_string();

    }
    ///creates a Vector lines Vector words, used for parsing.
    pub fn codetovector(&mut self,code: &str) -> Vec<Vec<String>>{
        let mut codearray: Vec<Vec<String>> = Vec::new();
        let linearray: Vec<String> = code.split("\n").map(|s| s.trim().to_string()).collect();
        for line in &linearray{
            let wordvec = line.trim().split(" ").map(|s| s.to_string()).collect();
            codearray.push(wordvec);
        }
        codearray
    }
    /// used for pre formatting,
    pub fn setcode(&mut self, codestring:String){
        //print(&codestring,"p");
        self.formattedcode.codeblock = codestring.to_string();
    }
    /// stores formatted code
     pub fn setcodevector(&mut self, codestring:Vec<Vec<String>>){
        self.formattedcode.code[0] = codestring;
    }
    ///gets the stringdata of a variable inside the block hashmap of a referenced variable
    // pub fn getvarstring(&mut self,name:&str)->String{
    //     let get = self.variables.get_mut(name);
    //     if let Some(this) = get{
    //         //this.stringdata = "aaa".to_string();
    //         return this.stringdata.to_string();
    //     }
    //     else{
    //         let thisvar = NscriptVar::new(name);
    //         self.variables.insert(name.to_string(), thisvar);
    //         return "".to_string();
    //     }
    // }
    // ///sets the stringdata of a variable inside the block hashmap to a referenced variable
    // pub fn setvarstring(&mut self,name:&str,data:&str){
    //     if let Some(this) = self.variables.get_mut(name){
    //         this.stringdata = data.to_string();
    //     }
    //     else{
    //         let mut thisvar = NscriptVar::new(name);
    //         thisvar.stringdata = data.to_string();
    //         self.variables.insert(name.to_string(), thisvar);
    //     }
    // }
    ///copies a variable from the block for mutable purposes
    pub fn getvar(&mut self,name:&str)->NscriptVar{
        let mut var = NscriptVar::new(&name);
        if let Some(string) = self.strings.get(name){
             var.stringdata = string.to_owned();
        }
        else{
             var.stringdata = "".to_owned();
        };
        if let Some(stringvec) = self.stringsvec.get(name){
             var.stringvec = stringvec.to_owned();
        }
        else{
             var.stringvec = Vec::new();
        };
        var
    }
    // pub fn getvarreference(&mut self,name:&str)->Option<&mut NscriptVar>{
    //
    //     if let Some(var) = self.variables.get_mut(name){
    //         return Some(var);
    //     }
    //     else{
    //         return None;
    //     };
    // }
    pub fn setvar(&mut self,name:&str,var:NscriptVar){
        self.strings.insert(name.to_string(),var.stringdata );
        self.stringsvec.insert(name.to_string(),var.stringvec);
        // if self.variableindex.contains(&name.to_string()) == false{
        //     self.variableindex.push(name.to_string());
        // }
        //self.variables.insert(name.to_string(), var);

    }
}
/// implement this to add new Nscript rust functions and bind them
pub trait NscriptStructBinding {
    fn neocat_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>) -> NscriptVar;
}
/// Temp struct for executing scopes, disposes when done. ( garbage collector)
pub struct NscriptScriptScope{
    name: String,
    pub  classrefs: Vec<String>,
    // pub variables:HashMap<String,NscriptVar>,
}

impl NscriptScriptScope{
    pub fn new(name:String)->NscriptScriptScope{
        NscriptScriptScope{
            name: name,
            classrefs: Vec::new(),
        }
    }
    pub fn name(&self) -> String{
        return self.name.to_string();
    }
}

pub struct NscriptVar{
    pub name: String,// in string
    pub stringdata: String,
    pub stringvec: Vec<String>,
}
/// Variable struct holds the neocat datatypes and data
impl NscriptVar{
    pub fn new(name:&str) -> NscriptVar{
        NscriptVar{
            name: name.to_string(),
            stringdata: "".to_string(),
            stringvec:Vec::new(),
        }
    }
    /// copies so they can be included to other blocks
    pub fn clone(&self)->NscriptVar{
        let  new = NscriptVar{
            name: self.name.to_string(),
            stringdata:self.stringdata.to_string(),
            stringvec:self.stringvec.clone(),
        };
        new
    }
    /// returns the string value of the variable
    pub fn getstring(&mut self) -> String{
        return self.stringdata.to_string();
    }
    pub fn getnumber(&mut self) -> u64{
        return self.stringdata.parse::<u64>().unwrap_or(0);
    }
    pub fn setstring(&mut self,newstring:&str){
        self.stringdata = newstring.to_string()
    }
}
/// Nscript user scripted functions
pub struct NscriptFunc{
    pub name: String,
    pub args:Vec<String>,
    pub codeblock:NscriptCodeBlock,
}

impl NscriptFunc{
    pub fn new(name:String,args:Vec<String>)->NscriptFunc{
        NscriptFunc{
            name: name.to_string(),
            args: args,
            codeblock: NscriptCodeBlock::new(&name),
        }
    }
    pub fn clone(&mut self)-> NscriptFunc{
        NscriptFunc{
            name:self.name.to_string(),
            args:self.args.clone(),
            codeblock:self.codeblock.copy(),
        }
    }
}


pub struct NscriptClass{
    pub name: String,
    pub index: Vec<String>,
    properties: HashMap<String,NscriptVar>,
    pub functionindex: Vec<String>,
    pub functions: HashMap<String,NscriptFunc>,
    pub parents: Vec<String>,
    pub children: Vec<String>,
}

impl NscriptClass{
    pub fn new(name:&str) -> NscriptClass{
        NscriptClass{
            name: name.to_string(),
            index: Vec::new(),
            properties: HashMap::new(),
            functionindex: Vec::new(),
            functions: HashMap::new(),
            parents: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn clone(&mut self) -> NscriptClass{
        let mut this = NscriptClass{
            name: self.name.to_string(),
            index: Vec::new(),//,
            properties: HashMap::new(),
            functionindex:Vec::new(),//self.functionindex.clone(),
            functions:HashMap::new(),
            parents: self.parents.clone(),
            children: self.children.clone(),
        };
        for xprop in self.index.clone(){
            this.setprop(&xprop, self.getprop(&xprop));
        };
        for xprop in self.functionindex.clone(){
            this.setfunc(&xprop, self.getfunc(&xprop));
        };
        this
    }
    pub fn copyto(&mut self, name:&str) -> NscriptClass{
        let mut this = NscriptClass{
            name: name.to_string(),
            index: self.index.clone(),
            properties: HashMap::new(),
            functionindex:Vec::new(),
            functions:HashMap::new(),
            parents: Vec::new(),
            children: Vec::new(),
        };

        for xprop in self.index.clone(){
            this.setprop(&xprop, self.getprop(&xprop));
        };
        for xprop in self.functionindex.clone(){
            this.setfunc(&xprop, self.getfunc(&xprop));
        };
        this.parents.push(self.name.to_string());
        self.children.push(this.name.to_string());

        this
    }
    /// expands a class with some other class.
    pub fn inherent(&mut self, fromclass:&mut NscriptClass){
        for xprop in fromclass.index.clone(){
            self.setprop(&xprop, fromclass.getprop(&xprop));
        };
        for xprop in fromclass.functionindex.clone(){
            self.setfunc(&xprop, fromclass.getfunc(&xprop));
        };
        self.parents.push(fromclass.name.to_string());
        fromclass.children.push(self.name.to_string());
    }
    pub fn setprop(&mut self, name:&str,prop:NscriptVar){
        if name != ""{
            if let Some(_) = self.properties.get(name){
                self.properties.insert(name.to_string(),prop);
            }
            else{
                self.index.push(name.to_string());
                self.properties.insert(name.to_string(),prop);
            }
        }

    }
    pub fn getprop(&mut self,name:&str) ->NscriptVar{
        if let Some(this) = self.properties.get_mut(name){
            this.clone()
        }
        else{
            NscriptVar::new(name)
        }
    }
    pub fn removeprop(&mut self,name:&str){
        self.index.retain(|x| x != name);
        self.properties.remove(name);
    }
    pub fn removefunc(&mut self,name:&str){
        self.functionindex.retain(|x| x != name);
        self.functions.remove(name);
    }
    pub fn setfunc(&mut self, name:&str,mut prop:NscriptFunc){
        let mut var = NscriptVar::new("self");
        var.stringdata = self.name.to_string();
        prop.codeblock.setvar("self", var);
        if let Some(_) = self.functions.get_mut(name){
            self.functions.insert(name.to_string(),prop);
        }
        else{
            self.functionindex.push(name.to_string());
            self.functions.insert(name.to_string(),prop);
        }
    }
    pub fn getfunc(&mut self,name:&str) ->NscriptFunc{
        if let Some(this) = self.functions.get_mut(name){
            return this.clone();
        }
        else{
            print(&format!("NscriptClass: Cant get func [{}]",&name),"r");
            NscriptFunc::new(name.to_string(),Vec::new())
        }
    }
    pub fn getfuncref(&mut self,name:&str) ->Option<&NscriptFunc>{
        if let Some(this) = self.functions.get(name){
            return Some(this);
        }
        else{
            None
        }
    }
}
pub struct Njh {

}

impl Njh {
    // a clone of the first functions i ever wrote back in 2008.
    // it saves a header with a entree to a .njh file
    // it splits by lines1, if found next line be result
    // load("#name"1,filename) / save("#name"1,namevar1,filename)
    // can be used to fastly load settings for programs.

    pub fn write(header: &str,data: &str,file: &str) {
        let dataf = Nfile::read(&file);
         Nfile::write(&file,&Njh::writeinvar(&header,&data,&dataf));
    }
    pub fn writeinvar(header: &str,newln:&str,data: &str) -> String{
        let mut check = false;
        let mut vec: Vec<&str> = vec![];
        let mut isfound = false;
        for line in data.lines() {
            if check == true {
                vec.push(newln);
                check = false; //done
                isfound = true;
            }else {
              vec.push(line);
            }
            if line == header {
                check = true;
            }
        }
        let mut outputdata = String::new();
        for lines in vec {
            outputdata = outputdata + lines + &NC_LINE_ENDING;
        }
        if isfound == false{
            outputdata = outputdata  + header + &NC_LINE_ENDING + newln+ &NC_LINE_ENDING;
        }
        return outputdata;
    }

    pub fn read(header: &str,file: &str) -> String {
        let data = Nfile::read(file);
       return Njh::readinvar(header,&data);
    }

    pub fn readinvar(header: &str,data: &str) -> String {
        let mut check = false;
        for line in data.to_owned().lines() {
            if check == true {
                return line.to_owned();
            }
            if line == header {
                check = true;
            }
        }
        return "@error".to_owned();
    }


}

