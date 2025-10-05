
use crate::*;
//pub type NscriptSimpleFunctions = fn(&Vec<NscriptVar>) -> NscriptVar;
pub type NscriptSimpleFunctions = fn(&Vec<&str>,block:&mut NscriptCodeBlock, &mut NscriptStorage) -> NscriptVar;

/// NscriptScript main struct
pub struct Nscript{
    // for user created structs
    pub ruststructsowned: HashMap<Box<str>,Box<dyn NscriptStructBinding>>, // map for all the rust fn bindings.
    //pub ruststructs: HashMap<&'a str, &'a mut dyn NscriptStructBinding>, // map for all the rust fn bindings.
    pub ruststructsindex: Vec<String>, // map for all the rust fn bindings.
    pub rustfunctions: HashMap<Box<str>, NscriptSimpleFunctions>, // map for all the rust fn bindings.
    pub rustfunctionsindex: Vec<String>, // map for all the rust fn bindings.
    pub rustfunctionshelpindex: Vec<String>, // map for all the rust fn bindings.
    pub coroutinesindex: Vec<String>,// all nonclass functions
    pub coroutines: HashMap<Box<str>,NscriptCoroutine>,// all nonclass functions
    pub emptyblock: NscriptCodeBlock,// all nonclass functions
    pub threadsreceiver: HashMap<String, mpsc::Receiver<NscriptVar>>,
    pub threadssenders: HashMap<String, mpsc::Sender<NscriptVar>>,
    pub tcplisteners: HashMap<String,TcpListener>,
    pub storage: NscriptStorage,
    pub formattedblocks: HashMap<String,NscriptFormattedCodeBlock>,
    pub executableblocks: HashMap<String,NscriptExecutableCodeBlock>,
    //pub codestorage: CodeStorage,
    pub userfunctions: HashMap<String,NscriptFunc>,
    pub emptyexecutableblock: NscriptExecutableCodeBlock,// <- so we can send back a ref
}
//ok

impl  Nscript{
    fn setclean() ->Nscript{
        Nscript {
            ruststructsowned: HashMap::new(),
            //ruststructs: HashMap::new(),
            ruststructsindex: Vec::new(),
            rustfunctions: HashMap::new(),
            rustfunctionsindex: Vec::new(),
            rustfunctionshelpindex: Vec::new(),
            coroutinesindex: Vec::new(),
            coroutines: HashMap::new(),
            emptyblock: NscriptCodeBlock::new("emptyblock"),
            threadsreceiver:HashMap::new(),
            threadssenders:HashMap::new(),
            tcplisteners:HashMap::new(),
            storage:NscriptStorage::new(),
            formattedblocks:HashMap::new(),
            executableblocks:HashMap::new(),
            //codestorage: CodeStorage::new(),
            userfunctions:HashMap::new(),
            emptyexecutableblock:NscriptExecutableCodeBlock::new(),
        }
    }
    pub fn new() -> Nscript {
        let mut this = Nscript::setclean();
        this.setbasicfunctions();
        this.setcmdarguments();
        this
    }
    pub fn thread() -> Nscript {
        let mut this = Nscript::setclean();
        this.setcmdarguments();
        this
    }
    pub fn newthread(&mut self) -> Nscript{
        let mut this = Nscript::setclean();
        for xfn in self.rustfunctionsindex.clone(){
            if let Some(fnr) = self.rustfunctions.get(xfn.as_str().into()){
                this.insertfn(&xfn, fnr.clone(),"");
            }
        }
        this.setcmdarguments();
        this
    }


