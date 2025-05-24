
use crate::*;

impl <'a> Nscript<'a>{
    pub fn parsefile(&mut self,file:&str) -> NscriptVar{
        let filedata = "\n".to_string() + &read_file_utf8(file);
        return self.parsecode(&filedata,&file);
    }
    pub fn parsecode(&mut self,code:&str,name:&str) -> NscriptVar{
        let mut initblock = NscriptCodeBlock::new(&name);

        let filedata = self.formatcode(code, name);
        let mut formattedblock = NscriptFormattedCodeBlock::new(name);
        formattedblock.setcode(filedata);
        //print(&format!("->"),"bm");
        formattedblock.formatblock(&mut initblock);
        //print(&format!("->"),"br");
        self.preproccessblock(&mut formattedblock);
        self.formattedblocks.insert(name.to_string(), formattedblock.clone());
        //let formattedblock = initblock.formattedcode.clone();
        //print(&formattedblock.code.len().to_string(),"g");
        //self.compiledblocks[0] = NscriptCompiledCodeBlock::compileblock(&initblock.codeblockvector,&initblock.subblockmap);

        let executableblock = self.getexecutableblock(&formattedblock.name);
        if let Some(ret) = self.executescope(&executableblock.boxedcode[0],&executableblock,&mut initblock){
            return ret.clone();
        }else{
            return NscriptVar::new("end");
        };
    }
    pub fn parsecodewithvars(&mut self,code:&str,name:&str,vars:Vec<NscriptVar>) -> NscriptVar{
        let filedata = self.formatcode(code, name);
        let mut initblock = NscriptCodeBlock::new("start");
        let mut formattedblock = NscriptFormattedCodeBlock::new("start");
        formattedblock.setcode(filedata);
        formattedblock.formatblock(&mut initblock);
        self.preproccessblock(&mut formattedblock);
        self.formattedblocks.insert(initblock.name.to_string(),formattedblock.clone()); // a copy not in the func maybe can go
        let executableblock = self.getexecutableblock(&formattedblock.name);
        for xvar in vars{
            self.setdefiningword(&xvar.name, xvar.clone(),&executableblock,&mut initblock);
        }
        if let Some(ret) = self.executescope(&executableblock.boxedcode[0],&executableblock,&mut initblock){
            return ret.clone();
        }else{
            return NscriptVar::new("end");
        };
    }

    fn getvar(&mut self,varname:&str,block:&mut NscriptCodeBlock)->NscriptVar{
        self.storage.getvar(varname, block)
    }
    fn formatcode(&mut self,code:&str,name:&str)-> String{
        let mut thiscodescope = NscriptScriptScope::new(name.to_string());
        //let mut thiscodeblock = NscriptCodeBlock::new(name);

        let mut filedata = self.raw_scopeextract(&code);
        filedata = "\n".to_string() + &Nstring::replace(&filedata,"\"{","\" {");

        filedata = self.stripcomments(&filedata);

        filedata = self.batchrustsinglelinefunctions(&filedata);
        filedata = Nstring::replace(&filedata,"\n.", ".");// multiline chains to singleline
        filedata = self.stringextract(&filedata);// pre work creates it to hex! ^hexed,
        filedata = Nstring::replace(&filedata,"else if", "elseif");
        filedata = self.prefixbooleans(&filedata);
        filedata = self.array_scopeextract(&filedata);// multiline array parse to 1 line,
        filedata = self.fixdoublespaces(&filedata);
        filedata = "\n".to_string() + &filedata;
        filedata = self.thread_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.class_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.func_scopeextract(&filedata,"");

        filedata
    }
    fn batchrustsinglelinefunctions(&mut self,filedata:&String) -> String{
        let mut returnstring = "".to_string();
        let mut thisline = "".to_string();
        for xline in split(&filedata,"\n"){
            if split(xline," ").len() == 1 {
                let spl = split(xline,"(");
                if spl.len() == 2 {
                    match self.rustfunctions.get(&spl[0].to_string()){
                        Some(_) =>{

                            thisline = thisline + " " + &xline; // buffer the line into 1 line
                        }
                        _ =>{

                    (returnstring,thisline) = Nscript::checkbatchedfnbuffer(thisline.to_string(),xline, returnstring);
                        }
                    }

                }
                else{

                (returnstring,thisline) = Nscript::checkbatchedfnbuffer(thisline.to_string(),xline, returnstring);

                }

            }
            else{
                (returnstring,thisline) = Nscript::checkbatchedfnbuffer(thisline.to_string(),xline, returnstring);
            }
        }
        if thisline != "" {

            returnstring = returnstring + "\n" + &thisline +"\n";
        }
        //print("done with batching","g");
        //print(&returnstring,"bg");
        returnstring
    }
    fn checkbatchedfnbuffer(mut thisline:String,xline:&str,mut returnstring: String) ->(String,String){
                if split(&thisline," ").len() > 2{
                    returnstring = returnstring + "\nBRF " + &thisline +"\n"+&xline;
                    thisline = "".to_string(); // reset the string
                }else{
                    if thisline != ""{
                        returnstring = returnstring + "\n " +&thisline+ "\n"+ &xline;
                        thisline = "".to_string(); // reset the string
                    }
                    else{
                        returnstring = returnstring + "\n"+ &xline;
                    }
                }
        return (returnstring.to_owned(),thisline.to_owned() );

    }
    /// preproccessor insert word[0] on the line for the interpreter to speed things up..
    pub fn preproccessblock(&mut self,formattedcode:&mut NscriptFormattedCodeBlock){

        //let  fblock = formattedcode.clone();
        let mut prefixedcode = self.wordtypeprefixing(&formattedcode.code[0] );
        formattedcode.code[0] = self.preprocesscode(&mut prefixedcode);
        if formattedcode.code.len() > 0 {
            for xid in 1..formattedcode.code.len(){
                //let fblock = formattedcode.clone();
                let mut prefixedcode = self.wordtypeprefixing(&formattedcode.code[xid] );
                formattedcode.code[xid] = self.preprocesscode(&mut prefixedcode);
            }
        }
        self.convertboxedcodevec(formattedcode);
    }
    pub fn convertboxedcodevec(&mut self, formattedcode:&mut NscriptFormattedCodeBlock){
        let mut boxedvec:Vec<Vec<Vec<Box<str>>>> = Vec::new();
        for xscope in &formattedcode.code{

            let mut boxedscopevec:Vec<Vec<Box<str>>> = Vec::new();
            for xline in xscope {
                let mut boxedlinevec:Vec<Box<str>> = Vec::new();
                for xword in xline{
                    boxedlinevec.push(xword.to_string().into_boxed_str())
                }
                boxedscopevec.push(boxedlinevec);
            }
            boxedvec.push(boxedscopevec);
        }
        formattedcode.codeblock = String::new();
        formattedcode.code = Vec::new();
        formattedcode.boxedcode = boxedvec.to_owned();
        let mut xblock = NscriptExecutableCodeBlock::new();
        xblock.boxedcode = formattedcode.boxedcode.clone();
        self.executableblocks.insert(formattedcode.name.to_string(), xblock);
    }
    /// checks all the words in a codesheet and adds prefixes to make RT parsing go faster
    pub fn wordtypeprefixing(&mut self,codevec:&Vec<Vec<String>>) -> Vec<Vec<String>>{
        let mut proccessedvec : Vec<Vec<String>> = Vec::new();
        for mut xline in codevec.clone(){
            for x in 0..xline.len(){
                xline[x] = Nstring::replace(&xline[x],"(","( ");
                xline[x] = Nstring::replace(&xline[x],")"," )");
                xline[x] = Nstring::replace(&xline[x],"[","[ ");
                xline[x] = Nstring::replace(&xline[x],"]"," ]");
                xline[x] = Nstring::replace(&xline[x],","," , ");
                //xline[x] = Nstring::replace(&xline[x],"*","* ");
                let mut wordvec:Vec<String> = Vec::new();

                //first make sure syntax isnt conflicting
                let syntaxwords = ["BRF","SCOPE","return", "true","false","cat","&=", "class","func","coroutine","thread",
                    "spawnthread","string","vec",",","!!","!",":","for" ,"if","else","elseif","math","match","=",
                    "+","-","/","*","loop","==","++","--","in","to","=>","!=",">","<","<=","=>","&&","||","and",
                    "init","or","break","!","!!","]"];
                let mut proceed = true;
                let max = syntaxwords.len();
                let mut i = 0;
                loop{
                    if syntaxwords[i] == xline[x] {
                        proceed = false;
                        break;
                    }
                    i +=1;
                    if i >= max {
                        break;
                    }
                }
                if Nstring::fromleft(&xline[x], 1) == "!" || Nstring::fromleft(&xline[x], 1) == "@"  || Nstring::fromleft(&xline[x], 1) == "&"||
                Nstring::fromleft(&xline[x], 1) == "+"  || Nstring::fromleft(&xline[x], 1) == "^" ||
                Nstring::fromleft(&xline[x], 1) == "~" {
                    proceed = false;
                }
                if Nstring::fromleft(&xline[x],2) == "c:" || Nstring::fromleft(&xline[x],2) == "f:"|| Nstring::fromleft(&xline[x],2) == "v:"{
                    proceed = false;
                }
                if &xline[x] == ""{
                    proceed = false;
                }
                // if all syntax is checked we parse the word and add prefixes
                if proceed{
                    for mut arg in Nstring::split(&xline[x]," "){
                        // if its a number we add %
                        proceed = true;
                        if  arg == "" || arg == "," ||  arg == ")" ||
                        Nstring::fromleft(&arg, 1) == "!" || Nstring::fromleft(&arg, 1) == "@" ||
                        Nstring::fromleft(&arg, 1) == "^" || Nstring::fromleft(&arg, 1) == "~" || arg == "cat["  {
                            proceed = false;
                        }

                        if proceed{
                            if Nstring::instring(&arg, "::"){
                                arg = "|".to_string() + &arg;
                                wordvec.push(arg);
                            }
                            else if arg.parse::<f64>().is_ok(){
                                arg = "%".to_string() + &arg;
                                wordvec.push(arg);
                            }
                            else if Nstring::instring(&arg, ".") && Nstring::postfix(&arg) == "(" {

                                arg = "\\".to_string() + &arg;
                                wordvec.push(arg);
                            }
                            else if Nstring::instring(&arg, ".") == false && Nstring::postfix(&arg) == "(" {
                                match self.rustfunctions.get(&Nstring::trimright(&arg,1)){
                                    Some(_) =>{
                                        arg = "1".to_string() + &arg;
                                    }
                                    None =>{
                                        arg = "2".to_string() + &arg;
                                    }
                                }
                                wordvec.push(arg);
                            }
                            //if it is a property we add .
                            else if Nstring::instring(&arg, ".") && Nstring::postfix(&arg) != "(" {

                                arg = "&".to_string() + &arg;
                                wordvec.push(arg);
                            }
                            // // if its a array we add #
                            else if Nstring::prefix(&arg) != "[" && Nstring::postfix(&arg) == "[" {
                                arg = "#".to_string() + &arg;
                                wordvec.push(arg);
                            }
                            else{
                                wordvec.push(arg);
                            }
                        }
                        else{
                            wordvec.push(arg);
                        }
                    }
                    //print(&format!("word [{}] became [{}]",&xline[x],wordvec.join("")),"bg");
                    xline[x] = wordvec.join("");
                    if split(&xline[x],"(").len() > 2 && Nstring::instring(&xline[x],").") == false{
                        xline[x] = "3".to_string() + &xline[x];
                    }

                }
            }
            // let mut teststring = String::new();
            // for x in &proccessedvec{
            //    teststring = teststring + &x.join(" ") + "\n";
            // }
            // print(&teststring,"bb");
            let wlen = xline.len();
            if wlen > 1 {
                if xline[wlen-2] == "SCOPE" {
                    xline[wlen-1] = Nstring::trimleft(&xline[wlen-1], 1);
                }
            }
            proccessedvec.push(xline);

        }
        return proccessedvec;
    }
    /// this formats a multiline array to a single line
    pub fn array_scopeextract(&mut self,code: &str) -> String {
        let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
        let mut fixedcode = code.to_string();
        let classes: Vec<String> = fixedcode.split("= [\n").map(String::from).collect();
        for eachclass in classes {
            if i > 0 {
                if eachclass != "" {
                    let blockend = split(&eachclass, "\n]")[0];
                    let isblockorigin = "= [\n".to_owned() + blockend + "\n]";
                    let replacement = Nstring::replace(blockend, ",\n", "\n");
                    let replacement = Nstring::replace(&replacement, "\n", " ");
                    let replacement = "= cat[] ".to_owned() + &replacement + "";
                    let replacement = Nstring::replace(&replacement, ",]", "]");
                    fixedcode = fixedcode.replace(&isblockorigin, &replacement);
                }
            }
            i += 1;
        }
        fixedcode
    }
    fn fixdoublespaces(&mut self,code:&str)->String{
        let mut fixed = code.to_string();
        loop {
            if Nstring::instring(&fixed, "  ") {
                fixed = Nstring::replace(&fixed, "  ", " ");
            }else{
                break;
            }
        }
        return fixed;
    }

