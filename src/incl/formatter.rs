
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


    fn formatcode(&mut self,code:&str,name:&str)-> String{
        let mut thiscodescope = NscriptScriptScope::new(name.to_string());
        //let mut thiscodeblock = NscriptCodeBlock::new(name);
        let mut filedata = "\n".to_string() + &Nstring::replace(&code,"\"{","\" {");
        filedata = self.stripcomments(&filedata);
        filedata = self.stringextract(&filedata);// pre work creates it to hex! ^hexed,
        filedata = self.fixdoublespaces(&filedata);
        filedata = "\n".to_string() + &filedata;
        filedata = self.thread_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.class_scopeextract(&filedata,&mut thiscodescope);
        filedata = self.func_scopeextract(&filedata,"");
        filedata
    }
    pub fn preproccessblock(&mut self,block:&mut NscriptCodeBlock){
        block.formattedcode.code[0] = self.preprocesscode(&mut block.formattedcode.code[0]);
        if block.formattedcode.code.len() > 0 {
            for xid in 1..block.formattedcode.code.len(){
                block.formattedcode.code[xid] = self.preprocesscode(&mut block.formattedcode.code[xid]);
            }
        }
        self.formattedblocks.insert(block.name.to_string(), block.formattedcode.clone());

    }
    /// preproccessor insert word[0] on the line for the interpreter to speed things up..

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
    pub fn executecoroutines(&mut self){
        for xroutine in self.coroutines.clone(){
            let  mut thisblock = self.getblock(&xroutine);
            self.executeblock(&mut thisblock);
            self.storage.codeblocks.insert(xroutine,thisblock);
        }
    }
    /// entree point for executing a new block
    pub fn executeblock(&mut self,block:&mut NscriptCodeBlock) -> NscriptVar{
        //let mut blockref = blocki.codeblock;
        let formattedblock = block.formattedcode.clone();
        if let Some(returnvar) = self.executescope(&formattedblock.code[0],&formattedblock, block){
                if returnvar.name == "return" {
                    print(&format!("final execblock: returing [{}]",&returnvar.name),"g");
                    return returnvar;
                }
        }
        let returnvar = NscriptVar::new("blockend");
        returnvar
    }

    /// recursively used to parse subscopes and jump blocks
    fn executescope(&mut self, blockvec:&Vec<Vec<String>>,formattedblock: &NscriptFormattedCodeBlock, block: &mut NscriptCodeBlock)->Option<NscriptVar>{
        for lines in blockvec{
            let result = self.executepreproccessedline(&lines, &formattedblock,block);
            if result.name == "return" {
                return Some(result);
            }
        }
        return None;
    }
    fn executesubscope(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) -> Option<NscriptVar> {
        let toreturn:Option<NscriptVar> = None;
        if line.len() >= 1{
            let fromblock = block.insubblock.clone();
            block.insubblock = line[line.len()-1].parse::<usize>().unwrap_or(0);
            block.breakloop.push(false);
            if block.insubblock < block.breakloop.len() {
                block.breakloop[block.insubblock] = false;
            }
            let index = block.insubblock-1;
            //let sublen = block.subblockmap.len();
            if formattedblock.code.len() >=1{
                if let Some(result) = self.executescope(&formattedblock.code[index],&formattedblock,block){
                    if result.name == "return"{
                        block.insubblock = fromblock;
                        return Some(result);
                    }
                }
            }
            else{
                println!("block:[{}] doesnt meet the sublen of >=1",&block.name);
            }
            block.insubblock = fromblock;
        }else{
            println!("cant execute subscope [{}] on line [{}]",&line[line.len()-1] , &line.join(" ")) ;
        }
        return toreturn;
    }
    /// inserts a keystring infront of the lines to speed up the runtime.
    /// this ensures that word[0] will be the right instruction
    pub fn preprocesscode(&mut self, code:&mut Vec<Vec<String>>) -> Vec<Vec<String>>{
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
                            xline.insert(0,"BRK".to_string());
                            preprocessedvec.push(xline.to_owned());
                        }
                        "exit" => {
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
                                            //xline.insert(0,"FN".to_string());
                                            //preprocessedvec.push(xline.to_owned());

                                            let getargs = Nstring::stringbetween(&xline[0], "(", ")");
                                            let givenargs = Nstring::split(&getargs,",");
                                            let mut newline: Vec<String>  = Vec::new();
                                            newline.push("FN".to_string());
                                            newline.push(xline[0].to_string());
                                            newline.push(" ".to_string());
                                            //newline.push(" ".to_string());
                                            newline.push(split(&xline[0],"(")[0].to_string());
                                            for xword in givenargs{
                                                newline.push(xword.to_string());

                                            }

                                            preprocessedvec.push(newline.to_owned());
                                        }
                                }
                                NscriptWordTypes::Classfunc =>{
                                    xline.insert(0,"CFN".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                NscriptWordTypes::Nestedfunc =>{
                                    xline.insert(0,"NFN".to_string());
                                    preprocessedvec.push(xline.to_owned());
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
                                            //let givenargs = Nstring::split(&getargs,",");
                                            newline.push("R_RFN".to_string());
                                            newline.push(xline[0].to_string());
                                            newline.push(getargs);
                                            preprocessedvec.push(xline.to_owned());
                                        }
                                        else{
                                            //xline.insert(0,"R_FN".to_string());
                                            //xline.insert(0,"SETVVF".to_string());
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
                                    _ =>{
                                        preprocessedvec.push(xline.to_owned());
                                    }
                                }
                            //xline.insert(0,"RETV".to_string());
                            //print(&xline.join(" "),"bb");
                        }
                        "break" => {
                            xline.insert(0,"BRKC".to_string());
                            preprocessedvec.push(xline.to_owned());
                        }
                        "init" => {
                            preprocessedvec.push(xline.to_owned());
                        }
                        "SCOPE" =>{
                            preprocessedvec.push(xline.to_owned());
                        }
                        _ =>{
                            match xline[1].as_str(){
                                "++" => {
                                    xline.insert(0,"SADD".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                "--" => {
                                    xline.insert(0,"SSUB".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                "!" => {
                                    xline.insert(0,"DBGY".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                "!!" => {
                                    xline.insert(0,"DBGR".to_string());
                                    preprocessedvec.push(xline.to_owned());
                                }
                                _ =>{}
                            }
                        }
                    }

                }
                _ =>{
                    match xline[0].as_str(){
                        "if" | "elseif" | "else" | "match" | "spawnthread" | "coroutine" | "loop" =>{
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
                                    xline.insert(0,"SETC".to_string());
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
                                            xline.insert(0,"SMATCH".to_string());
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
                                                    xline.insert(0,"M3".to_string());
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
                                                                        newline.push("SETVRFN".to_string());
                                                                        newline.push(xline[0].to_string()); // variable
                                                                        newline.push(xline[1].to_string());// =
                                                                        newline.push(funcname.to_string());// =
                                                                        newline.push(getargs);
                                                                        preprocessedvec.push(newline.to_owned());
                                                                    }
                                                                    else{
                                                                        let getargs = Nstring::stringbetween(&xline[2], "(", ")");
                                                                        let givenargs = Nstring::split(&getargs,",");
                                                                        //xline.insert(0,"SETVVF".to_string());
                                                                        let mut newline: Vec<String>  = Vec::new();
                                                                        newline.push("SETVVF".to_string());
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
                                                                    xline.insert(0,"SETVF".to_string());

                                                                    preprocessedvec.push(xline.to_owned());
                                                                }
                                                            }
                                                        }
                                                        NscriptWordTypes::Classfunc =>{

                                                            match self.checkwordtype(&xline[0]){
                                                                NscriptWordTypes::Variable =>{
                                                                    xline.insert(0,"SETVVCF".to_string());
                                                                }
                                                                _ =>{
                                                                    xline.insert(0,"SETVCF".to_string());
                                                                }
                                                            }

                                                            preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Structfn =>{
                                                            xline.insert(0,"SETVSF".to_string());
                                                            preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Nestedfunc =>{
                                                            match self.checkwordtype(&xline[0]){
                                                                NscriptWordTypes::Variable =>{

                                                                    xline.insert(0,"SETVVNF".to_string());
                                                                }
                                                                _ =>{
                                                                    xline.insert(0,"SETVNF".to_string());
                                                                }
                                                            }
                                                            //xline.insert(0,"SETVNF".to_string());
                                                            preprocessedvec.push(xline.to_owned());
                                                        }
                                                        NscriptWordTypes::Variable | NscriptWordTypes::Global | NscriptWordTypes::Property |  NscriptWordTypes::Arraydeclaration | NscriptWordTypes::Bool | NscriptWordTypes::Static | NscriptWordTypes::Number | NscriptWordTypes::Macro | NscriptWordTypes::Array =>{
                                                            xline.insert(0,"SETV".to_string());
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
        let mut retvar = NscriptVar::new("line");

        //print(&line.join(" "),"y");
        match line[0].as_str(){
            "SCOPE" =>{
                if let Some(ret) = self.executesubscope(&line,&formattedblock,block){
                    return ret.clone();
                };
                return retvar;
            }
            "NFN" =>{
                 self.execute_nestedfunction(&line[1],&formattedblock, block);
            }
            "SFN" =>{
                 self.execute_ruststructfn(&line[1],&formattedblock, block) ;
            }
            "FN" =>{
                 self.execute_prencfunction(&line[3],&line,&formattedblock, block) ;
            }
            "CFN" =>{
                 self.execute_classfunction(&line[1],&formattedblock, block) ;
            }
            "SETVF" =>{
                let mut onvar = self.execute_function(&line[3],&formattedblock, block);
                if onvar.name == "return" {
                    onvar.name = line[3].to_string();
                }
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "SETVVF" =>{
                let mut onvar = self.execute_prencfunction(&line[3],&line, &formattedblock,block);
                if onvar.name == "return" {
                    onvar.name = line[1].to_string();
                }
                block.setvar(&line[1], onvar);

            }
            "SETVRFN" =>{
                let mut onvar = self.execute_prerustfunction(&line[3],&line[4], block);
                if onvar.name == "return" {
                    onvar.name = line[1].to_string();
                }
                block.setvar(&line[1], onvar);

            }
            "SETVNF" =>{
                let mut onvar = self.execute_nestedfunction(&line[3],&formattedblock, block);
                if onvar.name == "return" {
                    onvar.name = line[1].to_string();
                }
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "SETVVNF" =>{
                let mut onvar = self.execute_nestedfunction(&line[3],&formattedblock, block);
                if onvar.name == "return" {
                    onvar.name = line[3].to_string();
                }
                block.setvar(&line[1], onvar);
            }
            "SETVSF" =>{
                let onvar = self.execute_ruststructfn(&line[3],&formattedblock, block);
                // if onvar.name == "return" {
                //     onvar.name = line[3].to_string();
                // }
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "SETVCF" =>{
                let mut onvar = self.execute_classfunction(&line[3],&formattedblock, block);
                if onvar.name == "return" {
                    onvar.name = line[3].to_string();
                }
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "SETVVCF" =>{
                let mut onvar = self.execute_classfunction(&line[3],&formattedblock, block);
                if onvar.name == "return" {
                    onvar.name = line[3].to_string();
                }
                block.setvar(&line[1], onvar);
            }
            "RFN" =>{
                self.execute_prerustfunction(&line[1],&line[2], block);
            }
            "init" => {
                let script = self.getwordstring(&line[1],&formattedblock, block);
                return self.parsefile(&script);
            }
            "match" =>{
                let tomatch = self.executeword(&line[1],&formattedblock, block);
                let thisvar =  self.matchscope(&tomatch.stringdata,line[line.len()-1].parse::<usize>().unwrap_or(0)-1,&formattedblock, block);
                return thisvar;
            }
            "BRK" =>{
                retvar.name = "return".to_string();
                retvar.stringdata = "break".to_string();
                return retvar;
            }
            "RET" =>{
                retvar.name = "return".to_string();
                retvar.stringdata = "".to_string();
                return retvar;
            }
            "DBGY" =>{
                retvar = self.executeword(&line[1],&formattedblock,block);
                print(&format!("{} => [{}]",&line[1],&retvar.stringdata),"y");
                return retvar;
            }
            "DBGR" =>{
                retvar = self.executeword(&line[1],&formattedblock,block);

                print(&format!("{} => [{}]",&line[1] ,&retvar.stringdata),"r");

                return retvar;
            }
            "RV" =>{
                retvar = self.executeword(&line[1],&formattedblock, block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "return" =>{
                retvar = self.executeword(&line[1], &formattedblock,block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_FN" =>{
                retvar = self.execute_prencfunction(&line[3],&line,&formattedblock, block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_RFN" =>{
                retvar = self.execute_prerustfunction(&line[2],&line[2], block);
                retvar.name = "return".to_string();
                return retvar;
            }
            "R_NFN" =>{
                retvar = self.execute_nestedfunction(&line[2],&formattedblock, block);
                retvar.name = "return".to_string();
                return retvar;
            }

            "SETC" =>{
                self.execute_setclassfromclass(&line[1],&line[3],&formattedblock, block);
                return retvar;
            }
            "loop" =>{
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
            "SSUB" =>{
                let mut onvar = self.executeword(&line[1], &formattedblock,block);
                onvar.stringdata = (onvar.getnumber() - 1).to_string();
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "SETV" =>{
                let mut onvar = self.executeword(&line[3], &formattedblock,block);
                if onvar.name == "return" {
                    onvar.name = line[3].to_string();
                    //print(&line[3],"br");
                }

                //print(&onvar.stringdata,"br");
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "SADD" =>{
                let mut onvar = self.executeword(&line[1], &formattedblock,block);
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
            "SMATCH" =>{
                let mut onvar: NscriptVar;
                let tomatch = self.executeword(&line[4], &formattedblock,block);
                onvar = self.matchscope(&tomatch.stringdata,line[line.len()-1].parse::<usize>().unwrap_or(1)-1, &formattedblock,block);
                onvar.name = line[1].to_string();
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "M4" =>{
                let onvar = self.runmath(&line, 4,&formattedblock,  block);
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "M3" =>{
                let onvar = self.runmath(&line, 3,&formattedblock,  block);
                self.setdefiningword(&line[1], onvar,&formattedblock, block);
            }
            "if" =>{
                return self.execute_ifline(&line,&formattedblock, block);
            }
            "elseif" =>{
                return self.execute_elseifline(&line,&formattedblock,block);
            }
            "else" =>{
                return self.execute_elseline(&line,&formattedblock,block);
            }
            "BRKC" =>{
                let tobreak = self.getwordstring(&line[2], &formattedblock,block);
                self.removecoroutine(&tobreak);
            }
            "spawnthread" => {
                return self.execute_spawnthread(line, &formattedblock,block);
            }
            "FRIN" =>{
                self.execute_forinloop(&line,&formattedblock,block);
            }
            "FRTO" =>{
                self.execute_fortoloop(&line,&formattedblock,block);
            }
            "CC" =>{//concate self
                let mut equalsfrom = self.executeword(&line[1], &formattedblock,block);
                for xadd in 3..line.len(){
                    equalsfrom.stringdata = equalsfrom.stringdata + &self.executeword(&(line[xadd].to_string()),&formattedblock,block).stringdata.to_string();
                }
                self.setdefiningword(&line[1], equalsfrom, &formattedblock,block);
            }
            "SS" =>{// sebtract self
                let mut onvar = self.executeword(&line[1],&formattedblock, block);
                let mut total = onvar.getnumber();
                for x in 3..line.len(){
                    total -= self.executeword(&line[x],&formattedblock, block).getnumber();
                }
                onvar.stringdata = total.to_string();
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "AA" =>{ // add se;f
                let mut onvar = self.executeword(&line[1],&formattedblock, block);
                let mut total = onvar.getnumber();
                for x in 3..line.len(){
                    total += self.executeword(&line[x], &formattedblock,block).getnumber();
                }
                onvar.stringdata = total.to_string();
                self.setdefiningword(&line[1], onvar, &formattedblock,block);
            }
            "SBL" =>{// set bool
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
            "coroutine"=>{
                self.execute_spawncoroutine(&line,&formattedblock,block);
            }
            _ =>{}
        }
        retvar
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
        //let scopeid = line[line.len()-1].parse::<usize>().unwrap_or(0);
        //let getcode = block.subblocktostring(scopeid);
        //print(&scopeid.to_string(),"p");
        //print(&getcode,"y");
        coroutineblock.staticstrings = block.staticstrings.clone();
        coroutineblock.formattedcode = block.formattedcode.clone();
        //coroutineblock.subblockmap = block.subblockmap.clone();
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
            block.ifset(false);
            block.ifup();
            if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                if result.name == "return" {
                    block.ifdown();
                    return result.clone();
                }
            }
            block.ifdown();

        }
        let result = NscriptVar::new("else");
        return result;
    }
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
    }
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
    fn execute_vecloopsto(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{
        let mut vecvar = NscriptVar::new(&line[1]);
            let splitfrom = split(&line[4],"=");
            let mut start = 0;
            if splitfrom.len() > 1{
                start = self.executeword(&splitfrom[1],&formattedblock,block).stringdata.parse::<usize>().unwrap_or(0);
            }
            let mut iteratevar = NscriptVar::new(&splitfrom[0]);
            for index in start..self.executeword(&line[6],&formattedblock, block).stringdata.parse::<usize>().unwrap_or(0)+1 {
                iteratevar.stringdata = index.to_string();
                block.setvar(&splitfrom[0], iteratevar.clone());
                if let Some(result) = self.executesubscope(&line,&formattedblock, block){
                    if result.name == "return"{
                        vecvar.stringvec.push(result.stringdata.to_string());
                    }
                }
            }
            return vecvar;
    }

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
    fn execute_stringloopsto(&mut self,line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block: &mut NscriptCodeBlock) -> NscriptVar{

        let mut var = NscriptVar::new(&line[1]);
        let splitfrom = split(&line[4],"=");
        let mut start = 0;
        if splitfrom.len() > 1{
            start = self.executeword(&splitfrom[1],&formattedblock,block).stringdata.parse::<usize>().unwrap_or(0);
        }
        let mut createstring = "".to_string();
        let mut iteratevar = NscriptVar::new(&splitfrom[0]);
        for index in start..self.executeword(&line[6], &formattedblock,block).stringdata.parse::<usize>().unwrap_or(0)+1 {
            iteratevar.stringdata = index.to_string();
            block.setvar(&splitfrom[0], iteratevar.clone());
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
        let mut iteratevar = NscriptVar::new(&splitfrom[0]);
        for index in start..self.executeword(&line[4], &formattedblock,block).stringdata.parse::<usize>().unwrap_or(0)+1 {
            iteratevar.stringdata = index.to_string();
            block.setvar(&splitfrom[0], iteratevar.clone());
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
                        varsvec.push(self.executeword(&splitword[1],&formattedblock,block));
                    }
                    _ =>{}
                }
            }
        }
        // get all rustfunctions out the main and copy em to the thread
        let builtins = self.rustfunctionsindex.clone();
        let mut builtinsvec:Vec<NscriptSimpleFunctions> = Vec::new();
        for x in builtins.clone(){
            if let Some(f) = self.rustfunctions.get(&x){
                builtinsvec.push(f.to_owned());
            };
        }

        thread::spawn(move || {
            // yeah used for exedcuteword
            // ( however in this case unused)

            let formattedblockcloned = NscriptFormattedCodeBlock::new();             let mut threadstruct = Nscript::thread();
            // insert all rustfunctions to the threadstruct
            let mut i = 0;
            for x in builtins{
                threadstruct.insertfn(&x, builtinsvec[i]);
                i +=1;
            }
            for mut xfunc in funcvec{
                threadstruct.storage.functions.insert(xfunc.name.to_string(), xfunc.clone());
            }
            for mut xclass in classvec{
                threadstruct.storage.classes.insert(xclass.name.to_string(), xclass.clone());
            }
            threadstruct.parsecodewithvars(&threadcode.formattedcode.codeblock.to_string(), &threadname,varsvec);
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
                    let mut funcblock = threadstruct.getblock("threadreceive");
                    let ncreturn = threadstruct.executeword(&ncfunc,&formattedblockcloned,&mut funcblock);
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
        let  vartype = self.checkdefiningwordtype(&word);
        match vartype{
            NscriptWordTypes::Global => {
                self.storage.setglobal(&word, equalsfrom);
            }
            NscriptWordTypes::Property => {
                let splitword = split(&word,".");
                if splitword.len() > 1{
                    let mut classname = splitword[0].to_string();
                    let trimmedname = Nstring::trimleft(&splitword[0],1);
                    if Nstring::fromleft(&classname, 1) == "*" {
                        classname = self.getwordstring(&trimmedname,&formattedblock,block).to_string();
                    }
                    let mut propname = splitword[1].to_string();
                    let trimmedprop = Nstring::trimleft(&splitword[1],1);
                    if  Nstring::fromleft(&propname, 1)  == "*" {
                        propname = self.getwordstring(&trimmedprop,&formattedblock,block).to_string();
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
                let mut var = self.executeword(&wordsplit[0],&formattedblock,block);
                let  idvar = self.getwordstring(&split(&wordsplit[1],"]")[0],&formattedblock,block).parse::<usize>().unwrap_or(0);
                if idvar < var.stringvec.len(){
                    var.stringvec[idvar] = equalsfrom.stringdata.to_string();
                }
                else {
                    print(&format!("array [{}] tries to set a index but its out of bounds",&wordsplit[0]),"r");
                }
                self.setdefiningword(wordsplit[0], var,&formattedblock, block);

            }
            _ =>{

            }
        };

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
                                part1 = self.getwordstring(&Nstring::trimleft(&splitdot[0], 1),&formattedblock, block);
                            }

                            if Nstring::fromleft(&splitdot[1], 1) == "*"{
                                part2 = self.getwordstring(&Nstring::trimleft(&splitdot[1], 1),&formattedblock, block);
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
                    let mut tmpvar = self.executeword(&subfunction,&formattedblock, block);
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

        //let prefix = self.checkwordtype(word);
        let prefix = self.checkwordtype(word);
         match prefix{
            NscriptWordTypes::Static =>{
                return block.staticstrings[Nstring::trimleft(word, 1).parse::<usize>().unwrap_or(0)].to_string();
            }
            // NscriptWordTypes::Hexstring =>{
            //     return hex_to_string(&Nstring::trimleft(word, 1));
            // }
            NscriptWordTypes::Macro =>{
                return self.storage.getmacrostring(word);
            }
            NscriptWordTypes::Global => {
                return self.storage.getglobal(&word).stringdata.to_string();
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
                        return thisclass.getprop(&pname).stringdata.to_string();
                    }else{
                        print(&format!("word is a prop but theres no class on cname [{}] pname[{}]",&cname,&pname),"r");
                        return "".to_owned();
                    }
                }
            }
           NscriptWordTypes::Number | NscriptWordTypes::Bool =>{
                return word.to_owned();
            }
            NscriptWordTypes::Function => {
                return self.execute_function(word,&formattedblock, block).stringdata.to_string();
            }
            NscriptWordTypes::Reflection =>{
                let toreflect = Nstring::trimleft(word, 1);
                let evaluated = self.getwordstring(&toreflect,&formattedblock, block).to_string();
                return evaluated;
            }
            NscriptWordTypes::Array =>{
                let arrays = split(word,"[");
                    let thisvar = self.executeword(arrays[0],&formattedblock, block);
                    let index = self.getwordstring(&split(&arrays[1], "]")[0],&formattedblock,block).parse::<usize>().unwrap_or(0);
                    if thisvar.stringvec.len() > index{
                        return thisvar.stringvec[index].to_string();
                    }else{
                    print(&format!("array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return "".to_owned();
            }
            NscriptWordTypes::Classfunc => {
                return self.evaluateword(word,&NscriptWordTypes::Classfunc,&formattedblock, block).stringdata.to_string();
            }
            NscriptWordTypes::Structfn => {
                return self.evaluateword(word,&NscriptWordTypes::Structfn,&formattedblock, block).stringdata.to_string();
            }
            _ => {
                return self.evaluateword(word,&NscriptWordTypes::Variable,&formattedblock, block).stringdata.to_string();
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
                return self.execute_classfunction(word,&formattedblock, block);
            }
            // NscriptWordTypes::Hexstring =>{
            //     let mut thisvar = NscriptVar::new("hexstring");
            //     thisvar.stringdata = hex_to_string(&Nstring::trimleft(word, 1));
            //     return thisvar;
            // }
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
                        cname = self.getwordstring(&Nstring::trimleft(&wordsplit[0],1),&formattedblock, block);
                    }
                    if Nstring::fromleft(&cname, 1) ==  "$" {
                        cname = self.getwordstring(&wordsplit[0],&formattedblock, block);
                    }
                    if Nstring::fromleft(&wordsplit[1], 1) ==  "*" {
                        pname = self.getwordstring(&Nstring::trimleft(&wordsplit[1], 1),&formattedblock,block) ;
                    }
                    if let Some(thisclass) = self.getclassref(&cname){
                        return thisclass.getprop(&pname);
                    }else{
                        print(&format!("word is a prop but theres no class on cname [{}] pname[{}]",&cname,&pname),"r");
                    }
                }
            }
            NscriptWordTypes::Number | NscriptWordTypes::Bool =>{
                let mut newvar = NscriptVar::new(word);
                newvar.setstring(&word);
                return newvar;
            }
            NscriptWordTypes::Function => {
                return self.execute_function(word,&formattedblock, block);
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
                let index = self.getwordstring(&split(&arrays[1], "]")[0],&formattedblock,block).parse::<usize>().unwrap_or(0);
                if thisvar.stringvec.len() > index{
                    returnvar.stringdata = thisvar.stringvec[index].to_string();
                }else{
                    print(&format!("array:{} index out of bounds! returning emptyvar, [{}] requested but len = [{}]",&arrays[0],&index,&thisvar.stringvec.len()),"r");
                }
                return returnvar;
            }
            NscriptWordTypes::Arraydeclaration => {
                let mut thisarrayvar = NscriptVar::new("array");
                let between = Nstring::trimright(&Nstring::trimleft(&word,1),1);
                let inarray = split(&between,",");
                for arrayitem in inarray{
                    thisarrayvar.stringvec.push(self.executeword(&arrayitem,&formattedblock, block).stringdata);
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
        let getargs = Nstring::stringbetween(&word, "(", ")");
        let givenargs = Nstring::split(&getargs,",");
        let  funcname = splitfunc[0].to_string();
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
    pub fn execute_ncfunction(&mut self,word:&str,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{

        //let mut args:Vec<String> = Vec::new();
        let splitfunc = split(&word,"(");
        let getargs = Nstring::stringbetween(&word, "(", ")");
        let givenargs = Nstring::split(&getargs,",");
        //let mut getblock= NscriptCodeBlock::new("");
        let  funcname = splitfunc[0].to_string();
        let mut i = 0;
        if let Some(func) = self.storage.functions.get_mut(&split(&funcname,"(")[0].to_string()){
            let mut getblock = func.codeblock.copy();
            //args = func.args.clone();
            for xarg in &func.args.clone(){
                if givenargs.len() > i{
                    let get = self.executeword(&givenargs[i],&formattedblock,block);
                    getblock.setvar(&xarg,get);
                }
                i +=1;
            }

            let formattedblockfunc = &getblock.formattedcode.clone();
            if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar.clone();
                }
            };
        }else{
            print(&format!("no ncfunctions found for [{}]",&funcname),"r");
        }

        return NscriptVar::new("ncfunc");
    }

    /// preproccessed lines use this one to skip some checks and gain performance
    pub fn execute_prencfunction(&mut self,funcname:&str, line:&Vec<String>,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{


        let mut i = 0;
        if let Some(func) = self.storage.functions.get_mut(&funcname.to_string()){
            let mut getblock = func.codeblock.copy();
            let  args = func.args.clone();
            for xarg in 4..line.len(){
                if i <= args.len(){
                    let get = self.executeword(&line[xarg],&formattedblock,block);
                    getblock.setvar(&args[i],get);

                }
                i +=1;
            }

            let formattedblockfunc = getblock.formattedcode.clone();
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
    fn execute_function(&mut self,word:&str,formattedblock: &NscriptFormattedCodeBlock, block:&mut NscriptCodeBlock) ->NscriptVar{
        let mut args:Vec<String> = Vec::new();
        let splitfunc = split(&word,"(");
        let getargs = Nstring::stringbetween(&word, "(", ")");
        let givenargs = Nstring::split(&getargs,",");
        let mut getblock= NscriptCodeBlock::new("");
        let mut i = 0;
        let mut funcname = splitfunc[0].to_string();
        if Nstring::fromleft(&splitfunc[0],1) == "*"{
            funcname = self.getwordstring(&Nstring::trimleft(&splitfunc[0],1),&formattedblock, block);
        }

        if let Some(rustfn) = self.rustfunctions.get(&funcname.to_string()){
            return rustfn(&givenargs,block,&mut self.storage);
        }
        if let Some(func) = self.storage.functions.get_mut(&split(&funcname,"(")[0].to_string()){
            getblock = func.codeblock.copy();
            args = func.args.clone();
        }else{
            print(&format!("no functions found for [{}]",&funcname),"r");
        }
        for xarg in args{
            if givenargs.len() > i{
                let get = self.executeword(&givenargs[i],&formattedblock,block);
                getblock.setvar(&xarg,get);
            }
            i +=1;
        }

        let formattedblockfunc = getblock.formattedcode.clone();
        if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
            if resultvar.name == "return"{
                return resultvar.clone();
            }
        };
        return NscriptVar::new("func");
    }
    fn execute_classfunction(&mut self,word:&str,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{
        let args:Vec<String>;
        let splitfunc = split(&split(&word,"(")[0],".");
        let getargs = Nstring::stringbetween(&word, "(", ")");
        let givenargs = split(&getargs,",");
        let mut getblock: NscriptCodeBlock;
        let mut i = 0;
        let mut classname = splitfunc[0].to_string();
        if Nstring::fromleft(&splitfunc[0],1) == "*"{
            classname = self.getwordstring(&Nstring::trimleft(&splitfunc[0],1),&formattedblock, block);
        }
        let mut funcname = splitfunc[1].to_string();
        if Nstring::fromleft(&splitfunc[1],1) == "*"{
            funcname = self.getwordstring(&Nstring::trimleft(&splitfunc[1],1),&formattedblock, block);
        }

        if let Some(class) = self.getclassref(&classname){
            let mut thisfunc = class.getfunc(&funcname);
            args = thisfunc.args.clone();
            getblock = thisfunc.codeblock.copy();
            for xarg in args{
                if givenargs.len() > i{
                    let get = self.executeword(&givenargs[i],&formattedblock,block);
                    getblock.setvar(&xarg,get);
                }
                i +=1;
            }

            let formattedblockfunc = getblock.formattedcode.clone();
            if let Some(resultvar) = self.executescope(&formattedblockfunc.code[0],&formattedblockfunc,&mut getblock){
                if resultvar.name == "return"{
                    return resultvar.clone();
                }
            };
            return NscriptVar::new("func");
        }else{
            print(&format!("cant find classfn [{}]",&word),"r");
            return NscriptVar::new("error");
        }
    }
    /// this executes rust structs which the user injected, their only on the main thread
    /// some built ins like object and njh are defined here
    fn execute_ruststructfn(&mut self,word:&str,formattedblock:&NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) ->NscriptVar{
        let splitfunc = split(&split(&word,"(")[0],".");
        let getargs = Nstring::stringbetween(&word, "(", ")");
        let givenargs = split(&getargs,",");
        let mut i = 0;
        let mut argvarvec :Vec<NscriptVar> = Vec::new();
        for xarg in &givenargs{
            if givenargs.len() > i{
                let mut get = self.executeword(&givenargs[i],&formattedblock, block);
                get.name = xarg.to_string();
                argvarvec.push(get);
            }
            i +=1;
        }
        let splitstruct = split(&splitfunc[0],"::");
        // check for special functions which require self. for class refs etc.
        match splitstruct[0]{
            "threadsend" =>{
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
                    "objecttostring"=>{
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
        if Nstring::instring(&word, ".") && Nstring::instring(&word, "(") == false{
            return NscriptWordTypes::Property;//"property".to_string();
        }
        if Nstring::fromleft(word, 1) != "[" && Nstring::instring(&word, "[") &&  Nstring::instring(&word, "]") {
            return NscriptWordTypes::Array;//"array".to_string();
        }
        let prefix = Nstring::fromleft(word, 1);
        match prefix.as_str(){
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
        let splitsubs = split(&word,"(");
        if Nstring::instring(&word, "(") &&  Nstring::instring(&word, ")") {
            if splitsubs.len()>2{
                return NscriptWordTypes::Nestedfunc;
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
        if Nstring::instring(&word, ".") && Nstring::instring(&word, "(") == false{
            return NscriptWordTypes::Property;
        }
        if Nstring::fromleft(word, 1) != "[" && Nstring::instring(&word, "[") &&  Nstring::instring(&word, "]") {
            return NscriptWordTypes::Array;
        }

        if word == "true" || word == "false"{
            return NscriptWordTypes::Bool;
        }
        let prefix = Nstring::fromleft(word, 1);
        match prefix.as_str(){
            "$" => {
                return NscriptWordTypes::Global;
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
            // "^" => {
            //     return NscriptWordTypes::Hexstring;
            // }
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
                    self.storage.codeblocks.insert("thread_".to_string() + &thisname, codeblock);
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

 fn parse_and_check_statements(&mut self ,words: &Vec<String>,formattedblock: &NscriptFormattedCodeBlock,block:&mut NscriptCodeBlock) -> bool {
    // this is how you parse a unknown lenght of statements
    // they can be mixed And/or
    // this function will return a bool.
        // -------------------------------------------------------------
        let linelen = words.len();
        if linelen < 4 {
            if words[0] == "if" || words[0] == "elseif" {
                print("is scope failed, line is smaller then 5 words!!!","br");
                return false; // Invalid syntax or empty statement
            }
        }

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