    /// inserts a Rust function into the fnmap users can create their own function bindings using
    /// this. functions are required to have the NscriptFunc Trait implemented.
    pub fn insertstructowned(&mut self, key: &str, value: impl NscriptStructBinding +'static)   {
        self.ruststructsindex.push(key.to_string());
        self.ruststructsowned.insert(key.to_string().into(), Box::new(value));
    }
    pub fn insertuserfunction(&mut self, key: String, value: NscriptFunc)   {
        self.userfunctions.insert(key, value);
    }
    /// inserts function bindings from rust to Nscript these functions must be of public type NscriptSimpleFunctions
    pub fn insertfn(&mut self,name:&str,func: NscriptSimpleFunctions,explanationstring:&str){
        self.rustfunctionsindex.push(name.to_string());
        self.rustfunctionshelpindex.push(explanationstring.to_string());
        self.rustfunctions.insert(name.into(),func);
    }
    fn setbasicfunctions(&mut self){
        // only done once , the whole Vec is copied during threading so it wont push twice
        if self.rustfunctionsindex.len() < 2 {
            self.insertfn("printraw", nscriptfn_printraw,"(string,color) // returns the given string, colors first char as a string \"r\" for red, blue,purple yellow green,magenta, for brightcolors add \"b\" so for red \"br\" for bright blue \"bb\"");
            self.insertfn("cos", nscriptfn_cos,"(number) // cos on number");
            self.insertfn("sin", nscriptfn_sin,"(number) // sin on number");
            self.insertfn("add", nscriptfn_add,"(number,toadd)  // adds the number given by the numberto add");
            self.insertfn("subtract", nscriptfn_subtract,"(number,tosubtract)  // subtracts the number given by the numbertosubtract");
            self.insertfn("multiply", nscriptfn_multiply, "(number,tomultiply)  // multiplies the number given by the numbertomultiply");
            self.insertfn("devide", nscriptfn_devide, "(number,todevide)  // subtracts the number given by the numbertodevide");
            self.insertfn("add", nscriptfn_isalpthabetic,"(number,toadd)  // adds the number given by the numberto add");
            self.insertfn("is_alphabetic", nscriptfn_isalpthabetic,"(string)  // Checks if a string is alphabetic, returns a bool");
            self.insertfn("timerdiff", nscriptfn_timerdiff,"(timerinit)  // takes a timervar created by timerinit() returns the difference in ms ");
            self.insertfn("timerinit", nscriptfn_timerinit,"()  // returns a timervar, can be used with timerdiff(var) to check difference in time ms");
            self.insertfn("trim", nscriptfn_trim,"(string)  // returns a trimmed string, this strips spaces in front and at end");
            self.insertfn("len", nscriptfn_len,"(vec)  // takes a vector returns the size");
            self.insertfn("vec", nscriptfn_vec,"(string,a,b,c,..endless)  // can take any ammount of arguments , creates a vector out of each given argument and returns it.");
            self.insertfn("uppercase", nscriptfn_toupper,"(string)  // returns the given string in UPPERcase");
            self.insertfn("replacebyref", nscriptfn_replacebyref,"(referencevar,find,replace)  // this doesnt return anything, it will replace found string in the var given as referencevar,\n if using multiple lines with replace() this will optimize it by a lot");
            self.insertfn("lowercase", nscriptfn_tolower,"(string)  // returns the given string as lowercase");
            self.insertfn("stringbetween", nscriptfn_stringbetween,"(string,beginstring,endstring)  // searches a string by a begin and end, returns the first result as a string.\n if none found returns a empty string");
            self.insertfn("split", nscriptfn_split,"(string,splitdelimeter)  // splits a string into a vector by a given delimeter. \n to split a string by each character give a empty string as delimeter");
            self.insertfn("contains", nscriptfn_contains,"(string,containsstring)  // returns a bool if a string contains a substring");
            self.insertfn("stringtoeval", nscriptfn_stringtoeval,"(string)  // replaces all spaces and special characters of a given string \n usecases: to set properties of them, or create identifiers during runtime");
            self.insertfn("replace", nscriptfn_replace,"(string,find,replace)  // returns a new string if the substring is found it will replace it.");
            self.insertfn("join", nscriptfn_join,"(vector,delimeter)  // returns a string of a given vector it will join the items by the delimeter");
            self.insertfn("instring", nscriptfn_instring,"(string,substring)  // returns a bool if the string contains a substring ( same as contains() ) ");
            self.insertfn("fromleft", nscriptfn_fromleft,"(string,int:characters)  // returns the first x characters of a string");
            self.insertfn("fromright", nscriptfn_fromright,"(string,int:characters)  // returns the last x characters of a string");
            self.insertfn("trimright", nscriptfn_trimright,"(string,int:totrim)  // trims a string at the end by totrim and returns that as a string");
            self.insertfn("trimleft", nscriptfn_trimleft,"(string,int:totrim)  //  trims a string at the beginning by a given number returns the result as a new string");
            self.insertfn("stringtohex", nscriptfn_stringtohex,"(string)  // returns a hexed string from the givenstring");
            self.insertfn("hextostring", nscriptfn_hextostring,"(hexstring)  // returns a string from a given hexstring");
            self.insertfn("print", nscriptfn_print,"(string,string:color default=white)  // prints a string to the console,\n the color argument is optional colors can be given as the first character \n every color has a bright version \nred = r \n blue = b \n bright blue = bb \n red = r etc"  );
            self.insertfn("fileread", nscriptfn_fileread,"(filepath)  // reads a file and returns the contents as a string");
            self.insertfn("filewrite", nscriptfn_filewrite,"(filepath,string)  // writes a string to a file");
            self.insertfn("fileexists", nscriptfn_fileexists,"(filepath)  // returns a bool true if the filepath contains a file, and false if theres no file");
            self.insertfn("filedelete", nscriptfn_filedelete,"(filepath)  // deletes a file at a given path");
            self.insertfn("filemove", nscriptfn_filemove,"(filepath,newpath)  // moves a file from a given path to another");
            self.insertfn("filecopy", nscriptfn_filecopy,"(filepath,copiedpath)  // copies a file ");
            self.insertfn("dirmove", nscriptfn_directory_move,"(directorypath, newpath)  // moves a directory to a new location");
            self.insertfn("dirdelete", nscriptfn_directory_delete,"(directorypath)  // deletes a directory");
            self.insertfn("listdir", nscriptfn_listdir,"(directorypath,bool:fullpathasresult default=false)  // returns a vector with all the files \n if the second argument is set to true all the entrees will have a full filepath \n if set false, or not given at all the entrees will only contain the filenames");
            self.insertfn("filesize", nscriptfn_filesize,"(filepath) // returns a kb/mb/gb floored number of the filesize");
            self.insertfn("filesizebytes", nscriptfn_filesizebytes,"(filepath)  // returns the filesize in bytes");
            self.insertfn("runwait", nscriptfn_call_programwait,"(shellcommandstring)  // executes a shell command, returns the result \n this is a blocking function if the called program doesnt exit , relevant see run()");
            self.insertfn("run", nscriptfn_call_program,"(shellcommandstring) // executes a shell command, returns the status as a string. (none blocking) relevant : runwait()");
            self.insertfn("round", nscriptfn_round,"(numbervar,decimals) // returns a rounded number by the given decimals.");
            self.insertfn("sleep", nscriptfn_sleep,"(int:timeinms) //  will pause the thread for x ms seconds.\n can be usefull for lowering powerconsumption");
            self.insertfn("cat", nscriptfn_cat,"(a,b,c,..) //  concatinates all arguments to eachother returns that as a new string. \n theres no limit on the ammount of arguments");
            self.insertfn("random", nscriptfn_random,"(int:min, int:max, int:decimals default= maximum) //  generates a random number by a minimum and maximum. \nset decimal to 0 to get flat numbers");
            self.insertfn("arraycontains", nscriptfn_arraycontains,"(vector,string) //  returns a bool if a vector contains the given string.");
            self.insertfn("arrayroll", nscriptfn_arrayroll,"(vector,string)  // returns a new vec, LiFo , if the vector has a size of 6 is will remain the size the new entree will be inserted as 0 and the last one will be left out, each one will shift one spot.");
            self.insertfn("arraypush", nscriptfn_arraypush,"(vector,string)  // pushes the string at the end of a vector returns a new vector");
            self.insertfn("arraymerge", nscriptfn_arraymerge,"(vector,vec,vec..)  // combines all entrees of all given vectors returns a new vector");
            self.insertfn("arrayinsert", nscriptfn_arrayinsert,"(vector, string) // inserts the string to the vector and returns that as a new vector");
            self.insertfn("arraysort", nscriptfn_arraysort,"(vector)  // sorts the vector by a alphabetic order and returns that as a new vector");
            self.insertfn("arrayretain", nscriptfn_arrayretain,"(vector,string) //  will remove the string from a vector, returns that as a new vector");
            self.insertfn("arrayshuffle", nscriptfn_arrayshuffle,"(vector)  // returns a shuffled vector as a new");
            self.insertfn("arrayreverse", nscriptfn_arrayreverse,"(vector)  // reverses the vector returns that as a new");
            self.insertfn("arraysearch", nscriptfn_arraysearch,"(vector,string) //  will create a new vector with all entrees containing the given string");
            self.insertfn("arrayfilter", nscriptfn_arrayfilter,"(vector,string) //  will create a new vector without all entrees containing the given string");
            self.insertfn("httpgetcontent", nscriptfn_get_http_content,"(ip,port,remotefile) //  will return the webcontent example : \n httpgetcontent(\"127.0.0.1\",80,\"/index.nc\")");
            self.insertfn("terminalinput", nscriptfn_terminalinput,"(msgstring,defaultoption) //  the terminal will listen for given input,\n this function returns when the terminal gives a enter");
            self.insertfn("splitselect", nscriptfn_splitselect,"(string,splitbydelimeter,int:vectorentree) //  will split a string with the given delimeter \n instead of returning a vector it will return the string by the given number ");
            self.insertfn("base64tofile", nscriptfn_base64tofile,"(base64string,filepath) //  decodes base64 string and writes it as a file");
            self.insertfn("filetobase64", nscriptfn_filetobase64,"(filepath) //  reads a file and encodes it to base64");
            self.insertfn("base64tostring", nscriptfn_base64tostring,"(string) //  returns a decoded base64string");
            self.insertfn("stringtobase64", nscriptfn_stringtobase64,"(base64string) //  decodes the string and returns that");
            self.insertfn("tcplistener", nscriptfn_tcplistener,"(ip,port) //  returns a listenersocket, can be used by other tcp***()");
            self.insertfn("tcpaccept", nscriptfn_tcpaccept,"(listenersocket) //  returns a clientsocket when a client connects");
            self.insertfn("tcpconnect", nscriptfn_tcpconnect,"(ip,port) //  returns a clientsocket \n can be used by tcpreceive");
            self.insertfn("tcpdisconnect", nscriptfn_tcpdisconnect,"(clientsocket) // closes a clientsocket");
            self.insertfn("tcpreceive", nscriptfn_tcpreceive,"(clientsocket) // returns a string if the clientsocket receives data");
            self.insertfn("tcpsend", nscriptfn_tcpsend,"(clientsocket,string) // sends a string to a clientsocket\n returns the status or send bytes");
            self.insertfn("aabb_newbox", nscriptfn_aabb_newbox,"(uniqueidentifierstring) // returns a object reference, usable for 3D collision checks");
            self.insertfn("aabb_sizedbox", nscriptfn_aabb_sizedbox,"(uniqueidentifierstring,scalex,scaley,scalez) // creates a 3d boundingbox by given scalesize \nreturns a object reference usable for 3D collision checks");
            self.insertfn("aabb_setposition", nscriptfn_aabb_setposition,"(idref,x,y,z) // sets a boundingbox to 3d coordinates ( no returns)");
            self.insertfn("aabb_setrotation", nscriptfn_aabb_setrotation,"(idref,x,y,z) // sets a boundingbox to 3d coordinates ( no returns)");
            self.insertfn("aabb_setscale", nscriptfn_aabb_setscale,"(idref,x,y,z) // sets a boundingbox to 3d coordinates ( no returns)");
            self.insertfn("aabb_addtogroup", nscriptfn_aabb_addtogroup,"(idref,groupidref) // add a object to a collisiongroup");
            self.insertfn("aabb_getgroup", nscriptfn_aabb_getgroup,"(groupidref) // returns a vector of all objects ina collisiongroup");
            self.insertfn("aabb_removefromgroup", nscriptfn_aabb_removefromgroup,"(groupidref,idtoremove) // removes a entree from a group");
            self.insertfn("aabb_getcollisions", nscriptfn_aabb_getcollisions,"(targetid,groupid) // will return a vector of all entrees who are colliding in 3d with the targetid within a collisiongroup");
            self.insertfn("aabb_removegroup", nscriptfn_aabb_removegroup,"(groupidref) // deletes a whole group.");
            self.insertfn("decode_html_url", nscriptfn_decode_html_url,"(string) // decodes html content like arguments %12 etc");
            self.insertfn("mod", nscriptfn_mod,"(number,maxnumber) // will keep the number in range, so lets say mod(10,8) will return 2");
            self.insertfn("encrypt", nscriptfn_encrypt,"(datastring,passwordstring) // returns a encrypted string, can be used with decrypt(datastring,passwordstring)");
            self.insertfn("decrypt", nscriptfn_decrypt,"(datastring,passwordstring) // returns a decrypted string, created by encrypt(str,pss)");
            self.insertfn("arraynew", nscriptfn_arraynew,"() // returns a new array");
            self.insertfn("arraynewsized", nscriptfn_arraynewsized,"(size) // returns a new array with empty strings by the given size");
            self.insertfn("terminalkey", nscriptfn_terminalkey,"() // returns the pressed key as a string");
            self.insertfn("terminalenableraw", nscriptfn_terminalenableraw,"() // enables raw mode terminal printing");
            self.insertfn("terminaldisableraw", nscriptfn_terminaldisableraw,"() // disables raw mode terminal printing");
            self.insertfn("terminalupdate", nscriptfn_updateterminal,"(string) // prints a frame");
            self.insertfn("printpos", nscriptfn_printpos,"(string,color) // todo..");
            self.insertfn("terminalflush", nscriptfn_terminalflush,"() // flushes the terminal : rawmode");
            self.insertfn("terminalreset", nscriptfn_terminalreset,"() // resets the terminal : rawmode");
            self.insertfn("createqrcode", nscriptfn_createqrcode,"(url,filepathimage) // creates a qrcode link imagefile  ");
            self.insertfn("prefix", nscriptfn_prefix,"(string) // returns the first character");
            self.insertfn("suffix", nscriptfn_suffix,"(string) // returns the last character");
            self.insertfn("castray", nscriptfn_castray,"(rayid,vec:pos_a,vec:pos_b,f32:steps) // returns vec lenght creates a buffer vector , use with getraypoint(rayid,vecid)");
            self.insertfn("getraypoint", nscriptfn_getraypoint,"(rayid,step) // returns a vector with the position , by the given step of the ray");

        }
    }
    pub fn setcmdarguments(&mut self){
        let args: Vec<String> = env::args().collect();
        let mut i = 0;
        for givenarg in &args {
            let v = "".to_owned() + &givenarg.to_owned();
            let key ="$cmdarg".to_owned() + &i.to_string();
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
                        match self.threadsreceiver.get(&tothread){
                            Some(receiver) =>{
                                let msg: NscriptVar = match receiver.try_recv(){
                                    Ok(m) =>m,
                                    Err(_e) =>{
                                        NscriptVar::new("error")
                                    },
                                };
                                match msg.stringdata.as_str(){
                                    _ =>{
                                        if msg.stringdata.as_str() != ""{
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
                        return NscriptVar::new("error");
                    }
                };
                return NscriptVar::new("ok");
            }
            None => {
                //println!("no threads found");
                return NscriptVar::newstring("error","thread not found".to_string());

            }
        };
    }
    pub fn removecoroutine(&mut self,routine:&str){
        self.coroutinesindex.retain(|x| x != routine);
        self.coroutines.remove(routine.into());
    }
    pub fn addcoroutine(&mut self,name:&str,routine:NscriptCoroutine){
        let string = name.to_string();
        if self.coroutinesindex.contains(&string) != true {
            self.coroutinesindex.push(string);
            self.coroutines.insert(name.into(),routine);
        }
    }
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
    pub fn getfuncref(&mut self,name:&str)->&NscriptFunc{
        if let Some(thisclass) = self.storage.functions.get(name){
            thisclass
        }
        else{
            &self.storage.emptyfunc
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
            return this.clone();
        }
        else{
            print(&format!("returning a emptyblock for [{}]",&blockref),"r");
            return self.emptyblock.clone();
        }
    }
    pub fn getformattedblock(&mut self,blockref:&str)->NscriptFormattedCodeBlock{
        if let Some(this) = self.formattedblocks.get(blockref){
            return this.clone();
        }
        else{
            print(&format!("returning a empty formattedblock for [{}]",&blockref),"r");
            return NscriptFormattedCodeBlock::new("NULLBLOCK");
        }
    }
    pub fn getexecutableblock(&self,blockref:&str)->NscriptExecutableCodeBlock{
        if let Some(this) = self.executableblocks.get(blockref){
            return this.clone();
        }
        else{
            print(&format!("returning a empty formattedblock for [{}]",&blockref),"r");
            return NscriptExecutableCodeBlock::new();
        }
    }
    pub fn getexecutableblockref(&self,blockref:&str)->&NscriptExecutableCodeBlock{
        if let Some(this) = self.executableblocks.get(blockref){
            return this;
        }
        else{
            print(&format!("returning a empty formattedblock for [{}]",&blockref),"r");
            return &self.emptyexecutableblock;
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
        let json = Nstring::trimright(&Nstring::trimleft(&Nstring::replace(&json,":\"",": \""), 1), 1); // strip {}
        // if it exists, extent it
        if let Some(class) = self.storage.getclassref(&objectname){
            for each in split(&json, "\",") {
                let splitprop = split(&each, "\": \"");
                if splitprop.len() > 1 {
                    let mut var = NscriptVar::new("prop");
                    if Nstring::postfix(splitprop[1]) == "\"" {
                        var.name = Nstring::trimleft(&splitprop[0],1);
                        var.stringdata =Nstring::trimright(&splitprop[1],1);
                        class.setprop(&Nstring::trimleft(&splitprop[0],1),var)
                    }
                    else{
                        var.name = Nstring::trimleft(&splitprop[0],1);
                        var.stringdata =splitprop[1].to_string();
                        class.setprop(&Nstring::trimprefix(&splitprop[0]),var)
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
    RustFunction,
    Structfn,
    Number,
    Bool,
    Static,
    Classfunc,
    Nestedfunc,
    Arraydeclaration,
}

#[derive(Clone)]
pub struct NscriptCoroutine{
    pub name:Box<str>,
    pub storageblock:NscriptCodeBlock,
    pub executableblock:NscriptExecutableCodeBlock,
    pub timedroutine:bool,
    pub timed:i64,
    pub timer:i64,
}
impl NscriptCoroutine{
    pub fn new(name:&str,block:NscriptCodeBlock,executableblock:NscriptExecutableCodeBlock,timedroutine:bool,timed:i64)->NscriptCoroutine{
        NscriptCoroutine{
            name:name.into(),
            storageblock:block,
            executableblock:executableblock,
            timedroutine:timedroutine,
            timed:timed,
            timer:Ntimer::init(),
        }
    }
}

/// contains customdata hashmaps can be used in rustfn
pub struct NscriptData{
    pub map_vec_int:HashMap<String,Vec<i64>>,
    pub map_vec_float:HashMap<String,Vec<f64>>,
    pub map_vec_string:HashMap<String,Vec<String>>,
    pub map_vec_vecstring:HashMap<String,Vec<Vec<String>>>,
    pub map_vec_vec3f64:HashMap<String,Vec<[f64;3]>>,
    pub static_vec_string:Vec<String>,
    pub static_vec_bool:Vec<bool>,
    pub static_vec_vec_string:Vec<Vec<String>>,
    pub static_vec_vec_vec_string:Vec<Vec<Vec<String>>>,
    pub static_vec_vec_string_vector3:Vec<Vec<(String,f64,f64,f64)>>,
    pub static_vec_vec_string_vector3_32:Vec<Vec<(String,f32,f32,f32)>>,

}
impl NscriptData{
    fn new()->NscriptData{
        NscriptData{
            map_vec_string:HashMap::new(),
            map_vec_int:HashMap::new(),
            map_vec_float:HashMap::new(),
            map_vec_vecstring:HashMap::new(),
            map_vec_vec3f64:HashMap::new(),
            static_vec_string:Vec::new(),
            static_vec_bool:Vec::new(),
            static_vec_vec_string:Vec::new(),
            static_vec_vec_vec_string:Vec::new(),
            static_vec_vec_string_vector3:Vec::new(),
            static_vec_vec_string_vector3_32:Vec::new(),
        }
    }
}
pub struct NscriptStorage{
    pub globalvars:HashMap<String,NscriptVar>,
    pub codeblocks:HashMap<String,NscriptCodeBlock>,
    pub classes:HashMap<String,NscriptClass>,
    pub functions:HashMap<String,NscriptFunc>,
    pub tcp: NscriptTcp,
    pub nscript3d: Nscript3d,
    pub emptyfunc: NscriptFunc,
    pub emptyblock: NscriptCodeBlock,
    pub customdata: NscriptData,
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
            emptyfunc:NscriptFunc::new("".to_string(),Vec::new()),
            emptyblock:NscriptCodeBlock::new(""),
            customdata: NscriptData::new(),
        }
    }
    pub fn setdefiningword(&mut self,word:&str,equalsfrom: NscriptVar, block:&mut NscriptCodeBlock)->NscriptVar{
        match self.checkdefiningwordtype(&word){
            NscriptWordTypes::Variable => {
                block.setvar(&word, equalsfrom);
            }
            NscriptWordTypes::Property => {
                let thisword = Nstring::trimprefix(&word);
                let splitword = split(&thisword,".");
                if splitword.len() > 1{
                    let classname:String; // = splitword[0].to_string();
                    if Nstring::prefix(&splitword[0]) == "*" {
                        classname = self.getargstring(&Nstring::trimprefix(&splitword[0]),block);
                    }
                    else{
                        classname = splitword[0].to_string();
                    }
                    let propname:String;// = splitword[1].to_string();
                    if  Nstring::prefix(&splitword[1])  == "*" {
                        propname = self.getargstring(&Nstring::trimprefix(&splitword[1]),block);
                    }
                    else{
                        propname = splitword[1].to_string();
                    }
                    if let Some(thisclass) = self.getclassref(&classname){
                        thisclass.setprop(&propname, equalsfrom);
                    }
                    else{
                        let mut newclass = NscriptClass::new(&classname);
                        newclass.setprop(&propname, equalsfrom);
                        self.classes.insert(classname,newclass);
                    }
                }
            }
            NscriptWordTypes::Global => {
                self.setglobal(&word, equalsfrom);
            }
            NscriptWordTypes::Array =>{
                let thisword = Nstring::trimprefix(&word);
                let wordsplit = split(&thisword,"[");
                let mut var = self.getvar(&wordsplit[0],block);
                let idvar = self.getargstring(&Nstring::trimsuffix(&wordsplit[1]),block).parse::<usize>().unwrap_or(0);
                if idvar < var.stringvec.len(){
                    var.stringvec[idvar] = equalsfrom.stringdata;
                }
                else {
                    print(&format!("block [{}] array [{}] tries to set a index but its out of bounds",&block.name,&wordsplit[0]),"r");
                }
                self.setdefiningword(wordsplit[0], var, block);
            }
            NscriptWordTypes::Reflection =>{
                self.setdefiningword(&Nstring::trimprefix(word), equalsfrom,block);
            }
            _ =>{

            }
        };
        return NscriptVar::new("v");
    }

    pub fn getglobal(&mut self,name:&str) ->NscriptVar{
        if let Some(res) = self.globalvars.get(name){
            return res.clone();
        }
        NscriptVar::new(name)//<-not found,ret new
    }
    pub fn setglobal(&mut self,name:&str,data:NscriptVar){
        self.globalvars.insert(name.to_string(), data);
    }
    /// duplicate also in nvar, this one is for rustfnbinds
    pub fn getclassref(&mut self,name:&str)->Option<&mut NscriptClass>{
        if let Some(thisclass) = self.classes.get_mut(name.trim()){
            Some(thisclass)
        }
        else{
            None
        }
    }
    /// used for rust made functions , this is used on storage to fastly get object properties
    /// (objectname, Propertyname ) -> NscriptVar
    pub fn objectgetprop(&mut self,name:&str,prop:&str) ->NscriptVar{
        if let Some(thisclass) = self.classes.get_mut(&name.to_string()){
            return thisclass.getprop(&prop);
        }
        println!("cant find obj {} prop {}",&name,&prop);
        NscriptVar::new(name)
    }
    /// used for rust made functions , this is used on storage to fastly set object properties
    /// (objectname, Propertyname ) -> NscriptVar
    pub fn objectsetprop(&mut self,name:&str,prop:&str,var:NscriptVar){
        if let Some(thisclass) = self.classes.get_mut(&name.to_string()){
            thisclass.setprop(&prop, var);
        }
    }
    pub fn setprop(&mut self,name:&str,prop:&str,var:NscriptVar,block:&mut NscriptCodeBlock){
        let  cname:Box<str>;
        let  pname :Box<str>;
        if Nstring::prefix(&name) ==  "*" {
            cname = self.getevaluatablewordstr(&Nstring::trimprefix(&name), block).into();
        }
        else{
            cname = name.trim().into();
        }
        if Nstring::prefix(&name) ==  "*" {
            pname = self.getevaluatablewordstr(&Nstring::trimprefix(&name),block).into() ;
        }
        else{
            pname = prop.trim().into();
        }
        if let Some(thisclass) = self.classes.get_mut(&cname.to_string()){
            thisclass.setprop(&pname, var);
        }
        else{
            let mut newclass = NscriptClass::new(&cname);
            newclass.setprop(&pname, var);
            self.classes.insert(cname.to_string(),newclass);
        }
    }
    /// used for simplerustfn and interpreter know variables(non nc functions nor word function
    /// check) for retrieving strings
    pub fn getargstring(&mut self,word:&str,block: &mut NscriptCodeBlock) -> String{
        match self.argtype(word){
            NscriptWordTypes::Static =>{
                return block.staticstrings[Nstring::trimprefix(word).parse::<usize>().unwrap_or(0)].to_string();
            }
            NscriptWordTypes::Variable=>{
                return block.getstring(word).to_string();
            }
            NscriptWordTypes::Property=>{
                let thisword = Nstring::trimprefix(word);
                let wordsplit = split(&thisword,".");
                //if wordsplit.len() > 1{
                let  cname:Box<str>;
                let  pname :Box<str>;
                if Nstring::prefix(&wordsplit[0]) ==  "*" {
                    cname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[0]), block).into();
                }
                else{
                    cname = wordsplit[0].trim().into();
                }
                if Nstring::prefix(&wordsplit[1]) ==  "*" {
                    pname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[1]),block).into() ;
                }
                else{
                    pname = wordsplit[1].trim().into();
                }
                if let Some(thisclass) = self.getclassref(&cname){
                    return thisclass.getpropstr(&pname).to_string();
                }else{
                    print(&format!("getargstring() storage block:[{}] word[{}] is a prop but theres no class on cname [{}] pname[{}]",&block.name,&word,&cname,&pname),"r");
                    return "".to_owned();
                }
                //}
            }
            NscriptWordTypes::Number =>{
                let thisword = Nstring::trimprefix(word);
                return thisword.to_string();
            }
            NscriptWordTypes::Bool => {
                return Nstring::trimprefix(&word).to_owned();
            }
            NscriptWordTypes::Macro => {
                return self.getmacrostring(word).to_string();
            }
            NscriptWordTypes::Global => {
                return self.getglobal(&word).stringdata;
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimprefix(word);
                let evaluated = self.getargstring(&toreflect, block);
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let thisword = Nstring::trimprefix(&word);
                let arrays = split(&thisword,"[");
                let thisvar = self.getargstringvec(arrays[0], block);
                let index = self.getargstring(&Nstring::trimsuffix(&arrays[1]),block).parse::<usize>().unwrap_or(0);
                if thisvar.len() > index{
                    return thisvar[index].to_string();
                }else{
                    print(&format!("getargstring() storage block:[{}] word:[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&word,&block.name,&arrays[0],&index,&thisvar.len()),"r");
                }
                return "".to_owned();
            }
            _ => {
            }
        }
        word.to_string()
    }
    pub fn checkdefiningwordtype(&mut self,word:&str) -> NscriptWordTypes{
        match Nstring::prefix(word){
            "&" => {
                return NscriptWordTypes::Property;
            }
            "$" => {
                return NscriptWordTypes::Global;
            }
            "*" => {
                return NscriptWordTypes::Reflection;
            }
            "#" =>{
                return NscriptWordTypes::Array;
            }
            _ => {
                return NscriptWordTypes::Variable;
            }
        }
    }
    pub fn getevaluatablewordstr(&mut self,word:&str, block:&mut NscriptCodeBlock) -> Box<str>{

        match self.checkdefiningwordtype(word){
            NscriptWordTypes::Static =>{
                return block.staticstrings[Nstring::trimprefix(word).parse::<usize>().unwrap_or(0)].to_string().into();
            }
            NscriptWordTypes::Variable=>{
                return block.getstr(word).into();
            }
            NscriptWordTypes::Property=>{
                let thisword = Nstring::trimprefix(word);
                let wordsplit = split(&thisword,".");
                if wordsplit.len() > 1{
                    let  cname:Box<str>;// = wordsplit[0].trim().into();
                    let  pname :Box<str>;//= wordsplit[1].trim().into();
                    if Nstring::prefix(&wordsplit[0]) ==  "*" {
                        cname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[0]), block).into();
                    }
                    else{
                        cname = wordsplit[0].trim().into();
                    }
                    if Nstring::prefix(&wordsplit[1]) ==  "*" {
                        pname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[1]),block).into() ;
                    }
                    else{
                        pname = wordsplit[1].trim().into();
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getpropstr(&pname).into();
                    }else{
                        print(&format!("nscript::getwordstring() block[{}] word [{}]is a prop but theres no class on cname [{}] pname[{}]",&block.name,&word,&cname,&pname),"r");
                        return "".into();
                    }
                }
            }

            NscriptWordTypes::Global => {
                return self.getglobal(&word).stringdata.into();
            }
            NscriptWordTypes::Array =>{
                let thisword = Nstring::trimprefix(&word);
                let arrays = split(&thisword,"[");
                let thisvar = self.getvar(arrays[0], block);
                let index = self.getevaluatablewordstr(&Nstring::trimsuffix(&arrays[1]),block).parse::<usize>().unwrap_or(0);
                if thisvar.stringvec.len() > index{
                    return thisvar.stringvec[index].as_str().into();
                }else{
                    print(&format!("nscript::getwordstring() block[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&block.name,&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return "".into();
            }

            _ => {
                //return self.getvar(word,&NscriptWordTypes::Variable,&formattedblock, block).stringdata.into();
                return block.getstr(word).into();
            }
        };

        return "".into();
    }


    //used to get the Vec<String> from injected simplerustfn
    pub fn getargstringvec(&mut self,word:&str,block: &mut NscriptCodeBlock) -> Vec<String>{
        match self.argtype(word){
            NscriptWordTypes::Variable=>{
                return block.getstringvec(word);
            }
            NscriptWordTypes::Property=>{
                let thisword = Nstring::trimprefix(&word);
                let wordsplit = split(&thisword,".");
                if wordsplit.len() > 1{
                    let  cname:Box<str>;
                    let  pname :Box<str>;
                    if Nstring::prefix(&wordsplit[0]) ==  "*" {
                        cname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[0]), block).into();
                    }
                    else{
                        cname = wordsplit[0].trim().into();
                    }
                    if Nstring::prefix(&wordsplit[1]) ==  "*" {
                        pname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[1]),block).into() ;
                    }
                    else{
                        pname = wordsplit[1].trim().into();
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname).stringvec;
                    }else{
                        print(&format!(" getargstringvec() storage block:[{}]  word[{}] is a prop but theres no class on cname [{}] pname[{}]",&block.name,&word,&cname,&pname),"r");
                    }
                }
            }
            NscriptWordTypes::Global => {
                return self.getglobal(&word).stringvec;
            }
            _ => {
            }
        }

        Vec::new()
    }
    //used to get the Vec<String> from injected simplerustfn
    pub fn getvar(&mut self,word:&str,block: &mut NscriptCodeBlock) -> NscriptVar{
        match self.argtype(word){
            NscriptWordTypes::Variable=>{
                return NscriptVar::newvar(&word, block.getstring(word),block.getstringvec(word));
            }
            NscriptWordTypes::Property=>{
                let thisword = Nstring::trimprefix(word);
                let wordsplit = split(&thisword,".");
                //if wordsplit.len() > 1{
                let  cname:Box<str>;
                let  pname :Box<str>;
                if Nstring::prefix(&wordsplit[0]) ==  "*" {
                    cname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[0]), block).into();
                }
                else{
                    cname = wordsplit[0].trim().into();
                }
                if Nstring::prefix(&wordsplit[1]) ==  "*" {
                    pname = self.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[1]),block).into() ;
                }
                else{
                    pname = wordsplit[1].trim().into();
                }
                if let Some(thisclass) = self.getclassref(&cname){
                    return thisclass.getprop(&pname);
                }else{
                    print(&format!(" getvar() storage block:[{}]  word:[{}] word is a prop but theres no class on cname [{}] pname[{}]",&block.name,&word,&cname,&pname),"r");
                }
                //}
            }
            NscriptWordTypes::Static=>{
                return NscriptVar::newstring("stc", block.staticstrings[Nstring::trimleft(&word,1).parse::<usize>().unwrap_or(0)].to_string());
            }
            NscriptWordTypes::Number =>{
                let thisword = Nstring::trimprefix(word);
                return NscriptVar::newstring(&thisword,thisword.to_string());
            }
            NscriptWordTypes::Global => {
                return self.getglobal(&word);
            }
            NscriptWordTypes::Macro=>{
                return NscriptVar::newstring("macro", self.getmacrostring(&word).to_string());
            }

            NscriptWordTypes::Bool => {
                return NscriptVar::newstring("bool", Nstring::trimprefix(&word).to_string());
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimprefix(word);
                let evaluated = self.getvar(&toreflect, block);
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let thisword = Nstring::trimprefix(&word);
                let arrays = split(&thisword,"[");
                let thisvar = self.getvar(arrays[0],block);
                let index = self.getargstring(&Nstring::trimsuffix(&arrays[1]),block).parse::<usize>().unwrap_or(0);
                if thisvar.stringvec.len() > index{
                    return NscriptVar::newstring("e",thisvar.stringvec[index].to_string());
                }else{
                    print(&format!(" getvar() block:[{}] word:[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&block.name,&word,&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return NscriptVar::new("entree");
            }
            _ => {
            }
        }
        return NscriptVar::new("error");
        //thisvar
    }
    pub fn classgetprop(&mut self,class:&str,prop:&str, block:&mut NscriptCodeBlock) ->NscriptVar{
        let mut cname = class.to_string();
        let mut pname = prop.to_string();
        let mut thisvar = NscriptVar::new("c");
        if Nstring::prefix(&cname) ==  "*" {
            cname = self.getargstring(&Nstring::trimprefix(&cname), block);
        }
        if Nstring::prefix(&pname) ==  "*" {
            pname = self.getargstring(&Nstring::trimprefix(&pname),block) ;
        }
        if let Some(thisclass) = self.getclassref(&cname){
            thisvar = thisclass.getprop(&pname);
        }else{
            print(&format!("storage block:[{}] theres no class on cname [{}] pname[{}]",&block.name,&cname,&pname),"r");
        }
        thisvar
    }
    /// used to get the type of argument thats been given to a simplerustfn
    pub fn argtype(&mut self,word:&str) ->NscriptWordTypes{

        match Nstring::prefix(word){
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
            "!" => {
                return NscriptWordTypes::Bool;//"reflection".to_string();
            }
            "%" => {
                return NscriptWordTypes::Number;//"number".to_string();
            }
            "&" => {
                return NscriptWordTypes::Property;//"number".to_string();
            }
            "#" => {
                return NscriptWordTypes::Array;//"number".to_string();
            }
            _ => {
                return NscriptWordTypes::Variable;//"variable".to_string();
            }
        }
    }
    pub fn getmacrostring(&mut self,word:&str)->Box<str>{

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
            "@e_bread" => { "🍞".to_string() },
            "@e_sandwich" => { "🥪".to_string() },
            "@e_fish" => { "🐟".to_string() },
            "@e_fries" => { "🍟".to_string() },
            "@e_coke" => { "🥤".to_string() },
            "@e_water" => { "💧".to_string() },
            "@e_wine" => { "🍷".to_string() },
            "@e_burger" => { "🍔".to_string() },
            "@e_hot_dog" => { "🌭".to_string() },
            "@e_ice_cream" => { "🍦".to_string() },
            "@e_drinks" => { "🥤".to_string() },
            "@e_taco" => { "🌯".to_string() },
            "@e_pizza" => { "🍕".to_string() },
            "@e_sushi" => { "🍣".to_string() },
            "@e_banana" => { "🍌".to_string() },
            "@e_apple" => { "🍎".to_string() },
            "@e_cash" => { "💸".to_string() },
            "@e_book" => { "📖".to_string() },
            "@e_pen" => { "✏️".to_string() },
            "@e_phone" => { "📱".to_string() },
            "@e_TV" => { "📺".to_string() },
            "@e_computer" => { "💻".to_string() },
            "@e_gamepad" => { "🎮".to_string() },
            "@e_bike" => { "🚴".to_string()},
            "@e_airplane" => { "✈️".to_string() },
            "@e_ship" => { "🛳️".to_string() },
            "@e_sun" => { "☀️".to_string() },
            "@e_moon" => { "🌕".to_string() },
            "@e_clouds" => { "☁️".to_string() },
            "@e_smile" => { "🙂".to_string() },
            "@e_bigsmile" => { "😁".to_string() },
            "@e_invertedsmile" => { "🙃".to_string() },
            "@e_meltmile" => { "🫠".to_string() },
            "@e_wink" => { "😉".to_string() },
            "@e_blush" => { "😊".to_string() },
            "@e_tearsmile" => { "🥲".to_string() },
            "@e_yum" => { "😋".to_string() },
            "@e_tongue" => { "😛".to_string() },
            "@e_tonguewink" => { "😜".to_string() },
            "@e_thinking" => { "🤔".to_string() },
            "@e_salute" => { "🫡".to_string() },
            "@e_zippedmouth" => { "🤐".to_string() },
            "@e_tired" => { "🫩".to_string() },
            "@e_sick" => { "🤢".to_string() },
            "@e_puke" => { "🤮".to_string() },
            "@e_sneeze" => { "🤧".to_string() },
            "@e_hot" => { "🥵".to_string() },
            "@e_cold" => { "🥶".to_string() },
            "@e_drunk" => { "🥴".to_string() },
            "@e_mindblown" => { "🤯".to_string() },
            "@e_cowboy" => { "🤠".to_string() },
            "@e_party" => { "🥳".to_string() },
            "@e_disguised" => { "🥸".to_string() },
            "@e_glasses" => { "😎".to_string() },
            "@e_sad" => { "🙁".to_string() },
            "@e_worried" => { "😟".to_string() },
            "@e_shocked" => { "😮".to_string() },
            "@e_hushed" => { "😯".to_string() },
            "@e_mad" => { "🤬".to_string() },
            "@e_skull" => { "☠".to_string() },
            "@e_turd" => { "💩".to_string() },
            "@e_ghost" => { "👻".to_string() },
            "@e_blueheart" => { "💙".to_string() },
            "@e_heart" => { "🧡".to_string() },
            "@e_blackheart" => { "🖤".to_string() },
            "@e_okhand" => { "👌".to_string() },
            "@e_midlefinger" => { "🖕".to_string() },
            "@e_thumb" => { "👍".to_string() },
            "@e_strong" => { "💪".to_string() },
            "@e_rice" => { "🍚".to_string() },
            "@e_floppy" => { "💾".to_string() },
            "@e_cdbox" | "@e_dvdbox" => { "💽".to_string() },
            "@e_cd" | "@e_dvd" => { "📀".to_string() },
            "@e_magnifier" => { "🔍".to_string() },
            "@e_printer" => { "🖨".to_string() },
            "@e_speaker" => { "🔊".to_string() },
            "@e_check" => { "✅".to_string() },
            "@e_cross" => { "❌".to_string() },
            "@nscriptpath" => {
                let  string = "~/nscript".to_string();
                if let Ok(value) = env::var("NSCRIPT_PATH") {
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
        macrostring.into()

    }

}
#[derive(Clone)]
pub enum FormattedLineTypes{
    SetVar,
    SetVarFn,
    SetLocalVarFn,
    SetLocalVarClassFn,
    SetLocalVarNestedFn,
    SetLocalVarRustFn,
    SetLocalVarRustStructFn,
    SetPropFn,
    SetVarClassFn,
    SetVarNestedFn,
    SetVarRustFn,
    SetVarRustStructFn,
    SetVec,
    SetStringLoopIn,
    SetStringLoopTo,
    SetVecLoopIn,
    SetVecLoopTo,
    Loop,
    Scope,
    If,
    ElseIf,
    Else,
    Match,
    SetMatch,
    ForTo,
    ForIn,
    Coroutine,
    SpawnThread,
    ReturnVar,
    Return,
    Fn,
    Init,
    StructFn,
    ClassFn,
    NestedFn,
    RustFn,
    Exit,
    Chain,
    Break,
    BreakCo,
    ReturnNestedFn,
    ReturnClassFn,
    ReturnRustFn,
    ReturnFn,
    ReturnSome,
    AddOne,
    SubOne,
    Cat,
    CatVec,
    CatSelf,
    SetClass,
    AddSelf,
    SubSelf,
    VecTo,
    VecIn,
    StringTo,
    StringIn,
    Math,
    SetBool,
    DbgR,
    DbgY,
    End,
}
#[derive(Clone)]
pub struct NscriptExecutableCodeBlock{
    pub boxedcode: Vec<Vec<Vec<Box<str>>>>,// all the subscopes, if else loop coroutines
}
impl NscriptExecutableCodeBlock{
    pub fn new()->NscriptExecutableCodeBlock{
        NscriptExecutableCodeBlock{
            boxedcode: Vec::new(),
        }
    }
}
#[derive(Clone)]
pub struct NscriptFormattedCodeBlock{

    pub name: Box<str>,
    pub codeblock: String,
    pub code: Vec<Vec<Vec<String>>>,// all the subscopes, if else loop coroutines
    pub boxedcode: Vec<Vec<Vec<Box<str>>>>,// all the subscopes, if else loop coroutines
}
impl NscriptFormattedCodeBlock{
    pub fn new(blockname:&str)->NscriptFormattedCodeBlock{
        NscriptFormattedCodeBlock{
            name: blockname.into(),
            codeblock: String::new(),
            code: Vec::new(),
            boxedcode: Vec::new(),
        }
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
    /// pre-formatting: all the converted hexed static strings will be assinged a variable
    fn convertstaticstrings(&mut self,block:&mut NscriptCodeBlock){
        let mut parsingtext = Nstring::replace(&self.codeblock.to_string(),"\"{","\" {");
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
                    block.staticstrings.push(stringdata.into_boxed_str());
                }
            }
        }
        block.staticstrings.push("".to_string().into_boxed_str());
        self.codeblock = parsingtext.to_string();
    }
    /// tokenizing and assinging subscopes
    pub fn formatblock(&mut self,block:&mut NscriptCodeBlock) {
        //self.subblockmap = Vec::new();
        self.convertstaticstrings(block);
        let mut parsingtext = self.codeblock.to_string();
        let mut toreturn: String;
        let mut scopecounter = 1;
        block.breakloop.push(false);
        self.code.push(Vec::new());
        loop {
            let splitstr = split(&parsingtext, "{");
            if splitstr.len() > 1 {
                let isscope = split(&splitstr[splitstr.len() - 1], "}")[0];
                scopecounter +=1;
                let scopeid = scopecounter.to_string();
                let scopekey =" SCOPE ".to_string() + &scopeid;
                block.breakloop.push(false);
                let formattedscope = self.formatargumentspaces(&isscope);
                let codevec = self.codetovector(&formattedscope);
                self.code.push(codevec);
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
        self.code[0] = codevec.clone();
        self.codeblock = toreturn.to_string();

    }
    /// used for pre formatting,
    pub fn setcode(&mut self, codestring:String){
        //print(&codestring,"p");
        self.codeblock = codestring.to_string();
    }
    /// stores formatted code
    pub fn setcodevector(&mut self, codestring:Vec<Vec<String>>){
        self.code[0] = codestring;
    }

}


/// this struct contains the vectors with code
#[derive(Clone)]
pub struct NscriptCodeBlock{
    pub name: String,
    pub insubblock: usize,// all the subscopes, if else loop coroutines
    pub variables: HashMap< Box<str>,NscriptVar>,// scope variables
    //pub strings: HashMap< String,String>,// scope variables
    //pub stringsvec: HashMap<String,Vec<String>>,// scope variables
    pub staticstrings: Vec<Box<str>>,// scope variables
    pub ifscopedepth: usize,//used for parsing nested ifscopes
    pub ifscopes: Vec<bool>,// used for nested elseif else scopes
    pub inloop: usize, // used for nested loops
    pub breakloop: Vec<bool>, // used to break the right nested loop.
}

impl NscriptCodeBlock{
    pub fn new(nameref:&str) -> NscriptCodeBlock{
        let mut this = NscriptCodeBlock{
            name: nameref.to_string(),
            insubblock: 0,
            variables: HashMap::new(),
            //strings: HashMap::new(),
            //stringsvec: HashMap::new(),
            staticstrings: Vec::new(),
            ifscopedepth: 0,
            ifscopes: Vec::new(),
            inloop: 0,
            breakloop: Vec::new(),
        };
        this.ifscopes.push(false);
        this.breakloop.push(false);
        this
    }
    pub fn setstring(&mut self,nameref:&str,string:String){
        self.variables.insert(nameref.into(),NscriptVar::newstring(&nameref,string));
    }
    pub fn getstring(&mut self,namref:&str) ->String{
        if let Some(data) = self.variables.get(namref.into()){
            return data.stringdata.to_string();
        }
        else{
            return "".to_string();
        }
    }
    pub fn getstr(&mut self,namref:&str) ->&str{
        if let Some(data) = self.variables.get(namref.into()){
            return &data.stringdata;
        }
        else{
            return &EMPTYSTR;
        }
    }
    /// stored Vec<String> for NscriptVar types.
    pub fn setstringvec(&mut self,nameref:&str,stringvec:Vec<String>){
        self.variables.insert(nameref.into(),NscriptVar::newvec(nameref.into(), stringvec));
    }

    /// stored Vec<String> for NscriptVar types.
    pub fn getstringvec(&mut self,nameref:&str) ->Vec<String>{
        if let Some(data) = self.variables.get(nameref.into()){
            return data.stringvec.to_owned();
        }
        else{
            return Vec::new();
        }
    }
    pub fn ifsetup(&mut self,set:bool){
        self.ifscopes[self.ifscopedepth] = set;
        self.ifscopes.push(false);
        self.ifscopedepth +=1;
    }
    // pub fn ifset(&mut self,set:bool){
    //     self.ifscopes[self.ifscopedepth] = set;
    // }
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
    // pub fn ifdown(&mut self){
    //     self.ifscopedepth -=1;
    //     self.ifscopes =self.ifscopes[0..self.ifscopes.len()-1].to_vec();
    // }
    // pub fn ifup(&mut self){
    //     self.ifscopes.push(false);
    //     self.ifscopedepth +=1;
    // }
    ///copies a variable from the block for mutable purposes
    pub fn getvar(&mut self,name:&str)->NscriptVar{
        if let Some(var) = self.variables.get(name){
            return var.clone();
        }
        // else{
        //      var.stringdata = "".to_owned();
        // };
        // if let Some(stringvec) = self.stringsvec.get(name){
        //      var.stringvec = stringvec.to_owned();
        // }
        // else{
        //      var.stringvec = Vec::new();
        // };

        NscriptVar::new(&name)

    }
    pub fn setvar(&mut self,name:&str,var:NscriptVar){
        self.variables.insert(name.into(),var);
        // self.strings.insert(name.to_string(),var.stringdata );
        // self.stringsvec.insert(name.to_string(),var.stringvec);
    }
    pub fn setvarstring(&mut self,name:&str,var:NscriptVar){
        if let Some(onvar) = self.variables.get_mut(name){
            onvar.stringdata = var.stringdata;
        }
        else{
            self.variables.insert(name.into(), var);
        }
        //self.strings.insert(name.to_string(),var.stringdata );
    }
    pub fn setvarvec(&mut self,name:&str,var:NscriptVar){
        if let Some(onvar) = self.variables.get_mut(name){
            onvar.stringvec = var.stringvec;
        }
        else{
            self.variables.insert(name.into(), var);
        }
        //self.stringsvec.insert(name.to_string(),var.stringvec);
    }
}
/// implement this to add new Nscript rust functions and bind them
pub trait NscriptStructBinding {
    fn nscript_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>,storage: &mut NscriptStorage) -> NscriptVar;
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
#[derive(Clone)]
pub struct NscriptVar{
    pub name: String,// in string
    pub stringdata: String,
    pub stringvec: Vec<String>,
}
/// Variable struct holds the nscript datatypes and data
impl NscriptVar{
    pub fn new(name:&str) -> NscriptVar{
        NscriptVar{
            name: name.to_string(),
            stringdata: "".to_string(),
            stringvec:Vec::new(),
        }
    }
    pub fn newstring(name:&str,stringset:String) -> NscriptVar{
        NscriptVar{
            name: name.to_string(),
            stringdata: stringset,
            stringvec:Vec::new(),
        }
    }
    pub fn newvar(name:&str,stringset:String,vecset:Vec<String>) -> NscriptVar{
        NscriptVar{
            name: name.to_string(),
            stringdata: stringset,
            stringvec:vecset,
        }
    }
    pub fn newvec(name:&str,vecset:Vec<String>) -> NscriptVar{
        NscriptVar{
            name: name.to_string(),
            stringdata: "".to_string(),
            //number: None,
            //float: None,
            stringvec:vecset,
        }
    }

    pub fn setreturn(&mut self){
        self.name = "return".to_string();
    }
    /// returns the string value of the variable
    pub fn getstring(&mut self) -> &str{
        return &self.stringdata;
    }
    pub fn getnumber(&mut self) -> u64{
        let i = self.stringdata.parse::<u64>().unwrap_or(0);
        return i;
    }
    pub fn getfloat32(&mut self) -> f32{
        let i = self.stringdata.parse::<f32>().unwrap_or(0.0);
        return i;
    }
    pub fn getfloat64(&mut self) -> f64{
        let i = self.stringdata.parse::<f64>().unwrap_or(0.0);
        return i;
    }
    pub fn setstring(&mut self,newstring:&str){
        self.stringdata = newstring.to_string()
    }
}
/// Nscript user scripted functions
#[derive(Clone)]
pub struct NscriptFunc{
    pub name: Box<str>,
    pub args:Vec<Box<str>>,
    pub codeblock:NscriptCodeBlock,
    pub executablecodeblock:NscriptExecutableCodeBlock,
}

impl NscriptFunc{
    pub fn new(name:String,args:Vec<Box<str>>)->NscriptFunc{
        NscriptFunc{
            name: name.to_string().into_boxed_str(),
            args: args,
            codeblock: NscriptCodeBlock::new(&name),
            executablecodeblock: NscriptExecutableCodeBlock::new(),
        }
    }
}


#[derive(Clone)]
pub struct NscriptClass{
    pub name: Box<str>,
    pub index: Vec<Box<str>>,
    properties: HashMap<String,NscriptVar>,
    pub functionindex: Vec<Box<str>>,
    pub functions: HashMap<String,NscriptFunc>,
}

impl NscriptClass{
    pub fn new(name:&str) -> NscriptClass{
        NscriptClass{
            name: name.into(),
            index: Vec::new(),
            properties: HashMap::new(),
            functionindex: Vec::new(),
            functions: HashMap::new(),
        }
    }

    pub fn copyto(&mut self, name:&str) -> NscriptClass{
        let mut this = NscriptClass{
            name: name.into(),
            index: self.index.clone(),
            properties: self.properties.clone(),//HashMap::new(),
            functionindex:self.functionindex.clone(),
            functions:HashMap::new(),// required for the self var to be good
        };
        for xprop in self.functionindex.clone(){
            this.setfunc(&xprop, self.getfunc(&xprop)); // will set self.
        };
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
    }
    pub fn setprop(&mut self, name:&str,prop:NscriptVar){
        if name != ""{
            if let Some(_) = self.properties.get(name){
                self.properties.insert(name.to_string(),prop);
            }
            else{
                self.index.push(name.into());
                self.properties.insert(name.to_string(),prop);
            }
        }

    }
    pub fn getprop(&mut self,name:&str) ->NscriptVar{
        if let Some(this) = self.properties.get(name){
            this.clone()
        }
        else{
            NscriptVar::new(name)
        }
    }
    pub fn getpropstr(&mut self,name:&str) ->Box<str>{
        if let Some(this) = self.properties.get(name){
            this.stringdata.to_string().into()
        }
        else{
            "".into()
        }
    }
    pub fn removeprop(&mut self,name:&str){
        self.index.retain(|x| x.to_owned().into_string() != name);
        self.properties.remove(name);
    }
    pub fn removefunc(&mut self,name:&str){
        //self.functionindex.retain(| x:Box<str>| x.into() != name);
        self.functionindex.retain(|x| x.to_owned().into_string() != name);
        self.functions.remove(name);
    }
    pub fn setfunc(&mut self, name:&str,mut prop:NscriptFunc){
        let mut var = NscriptVar::new("self");
        var.stringdata = self.name.to_string();
        prop.codeblock.setvar("self", var);
        if let Some(_) = self.functions.get(name){
            self.functions.insert(name.to_string(),prop);
        }
        else{
            self.functionindex.push(name.into());
            self.functions.insert(name.to_string(),prop);
        }
    }
    pub fn getfunc(&mut self,name:&str) ->NscriptFunc{
        if let Some(this) = self.functions.get(name){
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