    /// this keeps the coroutines running , after all routines are ran the code returns!
    pub fn executecoroutines(&mut self){
        for xroutine in self.coroutines.clone(){
            let mut thisblock = self.getblock(&xroutine);
            let thisformattedblock = self.getexecutableblock(&thisblock.name);
            self.executeblock(&mut thisblock,&thisformattedblock);
            self.storage.codeblocks.insert(xroutine,thisblock);
        }
    }

    /// entree point for executing a new block
    pub fn executeblock(&mut self,block:&mut NscriptCodeBlock,formattedblock: &NscriptExecutableCodeBlock) -> NscriptVar{
        //let formattedblock = block.formattedcode.clone();
        if let Some(returnvar) = self.executescope(&formattedblock.boxedcode[0],&formattedblock, block){
            if &returnvar.name == "return" {
                return returnvar;
            }
        }
        NscriptVar::new("blockend")
        //let returnvar = NscriptVar::new("blockend");
        //returnvar
    }

    fn executescope(&mut self, blockvec:&Vec<Vec<Box<str>>>, formattedblock: &NscriptExecutableCodeBlock, block: &mut NscriptCodeBlock) -> Option<NscriptVar> {
        blockvec.iter().map(|line| {
            self.executepreproccessedline(line, &formattedblock, block)
        }).find(|result|result.name == "return")
    }
    /// executes if scopes which are still part of a scope so they use the local variables.
    fn executesubscope(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) -> Option<NscriptVar> {
        block.insubblock = line[line.len()-1].parse::<usize>().unwrap_or(0);
        block.breakloop.push(false);
         if block.insubblock < block.breakloop.len() {
             block.breakloop[block.insubblock] = false;
         }
        let index = block.insubblock-1;
        if let Some(result) = self.executescope(&formattedblock.boxedcode[index],&formattedblock,block){
            if &result.name == "return"{
                return Some(result);
            }
        }

        //let toreturn:Option<NscriptVar> = None;
        return None//toreturn;
    }
    /// inserts a keystring infront of the lines to speed up the runtime.
    /// this ensures that word[0] will be the right instruction
    pub fn preprocesscode(&mut self, code:&mut Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut preprocessedvec:Vec<Vec<String>> = Vec::new();
        for xline in code{

            //print(&xline.join(" "),"bg");
            if xline[0] != "" {
            match xline.len(){
                0 => {}
                1 => {
                        match xline[0].as_str(){
                            "return" => {
                                xline.insert(0,"RET".to_string());
                                preprocessedvec.push(xline.to_owned());
                            }
                            "break" => {
                                xline.insert(0,"B".to_string());
                                preprocessedvec.push(xline.to_owned());

                            }
                            "exit" => {

                                xline[0] = "X".to_string();

                                preprocessedvec.push(xline.to_owned());

                            }

                            _ =>{
                                match self.checkwordtype(&xline[0]){
                                    NscriptWordTypes::Variable | NscriptWordTypes::Global | NscriptWordTypes::Property |  NscriptWordTypes::Arraydeclaration |NscriptWordTypes::Bool | NscriptWordTypes::Static | NscriptWordTypes::Number | NscriptWordTypes::Macro | NscriptWordTypes::Array =>{
                                        xline.insert(0,"RV".to_string());
                                        preprocessedvec.push(xline.to_owned());

                                    }
                                    NscriptWordTypes::Structfn =>{
                                        xline.insert(0,"SFN".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    NscriptWordTypes::RustFunction |NscriptWordTypes::Function =>{
                                        if let Some(_) = self.rustfunctions.get_mut(&split(&Nstring::trimleft(&xline[0],1),"(")[0].to_string()){
                                            let mut insertvec:Vec<String> = Vec::new();
                                            insertvec.push("RFN".to_string());
                                            let splitfunc = split(&xline[0],"(")[0];
                                            insertvec.push(splitfunc.to_string());
                                            let getargs = Nstring::stringbetween(&xline[0],"(", ")");
                                            insertvec.push(getargs);
                                            preprocessedvec.push(insertvec.to_owned());
                                        }
                                        else{
                                            let getargs = Nstring::stringbetween(&xline[0], "(", ")");
                                            let givenargs = Nstring::split(&getargs,",");
                                            let mut newline: Vec<String>  = Vec::new();
                                            newline.push("FN".to_string());
                                            newline.push(xline[0].to_string());
                                            newline.push(" ".to_string());
                                            newline.push(split(&xline[0],"(")[0].to_string());
                                            for xword in givenargs{
                                                newline.push(xword.to_string());

                                            }
                                            preprocessedvec.push(newline.to_owned());
                                        }
                                    }
                                    NscriptWordTypes::Classfunc =>{
                                        let getfirst = split(&xline[0],"(")[0];
                                        let selfname = split(&getfirst,".")[0];
                                        let toparse = split(&xline[0],&(selfname.to_string()+"."))[1];
                                        if Nstring::instring(&xline[0], ").") {
                                            let mut newwordvec:Vec<String> = Vec::new();
                                            newwordvec.push("CH".to_string());
                                            let splitstr = split(&toparse,").");
                                            for x in 0..splitstr.len(){
                                                let subfuncname = split(&splitstr[x],"(")[0];
                                                let args = split(&splitstr[x],"(")[1];
                                                let fullfnc = selfname.to_string() + "."+ &subfuncname +"("+ &args + ")";
                                                newwordvec.push(fullfnc.to_string());
                                            }
                                            preprocessedvec.push(newwordvec.to_owned());
                                        }
                                        else{
                                            let splitfunc = split(&split(&xline[0],"(")[0],".");
                                            let getargs = Nstring::stringbetween(&xline[0], "(", ")");
                                            let mut newline:Vec<String> = Vec::new();
                                            newline.push("CFN".to_string());
                                            newline.push(splitfunc[0].to_string());
                                            newline.push(splitfunc[1].to_string());
                                            newline.push(getargs.to_string());
                                            preprocessedvec.push(newline.to_owned());
                                        }

                                    }
                                    NscriptWordTypes::Nestedfunc =>{
                                        xline.insert(0,"NFN".to_string());
                                        preprocessedvec.push(xline.clone());
                                    }
                                    _ =>{}
                            }

                            //print(&xline.join(" "),"by");
                        }
                    }
                }
                2 =>{
                    match xline[0].as_str(){
                        "return" => {
                                match self.checkwordtype(&xline[1]){
                                    NscriptWordTypes::RustFunction |NscriptWordTypes::Function =>{
                                            let getargs = Nstring::stringbetween(&xline[1], "(", ")");
                                            let givenargs = Nstring::split(&getargs,",");
                                        if let Some(_) = self.rustfunctions.get_mut(&split(&Nstring::trimleft(&xline[1],1),"(")[0].to_string()){

                                            let mut newline: Vec<String>  = Vec::new();
                                            let getargs = Nstring::stringbetween(&xline[1], "(", ")");
                                            newline.push("R_RFN".to_string());
                                            newline.push(xline[0].to_string());
                                            newline.push(getargs);
                                            preprocessedvec.push(xline.to_owned());
                                        }
                                        else{

                                            let mut newline: Vec<String>  = Vec::new();
                                            newline.push("R_FN".to_string());
                                            newline.push(xline[0].to_string());
                                            newline.push(" ".to_string());
                                            //newline.push(" ".to_string());
                                            newline.push(split(&xline[1],"(")[0].to_string());
                                            for xword in givenargs{
                                                newline.push(xword.to_string());

                                            }

                                            preprocessedvec.push(newline.to_owned());
                                        }
                                    }
                                    NscriptWordTypes::Nestedfunc =>{
                                        xline.insert(0,"R_NFN".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    NscriptWordTypes::Classfunc =>{
                                        let mut newline: Vec<String>  = Vec::new();
                                        newline.insert(0,"R_CFN".to_string());
                                        let splitfunc = split(&split(&xline[1],"(")[0],".");
                                        let getargs = Nstring::stringbetween(&xline[1], "(", ")");
                                        newline.push(splitfunc[0].to_string());
                                        newline.push(splitfunc[1].to_string());
                                        newline.push(getargs.to_string());
                                        preprocessedvec.push(newline.to_owned());
                                    }
                                    NscriptWordTypes::Variable =>{
                                        let mut newline: Vec<String>  = Vec::new();
                                        newline.insert(0,"RV".to_string());

                                        newline.push(xline[1].to_string());
                                        preprocessedvec.push(newline.to_owned());
                                    }
                                    NscriptWordTypes::Static =>{
                                        let mut newline: Vec<String>  = Vec::new();
                                        newline.insert(0,"RS".to_string());

                                        newline.push(xline[1].to_string());
                                        preprocessedvec.push(newline.to_owned());
                                    }
                                    NscriptWordTypes::Property =>{
                                        let mut newline: Vec<String>  = Vec::new();
                                        newline.insert(0,"RP".to_string());
                                        let split = split(&xline[1],".");
                                        newline.push(split[0].to_string());
                                        newline.push(split[1].to_string());
                                        preprocessedvec.push(newline.to_owned());
                                    }
                                    _ =>{
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                }
                        }
                        "break" => {
                            xline.insert(0,"BC".to_string());
                            preprocessedvec.push(xline.to_owned());
                        }
                        "init" => {
                                xline[0] = "i".to_string();
                            preprocessedvec.push(xline.to_owned());

                        }
                        "SCOPE" =>{
                                xline[0] = "S".to_string();
                            preprocessedvec.push(xline.to_owned());
                            }
                            _ =>{
                                match xline[1].as_str(){
                                    "++" => {
                                        match self.checkwordtype(&xline[0]){
                                            NscriptWordTypes::Variable =>{
                                                xline.insert(0,"l+".to_string());
                                            }
                                            _ => {
                                                xline.insert(0,"+".to_string());
                                            }
                                        }
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "--" => {
                                        xline.insert(0,"-".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "!" => {
                                        xline.insert(0,"d".to_string());
                                        xline.insert(1,"y".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "!!" => {
                                        xline.insert(0,"d".to_string());
                                        xline.insert(1,"r".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    _ =>{}
                                }
                            }
                        }

                    }
                    _ =>{
                        match xline[0].as_str(){
                            "BRF" =>{
                                //print("pushing BRF","bg");
                                let mut newline :Vec<String> = Vec::new();
                                newline.push("BRF".to_string());
                                //let words = split(&xline," ");
                                for x in xline{
                                    if x != "BRF"{
                                        let splitwords = split(Nstring::trimsuffix(&x),"(");
                                        newline.push(splitwords[0].to_string());
                                        newline.push(splitwords[1].to_string());
                                    }
                                }
                                preprocessedvec.push(newline.to_owned());
                            }
                            "if" =>{
                                if xline.len() < 6 {
                                    print("parsing if scope failed, line is smaller then 5 words!!!","br");
                                }
                                match self.checkwordtype(&xline[1]){
                                    NscriptWordTypes::RustFunction =>{
                                        if Nstring::fromleft(&xline[1],11) =="1timerdiff(" && xline.len() == 6 && xline[2] ==">" && Nstring::prefix(&xline[3]) =="%"{
                                            xline[0] = "tI".to_string();
                                            xline[3] = Nstring::trimleft(&xline[3],1);
                                        }
                                        else{
                                            xline[0] = "I".to_string();
                                        }
                                    }
                                    NscriptWordTypes::Variable =>{
                                        if  xline.len() == 6 && xline[2] == ">" && Nstring::prefix(&xline[3]) =="%"{
                                            xline[0] = "iI".to_string();
                                            xline[3] = Nstring::trimleft(&xline[3],1);
                                        }
                                        else{
                                            xline[0] = "I".to_string();
                                        }
                                    }
                                    _=>{
                                        xline[0] = "I".to_string();
                                    }
                                }
                                if Nstring::fromleft(&xline[1],11) =="1timerdiff(" && xline.len() == 6 && xline[2] == ">" && Nstring::prefix(&xline[3]) =="%"{
                                    xline[0] = "tI".to_string();
                                    xline[3] = Nstring::trimleft(&xline[3],1);
                                }


                                preprocessedvec.push(xline.to_owned());
                            }
                            "elseif"=>{
                                if xline.len() < 6 {
                                    print("parsing if scope failed, line is smaller then 5 words!!!","br");
                                }
                                xline[0] = "EI".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "match"  =>{
                                xline[0] = "M".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "else"=>{
                                xline[0] = "E".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "spawnthread"=>{
                                xline[0] = "ST".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "coroutine" =>{

                                xline[0] = "CO".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "loop" =>{
                                xline[0] = "L".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "for" => {
                                if xline[2] == "in" {
                                    xline.insert(0,"FI".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                else if xline[2] == "to" {
                                    xline.insert(0,"FT".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                            }
                            _ =>{
                                match xline[1].as_str(){
                                    ":" =>{
                                        xline.insert(0,"SC".to_string());
                                        preprocessedvec.push(xline.to_owned());

                                    }
                                    "=" => {
                                        match xline[2].as_str(){
                                            "vec" => {
                                                if xline[4] == "to" {
                                                    xline.insert(0,"vt".to_string());
                                                    preprocessedvec.push(xline.to_owned());

                                                }
                                                if xline[4] == "in" {
                                                    xline.insert(0,"vi".to_string());
                                                    preprocessedvec.push(xline.to_owned());

                                                }
                                            }
                                            "string" => {
                                                if xline[4] == "to" {
                                                    xline.insert(0,"st".to_string());
                                                    preprocessedvec.push(xline.to_owned());
                                                }
                                                if xline[4] == "in" {
                                                    xline.insert(0,"si".to_string());
                                                    preprocessedvec.push(xline.to_owned());
                                                }
                                            }
                                            "cat" => {
                                                xline.insert(0,"1".to_string());
                                                preprocessedvec.push(xline.to_owned());
                                            }
                                            "cat[]" => {
                                                xline.insert(0,"2".to_string());
                                                preprocessedvec.push(xline.to_owned());

                                            }
                                            "match" => {
                                                xline.insert(0,"SM".to_string());
                                                preprocessedvec.push(xline.to_owned());
                                            }
                                            "math" =>{
                                                if xline[4] == "+" || xline[4] == "-" || xline[4] == "/"  || xline[4] == "*"{
                                                    xline.insert(0,"M4".to_string());
                                                    preprocessedvec.push(xline.to_owned());
                                                }

                                            }
                                            _ =>{
                                                if xline.len() > 3 {
                                                    if xline[3] == "+" || xline[3] == "-" || xline[3] == "/"  || xline[3] == "*"{
                                                        xline.insert(0,"M4".to_string());
                                                        xline.insert(2,"math".to_string());
                                                        preprocessedvec.push(xline.to_owned());
                                                    }
                                                    else if xline[3] == "!=" || xline[3] == ">=" || xline[3] == "<="  || xline[3] == "==" || xline[3] == "<" || xline[3] == ">"{
                                                        xline.insert(0,"SBL".to_string());
                                                        preprocessedvec.push(xline.to_owned());
                                                    }

                                                }
                                                if xline.len() == 3 {
                                                    match self.checkwordtype(&xline[2]){
                                                        NscriptWordTypes::RustFunction | NscriptWordTypes::Function=>{
                                                            match self.checkwordtype(&xline[0]){
                                                                NscriptWordTypes::Variable =>{
                                                                    if let Some(_) = self.rustfunctions.get_mut(&split(&Nstring::trimleft(&xline[2],1),"(")[0].to_string()){

                                                                        let mut newline: Vec<String>  = Vec::new();
                                                                        let funcname = split(&xline[2],"(")[0];
                                                                        let getargs = Nstring::stringbetween(&xline[2], "(", ")");
                                                                        //let givenargs = Nstring::split(&getargs,",");
                                                                        newline.push("xRFN".to_string());
                                                                        newline.push(xline[0].to_string()); // variable
                                                                        newline.push(xline[1].to_string());// =
                                                                        newline.push(funcname.to_string());// =
                                                                        newline.push(getargs);
                                                                        preprocessedvec.push(newline.to_owned());
                                                                    }
                                                                    else{
                                                                        let getargs = Nstring::stringbetween(&xline[2], "(", ")");
                                                                        let givenargs = Nstring::split(&getargs,",");
                                                                        let mut newline: Vec<String>  = Vec::new();
                                                                        newline.push("xVF".to_string());
                                                                        newline.push(xline[0].to_string());
                                                                        newline.push("=".to_string());
                                                                        newline.push(split(&xline[2],"(")[0].to_string());
                                                                        for xword in givenargs{
                                                                            newline.push(xword.to_string());

                                                                        }

                                                                        preprocessedvec.push(newline.to_owned());
                                                                    }
                                                                }
                                                                _ =>{
                                                                    xline.insert(0,"xF".to_string());

                                                                    preprocessedvec.push(xline.to_owned());
                                                                }
                                                            }
                                                        }
                                                        NscriptWordTypes::Classfunc =>{

                                                            let mut newline:Vec<String> = Vec::new();
                                                            match self.checkwordtype(&xline[0]){
                                                                NscriptWordTypes::Variable =>{
                                                                    newline.insert(0,"xVCF".to_string());
                                                                }
                                                                _ =>{
                                                                    newline.insert(0,"xCF".to_string());
                                                                }
                                                            }

                                                            newline.push(xline[0].to_owned()); // variable to set nme
                                                            let splitfunc = split(&split(&xline[2],"(")[0],".");
                                                            let getargs = Nstring::stringbetween(&xline[2], "(", ")");
                                                            newline.push(splitfunc[0].to_string());
                                                            newline.push(splitfunc[1].to_string());
                                                            newline.push(getargs.to_string());
                                                            preprocessedvec.push(newline.to_owned());
                                                            //preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Structfn =>{
                                                            xline.insert(0,"xSF".to_string());
                                                            preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Nestedfunc =>{
                                                            //
                                                            let mut newwordvec:Vec<String> = Vec::new();
                                                            match self.checkwordtype(&xline[0]){

                                                                NscriptWordTypes::Variable =>{

                                                                    newwordvec.push("xVNF".to_string());

                                                                    // xline.insert(0,"SETVVNF".to_string());
                                                                }
                                                                _ =>{

                                                                    newwordvec.push("xNF".to_string());
                                                                    //xline.insert(0,"SETVNF".to_string());
                                                                }
                                                            }

                                                            newwordvec.push(xline[0].clone());
                                                            newwordvec.push(xline[1].clone());
                                                            newwordvec.push(xline[2].clone());
                                                            preprocessedvec.push(newwordvec);
                                                        }
                                                        NscriptWordTypes::Static =>{
                                                            match self.checkwordtype(&xline[0]){
                                                                NscriptWordTypes::Variable =>{
                                                                    xline.insert(0,"~".to_string());
                                                                    xline[3] = Nstring::trimprefix(&xline[3]).to_string();
                                                                    preprocessedvec.push(xline.to_owned());
                                                                }
                                                                _ =>{
                                                                    xline.insert(0,"~p".to_string());
                                                                    xline[3] = Nstring::trimprefix(&xline[3]).to_string();
                                                                    preprocessedvec.push(xline.to_owned());
                                                                }
                                                            }
                                                        }
                                                        NscriptWordTypes::Number =>{

                                                            match self.checkwordtype(&xline[0]){
                                                            NscriptWordTypes::Variable =>{
                                                                    xline.insert(0,"%".to_string());
                                                                    xline[3] = Nstring::trimprefix(&xline[3]).to_string();
                                                                    preprocessedvec.push(xline.to_owned());
                                                                }
                                                                _ =>{
                                                                    xline.insert(0,"%p".to_string());
                                                                    xline[3] = Nstring::trimprefix(&xline[3]).to_string();
                                                                    preprocessedvec.push(xline.to_owned());
                                                                }
                                                            }

                                                        }
                                                        NscriptWordTypes::Variable | NscriptWordTypes::Global | NscriptWordTypes::Property |
                                                            NscriptWordTypes::Bool   |
                                                            NscriptWordTypes::Macro | NscriptWordTypes::Array =>{
                                                            xline.insert(0,"SETV".to_string());
                                                            preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Arraydeclaration =>{
                                                            xline.insert(0,"SETVEC".to_string());
                                                            preprocessedvec.push(xline.to_owned());
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "&=" =>{
                                        xline.insert(0,"CC".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "-=" =>{
                                    xline.insert(0,"SS".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }

                                "+=" =>{
                                    xline.insert(0,"AA".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                _ =>{// adds match subscopes
                                 //print(&format!("unknown line? [{}]",xline.join(" ")),"r");
                                    //xline.insert(0,"??".to_string());
                                    preprocessedvec.push(xline.to_owned());

                                }
                            }
                        }
                    }

                }
            }
        }
        }
        preprocessedvec
    }

    fn executepreproccessedline(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{

        //print("here","br");
        //:w
        //print(&line.join(" "),"y");
        match line[0].as_ref(){
            "S" =>{ // scope
                if let Some(ret) = self.executesubscope(&line,&formattedblock,block){
                    return ret;
                };
                return NscriptVar::new("line");
            }
            "NFN" =>{
                self.execute_nestedfunction(&line[1], formattedblock, block);

            }
            "SFN" =>{
                  return self.execute_ruststructfn(&line[1],&formattedblock, block) ;
            }
            "FN" =>{
                 self.execute_prencfunction(&line[3],&line, block) ;
            }
            "CFN" =>{
                 self.execute_preformattedclassfunction(&line[1],&line[2],&line[3], block) ;
            }
            "xF" =>{
                let  onvar = self.execute_function(&line[3], block);
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "xVF" =>{
                let  onvar = self.execute_prencfunction(&line[3],&line,block);
                block.setvar(&line[1], onvar);
            }
            "xRFN" =>{
                let  onvar = self.execute_prerustfunction(&line[3],&line[4], block);
                block.setvar(&line[1], onvar);
            }
            "xNF" =>{
                let var = self.execute_nestedfunction(&line[3], formattedblock, block);
                self.setdefiningword(&line[1], var,&formattedblock, block);
            }
            "xVNF" =>{
                let var = self.execute_nestedfunction(&line[3], formattedblock, block);
                block.setvar(&line[1],var);
            }

            "xSF" =>{
                let onvar = self.execute_ruststructfn(&line[3],&formattedblock, block);
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "xCF" =>{
                let  onvar = self.execute_preformattedclassfunction(&line[2],&line[3],&line[4], block);
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "xVCF" =>{
                let  onvar = self.execute_preformattedclassfunction(&line[2],&line[3],&line[4], block);
                block.setvar(&line[1], onvar);
            }
            "SETV" =>{
                let  onvar = self.storage.getvar(&line[3],block);
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "SETVEC" =>{
                let  onvar = self.executeword(&line[3],&formattedblock,block);
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "RFN" =>{
                return self.execute_prerustfunction(&line[1],&line[2], block);
            }
            "M" =>{
                let tomatch = self.getwordstring(&line[1],&formattedblock, block);
                return self.matchscope(&tomatch,line[line.len()-1].parse::<usize>().unwrap_or(0)-1,&formattedblock, block);
            }
            "B" =>{
                return NscriptVar::newstring("return","break".to_string());
            }
            "RET" =>{
                return NscriptVar::new("return");
            }
            "RV" =>{
                let mut retvar = block.getvar(&line[1]);
                retvar.setreturn();
                return retvar;
            }
            "RS" =>{
                return NscriptVar::newstring("return", block.staticstrings[Nstring::trimprefix(&line[1]).parse::<usize>().unwrap_or(0)].to_string() );
            }
            "RP" =>{
                let mut retvar = self.storage.classgetprop(&Nstring::trimprefix(&line[1]),&line[2], block);
                retvar.setreturn();
                return retvar;
            }
            "return" =>{
                let mut retvar = self.executeword(&line[1], &formattedblock,block);
                retvar.setreturn();
                return retvar;
            }
            "R_FN" =>{
                let mut retvar = self.execute_prencfunction(&line[3],&line, block);
                retvar.setreturn();
                return retvar;
            }
            "R_RFN" =>{
                let mut retvar = self.execute_prerustfunction(&line[2],&line[2], block);
                retvar.setreturn();
                return retvar;
            }
            "R_CFN" =>{
                let mut retvar = self.execute_preformattedclassfunction(&line[1],&line[2],&line[3], block);
                retvar.setreturn();
                return retvar;
            }
            "R_NFN" =>{
                let mut retvar = self.execute_nestedfunction(&line[2],&formattedblock, block);
                retvar.setreturn();
                return retvar;
            }
            "CH" =>{
                for x in 1..line.len(){
                    self.execute_classfunction(&line[x], block) ;
                }
            }
            "SC" =>{
                self.execute_setclassfromclass(&line[1],&line[3],&formattedblock, block);
                return NscriptVar::new("line");
            }
            "L" =>{
                 self.execute_spawnloop(&line,&formattedblock,block);
            }
            "vi" =>{
                let get = self.execute_vecloopsin(&line,&formattedblock, block);
                return self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "vt" =>{
                let get = self.execute_vecloopsto(&line,&formattedblock, block);
                return self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "si" =>{
                let get = self.execute_stringloopsin(&line,&formattedblock, block);
                return self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "st" =>{
                let get = self.execute_stringloopsto(&line,&formattedblock, block);
                return self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "-" =>{
                let mut onvar = self.storage.getvar(&line[1],block);
                onvar.stringdata = (onvar.getnumber() - 1).to_string();
                return self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "+" =>{
                let mut onvar = self.storage.getvar(&line[1], block);
                onvar.stringdata = (onvar.getnumber() + 1).to_string();
                return self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "l+" =>{
                let  onvar = block.getstr(&line[1]);
                let onvar = onvar.parse::<u64>().unwrap_or(0) + 1; //(onvar.getnumber() + 1).to_string();
                 block.setstring(&line[1], onvar.to_string());//,&formattedblock, block);
            }
            "1" => {
                let mut onvar = NscriptVar::new(&line[1]);
                for xadd in 4..line.len(){
                    onvar.stringdata += &self.getwordstring(&line[xadd],&formattedblock,block);
                }
                return self.setdefiningword(&line[1], onvar,&formattedblock,block);
            }
            "~" =>{ // set local with static
                block.setstring(&line[1], block.staticstrings[line[3].parse::<usize>().unwrap_or(0)].to_string());
            }
            "~p" =>{ // set definable with static
                self.setdefiningword(&line[1], NscriptVar::newstring(
                    &line[1],
                    block.staticstrings[line[3].parse::<usize>().unwrap_or(0)].to_string()
                ),&formattedblock,block);
            }
            "%" =>{ // set local with number
                block.setstring(&line[1], line[3].to_string());
            }
            "%p" =>{ // set definable with number
                self.setdefiningword(&line[1], NscriptVar::newstring(
                    &line[1],
                    line[3].to_string()
                ),&formattedblock,block);
            }
            "2" => {
                let mut onvar = NscriptVar::new(&line[1]);
                for xadd in 4..line.len(){
                    let this = self.getwordstring(&line[xadd],&formattedblock,block);
                    onvar.stringvec.push(this);
                }
                return self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "SM" =>{
                let tomatch = self.getwordstring(&line[4], &formattedblock,block);
                let mut onvar = self.matchscope(&tomatch,line[line.len()-1].parse::<usize>().unwrap_or(1)-1, &formattedblock,block);
                onvar.name = line[1].to_string();
                return self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "M4" =>{
                let onvar = self.runmath(&line, 4,&formattedblock,  block);
                return self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "I" =>{
                return self.execute_ifline(&line,&formattedblock, block);
            }
            "EI" =>{
                return self.execute_elseifline(&line,&formattedblock,block);
            }
            "E" =>{
                return self.execute_elseline(&line,&formattedblock,block);
            }
            "BC" =>{
                let tobreak = self.storage.getargstring(&line[2],block);
                self.removecoroutine(&tobreak);
            }
            "ST" => {
                return self.execute_spawnthread(line, &formattedblock,block);
            }
            "FI" =>{
                self.execute_forinloop(&line,&formattedblock,block);
            }
            "FT" =>{
                self.execute_fortoloop(&line,&formattedblock,block);
            }
            "CC" =>{//concate self
                let mut equalsfrom = self.storage.getvar(&line[1],block);
                for xadd in 3..line.len(){
                    equalsfrom.stringdata = equalsfrom.stringdata + &self.executeword(&(line[xadd].to_string()),&formattedblock,block).stringdata.to_string();
                }
                return self.setdefiningword(&line[1], equalsfrom, &formattedblock,block);
            }
            "SS" =>{// subtract self
                let mut onvar = self.storage.getvar(&line[1], block);
                let mut total = onvar.getnumber();
                for x in 3..line.len(){
                    total -= self.executeword(&line[x],&formattedblock, block).getnumber();
                }
                onvar.stringdata = total.to_string();
                return self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "BRF" =>{
                for x in (1..line.len()).step_by(2){
                    self.execute_prerustfunction(&line[x],&line[x+1], block);
                }
            }
            "AA" =>{ // add se;f
                let mut onvar = self.storage.getvar(&line[1], block);
                let mut total = onvar.getnumber();
                for x in 3..line.len(){
                    total += self.executeword(&line[x], &formattedblock,block).getnumber();
                }
                onvar.stringdata = total.to_string();
                return self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "tI" =>{//optimized timerchecks
                if f64(&self.execute_rustfunction(&line[1], block).stringdata) > f64(&line[3]){
                    return self.execute_ifscopeup(true,line,formattedblock,block);
                }
            }
            "iI" =>{//optimized localvar > number
                if f64(&block.getstring(&line[1])) > f64(&line[3]){
                    return self.execute_ifscopeup(true,line,formattedblock,block);
                }
            }
            "i" => {
                let script = self.getwordstring(&line[1],&formattedblock, block);
                return self.parsefile(&script);
            }
            "d" =>{
                let retvar = self.executeword(&line[2],&formattedblock,block);
                print(&format!("Debuggin!\n  block:[{}]>> {} => \n   >  var string:[{}] \n   >  array contents:[{}]",&block.name,&line[1],&retvar.stringdata,&retvar.stringvec.join(",")),&line[1]);
                //return retvar;
            }
            // "r" =>{
            //     let retvar = self.executeword(&line[1],&formattedblock,block);
            //     print(&format!("Debuggin!\n  bblock:[{}]>> {} => \n   > var string:[{}] \n   >  array contents:[{}]",&block.name,&line[1],&retvar.stringdata,&retvar.stringvec.join(",")),"r");
            //     return retvar;
            // }
            "SBL" =>{// set bool
                let mut stringvec:Vec<Box<str>> = Vec::new();
                stringvec.push("if".into());
                for x in 3..line.len(){
                    stringvec.push(line[x].to_string().into_boxed_str());
                }
                stringvec.push("".into());
                stringvec.push("".into());
                let retvar = NscriptVar::newstring("l",self.parse_and_check_statements(&stringvec,&formattedblock, block).to_string());
                return self.setdefiningword(&line[1], retvar, &formattedblock,block);
            }
            "CO"=>{
                self.execute_spawncoroutine(&line,&formattedblock,block);
            }
            "X"=>{
                //exit
            }
            _ =>{}
        }
        return NscriptVar::new("line");
    }
    fn execute_spawnloop(&mut self ,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        block.inloop +=1;
        block.breakloop[block.inloop] = false;
        loop {
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    if result.stringdata == "break"{
                        block.breakloop[block.inloop] = true;
                        block.inloop -=1;
                        break;
                    }
                    else{
                        block.inloop -=1;
                        return result;
                    }
                }
            }
            if block.breakloop[block.inloop] {
                block.breakloop[block.inloop] = true;
                block.inloop -=1;
                break;
            }
        }
        return NscriptVar::new("loop");
    }
    fn execute_spawncoroutine(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock){
        let coname = "coroutine_".to_string() + &self.getwordstring(&line[1],&formattedblock, block);
        let mut coroutineblock = NscriptCodeBlock::new(&coname);//NscriptCodeBlock::new(&coname);
        coroutineblock.name = coname.to_string();

        //let mut formattedcode = formattedblock.clone();//self.getformattedblock(&block.name);
        let mut executablecode = formattedblock.clone();//self.getexecutableblock(&block.name);
        //formattedcode.name = coname.to_string().into();
         coroutineblock.strings = block.strings.clone();
         coroutineblock.stringsvec = block.stringsvec.clone();
        let mut selfvar = NscriptVar::new("self");
        selfvar.stringdata = coname.to_string();
        coroutineblock.setvar("self", selfvar);
        coroutineblock.staticstrings = block.staticstrings.clone();
        let scopeid = Nstring::usize(&line[line.len()-1]);
        executablecode.boxedcode[0] = formattedblock.boxedcode[scopeid-1].clone();
        //executablecode.boxedcode = formattedcode.boxedcode.clone();
        self.executableblocks.insert(coname.to_string(),executablecode.clone());
        for xl in &executablecode.boxedcode[0]{
            for xa in xl {

                print(&xa,"r");
            }
        }
        self.addcoroutine(&coname);
        //self.formattedblocks.insert(coname.to_string(), executablecode);
        self.storage.codeblocks.insert(coname,coroutineblock );

    }
     fn execute_ifline(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        let statementresult = self.parse_and_check_statements(&line, &formattedblock,block);
        if statementresult == true{
            return self.execute_ifscopeup(statementresult,&line,&formattedblock, block);
            // block.ifsetup(statementresult);
            // //block.ifup();
            // if let Some(result) = self.executesubscope(&line,&formattedblock, block){
            //     block.ifdown();
            //     if result.name == "return" {
            //         return result;
            //     }
            // }
            // block.ifdown();
        }
        else{
            block.ifscopes[block.ifscopedepth] = false;
        }
        let result = NscriptVar::new("if");
        return result;
    }
    fn execute_ifscopeup(&mut self,bl:bool,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
            block.ifsetup(bl);
            //block.ifup();
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                block.ifdown();
                //if result.name == "return" {
                    return result;
                //}
            }
            block.ifdown();

    return NscriptVar::new("if");

    }
    /// executes a if elsescope
    fn execute_elseifline(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        if block.ifscopes[block.ifscopedepth] == false{
            let statementresult = self.parse_and_check_statements(&line,&formattedblock, block);
            if statementresult{
                block.ifsetup(statementresult);
                //block.ifup();
                if block.ifscopes[block.ifscopedepth] == false{
                    if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                        block.ifdown();
                        if result.name == "return" {
                            return result;
                        }
                    }
                    block.ifdown();
                }
            }
            else{
                block.ifscopes[block.ifscopedepth] = false;
        }
        }
        let result = NscriptVar::new("if");
        return result;
    }
    ///executes a else scope
    fn execute_elseline(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        if block.ifscopes[block.ifscopedepth] == false{
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return" {
                    return result;
                }
            }
        }
        let result = NscriptVar::new("else");
        return result;
    }
    /// used for inherenting to other classes
    fn execute_setclassfromclass(&mut self,classto:&str,classfrom:&str,formattedblock: &NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock){
        let mut thisclass = classto.to_string();
        match self.checkwordtype(&classto){
            NscriptWordTypes::Reflection => {
                thisclass = self.getwordstring(&classto, &formattedblock,block);
            }
            _ =>{}
        }
        let mut fromclass = classfrom.to_string();
        match self.checkwordtype(&classfrom){
            NscriptWordTypes::Reflection => {
                fromclass = self.getwordstring(&classfrom, &formattedblock,block);
            }
            _ =>{}
        }
        let mut parentclass = self.getclass(&fromclass);
        if let Some(class) = self.getclassref(&thisclass){
            class.inherent(&mut parentclass);
        }else{
            let class = parentclass.copyto(&thisclass);
            self.insertclass(&thisclass, class);
        }
    }
    /// a for loop : something = vec x to 10{}
    fn execute_vecloopsin(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let mut vecvar = NscriptVar::new(&line[1]);
        let arrayvar = self.executeword(&line[6], &formattedblock,block);
        let mut iteratevar = NscriptVar::new(&line[4]);
        for index in arrayvar.stringvec {
            iteratevar.stringdata = index.to_string();
            block.setvar(&line[4], iteratevar.clone());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    vecvar.stringvec.push(result.stringdata.to_string());
                }
            }
        }
        return vecvar;
    }
    /// a for loop : something = vec x to 10{}
    fn execute_vecloopsto(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let mut vecvar = NscriptVar::new(&line[1]);
        let splitfrom = split(&line[4],"=");
        let mut start = 0;
        if splitfrom.len() > 1{
            start = self.getwordstring(&splitfrom[1],&formattedblock,block).parse::<usize>().unwrap_or(0);
        }
        let iteratevar = self.executeword(&line[6],&formattedblock, block);
        for index in start..iteratevar.stringdata.parse::<usize>().unwrap_or(0)+1 {
            block.setstring(&splitfrom[0], index.to_string());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    vecvar.stringvec.push(result.stringdata.to_string());
                }
            }
        }
        return vecvar;
    }
    /// a for loop : something = string x in array{}
    fn execute_stringloopsin(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let arrayvar = self.executeword(&line[6], &formattedblock,block);
        let mut iteratevar = NscriptVar::new(&line[4]);
        let mut createstring = "".to_string();
        for index in arrayvar.stringvec {
            iteratevar.stringdata = index.to_string();
            block.setvar(&line[4], iteratevar.clone());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    createstring = createstring + &result.stringdata;
                }
            }
        }
        return NscriptVar::newstring(&line[0], createstring);
    }

    /// a for loop : something = string x to 10{}
    fn execute_stringloopsto(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{

        let splitfrom = split(&line[4],"=");
        let mut start = 0;
        if splitfrom.len() > 1{
            start = self.getwordstring(&splitfrom[1],&formattedblock,block).parse::<usize>().unwrap_or(0);
        }
        let mut createstring = "".to_string();
        let iteratevar = self.executeword(&line[6], &formattedblock,block);
        for index in start..iteratevar.stringdata.parse::<usize>().unwrap_or(0)+1 {
            block.setstring(&splitfrom[0], index.to_string());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    createstring = createstring + &result.stringdata;
                }
            }
        }

        return NscriptVar::newstring(&line[0], createstring);
    }
    //
    fn execute_forinloop(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let arrayvar = self.executeword(&line[4],&formattedblock, block);
        let mut iteratevar = NscriptVar::new(&line[2]);
        for index in arrayvar.stringvec {
            iteratevar.stringdata = index.to_string();
            block.setvarstring(&line[2], iteratevar.clone());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return" && result.name == "break"{
                    return result;
                }
            }
        }
        return iteratevar;
    }
     fn execute_fortoloop(&mut self,line:&Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
         let splitfrom = split(&line[2],"=");
         let mut start = 0;
         if splitfrom.len() > 1{
             start = self.getwordstring(&splitfrom[1],&formattedblock,block).parse::<usize>().unwrap_or(0);
         }
         let iteratevar = self.executeword(&line[4], &formattedblock,block);
         for index in start..iteratevar.stringdata.parse::<usize>().unwrap_or(0)+1 {
            block.setstring(&splitfrom[0],index.to_string());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return" && result.stringdata == "break"{
                    return result;
                }
             }
         }
         return iteratevar;
     }

     fn execute_spawnthread(&mut self,line:&Vec<Box<str>>,formattedblock:&NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) -> NscriptVar{
        let fromthreadname = "thread_".to_string()+&line[line.len()-1];
        let threadname = "thread_".to_string()+&self.executeword(&line[1],&formattedblock, block).stringdata;
        let mut threadcode = self.getblock(&fromthreadname);
        let formattedcode = self.getformattedblock(&fromthreadname);
        let mut selfvar = NscriptVar::new("self");
        selfvar.stringdata = threadname.to_string();
        let rawcode = formattedcode.codeblock.to_string();
        //formattedcode.codeblock = "\n".to_string() + &formattedcode.codeblock;
        let (main_to_worker_tx, main_to_worker_rx) = mpsc::channel();
        let (worker_to_main_tx, worker_to_main_rx) = mpsc::channel();
        self.threadsreceiver.insert(threadname.to_string(),worker_to_main_rx);
        self.threadssenders.insert(threadname.to_string(),main_to_worker_tx);
        let worker_to_main_tx = Arc::new(Mutex::new(worker_to_main_tx));
        let max = line.len();
        let mut classvec:Vec<NscriptClass> = Vec::new();
        let mut funcvec:Vec<NscriptFunc> = Vec::new();
        let mut varsvec:Vec<NscriptVar> = Vec::new();
        if max > 2 {
            for xmove in 1 .. max-1{
                let splitword = split(&line[xmove],":");
                match splitword[0]{
                    "c" =>{
                        classvec.push(self.getclass(&splitword[1]));
                    }
                    "f"=>{
                        funcvec.push(self.getfunc(&splitword[1]));
                    }
                    "v"=>{
                        varsvec.push(self.getvar(&splitword[1],block));
                    }
                    _ =>{}
                }
            }
        }
        varsvec.push(selfvar);
        // get all rustfunctions out the main and copy em to the thread
        let builtins = self.rustfunctionsindex.clone();
        let mut builtinsvec:Vec<NscriptSimpleFunctions> = Vec::new();
        for x in builtins.clone(){
            if let Some(f) = self.rustfunctions.get(&x){
                builtinsvec.push(f.to_owned());
            };
        }

        thread::spawn(move || {
            let mut threadstruct = Nscript::thread();
            // insert all rustfunctions to the threadstruct
            let mut i = 0;
            for x in builtins{
                threadstruct.insertfn(&x, builtinsvec[i]);
                i +=1;
            }
            for xfunc in funcvec{
                threadstruct.storage.functions.insert(xfunc.name.to_string(), xfunc.clone());
            }
            for xclass in classvec{
                threadstruct.storage.classes.insert(xclass.name.to_string(), xclass.clone());
            }
            threadstruct.parsecodewithvars(&rawcode,&threadname,varsvec);
            loop {
                threadstruct.executecoroutines();
                if threadstruct.coroutines.len() < 1 {
                    break;
                }
                let sender = worker_to_main_tx.lock().unwrap();
                let received_message: NscriptVar = match main_to_worker_rx.try_recv(){
                    Ok(rmsg) => {
                        rmsg
                    }
                    Err(_)=>{
                        NscriptVar::new("error")
                    }
                };
                if received_message.name != "error"{
                    threadstruct.storage.setglobal("$received",received_message);
                    let ncfunc = "2threadreceive($received)";
                    let ncreturn = threadstruct.execute_ncfunction(&ncfunc,&mut threadcode);
                    match sender.send(ncreturn){
                        Ok(_)=>{},
                        Err(_)=>{},
                    };
                    threadstruct.storage.setglobal("$received", NscriptVar::new("$received"));
                }
            }
        });
        return NscriptVar::new("THREAD")
    }
    /// this function is used to check a defining variable type and set it with a Nvar
    fn setdefiningword(&mut self,word:&str,equalsfrom: NscriptVar,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock)->NscriptVar{
        match self.storage.checkdefiningwordtype(&word){
            NscriptWordTypes::Variable => {
                block.setvar(&word, equalsfrom);
            }
            NscriptWordTypes::Property => {
                let thisword = Nstring::trimprefix(&word);
                let splitword = split(&thisword,".");
                if splitword.len() > 1{
                    let classname:String; // = splitword[0].to_string();
                    if Nstring::prefix(&splitword[0]) == "*" {
                        classname = self.storage.getargstring(&Nstring::trimprefix(&splitword[0]),block);
                    }
                    else{
                        classname = splitword[0].to_string();
                    }
                    let propname:String;// = splitword[1].to_string();
                    if  Nstring::prefix(&splitword[1])  == "*" {
                        propname = self.storage.getargstring(&Nstring::trimprefix(&splitword[1]),block);
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
                        self.insertclass(&classname,newclass);
                    }
                }
            }
            NscriptWordTypes::Global => {
                self.storage.setglobal(&word, equalsfrom);
            }
            NscriptWordTypes::Array =>{
                let thisword = Nstring::trimprefix(&word);
                let wordsplit = split(&thisword,"[");
                let mut var = self.storage.getvar(&wordsplit[0],block);
                let idvar = self.storage.getargstring(&Nstring::trimsuffix(&wordsplit[1]),block).parse::<usize>().unwrap_or(0);
                if idvar < var.stringvec.len(){
                    var.stringvec[idvar] = equalsfrom.stringdata;
                }
                else {
                    print(&format!("block [{}] array [{}] tries to set a index but its out of bounds",&block.name,&wordsplit[0]),"r");
                }
                self.setdefiningword(wordsplit[0], var,&formattedblock, block);
            }
            NscriptWordTypes::Reflection =>{
                 self.setdefiningword(&Nstring::trimprefix(word), equalsfrom,formattedblock,block);
            }
            _ =>{

            }
        };
        return NscriptVar::new("v");
    }

    fn execute_nestedfunction(&mut self,word:&str,formattedblock: &NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{
        let word = Nstring::trimprefix(&word);
        let mut resultstring = word.to_string();
        let mut packed: String;
        let mut subfunction: String;
        let mut varcounter = 0;
        loop {
            varcounter +=1;
            // get the last find in the string using (
            let splitstr = split(&resultstring, "(");
            // make sure its inside the main function so bigger>2
            let len =splitstr.len();
            if len > 2 {
                //take that substring and split up to the first )
                let splitscope = split(&splitstr[len - 1], ")");
                if splitscope.len() > 0 {
                    // important one, if a variable or string is infron it
                    // messes up the syntax so we split using comma
                    let splitargus = split(&splitstr[len - 2], ",");
                    // here we set thisfnname to the last part of the comma split
                    let thisfnnamefix = splitargus[splitargus.len() - 1]; // make sure the function
                    subfunction = format!("{}({})",&thisfnnamefix,&splitscope[0]);
                    let varname = format!("nestedfn_{}",&varcounter);
                    let mut tmpvar = self.executeword(&subfunction,&formattedblock, block);
                    tmpvar.name = varname.to_string();
                    packed = tmpvar.name.to_string();
                    block.setvar(&varname, tmpvar);
                    // here we evaluate the none function types.
                } else {
                    // this also evaluates variables macros strings etc
                    subfunction = splitscope[0].to_string(); //&splitstr[splitstr.len()-1];
                    let varname = format!("nestedfn_{}",&varcounter);
                    let mut tmpvar = self.storage.getvar(&subfunction, block);
                    tmpvar.name = varname.to_string();
                    packed = tmpvar.name.to_string();
                    block.setvar(&varname, tmpvar);
                }
                    resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
            } else {
                break;
            }
        }
        return self.executeword(&resultstring,&formattedblock, block);
    }

    pub fn getwordstring(&mut self,word:&str,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) -> String{

         match self.checkwordtype(word){
            NscriptWordTypes::Static =>{
                return block.staticstrings[Nstring::trimprefix(word).parse::<usize>().unwrap_or(0)].to_string();
            }
            NscriptWordTypes::Variable=>{
                return block.getstring(word);
            }
            NscriptWordTypes::Property=>{
                let thisword = Nstring::trimprefix(word);
                let wordsplit = split(&thisword,".");
                //if wordsplit.len() > 1{
                    let cname:Box<str>; //= wordsplit[0].trim().to_string();
                    let pname :Box<str>; //= wordsplit[1].trim().to_string();
                    if Nstring::prefix(&wordsplit[0]) ==  "*" {
                        cname = self.storage.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[0]), block);
                    }else{
                        cname = wordsplit[0].into();
                    }
                    if Nstring::prefix(&wordsplit[1]) ==  "*" {
                        pname = self.storage.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[1]),block) ;
                    }
                    else{
                        pname = wordsplit[1].into();
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname).stringdata;
                    }else{
                        print(&format!("nscript::getwordstring() block[{}] word [{}]is a prop but theres no class on cname [{}] pname[{}]",&block.name,&word,&cname,&pname),"r");
                        return "".to_owned();
                    }
                //}
            }
            NscriptWordTypes::Number  =>{
                let thisword = Nstring::trimprefix(word);
                return thisword.to_owned();
            }
            NscriptWordTypes::Function => {
                return self.execute_ncfunction(word, block).stringdata;
            }
            NscriptWordTypes::RustFunction => {
                return self.execute_rustfunction(word, block).stringdata;
            }
            NscriptWordTypes::Classfunc => {
                return self.execute_classfunction(word, block).stringdata;
            }
            NscriptWordTypes::Macro =>{
                return self.storage.getmacrostring(word).to_string();
            }
            NscriptWordTypes::Global => {
                return self.storage.getglobal(&word).stringdata;
            }
            NscriptWordTypes::Bool => {
                return Nstring::trimprefix(&word).to_string();
            }
            NscriptWordTypes::Structfn => {
                return self.execute_ruststructfn(word,&formattedblock, block).stringdata;
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimprefix(word);
                let evaluated = self.storage.getargstring(&toreflect, block);
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let thisword = Nstring::trimprefix(&word);
                let arrays = split(&thisword,"[");
                    let thisvar = self.storage.getvar(arrays[0], block);
                    let index = self.getwordstring(&Nstring::trimsuffix(&arrays[1]),&formattedblock,block).parse::<usize>().unwrap_or(0);
                    if thisvar.stringvec.len() > index{
                        return thisvar.stringvec[index].to_string();
                    }else{
                    print(&format!("nscript::getwordstring() block[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&block.name,&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return "".to_owned();
            }
            _ => {
                return self.evaluateword(word,&NscriptWordTypes::Variable,&formattedblock, block).stringdata;
            }
        };

        //return "".to_owned();
    }
    pub fn evaluateword(&mut self,word:&str,vartype:&NscriptWordTypes,formattedblock: &NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock)-> NscriptVar{
       match vartype{
            NscriptWordTypes::Function => {
                return self.execute_ncfunction(&word, block);
            }
            NscriptWordTypes::RustFunction => {
                return self.execute_rustfunction(&word, block);
            }
            NscriptWordTypes::Structfn =>{
                return self.execute_ruststructfn(word,&formattedblock, block);
            }
            NscriptWordTypes::Nestedfunc =>{
                return self.execute_nestedfunction(word,&formattedblock, block);
            }
            NscriptWordTypes::Classfunc =>{
                return self.execute_classfunction(word, block);
            }
            NscriptWordTypes::Static =>{
                return NscriptVar::newstring("s", block.staticstrings[Nstring::trimprefix(word).parse::<usize>().unwrap_or(0)].to_string());
            }
            NscriptWordTypes::Variable=>{
                return block.getvar(word).clone();
            }
            NscriptWordTypes::Property=>{
                let thisword = Nstring::trimprefix(&word);
                let wordsplit = split(&thisword,".");
                let cname:Box<str>;
                let pname :Box<str>;
                if Nstring::prefix(&wordsplit[0]) ==  "*" {
                    cname = self.storage.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[0]), block);
                }else{
                    cname = wordsplit[0].into();
                }
                if Nstring::prefix(&wordsplit[1]) ==  "*" {
                    pname = self.storage.getevaluatablewordstr(&Nstring::trimprefix(&wordsplit[1]),block) ;
                }
                else{
                    pname = wordsplit[1].into();
                }
                if let Some(thisclass) = self.getclassref(&cname){
                    return thisclass.getprop(&pname);
                }else{
                    print(&format!("block[{}] word is a prop but theres no class on cname [{}] pname[{}]",&block.name,&cname,&pname),"r");
                }
            }
            NscriptWordTypes::Number  =>{
                return NscriptVar::newstring("n", Nstring::trimprefix(word).to_string());
            }
            NscriptWordTypes::Bool => {
                return NscriptVar::newstring("b", Nstring::trimprefix(word).to_string());
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimprefix(word);
                let evaluated = self.executeword(&toreflect,&formattedblock, block);
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let mut returnvar = NscriptVar::new("entree");
                let thisword = Nstring::trimprefix(&word);
                let arrays = split(&thisword,"[");
                let thisvar = self.executeword(arrays[0],&formattedblock, block);
                let index = self.storage.getargstring(&Nstring::trimsuffix(&arrays[1]),block).parse::<usize>().unwrap_or(0);
                if thisvar.stringvec.len() > index{
                    returnvar.stringdata = thisvar.stringvec[index].to_string();
                }else{
                    print(&format!("block:[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&block.name,&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return returnvar;
            }
            NscriptWordTypes::Macro =>{
                return NscriptVar::newstring("b", self.storage.getmacrostring(word).to_string());
            }
            NscriptWordTypes::Global => {
                return self.storage.getglobal(&word).clone();
            }
            NscriptWordTypes::Arraydeclaration => {
                let mut thisarrayvar = NscriptVar::new("array");
                let between = Nstring::trimsuffix(&Nstring::trimprefix(&word));
                let inarray = split(&between,",");
                for arrayitem in inarray{
                    thisarrayvar.stringvec.push(self.storage.getargstring(&arrayitem, block));
                }
                return thisarrayvar
            }
        }

        let mut retvar = NscriptVar::new("error");
        print(&format!("error from word {}",&word),"br");
        retvar.setstring("error");
        return retvar;

    }
    /// recursively used to parse a word ,include nested subfunctions in arguments etc
    pub fn executeword(&mut self,word:&str,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) -> NscriptVar {
        let prefix = self.checkwordtype(word);
        return self.evaluateword(word, &prefix,&formattedblock, block)
    }
    pub fn execute_rustfunction(&mut self,word:&str,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitfunc = split(&word,"(");
        return self.execute_prerustfunction(&splitfunc[0],&Nstring::trimsuffix(&splitfunc[1]),  block);
    }
    /// for preproccesed lines (optimizing)
    pub fn execute_prerustfunction(&mut self,funcname:&str,getargs:&str, block:&mut NscriptCodeBlock) ->NscriptVar{
        let funcname = Nstring::trimprefix(&funcname);
        if let Some(rustfn) = self.rustfunctions.get(&funcname.to_string()){
            return rustfn(&split(&getargs,","),block,&mut self.storage);
        }
        print(&format!("cant find func {}",funcname),"r");
        NscriptVar::new("wont")
    }

    /// executes a nsript function
    pub fn execute_ncfunction(&mut self,word:&str,block:&mut NscriptCodeBlock) ->NscriptVar{
        let word = Nstring::trimprefix(&word);
        let splitfunc = split(&word,"(");
        let givenargs = split(&Nstring::trimsuffix(&splitfunc[1]),",");
        if let Some(func) = self.userfunctions.get(&splitfunc[0].to_string()){
            let mut getblock = func.codeblock.clone();
            let formattedblockfunc = self.getexecutableblock(&getblock.name);//func.formattedcodeblock.clone();
            let len = givenargs.len();
            for xarg in 0..len{
                //if len > xarg{
                    getblock.setvar(&func.args[xarg],self.storage.getvar(&givenargs[xarg],block));
                //}else{break;}
            }
            if let Some(resultvar) = self.executescope(&formattedblockfunc.boxedcode[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar;
                }
            };
        }else{
            print(&format!("no ncfunctions found for [{}]",&splitfunc[0]),"r");
        }

        return NscriptVar::new("ncfunc");
    }

    fn execute_function(&mut self,wordr:&str, block:&mut NscriptCodeBlock) ->NscriptVar{
        match self.checkwordtype(wordr){
            NscriptWordTypes::Function =>{
                return self.execute_ncfunction(wordr, block);
            }
            NscriptWordTypes::Classfunc =>{
                return self.execute_classfunction(wordr, block);
            }
            NscriptWordTypes::RustFunction =>{
                return self.execute_rustfunction(wordr, block);
            }
            NscriptWordTypes::Structfn =>{
                let fblock =self.getexecutableblock(&block.name);
                return self.execute_ruststructfn(&wordr,&fblock, block);
            }
            _ =>{
                print(&format!("execute_function() no functions found for [{}] in block: [{}]",&wordr,&block.name),"r");
                return NscriptVar::new("func");
            }
        }
    }

    /// preproccessed lines use this one to skip some checks and gain performance
    pub fn execute_prencfunction(&mut self,funcname:&str, line:&Vec<Box<str>>, block:&mut NscriptCodeBlock) ->NscriptVar{
        let funcname = Nstring::trimprefix(&funcname);
        if let Some(func) = self.userfunctions.get(&funcname.to_string()){
            let mut getblock = func.codeblock.clone();
            let  formattedblockfunc = self.getexecutableblock(&getblock.name);//func.formattedcodeblock.clone();
            //let  args = func.args.clone();
            if line.len() > 4{
                for xarg in 4..4+func.args.len(){
                    getblock.setvar(&func.args[xarg-4],self.storage.getvar(&line[xarg],block));
                }
            }
            if let Some(resultvar) = self.executescope(&formattedblockfunc.boxedcode[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar;
                }
            };
        }else{
            print(&format!("no prencfunctions found for [{}]",&funcname),"r");
        }
        return NscriptVar::new("ncfunc");
    }
    fn execute_classfunction(&mut self,word:&str,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitf = split(&word,"(");
        let splitfunc = split(&splitf[0],".");
        self.execute_preformattedclassfunction(&splitfunc[0], &splitfunc[1], &Nstring::trimsuffix(&splitf[1]), block)
    }
    /// pre parsed execution from nscript.preproccedlines
    fn execute_preformattedclassfunction(&mut self,class:&str , func:&str, givenargs:&str ,block:&mut NscriptCodeBlock) ->NscriptVar{
        let class = Nstring::trimprefix(&class);
        let mut getblock: NscriptCodeBlock;
        let  formattedblockfunc: NscriptExecutableCodeBlock;
        let givenargs = split(&givenargs,",");
        let mut varvec: Vec<NscriptVar> = Vec::new();
        for xarg in &givenargs{
            varvec.push(self.storage.getvar(&xarg,block));
        }
        let classname :Box<str>;
        if Nstring::prefix(&class) == "*"{
            classname = self.storage.getevaluatablewordstr(&Nstring::trimprefix(&class), block);
        }
        else{
            classname = class.into();
        }
        let funcname  :Box<str>;
        if Nstring::prefix(&func) == "*"{
            funcname = self.storage.getevaluatablewordstr(&Nstring::trimprefix(&func), block);
        }
        else{
            funcname = func.into();
        }

        if let Some(class) = self.getclassref(&classname){

            if let Some(thisfunc) = class.functions.get(&funcname.to_string()){
                getblock = thisfunc.codeblock.clone();
                formattedblockfunc = thisfunc.executablecodeblock.clone();//.getexecutableblock(&format!("{}.{}",classname,funcname));//thisfunc.formattedcodeblock.clone();
                let len = givenargs.len();
                for xarg in 0..thisfunc.args.len(){
                    if len > xarg{
                        getblock.setvar(&thisfunc.args[xarg],varvec[xarg].clone());
                    }
                }
                if let Some(resultvar) = self.executescope(&formattedblockfunc.boxedcode[0],&formattedblockfunc,&mut getblock){
                    if resultvar.name == "return"{
                        return resultvar;
                    }
                };
                return NscriptVar::new("func");
            }else{
                print(&format!("cant find classfn [{}].[{}]",&funcname,&classname),"r");
                return NscriptVar::new("error");
            }
        };
        return NscriptVar::new("error");
    }

    fn execute_ruststructfn(&mut self,word:&str,formattedblock:&NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitfunc = split(&word,"(");
        return self.execute_preruststructfn(&splitfunc[0], &Nstring::trimsuffix(&splitfunc[1]),&formattedblock, block);
    }
    /// this executes rust structs which the user injected, their only on the main thread
    /// some built ins like object and njh are defined here
   fn execute_preruststructfn(&mut self, funcname:&str,getargs:&str, formattedblock:&NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{

        let funcname = Nstring::trimprefix(&funcname);
        let givenargs = split(&getargs,",");
        let mut i = 0;
        let mut argvarvec :Vec<NscriptVar> = Vec::new();
        for xarg in &givenargs{
            //if givenargs.len() > i{
                let mut get = self.storage.getvar(&givenargs[i], block);
                get.name = xarg.to_string();
                argvarvec.push(get);
            //}
            i +=1;
        }
        let splitstruct = split(&funcname,"::");
        // check for special functions which require self. for class refs etc.
        match splitstruct[0]{
            "TRD" =>{
                let thisthread = self.executeword(splitstruct[1], &formattedblock,block);
                let get = self.executeword(&givenargs[0],&formattedblock,block);
                return self.threadsend(&thisthread.stringdata, get);
            }
            "http" =>{
                return self.httpexec(splitstruct[1], &argvarvec);
            }
            "object" =>{
                let mut retvar = NscriptVar::new("object");
                match splitstruct[1]{
                    "index" =>{
                        if let Some(class) = self.getclassref(&argvarvec[0].stringdata){
                            retvar.stringvec = class.index.to_owned().into_iter().map(|s| s.into()).collect();
                        }
                    }
                    "delete" =>{
                        self.storage.classes.remove(&argvarvec[0].stringdata);
                    }
                    "tojson" =>{
                        return self.object_to_json(&argvarvec[0].stringdata);
                    }
                    "fromjson" =>{
                        if argvarvec.len() > 1{
                             self.object_from_json(&argvarvec[0].stringdata,&argvarvec[1].stringdata);
                        }
                        retvar.stringdata = argvarvec[0].stringdata.to_string();
                    }
                    _ => {

                    }
                }
                return retvar;
            }
            "njh" => {

                let mut var = NscriptVar::new("njh");
                match splitstruct[1]{
                    "objecttofile" =>{
                        if argvarvec.len() > 1 {
                            self.njh_objecttofile(&argvarvec[0].stringdata,& argvarvec[1].stringdata);
                        }
                    }
                    "filetoobject" =>{
                        if argvarvec.len() > 1 {
                            self.njh_filetoobject(&argvarvec[0].stringdata,& argvarvec[1].stringdata);
                        }
                    }
                    "stringtoobject" =>{
                        if argvarvec.len() > 1 {
                            self.njh_stringtoobject(&argvarvec[0].stringdata,& argvarvec[1].stringdata);
                        }
                    }
                    "objecttostring" | "fromobject"=>{
                        var.stringdata = self.njh_fromobject(&argvarvec[0].stringdata).to_string();
                    }
                    "load" =>{
                        if argvarvec.len() > 1 {
                            var.stringdata = Njh::read(&argvarvec[0].stringdata,&argvarvec[1].stringdata);
                        }
                    }
                    "save" =>{
                        if argvarvec.len() > 2 {
                            Njh::write(&argvarvec[0].stringdata,&argvarvec[1].stringdata,&argvarvec[2].stringdata);
                        }
                    }
                    _ =>{
                        print("error in njh::???","r");
                    }

                }
                return var;
            }
            _ =>{ // user imports !!
                if let Some(userstruct) = self.ruststructs.get_mut(splitstruct[0]){
                    return userstruct.neocat_exec(splitstruct[1], &argvarvec);
                }
                else{
                    print(&format!("cant find userstruct::[{}]::[{}]",&splitstruct[0],&splitstruct[1]),"r");
                    return NscriptVar::new("error");
                }
            }
        }
    }


    /// used to check what a word / type is
    fn checkwordtype(&mut self,word:&str) -> NscriptWordTypes{
        match Nstring::prefix(&word){
            "\\" =>{
                return NscriptWordTypes::Classfunc;
            }
            "1" =>{
                return NscriptWordTypes::RustFunction;
            }
            "2" =>{
                return NscriptWordTypes::Function;
            }
            "3" =>{
                return NscriptWordTypes::Nestedfunc;
            }
            "$" => {
                return NscriptWordTypes::Global;
            }
            "%" =>{
                return NscriptWordTypes::Number;
            }
            "#" =>{
                return NscriptWordTypes::Array;
            }
            "&" =>{
                return NscriptWordTypes::Property;
            }
            "!" => {
                return NscriptWordTypes::Bool;
            }
            "[" =>{
                return NscriptWordTypes::Arraydeclaration;
            }
            "~" => {
                return NscriptWordTypes::Static;
            }
            "|" => {
                return NscriptWordTypes::Structfn;
            }
            "@" => {
                return NscriptWordTypes::Macro;
            }
            "*" => {
                return NscriptWordTypes::Reflection;
            }

            _ => {
                return NscriptWordTypes::Variable;
            }
        }
    }

    /// extracts the scope from the codesheet
    fn extract_scope(&mut self,filedata: &str) -> String {
        let mut stack = Vec::new();
        let mut start = None;
        let mut end = None;
        let mut depth = 0;
        for (index, ch) in filedata.char_indices() {
            match ch {
                '{' => {
                    if stack.is_empty() {
                        start = Some(index);
                    }
                    stack.push(ch);
                    depth += 1;
                }
                '}' => {
                    stack.pop();
                    depth -= 1;
                    if stack.is_empty() && depth == 0 {
                        end = Some(index + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        match (start, end) {
            (Some(start), Some(end)) => filedata[start..end].to_string(),
            _ => String::new(),
        }
    }

    /// strips off all comments per lines and trims the lines.
    fn stripcomments(&mut self, filedata: &str) -> String {
        let lines = filedata.split("\n");
        let mut newcode = String::new();
        for line in lines {
            if line != "" {
                newcode = newcode + &split(&line, "//")[0].trim() + "\n";
            }
        }
        newcode
    }
    fn prefixbooleans(&mut self,filedata:&str) -> String{
        return Nstring::replace(&Nstring::replace(&filedata,"true","!true"),"false","!false");
    }
    /// encodes the static strings, will later on be parsed per scope and set as variables.
    /// pre-formatting
    fn stringextract(&mut self,filedata : &str) -> String {
        let mut parsingtext = Nstring::replace(&filedata.to_string(), "\\\"", "#!@NSCRIPTQUOTE#@!");
        parsingtext = Nstring::replace(&parsingtext, "\"\"", "@emptystring");
        loop {
            let splitstr = Nstring::stringbetween(&parsingtext, "\"", "\"");
            if splitstr != "" {
                let packed = "^".to_owned()
                + &string_to_hex(&Nstring::replace(&splitstr, "#!@NSCRIPTQUOTE#@!", "\" "));
                let toreplace = "\"".to_owned() + &splitstr + "\"";
                parsingtext = Nstring::replace(&parsingtext, &toreplace, &packed);
            } else {
                break;
            }
        }
        parsingtext
    }
    /// sets the blocks only with the raw code, will be parsed by the thread on spawn.
    fn thread_scopeextract(&mut self,codefiledata:&str,_scriptscope: &mut NscriptScriptScope) -> String{

        let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
        let mut parsecode = codefiledata.to_string();
        let threads = split(&codefiledata, "\nthread");
        for xthread in threads {
            if i > 0 {
                if xthread != "" {
                    let namepart = split(&xthread, "{")[0];
                    let name = split(&namepart, "|");
                    let thisname = name[0].trim();
                    let subblockraw = self.extract_scope(&xthread); // extract the thread scope between { }
                    let  codeblock = NscriptCodeBlock::new(&thisname);
                    let mut formattedcodeblock = NscriptFormattedCodeBlock::new(&thisname);
                    formattedcodeblock.setcode(subblockraw.clone());
                    let entreename = "thread_".to_string() + &thisname;
                    self.storage.codeblocks.insert(entreename.to_string(), codeblock);
                    self.formattedblocks.insert(entreename.to_string(), formattedcodeblock);
                    let toreplace = "thread".to_owned() + &namepart + &subblockraw;
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        parsecode = Nstring::replace(&parsecode,&toreplace, "");
                    }
                }
            }
            i += 1;
        }
        parsecode
    }
    /// parses the code for all class scopes, will subparse for all functions inside.
    /// executes the left over block during parsing. in-order
    fn class_scopeextract(&mut self,codefiledata:&str,scriptscope: &mut NscriptScriptScope) -> String{

        let mut i = 0; //<-- serves to filter first split wich isnt if found but default.
        let mut parsecode = codefiledata.to_string();
        let classes = split(&codefiledata, "\nclass");
        for eachclass in classes {
            if i > 0 {
                if eachclass != "" {
                    let classnamepart = split(&eachclass, "{")[0];
                    let classname = split(&classnamepart, ":");
                    let thisclassname = classname[0].trim();
                    let mut thisclass: NscriptClass;
                    if let Some(_) = self.getclassref(thisclassname){
                    }else{// insert new classes
                        thisclass = NscriptClass::new(&thisclassname);
                        self.insertclass(&thisclassname, thisclass.clone());
                        thisclass.name = thisclassname.into();
                        scriptscope.classrefs.push(thisclassname.to_string());
                    }
                    let subblockraw = self.extract_scope(&eachclass); // extract the class scope between { }
                    let mut subblock = subblockraw.clone();
                    subblock = Nstring::replace(&subblock, "self.", "*self.");
                    subblock = self.func_scopeextract(&subblock, &thisclassname);
                    let mut selfvar = NscriptVar::new("self");
                    selfvar.stringdata = thisclassname.to_string();
                    let mut codeblock = NscriptCodeBlock::new(&thisclassname);
                    let mut formattedcodeblock = NscriptFormattedCodeBlock::new(&thisclassname);
                    codeblock.setvar("self", selfvar);
                    formattedcodeblock.setcode(subblock.to_string());
                    formattedcodeblock.formatblock(&mut codeblock);
                    self.preproccessblock(&mut formattedcodeblock);
                    let xblock = self.getexecutableblock(&codeblock.name);
                    self.executeblock(&mut codeblock,&xblock);
                    if classname.len() > 1{
                        let mut fromclass = self.getclass(&classname[1].trim().to_string());
                        if let Some(thisclass) = self.getclassref(&classname[0].trim()) {
                            thisclass.inherent(&mut fromclass);
                        }
                    }
                    self.storage.codeblocks.insert("class_".to_string() + &thisclassname, codeblock);
                    self.formattedblocks.insert("class_".to_string() + &thisclassname, formattedcodeblock);
                    let toreplace = "class".to_owned() + &classnamepart + &subblockraw;
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        parsecode = Nstring::replace(&parsecode,&toreplace, "");
                    }
                }
            }
            i += 1;
        }
        parsecode
    }
    /// extracts all functions from the sheet, class is empty for plain functions.
    fn func_scopeextract(&mut self,filedata: &str,onclass:&str) -> String {
        let classes = split(&filedata, "\nfunc ");
        let mut parsecode = filedata.to_string();
        let mut i = 0;
        let mut arguments : Vec<String> = Vec::new();
        for eachclass in classes {
            if i > 0 {
                if eachclass.trim() != "" && Nstring::fromleft(&eachclass.trim(), 1) != "{" {
                    let firstline = split(&eachclass, "{")[0];
                    let funcname = split(&firstline, "(")[0].trim();
                    let  block = self.extract_scope(&eachclass);
                    let cleanblock = block.clone();
                    let argumentsraw = split(&firstline, "(");
                    if argumentsraw.len() > 1 {
                        let argumentsraw = split(&argumentsraw[1], ")");
                        arguments = argumentsraw[0].split(",").map(str::to_string).collect();
                    }

                    let mut argvec : Vec<Box<str>> = Vec::new();
                    for x in arguments.clone(){
                        argvec.push(x.into())
                    }
                    let toreplace = "func ".to_owned() + &split(&eachclass, "{")[0] + &cleanblock;
                    // set the modified code
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        parsecode = parsecode.replace(toreplace.trim(), "");
                        if onclass == "" {
                            let mut thisblock = NscriptCodeBlock::new(&("".to_string()+&funcname));
                            let mut thisformattedblock = NscriptFormattedCodeBlock::new(&("".to_string()+&funcname));
                            thisformattedblock.setcode(block.to_string());
                            thisformattedblock.formatblock(&mut thisblock);
                            self.preproccessblock(&mut thisformattedblock);
                            let mut  thisfunc = NscriptFunc::new(funcname.to_string(),argvec.clone());
                            thisfunc.codeblock = thisblock;
                            //thisfunc.formattedcodeblock = thisformattedblock.clone();
                            self.userfunctions.insert(funcname.to_string(),thisfunc);
                            self.formattedblocks.insert(funcname.to_string(),thisformattedblock.clone()); // a copy not in the func maybe can go

                        } else {
                            let mut thisblock = NscriptCodeBlock::new(&(onclass.trim().to_string()+"."+&funcname.trim()));
                            let mut thisformattedblock = NscriptFormattedCodeBlock::new(&(onclass.trim().to_string()+"."+&funcname.trim()));
                            thisformattedblock.setcode(block.to_string());
                            thisformattedblock.formatblock(&mut thisblock);
                            self.preproccessblock(&mut thisformattedblock);
                            let mut varself = NscriptVar::new("self");
                            varself.setstring(&onclass);
                            thisblock.setvar("self",varself);
                            let mut thisfunc = NscriptFunc::new(funcname.to_string(),argvec.clone());
                            self.formattedblocks.insert(onclass.trim().to_string()+"."+&funcname.trim() ,thisformattedblock.clone());

                            thisfunc.executablecodeblock = self.getexecutableblock(&thisblock.name);
                            thisfunc.codeblock = thisblock;
                            //thisfunc.formattedcodeblock = thisformattedblock.clone();
                            if let Some(thisclass) = self.getclassref(onclass){
                                thisclass.setfunc(&funcname.to_string(),thisfunc);
                            }else{
                                let mut thisclass = NscriptClass::new(&onclass);
                                thisclass.setfunc(&funcname.to_string(),thisfunc);
                                self.insertclass(&onclass, thisclass);
                            }
                        }
                    }

                }
            }
            i += 1;
        }
        parsecode
    }
    /// this extracts raw scope variables, this allows a full syntax escape and multiline string
    /// set. used at the beginning of parser
    fn raw_scopeextract(&mut self,filedata: &str) -> String {
        let classes = split(&filedata, " = raw");
        let mut parsecode = filedata.to_string();
        let mut i = 0;
        for eachclass in classes {
            if i > 0 {
                if Nstring::fromleft(&eachclass.trim(), 1) == "{" {
                    let extractedscope = self.extract_scope(&eachclass);
                    let block = " = ^".to_string() + &string_to_hex(&extractedscope);
                    let toreplace = " = raw".to_owned() + &split(&eachclass, "{")[0] + &extractedscope;
                    parsecode = parsecode.replace(&toreplace, &block);
                }
            }
            i += 1;
        }
        parsecode
    }


 fn parse_and_check_statements(&mut self ,words: &Vec<Box<str>>,formattedblock: &NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) -> bool {
    // this is how you parse a unknown lenght of statements
    // they can be mixed And/or
    // this function will return a bool.
        // -------------------------------------------------------------
        let linelen = words.len();


        let mut index = 1;
        let mut result = self.check_statement(&words[1], &words[2], &words[3],&formattedblock,block);

         // if linelen < 5{
         //     return result;
         // }

        let conditions = &words[3..linelen - 2];
        while index + 4 < conditions.len() + 1 {
            let operator = conditions[index].to_owned();//.to_string();
            let a = &conditions[index + 1];
            let b = &conditions[index + 2];
            let c = &conditions[index + 3];
            if operator == "and".into() || operator == "&&".into() {
                result = result && self.check_statement(&a, &b, &c,&formattedblock,block);
            } else if operator == "or".into() || operator == "||".into() {
                result = result || self.check_statement(&a, &b, &c,&formattedblock,block);
            }  else if operator == "xor".into() {
                result = result ^ self.check_statement(&a, &b, &c,&formattedblock,block);
            }else {
                print("error operator on if statement", "r");
            }
            index += 4;
        }
        result
    }
    fn math(&mut self,a: &f64, method: &str, b: &f64) -> f64 {
        // this handles math operations from nscript. this is being looped in nscript_runmath()
        // in case of variables or calls return vallues be used.
        // ----------------------------------------------------------
        //let a_val = self.getwordstring(&a,block);
        //let b_val = self.getwordstring(&b, block);
        let mut res: f64 = 0.0;

        match method {
            "+" => {
                res = a + b;
            }
            "-" => {
                res = a - b;
            }
            "/" => {
                res = a / b;
            }
            "*" => {
                res = a * b;
            }
            _ => {
                //
            }
        };
        return res;
    }

    fn runmath(&mut self, splitline: &Vec<Box<str>>, indexpars: usize,formattedblock: &NscriptExecutableCodeBlock, block: &mut NscriptCodeBlock) -> NscriptVar {
        // this will perform a line calculation
        // indexpars = where the math begins var = x + 1 mea word[2] is the beginning
        //----------------------------------------

        let mut index = indexpars; // begin after var =
        let a =f64(&self.getwordstring(&splitline[index],&formattedblock,block));
        let b =f64(&self.getwordstring(&splitline[index+2],&formattedblock,block));
        let mut result = self.math(
            &a,
            &splitline[index + 1],
            &b

        );
        index += 2;
        while index < splitline.len() - 1 {
        let b =f64(&self.getwordstring(&splitline[index+2],&formattedblock,block));
            result = self.math(&result, &splitline[index + 1], &b);
            index += 2;
        }
        return NscriptVar::newstring("r",result.to_string());
        // var.stringdata = result.to_string();
        // return var;
    }

    fn check_statement(&mut self, a: &str, b: &str, c: &str,formattedblock: &NscriptExecutableCodeBlock,block:&mut NscriptCodeBlock) -> bool {
        // this is used to check a single statement in nscript.
        // ---------------------------------------------------------------
        match b {
            "=" => {
                if &self.getwordstring(&a,&formattedblock,block).to_lowercase() == &self.getwordstring(&c,&formattedblock,block).to_lowercase(){
                    return true;
                }
            }
            "==" => {
                if &self.getwordstring(&a,&formattedblock,block) == &self.getwordstring(&c,&formattedblock,block){
                    return true;
                }
            }
            "!=" | "<>" => {
                if &self.getwordstring(&a,&formattedblock,block) != &self.getwordstring(&c,&formattedblock,block)  {
                    return true;
                }
            }
            ">" => {
                if f64(&self.getwordstring(&a,&formattedblock,block)) > f64(&self.getwordstring(&c,&formattedblock,block)) {
                    return true;
                }
            }
            ">=" => {
                if f64(&self.getwordstring(&a,&formattedblock,block)) >= f64(&self.getwordstring(&b,&formattedblock,block)) {
                    return true;
                }
            }
            "<=" => {
                if f64(&self.getwordstring(&a,&formattedblock,block)) <= f64(&self.getwordstring(&c,&formattedblock,block)) {
                    return true;
                }
            }
            "<" => {
                if f64(&self.getwordstring(&a,&formattedblock,block)) < f64(&self.getwordstring(&c,&formattedblock,block)) {
                    return true;
                }
            }

            _ => {}
        }
        return false;
    }
    fn matchscope(&mut self,tomatch:&str,subscope:usize,formattedblock: &NscriptExecutableCodeBlock, block:&mut NscriptCodeBlock) -> NscriptVar {
        //let matchscope = formattedblock[subscope].clone();
        for lines in &formattedblock.boxedcode[subscope]{
            if lines.len() >3{
                if lines[lines.len()-2] == "SCOPE".into() {
                    for xcheck in 0..lines.len() - 3{
                        if lines[xcheck] != "|".into() && lines[xcheck] != "".into() {
                            //let thisvar = self.getwordstring(&lines[xcheck], &formattedblock,block);
                            if &self.getwordstring(&lines[xcheck], &formattedblock,block) == tomatch && &tomatch.to_string() != ""{
                                if let Some(thisvar) = self.executesubscope(&lines, &formattedblock,block){
                                    return thisvar;
                                }
                                return NscriptVar::new("match");
                            }
                            if lines[xcheck] == "_".into(){
                                if let Some(thisvar) = self.executesubscope(&lines, &formattedblock,block){
                                    return thisvar;
                                };
                                return NscriptVar::new("match");
                            }
                        }
                    }
                }
            }
        }
        return NscriptVar::new("error");
    }
}
fn f64(string:&str) ->f64{
    string.parse::<f64>().unwrap_or(0.0)
}

pub fn split<'a>(s: &'a str, p: &str) -> Vec<&'a str> {
    // returns a str array vector
    let r: Vec<&str> = s.split(p).collect();
    return r;
}

fn read_file_utf8(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };

    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents) {
        return String::new();
    }

    let (decoded, _, _) = UTF_8.decode(&contents);
    decoded.into_owned()
}

