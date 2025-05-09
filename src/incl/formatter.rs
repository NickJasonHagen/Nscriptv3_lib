
use crate::*;

impl <'a> Nscript<'a>{
    pub fn parsefile(&mut self,file:&str) -> NscriptVar{
        let filedata = "\n".to_string() + &read_file_utf8(file);
        return self.parsecode(&filedata,&file);
    }
    pub fn parsecode(&mut self,code:&str,name:&str) -> NscriptVar{
        let mut initblock = NscriptCodeBlock::new(&name);

        let filedata = self.formatcode(code, name);
        initblock.setcode(filedata);
        initblock.formatblock();
        self.preproccessblock(&mut initblock);
        let formattedblock = initblock.formattedcode.clone();
        //print(&formattedblock.code.len().to_string(),"g");
        //self.compiledblocks[0] = NscriptCompiledCodeBlock::compileblock(&initblock.codeblockvector,&initblock.subblockmap);
        if let Some(ret) = self.executescope(&formattedblock.code[0],&formattedblock,&mut initblock){
            return ret.clone();
        }else{
            return NscriptVar::new("end");
        };
    }
    pub fn parsecodewithvars(&mut self,code:&str,name:&str,vars:Vec<NscriptVar>) -> NscriptVar{
        let filedata = self.formatcode(code, name);
        let mut initblock = NscriptCodeBlock::new("start");
        initblock.setcode(filedata);
        initblock.formatblock();
        self.preproccessblock(&mut initblock);
        let formattedblock = initblock.formattedcode.clone();
        for xvar in vars{
            self.setdefiningword(&xvar.name, xvar.clone(),&formattedblock,&mut initblock);
        }
        if let Some(ret) = self.executescope(&formattedblock.code[0],&formattedblock,&mut initblock){
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
        let mut filedata = "\n".to_string() + &Nstring::replace(&code,"\"{","\" {");
        filedata = self.raw_scopeextract(&filedata);
        filedata = self.stripcomments(&filedata);
        filedata = self.prefixbooleans(&filedata);
        filedata = Nstring::replace(&filedata,"\n.", ".");// multiline chains to singleline
        filedata = self.stringextract(&filedata);// pre work creates it to hex! ^hexed,
        filedata = self.array_scopeextract(&filedata);// multiline array parse to 1 line,
        filedata = self.fixdoublespaces(&filedata);
        filedata = "\n".to_string() + &filedata;
        filedata = self.thread_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.class_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.func_scopeextract(&filedata,"");
        filedata
    }

    /// preproccessor insert word[0] on the line for the interpreter to speed things up..
    pub fn preproccessblock(&mut self,block:&mut NscriptCodeBlock){
        //let mut formattedblock = NscriptFormattedCodeBlock::new();
        //formattedblock.code.push(Vec::new());
        let mut fblock = block.formattedcode.clone();
        block.formattedcode.code[0] = self.preprocesscode(&mut fblock.code[0]);

        //formattedblock.code[0] = self.preprocesscode(&mut fblock.code[0]);
        if block.formattedcode.code.len() > 0 {
            for xid in 1..block.formattedcode.code.len(){
                let mut fblock = block.formattedcode.clone();

                //formattedblock.code.push(Vec::new());
                block.formattedcode.code[xid] = self.preprocesscode(&mut fblock.code[xid]);
                //formattedblock.code[xid] = self.preprocesscode(&mut fblock.code[xid]);
            }
            // let last = block.formattedcode.linetypes.len().clone() - 1;
            // block.formattedcode.linetypes[last].push(FormattedLineTypes::End);
            // let last = block.formattedcode.code.len().clone() - 1;
            // block.formattedcode.code[last].push(vec!("return".to_string()));
        }
        // block.formattedcode.code.push(Vec::new());
        //self.formattedblocks.insert(block.name.to_string(), formattedblock);
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
            self.executeblock(&mut thisblock);
            self.storage.codeblocks.insert(xroutine,thisblock);
        }
    }

    /// entree point for executing a new block
    pub fn executeblock(&mut self,block:&mut NscriptCodeBlock) -> NscriptVar{
        let formattedblock = block.formattedcode.clone();
        if let Some(returnvar) = self.executescope(&formattedblock.code[0],&formattedblock, block){
            if returnvar.name == "return" {
                return returnvar;
            }
        }
        let returnvar = NscriptVar::new("blockend");
        returnvar
    }

    /// recursively used to parse subscopes and jump blocks
    fn executescope(&mut self, blockvec:&Vec<Vec<String>>,formattedblock: &NscriptFormattedCodeBlock, block: &mut NscriptCodeBlock)->Option<NscriptVar>{
        for lines in 0..blockvec.len(){
            //print(&format!("block : [{}] line [{}] ",&block.name, &blockvec[lines].join(" ") ),"r");
            let result = self.executepreproccessedline(&blockvec[lines] ,&formattedblock,block);
            if result.name == "return" {
                return Some(result);
            }
        }
        return None;
    }
    /// executes if scopes which are still part of a scope so they use the local variables.
    fn executesubscope(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) -> Option<NscriptVar> {
        let toreturn:Option<NscriptVar> = None;
        //if line.len() >= 1{
        //let fromblock = block.insubblock.clone();
        block.insubblock = line[line.len()-1].parse::<usize>().unwrap_or(0);
        block.breakloop.push(false);
         if block.insubblock < block.breakloop.len() {
             block.breakloop[block.insubblock] = false;
         }
        let index = block.insubblock-1;
        //let sublen = block.subblockmap.len();
        //if formattedblock.code.len() >=1{
        if let Some(result) = self.executescope(&formattedblock.code[index],&formattedblock,block){
            if result.name == "return"{
                //block.insubblock = fromblock;
                return Some(result);
            }
        }
        //}
        // else{
        //     println!("block:[{}] doesnt meet the sublen of >=1",&block.name);
        // }
        //block.insubblock = fromblock;
        // }else{
        //     println!("cant execute subscope [{}] on line [{}]",&line[line.len()-1] , &line.join(" ")) ;
        // }
        return toreturn;
    }
    // pub fn setlinetypes(&mut self,code:&Vec<Vec<String>>) -> Vec<FormattedLineTypes>{
    //     let mut typevec = Vec::new();
    //
    //     for xline in code{
    //         match xline[0].as_str(){
    //             "RET" => typevec.push(FormattedLineTypes::Return),
    //             "return" => typevec.push(FormattedLineTypes::ReturnSome),
    //             "BRK" => typevec.push(FormattedLineTypes::Break),
    //             "exit" => typevec.push(FormattedLineTypes::Exit),
    //             "RV" => typevec.push(FormattedLineTypes::ReturnVar),
    //             "SFN" => typevec.push(FormattedLineTypes::StructFn),
    //             "RFN" => typevec.push(FormattedLineTypes::RustFn),
    //             "FN" => typevec.push(FormattedLineTypes::Fn),
    //             "CFN" => typevec.push(FormattedLineTypes::ClassFn),
    //             "chain" => typevec.push(FormattedLineTypes::Chain),
    //             "NFN" => typevec.push(FormattedLineTypes::NestedFn),
    //             "R_FN" => typevec.push(FormattedLineTypes::ReturnFn),
    //             "R_RFN" => typevec.push(FormattedLineTypes::ReturnRustFn),
    //             "R_NFN" => typevec.push(FormattedLineTypes::ReturnNestedFn),
    //             "BRKC" => typevec.push(FormattedLineTypes::BreakCo),
    //             "SCOPE" => typevec.push(FormattedLineTypes::Scope),
    //             "init" => typevec.push(FormattedLineTypes::Init),
    //             "if" => typevec.push(FormattedLineTypes::If),
    //             "elseif" => typevec.push(FormattedLineTypes::ElseIf),
    //             "else" => typevec.push(FormattedLineTypes::Else),
    //             "FRIN" => typevec.push(FormattedLineTypes::ForIn),
    //             "FRTO" => typevec.push(FormattedLineTypes::ForTo),
    //             "DBGY" => typevec.push(FormattedLineTypes::DbgY),
    //             "DBGR" => typevec.push(FormattedLineTypes::DbgR),
    //             "loop" => typevec.push(FormattedLineTypes::Loop),
    //             "coroutine" => typevec.push(FormattedLineTypes::Coroutine),
    //             "spawnthread" => typevec.push(FormattedLineTypes::SpawnThread),
    //             "SADD" => typevec.push(FormattedLineTypes::AddOne),
    //             "SSUB" => typevec.push(FormattedLineTypes::SubOne),
    //             "SSIN" => typevec.push(FormattedLineTypes::SetStringLoopIn),
    //             "SSTO" => typevec.push(FormattedLineTypes::SetStringLoopTo),
    //             "SVTO" => typevec.push(FormattedLineTypes::SetVecLoopTo),
    //             "SVIN" => typevec.push(FormattedLineTypes::SetVecLoopIn),
    //             "SCAT" => typevec.push(FormattedLineTypes::Cat),
    //             "SVCAT" => typevec.push(FormattedLineTypes::CatVec),
    //             "match" => typevec.push(FormattedLineTypes::Match),
    //             "SMATCH" => typevec.push(FormattedLineTypes::SetMatch),
    //             "M4" => typevec.push(FormattedLineTypes::Math),
    //             "SETC" => typevec.push(FormattedLineTypes::SetClass),
    //             "SBL" => typevec.push(FormattedLineTypes::SetBool),
    //             "SETVRFN" => typevec.push(FormattedLineTypes::SetLocalVarRustFn),
    //             "SETVVFN" => typevec.push(FormattedLineTypes::SetLocalVarFn),
    //             "SETVVF" => typevec.push(FormattedLineTypes::SetVarFn),
    //             "SETVCFN" => typevec.push(FormattedLineTypes::SetVarClassFn),
    //             "SETVVCFN" => typevec.push(FormattedLineTypes::SetLocalVarClassFn),
    //             "SETVNFN" => typevec.push(FormattedLineTypes::SetVarNestedFn),
    //             "SETVVNFN" => typevec.push(FormattedLineTypes::SetLocalVarNestedFn),
    //             "AA" => typevec.push(FormattedLineTypes::AddSelf),
    //             "SS" => typevec.push(FormattedLineTypes::SubSelf),
    //             "CC" => typevec.push(FormattedLineTypes::CatSelf),
    //             "SETV" => typevec.push(FormattedLineTypes::SetVar),
    //             "SETVEC" => typevec.push(FormattedLineTypes::SetVec),
    //             _ =>{}
    //         }
    //     }
    //     typevec.push(FormattedLineTypes::End);
    //     typevec
    // }
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
                                    NscriptWordTypes::Function =>{
                                        if let Some(_) = self.rustfunctions.get_mut(&split(&xline[0],"(")[0].to_string()){
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
                                        let extract =self.extract_nestedfunction(&xline[1]);
                                        let breakupword = split(&extract," ");
                                        let mut newwordvec:Vec<String> = Vec::new();
                                        newwordvec.push("NFN".to_string());
                                        for xword in breakupword.clone(){
                                            match self.checkwordtype(&xword){
                                                NscriptWordTypes::Structfn => {
                                                    newwordvec.push("SF".to_string());
                                                }
                                                NscriptWordTypes::Classfunc => {
                                                    newwordvec.push("CF".to_string());
                                                }
                                                _ => {
                                                    if let Some(_) = self.rustfunctions.get_mut(&split(&xword,"(")[0].to_string()){
                                                        newwordvec.push("RF".to_string());
                                                    }else{
                                                        newwordvec.push("F".to_string());
                                                    }
                                                }
                                            }
                                            let argsplit = split(xword,"(");
                                            newwordvec.push(argsplit[0].to_string());
                                            newwordvec.push(split(argsplit[1],")")[0].to_string());
                                            //newwordvec.push(xword.to_string());
                                        }
                                        preprocessedvec.push(newwordvec);
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
                                    NscriptWordTypes::Function =>{
                                            let getargs = Nstring::stringbetween(&xline[1], "(", ")");
                                            let givenargs = Nstring::split(&getargs,",");
                                        if let Some(_) = self.rustfunctions.get_mut(&split(&xline[1],"(")[0].to_string()){

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
                                        xline.insert(0,"+".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "--" => {
                                        xline.insert(0,"-".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "!" => {
                                        xline.insert(0,"y".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    "!!" => {
                                        xline.insert(0,"r".to_string());
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                    _ =>{}
                                }
                            }
                        }

                    }
                    _ =>{
                        match xline[0].as_str(){
                            "if" =>{
                                xline[0] = "I".to_string();
                                preprocessedvec.push(xline.to_owned());
                            }
                            "elseif"=>{
                                if xline.len() < 4 {
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
                                    xline.insert(0,"FRIN".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                else if xline[2] == "to" {
                                    xline.insert(0,"FRTO".to_string());
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
                                                    xline.insert(0,"SVTO".to_string());
                                                    preprocessedvec.push(xline.to_owned());

                                                }
                                                if xline[4] == "in" {
                                                    xline.insert(0,"SVIN".to_string());
                                                    preprocessedvec.push(xline.to_owned());

                                                }
                                            }
                                            "string" => {
                                                if xline[4] == "to" {
                                                    xline.insert(0,"SSTO".to_string());
                                                    preprocessedvec.push(xline.to_owned());
                                                }
                                                if xline[4] == "in" {
                                                    xline.insert(0,"SSIN".to_string());
                                                    preprocessedvec.push(xline.to_owned());
                                                }
                                            }
                                            "cat" => {
                                                xline.insert(0,"SCAT".to_string());
                                                preprocessedvec.push(xline.to_owned());
                                            }
                                            "cat[]" => {
                                                xline.insert(0,"SVCAT".to_string());
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
                                                        NscriptWordTypes::Function =>{
                                                            match self.checkwordtype(&xline[0]){
                                                                NscriptWordTypes::Variable =>{
                                                                    if let Some(_) = self.rustfunctions.get_mut(&split(&xline[2],"(")[0].to_string()){
                                                                        //xline.insert(0,"SETVRFN".to_string());
                                                                        //preprocessedvec.push(xline.to_owned());
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

                                                            newwordvec.push(xline[0].to_string());
                                                            let extract =self.extract_nestedfunction(&xline[2]);
                                                            let breakupword = split(&extract," ");
                                                            for xword in breakupword.clone(){
                                                                match self.checkwordtype(&xword){
                                                                    NscriptWordTypes::Structfn => {
                                                                        newwordvec.push("SF".to_string());
                                                                    }
                                                                    NscriptWordTypes::Classfunc => {
                                                                        newwordvec.push("CF".to_string());
                                                                    }
                                                                    _ => {
                                                                        if let Some(_) = self.rustfunctions.get_mut(&split(&xword,"(")[0].to_string()){
                                                                            newwordvec.push("RF".to_string());
                                                                        }else{
                                                                            newwordvec.push("F".to_string());
                                                                        }
                                                                    }
                                                                }
                                                                let argsplit = split(xword,"(");
                                                                newwordvec.push(argsplit[0].to_string());
                                                                newwordvec.push(split(argsplit[1],")")[0].to_string());
                                                            }
                                                            preprocessedvec.push(newwordvec);
                                                            //xline.insert(0,"SETVNF".to_string());
                                                            //preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Variable | NscriptWordTypes::Global | NscriptWordTypes::Property |   NscriptWordTypes::Bool | NscriptWordTypes::Static | NscriptWordTypes::Number | NscriptWordTypes::Macro | NscriptWordTypes::Array =>{
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
                                _ =>{
                                // print(&format!("unknown line? [{}]",xline.join(" ")),"r");
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

    fn executepreproccessedline(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{

        //print(&line.join(" "),"y");
        match line[0].as_str(){
            "S" =>{
                let retvar = NscriptVar::new("line");
                if let Some(ret) = self.executesubscope(&line,&formattedblock,block){
                    return ret.clone();
                };
                return retvar;
            }
            "NFN" =>{
                let mut i = 0;
                for x in (4..line.len()+3).step_by(3){
                    let varname = "nc_nfn_".to_string() + &i.to_string();
                    let var:NscriptVar;
                    match line[x-3].as_str(){
                        "SF" =>{
                            var = self.execute_preruststructfn(&line[x-2],&line[x-1], formattedblock, block);
                        }
                        "CF" =>{
                            let splitclass = split(&line[x-2],".");
                            var = self.execute_preformattedclassfunction(&splitclass[0],&splitclass[1],&line[x-1] , block);
                        }
                        "RF"  =>{
                            var = self.execute_prerustfunction(&line[x-2],&line[x-1], block);
                        }
                        "F" | _ =>{
                            var = self.execute_prencfunction(&line[x-2],&Nstring::split(&line[x-1],","), block);
                        }
                    }
                    i +=1;
                    block.setvar(&varname, var)
                }
                 //self.execute_nestedfunction(&line[1],&formattedblock, block);
            }
            "SFN" =>{
                 self.execute_ruststructfn(&line[1],&formattedblock, block) ;
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
                let  var = self.setvarfromnestedline(line, formattedblock, block);
                self.setdefiningword(&line[1], var,&formattedblock, block);
            }
            "xVNF" =>{
                let  var = self.setvarfromnestedline(line, formattedblock, block);
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
                self.execute_prerustfunction(&line[1],&line[2], block);
            }
            "M" =>{
                let tomatch = self.getwordstring(&line[1],&formattedblock, block);
                let thisvar =  self.matchscope(&tomatch,line[line.len()-1].parse::<usize>().unwrap_or(0)-1,&formattedblock, block);
                return thisvar;
            }
            "B" =>{
                let mut retvar = NscriptVar::new("line");
                retvar.name = "return".to_string();
                retvar.stringdata = "break".to_string();
                return retvar;
            }
            "RET" =>{
                let mut retvar = NscriptVar::new("line");
                retvar.name = "return".to_string();
                return retvar;
            }

            "RV" =>{
                let mut retvar = block.getvar(&line[1]);
                retvar.name = "return".to_string();
                return retvar;
            }
            "RS" =>{
                let mut retvar = NscriptVar::new("s");
                retvar.stringdata = block.staticstrings[Nstring::trimleft(&line[1],1).parse::<usize>().unwrap_or(0)].to_string();
                retvar.name = "return".to_string();
                return retvar;
            }
            "RP" =>{
                let mut retvar = self.storage.classgetprop(&line[1],&line[2], block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "return" =>{
                let mut retvar = self.executeword(&line[1], &formattedblock,block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_FN" =>{
                let mut retvar = self.execute_prencfunction(&line[3],&line, block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_RFN" =>{
                let mut retvar = self.execute_prerustfunction(&line[2],&line[2], block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_CFN" =>{
                let mut retvar = self.execute_preformattedclassfunction(&line[1],&line[2],&line[3], block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_NFN" =>{
                let mut retvar = self.execute_nestedfunction(&line[2],&formattedblock, block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "CH" =>{
                for x in 1..line.len(){
                    self.executeword(&line[x],&formattedblock, block) ;
                }
            }
            "SC" =>{
                let  retvar = NscriptVar::new("line");
                self.execute_setclassfromclass(&line[1],&line[3],&formattedblock, block);
                return retvar;
            }
            "L" =>{
                return self.execute_spawnloop(&line,&formattedblock,block);
            }
            "SVIN" =>{
                let get = self.execute_vecloopsin(&line,&formattedblock, block);
                self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "SVTO" =>{
                let get = self.execute_vecloopsto(&line,&formattedblock, block);
                self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "SSIN" =>{
                let get = self.execute_stringloopsin(&line,&formattedblock, block);
                self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "SSTO" =>{
                let get = self.execute_stringloopsto(&line,&formattedblock, block);
                self.setdefiningword(&line[1], get, &formattedblock,block);
            }
            "-" =>{
                let mut onvar = self.storage.getvar(&line[1],block);
                onvar.stringdata = (onvar.getnumber() - 1).to_string();
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }

            "+" =>{
                let mut onvar = self.storage.getvar(&line[1], block);
                onvar.stringdata = (onvar.getnumber() + 1).to_string();
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "SCAT" => {
                let mut onvar = NscriptVar::new(&line[1]);
                for xadd in 4..line.len(){
                    onvar.stringdata += &self.getwordstring(&line[xadd],&formattedblock,block);
                }
                self.setdefiningword(&line[1], onvar,&formattedblock,block);
            }
            "SVCAT" => {
                let mut onvar = NscriptVar::new(&line[1]);
                for xadd in 4..line.len(){
                    let this = self.getwordstring(&line[xadd],&formattedblock,block);
                    onvar.stringvec.push(this);
                }
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "SM" =>{
                let mut onvar: NscriptVar;
                let tomatch = self.getwordstring(&line[4], &formattedblock,block);
                onvar = self.matchscope(&tomatch,line[line.len()-1].parse::<usize>().unwrap_or(1)-1, &formattedblock,block);
                onvar.name = line[1].to_string();
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "M4" =>{
                let onvar = self.runmath(&line, 4,&formattedblock,  block);
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            // "M3" =>{
            //     let onvar = self.runmath(&line, 3,&formattedblock,  block);
            //     self.setdefiningword(&line[1], onvar,&formattedblock, block);
            // }
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
            "FRIN" =>{
                self.execute_forinloop(&line,&formattedblock,block);
            }
            "FRTO" =>{
                self.execute_fortoloop(&line,&formattedblock,block);
            }
            "CC" =>{//concate self
                let mut equalsfrom = self.storage.getvar(&line[1],block);
                for xadd in 3..line.len(){
                    equalsfrom.stringdata = equalsfrom.stringdata + &self.executeword(&(line[xadd].to_string()),&formattedblock,block).stringdata.to_string();
                }
                self.setdefiningword(&line[1], equalsfrom, &formattedblock,block);
            }
            "SS" =>{// sebtract self
                let mut onvar = self.storage.getvar(&line[1], block);
                let mut total = onvar.getnumber();
                for x in 3..line.len(){
                    total -= self.executeword(&line[x],&formattedblock, block).getnumber();
                }
                onvar.stringdata = total.to_string();
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "AA" =>{ // add se;f
                let mut onvar = self.storage.getvar(&line[1], block);
                let mut total = onvar.getnumber();
                for x in 3..line.len(){
                    total += self.executeword(&line[x], &formattedblock,block).getnumber();
                }
                onvar.stringdata = total.to_string();
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "i" => {
                let script = self.getwordstring(&line[1],&formattedblock, block);
                return self.parsefile(&script);
            }
            "y" =>{
                let retvar = self.executeword(&line[1],&formattedblock,block);
                print(&format!("block:[{}]>> {} => [{}]",&block.name,&line[1],&retvar.stringdata),"y");
                return retvar;
            }
            "r" =>{
                let  retvar = self.executeword(&line[1],&formattedblock,block);
                print(&format!("block:[{}]>> {} => [{}]",&block.name,&line[1],&retvar.stringdata),"r");
                return retvar;
            }
            "SBL" =>{// set bool
                let mut retvar = NscriptVar::new("line");
                let mut stringvec:Vec<String> = Vec::new();
                stringvec.push("if".to_string());
                for x in 3..line.len(){
                    stringvec.push(line[x].to_string());
                }
                stringvec.push("".to_string());
                stringvec.push("".to_string());
                retvar.stringdata  = self.parse_and_check_statements(&stringvec,&formattedblock, block).to_string();
                self.setdefiningword(&line[1], retvar.clone(), &formattedblock,block);
            }
            "CO"=>{
                self.execute_spawncoroutine(&line,&formattedblock,block);
            }
            "X"=>{
                //exit
            }

            _ =>{

            }
        }
        let retvar = NscriptVar::new("line");
        retvar
    }

    fn setvarfromnestedline(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock )->NscriptVar{
                    let mut i =0;
                for x in (5..line.len()).step_by(3){
                    let varname = "nc_nfn_".to_string() + &i.to_string();
                    let var:NscriptVar;
                    match line[x-3].as_str(){
                        "SF" =>{
                            var = self.execute_preruststructfn(&line[x-2],&line[x-1], formattedblock, block);
                        }
                        "CF" =>{
                            let splitclass = split(&line[x-2],".");
                            var = self.execute_preformattedclassfunction(&splitclass[0],&splitclass[1],&line[x-1], block);
                        }
                        "RF"  =>{
                            var = self.execute_prerustfunction(&line[x-2],&line[x-1], block);
                        }
                        "F" | _ =>{
                            var = self.execute_prencfunction(&line[x-2],&Nstring::split(&line[x-1],","), block);
                        }
                    }
                    i +=1;
                    block.setvar(&varname, var)
                }
                let var:NscriptVar;
                let len = line.len();
                match line[line.len()-3].as_str(){
                    "SF" =>{
                            var = self.execute_preruststructfn(&line[len-2],&line[len-1], formattedblock, block);
                    }
                    "CF" =>{
                            let splitclass = split(&line[len-2],".");
                            var = self.execute_preformattedclassfunction(&splitclass[0],&splitclass[1],&line[len-1] , block);
                    }
                    "RF" =>{
                        var = self.execute_prerustfunction(&line[len-2],&line[len-1], block);
                    }
                    "F" | _ =>{
                            var = self.execute_prencfunction(&line[len-2],&Nstring::split(&line[len-1],","), block);
                    }
                }
        var
    }
    fn execute_spawnloop(&mut self ,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
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
                        return result.clone();
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
    fn execute_spawncoroutine(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock){
        let coname = "coroutine_".to_string() + &self.getwordstring(&line[1],&formattedblock, block);
        let mut coroutineblock = NscriptCodeBlock::new(&coname);
        coroutineblock.strings = block.strings.clone();
        coroutineblock.stringsvec = block.stringsvec.clone();
        let mut selfvar = NscriptVar::new("self");
        selfvar.stringdata = coname.to_string();
        coroutineblock.setvar("self", selfvar);
        coroutineblock.staticstrings = block.staticstrings.clone();
        coroutineblock.formattedcode = block.formattedcode.clone();
        let scopeid = Nstring::usize(&line[line.len()-1]);
        coroutineblock.formattedcode.code[0] = coroutineblock.formattedcode.code[scopeid-1].clone();
        self.addcoroutine(&coname);
        self.storage.codeblocks.insert(coname,coroutineblock );
    }
    fn execute_ifline(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        let statementresult = self.parse_and_check_statements(&line, &formattedblock,block);
        if statementresult == true{
            block.ifset(statementresult);
            block.ifup();
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                block.ifdown();
                if result.name == "return" {
                    return result.clone();
                }
            }
            block.ifdown();

        }
        let result = NscriptVar::new("if");
        return result;
    }
    fn execute_elseifline(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        if block.ifscopes[block.ifscopedepth] == false{
            let statementresult = self.parse_and_check_statements(&line,&formattedblock, block);
            if statementresult{
                block.ifset(statementresult);
                block.ifup();
                if block.ifscopes[block.ifscopedepth] == false{
                    if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                        block.ifdown();
                        if result.name == "return" {
                            return result.clone();
                        }
                    }
                    block.ifdown();

                }
            }
        }
        let result = NscriptVar::new("if");
        return result;
    }
    fn execute_elseline(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        if block.ifscopes[block.ifscopedepth] == false{
            //block.ifset(false);
            //block.ifup();
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return" {
                    //block.ifdown();
                    return result.clone();
                }
            }
            //block.ifdown();

        }
        let result = NscriptVar::new("else");
        return result;
    }
    /// used for inherenting to other classes
    fn execute_setclassfromclass(&mut self,classto:&str,classfrom:&str,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock){
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
        // if let Some(class) = self.getclassref(&fromclass){
        //     class.children.push(thisclass.to_string());
        // }
        // insert adjusted parent class back
    }

    /// a for loop : something = vec x to 10{}
    fn execute_vecloopsin(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{

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
    fn execute_vecloopsto(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let mut vecvar = NscriptVar::new(&line[1]);
            let splitfrom = split(&line[4],"=");
            let mut start = 0;
            if splitfrom.len() > 1{
                start = self.executeword(&splitfrom[1],&formattedblock,block).stringdata.parse::<usize>().unwrap_or(0);
            }
            let iteratevar = self.executeword(&line[6],&formattedblock, block);
            for index in start..iteratevar.stringdata.parse::<usize>().unwrap_or(0)+1 {
                //iteratevar.stringdata = index.to_string();
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
    fn execute_stringloopsin(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let mut var = NscriptVar::new(&line[1]);
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
        var.stringdata = createstring;
        return var;
    }

    /// a for loop : something = string x to 10{}
    fn execute_stringloopsto(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{

        let mut var = NscriptVar::new(&line[1]);
        let splitfrom = split(&line[4],"=");
        let mut start = 0;
        if splitfrom.len() > 1{
            start = self.executeword(&splitfrom[1],&formattedblock,block).stringdata.parse::<usize>().unwrap_or(0);
        }
        let mut createstring = "".to_string();
        let iteratevar = self.executeword(&line[6], &formattedblock,block);
        for index in start..iteratevar.stringdata.parse::<usize>().unwrap_or(0)+1 {
            //iteratevar.stringdata = index.to_string();
            block.setstring(&splitfrom[0], index.to_string());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    createstring = createstring + &result.stringdata;
                }
            }
        }
        var.stringdata = createstring;
        return var;
    }

    fn execute_forinloop(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let arrayvar = self.executeword(&line[4],&formattedblock, block);
        let mut iteratevar = NscriptVar::new(&line[2]);
        for index in arrayvar.stringvec {
            iteratevar.stringdata = index.to_string();
            block.setvar(&line[2], iteratevar.clone());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    if result.stringdata == "break"{
                        return result.clone();
                    }
                    return result.clone();
                }
            }
        }
        return iteratevar;
    }

    fn execute_fortoloop(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let splitfrom = split(&line[2],"=");
        let mut start = 0;
        if splitfrom.len() > 1{
            start = self.executeword(&splitfrom[1],&formattedblock,block).stringdata.parse::<usize>().unwrap_or(0);
        }
        let iteratevar = self.executeword(&line[4], &formattedblock,block);
        for index in start..iteratevar.stringdata.parse::<usize>().unwrap_or(0)+1 {
            block.setstring(&splitfrom[0],index.to_string());
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return"{
                    if result.stringdata == "break"{
                        return result.clone();
                    }
                }
            }
        }

        return iteratevar;
    }
     fn execute_spawnthread(&mut self,line:&Vec<String>,formattedblock:&NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) -> NscriptVar{
        let fromthreadname = "thread_".to_string()+&line[line.len()-1];
        let threadname = "thread_".to_string()+&self.executeword(&line[1],&formattedblock, block).stringdata;
        let mut threadcode = self.getblock(&fromthreadname);
        let mut selfvar = NscriptVar::new("self");
        selfvar.stringdata = threadname.to_string();
        let rawcode =threadcode.formattedcode.codeblock.to_string();
        threadcode.formattedcode.codeblock = "\n".to_string() + &threadcode.formattedcode.codeblock;
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
                    let ncfunc = "threadreceive($received)";
                    let ncreturn = threadstruct.execute_function(&ncfunc,&mut threadcode);
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
    fn setdefiningword(&mut self,word:&str,equalsfrom: NscriptVar,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock){
        match self.checkdefiningwordtype(&word){
            NscriptWordTypes::Global => {
                self.storage.setglobal(&word, equalsfrom);
            }
            NscriptWordTypes::Property => {
                let splitword = split(&word,".");
                if splitword.len() > 1{
                    let mut classname = splitword[0].to_string();
                    let trimmedname = Nstring::trimleft(&splitword[0],1);
                    if Nstring::fromleft(&classname, 1) == "*" {
                        classname = self.storage.getargstring(&trimmedname,block);
                    }
                    let mut propname = splitword[1].to_string();
                    let trimmedprop = Nstring::trimleft(&splitword[1],1);
                    if  Nstring::fromleft(&propname, 1)  == "*" {
                        propname = self.storage.getargstring(&trimmedprop,block);
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
            NscriptWordTypes::Variable => {
                block.setvar(&word, equalsfrom);
            }
            NscriptWordTypes::Array =>{
                let wordsplit = split(&word,"[");
                let mut var = self.storage.getvar(&wordsplit[0],block);
                let idvar = self.storage.getargstring(&Nstring::trimright(&wordsplit[1],1),block).parse::<usize>().unwrap_or(0);
                if idvar < var.stringvec.len(){
                    var.stringvec[idvar] = equalsfrom.stringdata;
                }
                else {
                    print(&format!("block [{}] array [{}] tries to set a index but its out of bounds",&block.name,&wordsplit[0]),"r");
                }
                self.setdefiningword(wordsplit[0], var,&formattedblock, block);

            }
            _ =>{

            }
        };

    }

    fn extract_nestedfunction(&mut self,word:&str) -> String{
        let mut outputline = "".to_string();
        let mut resultstring = word.to_string();
        let mut packed: String;
        let mut subfunction: String;
        let mut i = 0;
        loop {
            // get the last find in the string using (
            let splitstr = split(&resultstring, "(");
            // make sure its inside the main function so bigger>2
            if splitstr.len() > 2 {
                //take that substring and split up to the first )
                let splitscope = split(&splitstr[splitstr.len() - 1], ")");
                if splitscope.len() > 0 {
                    let splitargus = split(&splitstr[splitstr.len() - 2], ",");
                    let thisfnnamefix = splitargus[splitargus.len() - 1]; // make sure the function
                    subfunction = "".to_owned() + &thisfnnamefix + "(" + &splitscope[0] + ")";
                    packed = "nc_nfn_".to_string() + &i.to_string();
                    i +=1;
                    resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                    outputline = outputline + &subfunction + " ";
                }
            } else {
                break;
            }
        }
        outputline = outputline + &resultstring;
        outputline

    }
    fn execute_nestedfunction(&mut self,word:&str,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{

        let mut resultstring = word.to_string();
        let mut packed: String;
        let mut subfunction: String;
        let mut varcounter = 0;
        loop {
            varcounter +=1;
            // get the last find in the string using (
            let splitstr = split(&resultstring, "(");
            // make sure its inside the main function so bigger>2
            if splitstr.len() > 2 {
                //take that substring and split up to the first )
                let splitscope = split(&splitstr[splitstr.len() - 1], ")");
                if splitscope.len() > 0 {
                    // important one, if a variable or string is infron it
                    // messes up the syntax so we split using comma
                    let splitargus = split(&splitstr[splitstr.len() - 2], ",");
                    // here we set thisfnname to the last part of the comma split
                    let thisfnnamefix = splitargus[splitargus.len() - 1]; // make sure the function
                    // here we check if the function given is reflected if so we evaluate the value of
                    // the var and executre the function of the data from that var as a string
                    if Nstring::fromleft(&splitstr[splitstr.len() - 2], 1) == "*" {
                        let splitdot = split(&thisfnnamefix,".");
                        if splitdot.len() > 1 {

                            let mut part1 = splitdot[0].to_string();
                            let mut part2 = splitdot[1].to_string();
                            if Nstring::fromleft(&splitdot[0], 1) == "*"{
                                part1 = self.storage.getargstring(&Nstring::trimleft(&splitdot[0], 1), block);
                            }

                            if Nstring::fromleft(&splitdot[1], 1) == "*"{
                                part2 = self.storage.getargstring(&Nstring::trimleft(&splitdot[1], 1), block);
                            }
                            let thisfnnamefix2 = part1.to_string() + "." + &part2;
                            //print(&thisfnnamefix2,"r");
                            subfunction = "".to_owned() + &thisfnnamefix2
                                + "(" + &splitscope[0]+ ")";

                        }else{
                            subfunction = "".to_owned()
                                + &self.executeword(&Nstring::replace(&thisfnnamefix, "*", ""),&formattedblock,block).stringdata
                                + "(" + &splitscope[0]+ ")";

                        }
                    } else {
                        // if its a normal funcion we run it.
                        subfunction = "".to_owned() + &thisfnnamefix + "(" + &splitscope[0] + ")";
                    }
                    let varname = "nestedfn_".to_string() + &varcounter.to_string();
                    let mut tmpvar = self.executeword(&subfunction,&formattedblock, block);
                    tmpvar.name = varname.to_string();
                    packed = tmpvar.name.to_string();
                    block.setvar(&varname, tmpvar);
                    // here we evaluate the none function types.
                } else {
                    // this also evaluates variables macros strings etc
                    subfunction = "".to_owned() + &splitscope[0]; //&splitstr[splitstr.len()-1];
                    let varname = "nestedfn_".to_string() + &varcounter.to_string();
                    let mut tmpvar = self.storage.getvar(&subfunction, block);
                    tmpvar.name = varname.to_string();
                    packed = tmpvar.name.to_string();
                    block.setvar(&varname, tmpvar);
                }
                let mut reflect = false;
                if splitscope.len() > 0 {
                    // so this replaces the evaluated values in the word's() when
                    // its all done it will return 1 function to parseline() wich be used to set the
                    // variable
                    if Nstring::fromleft(&splitstr[splitstr.len() - 2], 1) == "*" {
                        subfunction = "".to_owned() + &splitstr[splitstr.len() - 2] + "(" + &splitscope[0] + ")";
                        resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                        reflect = true
                    }
                }
                if reflect == false {
                    // very important! this reforms the strings till its made back to 1 function with
                    // all evaluated data types. when this is done theres no double (( )) insde the
                    // code and this function will exit and return the 1-function to parse_line()
                    resultstring = Nstring::replace(&resultstring, &subfunction, &packed);
                }
            } else {
                break;
            }
        }
        return self.executeword(&resultstring,&formattedblock, block);

    }
    pub fn getwordstring(&mut self,word:&str,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) -> String{

         match self.checkwordtype(word){
            NscriptWordTypes::Static =>{
                return block.staticstrings[Nstring::trimleft(word, 1).parse::<usize>().unwrap_or(0)].to_string();
            }
            NscriptWordTypes::Macro =>{
                return self.storage.getmacrostring(word);
            }
            NscriptWordTypes::Global => {
                return self.storage.getglobal(&word).stringdata;
            }
            NscriptWordTypes::Variable=>{
                return block.getstring(word);
            }
            NscriptWordTypes::Property=>{
                let wordsplit = split(&word,".");
                if wordsplit.len() > 1{
                    let mut cname = wordsplit[0].trim().to_string();
                    let mut pname = wordsplit[1].trim().to_string();
                    if Nstring::fromleft(&wordsplit[0], 1) ==  "*" {
                        cname = self.getwordstring(&Nstring::trimleft(&wordsplit[0],1),&formattedblock, block);
                    }
                    if Nstring::fromleft(&cname, 1) ==  "$" {
                        cname = self.getwordstring(&wordsplit[0],&formattedblock, block);
                    }
                    if Nstring::fromleft(&wordsplit[1], 1) ==  "*" {
                        pname = self.getwordstring(&Nstring::trimleft(&wordsplit[1], 1),&formattedblock,block) ;
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname).stringdata;
                    }else{
                        print(&format!("block[{}] word is a prop but theres no class on cname [{}] pname[{}]",&block.name,&cname,&pname),"r");
                        return "".to_owned();
                    }
                }
            }
            NscriptWordTypes::Number  =>{
                return word.to_owned();
            }
            NscriptWordTypes::Bool => {
                return Nstring::trimleft(&word,1);
            }
            NscriptWordTypes::Function => {
                return self.execute_function(word, block).stringdata;
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimleft(word, 1);
                let evaluated = self.storage.getargstring(&toreflect, block);
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let arrays = split(word,"[");
                    let thisvar = self.storage.getvar(arrays[0], block);
                    let index = self.getwordstring(&split(&arrays[1], "]")[0],&formattedblock,block).parse::<usize>().unwrap_or(0);
                    if thisvar.stringvec.len() > index{
                        return thisvar.stringvec[index].to_string();
                    }else{
                    print(&format!("block[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&block.name,&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return "".to_owned();
            }
            NscriptWordTypes::Classfunc => {
                return self.execute_classfunction(word, block).stringdata;
            }
            NscriptWordTypes::Structfn => {
                return self.execute_ruststructfn(word,&formattedblock, block).stringdata;
            }
            _ => {
                return self.evaluateword(word,&NscriptWordTypes::Variable,&formattedblock, block).stringdata;
            }
        };

        return "".to_owned();
    }
    pub fn evaluateword(&mut self,word:&str,vartype:&NscriptWordTypes,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock)-> NscriptVar{
       match vartype{
            NscriptWordTypes::Static =>{
                let mut var = NscriptVar::new("static");
                var.stringdata = block.staticstrings[Nstring::trimleft(word, 1).parse::<usize>().unwrap_or(0)].to_string();
                return var;
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
            NscriptWordTypes::AssignedFunc =>{

                //print(&word,"br");
                let toeval =block.getvar(&Nstring::trimleft(&word,1)).stringdata;
                //print(&toeval,"br");
                return self.storage.getvar(&toeval, block);
            }
            NscriptWordTypes::Macro =>{

                let mut var = NscriptVar::new("macro");
                var.stringdata =self.storage.getmacrostring(word);
                return var;
            }
            NscriptWordTypes::Global => {
                return self.storage.getglobal(&word).clone();
            }
            NscriptWordTypes::Variable=>{
                return block.getvar(word).clone();
            }
            NscriptWordTypes::Property=>{
                let wordsplit = split(&word,".");
                if wordsplit.len() > 1{
                    let mut cname = wordsplit[0].trim().to_string();
                    let mut pname = wordsplit[1].trim().to_string();
                    if Nstring::fromleft(&wordsplit[0], 1) ==  "*" {
                        cname = self.storage.getargstring(&Nstring::trimleft(&wordsplit[0],1), block);
                    }
                    if Nstring::fromleft(&wordsplit[1], 1) ==  "*" {
                        pname = self.storage.getargstring(&Nstring::trimleft(&wordsplit[1], 1),block) ;
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname);
                    }else{
                        print(&format!("block[{}] word is a prop but theres no class on cname [{}] pname[{}]",&block.name,&cname,&pname),"r");
                    }
                }
            }
            NscriptWordTypes::Number  =>{
                let mut newvar = NscriptVar::new(word);
                newvar.setstring(&word);
                return newvar;
            }
            NscriptWordTypes::Bool => {
                let mut newvar = NscriptVar::new(word);
                 newvar.setstring(&Nstring::trimleft(&word,1));
                return newvar;
            }
            NscriptWordTypes::Function => {
                return self.execute_function(word, block);
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimleft(word, 1);
                let evaluated = self.executeword(&toreflect,&formattedblock, block);
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let mut returnvar = NscriptVar::new("entree");
                let arrays = split(word,"[");
                let thisvar = self.executeword(arrays[0],&formattedblock, block);
                let index = self.storage.getargstring(&split(&arrays[1], "]")[0],block).parse::<usize>().unwrap_or(0);
                if thisvar.stringvec.len() > index{
                    returnvar.stringdata = thisvar.stringvec[index].to_string();
                }else{
                    print(&format!("block:[{}] array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&block.name,&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return returnvar;
            }
            NscriptWordTypes::Arraydeclaration => {
                let mut thisarrayvar = NscriptVar::new("array");
                let between = Nstring::trimright(&Nstring::trimleft(&word,1),1);
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
    pub fn executeword(&mut self,word:&str,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) -> NscriptVar {

        let prefix = self.checkwordtype(word);
        return self.evaluateword(word, &prefix,&formattedblock, block)
    }
    pub fn execute_rustfunction(&mut self,word:&str,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitfunc = split(&word,"(");
        let givenargs = Nstring::split(&Nstring::stringbetween(&word, "(", ")"),",");
        let  funcname = splitfunc[0];
        if let Some(rustfn) = self.rustfunctions.get(&funcname.to_string()){
            return rustfn(&givenargs,block,&mut self.storage);
        }
        NscriptVar::new("shouldnothappen")
    }
    /// for preproccesed lines (optimizing)
    pub fn execute_prerustfunction(&mut self,funcname:&str,getargs:&str, block:&mut NscriptCodeBlock) ->NscriptVar{
        let givenargs = Nstring::split(&getargs,",");
        if let Some(rustfn) = self.rustfunctions.get(&funcname.to_string()){
            return rustfn(&givenargs,block,&mut self.storage);
        }
        NscriptVar::new("shouldnothappen")
    }
    /// executes a nsript function
    pub fn execute_ncfunction(&mut self,word:&str,block:&mut NscriptCodeBlock) ->NscriptVar{

        let splitfunc = split(&word,"(");
        let givenargs = Nstring::split(&Nstring::stringbetween(&word, "(", ")"),",");
        let  funcname = splitfunc[0];
        if let Some(func) = self.storage.functions.get_mut(&funcname.to_string()){
            let mut getblock = func.codeblock.clone();
            let args = func.args.clone();
            let len = givenargs.len();
            for xarg in 0..func.args.len(){
                if len > xarg{
                    let get = self.storage.getvar(&givenargs[xarg],block);
                    getblock.setvar(&args[xarg],get);
                }
            }

            let formattedblockfunc = getblock.formattedcode.clone();
            //let formattedblockfunc = self.getformattedblock(&getblock.name);

            if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar.to_owned();
                }
            };
        }else{
            print(&format!("no ncfunctions found for [{}]",&funcname),"r");
        }

        return NscriptVar::new("ncfunc");
    }

    /// preproccessed lines use this one to skip some checks and gain performance
    pub fn execute_prencfunction(&mut self,funcname:&str, line:&Vec<String>, block:&mut NscriptCodeBlock) ->NscriptVar{
        let mut i = 0;
        if let Some(func) = self.storage.functions.get_mut(&funcname.to_string()){
            let mut getblock = func.codeblock.clone();
            let  args = func.args.clone();
            let len = args.len();
            for xarg in 4..line.len(){
                if i <= len{
                    let get = self.storage.getvar(&line[xarg],block);
                    getblock.setvar(&args[i],get);
                }
                i +=1;
            }

            //let formattedblockfunc = self.getformattedblock(&getblock.name);
            let formattedblockfunc = getblock.formattedcode.to_owned();
            if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar.clone();
                }
            };
        }else{
            print(&format!("no prencfunctions found for [{}]",&funcname),"r");
        }

        return NscriptVar::new("ncfunc");
    }
    /// used in nests, as it doesnt know wheter its a udf or a built in fn
    /// this can parse both
    fn execute_function(&mut self,word:&str, block:&mut NscriptCodeBlock) ->NscriptVar{
        //let getargs = Nstring::stringbetween(&word, "(", ")");
        let givenargs = Nstring::split(&Nstring::stringbetween(&word, "(", ")"),",");
        let splitfunc = split(&word,"(");
        let mut funcname = splitfunc[0].to_string();
        if let Some(rustfn) = self.rustfunctions.get(&funcname){
            return rustfn(&givenargs,block,&mut self.storage);
        }
        if Nstring::fromleft(&splitfunc[0],1) == "*"{
            funcname = self.storage.getargstring(&Nstring::trimleft(&splitfunc[0],1), block);
        }

        if let Some(func) = self.storage.functions.get(&funcname){
            let mut getblock = func.codeblock.to_owned();
            let  args = func.args.to_owned();
            let len = givenargs.len();
            for xarg in 0..args.len(){
                if len > xarg{
                    let get = self.storage.getvar(&givenargs[xarg],block);
                    getblock.setvar(&args[xarg],get);
                }
            }
            let formattedblockfunc = getblock.formattedcode.clone();

            //let formattedblockfunc = self.getformattedblock(&getblock.name);

            if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar.clone();
                }
            };

        }else{
            print(&format!("no functions found for [{}] in block: [{}]",&funcname,&block.name),"r");
        }

        return NscriptVar::new("func");
    }
    fn execute_classfunction(&mut self,word:&str,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitfunc = split(&split(&word,"(")[0],".");
        let getargs = Nstring::stringbetween(&word, "(", ")");
        self.execute_preformattedclassfunction(&splitfunc[0], &splitfunc[1], &getargs, block)
    }
    fn execute_preformattedclassfunction(&mut self,class:&str , func:&str, givenargs:&str ,block:&mut NscriptCodeBlock) ->NscriptVar{

        //let args:Vec<String>;
        let mut getblock: NscriptCodeBlock;
        let givenargs = split(&givenargs,",");
        let mut classname = class.to_string();
        if Nstring::fromleft(&classname,1) == "*"{
            classname = self.storage.getargstring(&Nstring::trimleft(&classname,1), block);
        }
        let mut funcname = func.to_string();
        if Nstring::fromleft(&func,1) == "*"{
            funcname = self.storage.getargstring(&Nstring::trimleft(&func,1), block);
        }

        if let Some(class) = self.getclassref(&classname){
            let thisfunc = class.getfunc(&funcname);
            //args = thisfunc.args.clone();
            getblock = thisfunc.codeblock.clone();
            for xarg in 0..thisfunc.args.len(){
                if givenargs.len() > xarg{
                    let get = self.storage.getvar(&givenargs[xarg],block);
                    getblock.setvar(&thisfunc.args[xarg],get);
                }
            }

            let formattedblockfunc = getblock.formattedcode.clone();
            //let formattedblockfunc = self.getformattedblock(&getblock.name);
            if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar.clone();
                }
            };
            return NscriptVar::new("func");
        }else{
            print(&format!("cant find classfn [{}].[{}]",&funcname,&classname),"r");
            return NscriptVar::new("error");
        }
    }

    fn execute_ruststructfn(&mut self,word:&str,formattedblock:&NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitfunc = split(&word,"(")[0];
        let getargs = Nstring::stringbetween(&word, "(", ")");
        return self.execute_preruststructfn(&splitfunc, &getargs,&formattedblock, block);
    }
    /// this executes rust structs which the user injected, their only on the main thread
    /// some built ins like object and njh are defined here
   fn execute_preruststructfn(&mut self, funcname:&str,getargs:&str, formattedblock:&NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{

        let givenargs = split(&getargs,",");
        let mut i = 0;
        let mut argvarvec :Vec<NscriptVar> = Vec::new();
        for xarg in &givenargs{
            if givenargs.len() > i{
                let mut get = self.storage.getvar(&givenargs[i], block);
                get.name = xarg.to_string();
                argvarvec.push(get);
            }
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
                            retvar.stringvec = class.index.clone();
                        }
                    }
                    "delete" =>{
                        self.storage.classes.remove(&argvarvec[0].stringdata);
                    }
                    "children" =>{
                        if let Some(class) = self.getclassref(&argvarvec[0].stringdata){
                            retvar.stringvec = class.children.clone();
                        }
                    }
                    "parents" =>{
                        if let Some(class) = self.getclassref(&argvarvec[0].stringdata){
                            retvar.stringvec = class.parents.clone();
                        }
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

    fn checkdefiningwordtype(&mut self,word:&str) -> NscriptWordTypes{
        if Nstring::instring(&word, ".") && Nstring::fromright(&word,1) != ")"{
            return NscriptWordTypes::Property;//"property".to_string();
        }
        if Nstring::fromleft(word, 1) != "["  &&  Nstring::fromright(&word,1) == "]" {
            return NscriptWordTypes::Array;//"array".to_string();
        }
        match Nstring::fromleft(word, 1).as_str(){
            "$" => {
                return NscriptWordTypes::Global;
            }

            "*" => {
                return NscriptWordTypes::Reflection;
            }
            _ => {
                return NscriptWordTypes::Variable;
            }
        }
    }
    /// used to check what a word / type is
    fn checkwordtype(&mut self,word:&str) -> NscriptWordTypes{

        if Nstring::fromright(&word,1) == ")" {
            let splitsubs = split(&word,"(");
            if splitsubs.len()>2{
                return NscriptWordTypes::Nestedfunc;
            }
            if Nstring::instring(&word, ").") {
                return NscriptWordTypes::Classfunc;
            }

            if Nstring::instring(&word, "::") {
                return NscriptWordTypes::Structfn;
            }
            if Nstring::instring(splitsubs[0], ".") {
                return NscriptWordTypes::Classfunc;
            }
            return NscriptWordTypes::Function;
        }
        if word.parse::<f64>().is_ok(){
            return NscriptWordTypes::Number;
        }
        if Nstring::instring(&word, ".") {
            return NscriptWordTypes::Property;
        }
        if Nstring::fromleft(word, 1) != "["  &&  Nstring::fromright(&word,1) == "]" {
            return NscriptWordTypes::Array;
        }
        let prefix = Nstring::fromleft(word, 1);
        match prefix.as_str(){
            "$" => {
                return NscriptWordTypes::Global;
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
            "@" => {
                return NscriptWordTypes::Macro;
            }
            "*" => {
                return NscriptWordTypes::Reflection;
            }
            "?" => {
                return NscriptWordTypes::AssignedFunc;
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
        // let toreplace =["true)","false)","true,","false,"," true"," false","\ntrue","\nfalse","= false","= true","!!true","!!false"];
        // let with =    ["!true)","!false)","!true,","!false,"," !true"," !false","\n!true","\n!false","= !false","= !true","!true","!false"];
        // let mut fdata = filedata.to_string();
        // for x in 0..toreplace.len(){
        //     fdata = Nstring::replace(&fdata, toreplace[x], with[x]);
        // }
        // fdata
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
                    let mut  codeblock = NscriptCodeBlock::new(&thisname);
                    codeblock.setcode(subblockraw.clone());
                    let entreename = "thread_".to_string() + &thisname;
                    self.storage.codeblocks.insert(entreename.to_string(), codeblock);
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
                        thisclass.name = thisclassname.to_string();
                        scriptscope.classrefs.push(thisclassname.to_string());
                    }
                    let subblockraw = self.extract_scope(&eachclass); // extract the class scope between { }
                    let mut subblock = subblockraw.clone();
                    subblock = Nstring::replace(&subblock, "self.", "*self.");
                    subblock = self.func_scopeextract(&subblock, &thisclassname);
                    let mut selfvar = NscriptVar::new("self");
                    selfvar.stringdata = thisclassname.to_string();
                    let mut codeblock = NscriptCodeBlock::new(&thisclassname);
                    codeblock.setvar("self", selfvar);
                    codeblock.setcode(subblock.to_string());
                    codeblock.formatblock();
                    self.preproccessblock(&mut codeblock);
                    self.executeblock(&mut codeblock);
                    if classname.len() > 1{
                        let mut fromclass = self.getclass(&classname[1].trim().to_string());
                        if let Some(thisclass) = self.getclassref(&classname[0].trim()) {
                            thisclass.inherent(&mut fromclass);
                        }
                    }
                    self.storage.codeblocks.insert("class_".to_string() + &thisclassname, codeblock);
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
                    let toreplace = "func ".to_owned() + &split(&eachclass, "{")[0] + &cleanblock;
                    // set the modified code
                    if Nstring::instring(&toreplace, "{") && Nstring::instring(&toreplace, "}") {
                        parsecode = parsecode.replace(toreplace.trim(), "");
                        if onclass == "" {
                            let mut thisblock = NscriptCodeBlock::new(&("".to_string()+&funcname));
                            thisblock.setcode(block.to_string());
                            thisblock.formatblock();
                            self.preproccessblock(&mut thisblock);
                            let mut  thisfunc = NscriptFunc::new(funcname.to_string(),arguments.clone());
                            thisfunc.codeblock = thisblock;
                            self.storage.functions.insert(funcname.to_string(),thisfunc);

                        } else {
                            let mut thisblock = NscriptCodeBlock::new(&(onclass.trim().to_string()+"."+&funcname.trim()));
                            thisblock.setcode(block.to_string());
                            thisblock.formatblock();
                            self.preproccessblock(&mut thisblock);
                            let mut varself = NscriptVar::new("self");
                            varself.setstring(&onclass);
                            thisblock.setvar("self",varself);
                            let mut thisfunc = NscriptFunc::new(funcname.to_string(),arguments.clone());
                            thisfunc.codeblock = thisblock;
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


 fn parse_and_check_statements(&mut self ,words: &Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) -> bool {
    // this is how you parse a unknown lenght of statements
    // they can be mixed And/or
    // this function will return a bool.
        // -------------------------------------------------------------
        let linelen = words.len();


        let mut index = 1;
        let mut result = self.check_statement(&words[1], &words[2], &words[3],&formattedblock,block);

         if linelen < 5{
             return result;
         }

        let conditions = &words[3..linelen - 2];
        while index + 4 < conditions.len() + 1 {
            let operator = conditions[index].as_str();
            let a = conditions[index + 1].as_str();
            let b = conditions[index + 2].as_str();
            let c = conditions[index + 3].as_str();
            if operator == "and" || operator == "&&" {
                result = result && self.check_statement(&a, &b, &c,&formattedblock,block);
            } else if operator == "or" || operator == "||" {
                result = result || self.check_statement(&a, &b, &c,&formattedblock,block);
            } else {
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

    fn runmath(&mut self, splitline: &Vec<String>, indexpars: usize,formattedblock: &NscriptFormattedCodeBlock, block: &mut NscriptCodeBlock) -> NscriptVar {
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
        let mut var = NscriptVar::new("result");
        var.stringdata = result.to_string();
        return var;
    }
    fn check_statement(&mut self, a: &str, b: &str, c: &str,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) -> bool {
        // this is used to check a single statement in nscript.
        // ---------------------------------------------------------------
        match b {
            "=" | "==" => {
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
    fn matchscope(&mut self,tomatch:&str,subscope:usize,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) -> NscriptVar {
        //let matchscope = formattedblock[subscope].clone();
        for lines in &formattedblock.code[subscope]{
            if lines.len() >3{
                if lines[lines.len()-2] == "SCOPE" {
                    for xcheck in 0..lines.len() - 3{
                        if &lines[xcheck] != "|" && &lines[xcheck] != "" {
                            let thisvar = self.executeword(&lines[xcheck], &formattedblock,block);
                            if &thisvar.stringdata == tomatch && &tomatch.to_string() != ""{
                                if let Some(thisvar) = self.executesubscope(&lines, &formattedblock,block){
                                    return thisvar.clone();
                                }
                                return NscriptVar::new("match");
                            }
                            if &lines[xcheck] == "_" {
                                if let Some(thisvar) = self.executesubscope(&lines, &formattedblock,block){
                                    return thisvar.clone();
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

