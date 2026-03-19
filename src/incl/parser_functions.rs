use crate::*;
#[derive(Clone)]
pub struct NscriptExecutableFn{
    pub fntype:NscriptWordTypes,//holds the type for the RT
    pub fnname: Vec<Box<str>>,// holds the name part , ( its vector for structfn("split by ::") and classfn("split by .") )
    pub fnargs: Vec<Box<str>>,// holds the arguments for the function

}
// creates a Vec<Box<str>> using split
pub fn boxsplit(word:&str,delim:&str) ->Vec<Box<str>>{
    let mut vec: Vec<Box<str>> = Vec::new();
    for x in split(&word,&delim){
        vec.push(x.into());
    }
    vec
}
//concept / WIP / underconstruction unused
impl Nscript{
    // used pre split the functions used in the sheets so that the runtime doesnt have to use split
    // functions will be pushed into a map in Nscript, the name of the word be used as a key
    pub fn presplitfunction(&mut self,word:&str){
        if Nstring::instring(&word,"(") {
            match self.checkwordtype(word){
                NscriptWordTypes::RustFunction =>{
                    let splitword = split(&word,"(");
                    let splitargs = boxsplit(&splitword[0],",");
                    let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::RustFunction,fnname:vec!(Nstring::trimprefix(splitword[0]).into()),fnargs:splitargs};
                    self.parsedfunctions.insert(word.into(), thisfunc);
print(&format!("presplit rustfn: [{}]",&word),"y");
                }
                NscriptWordTypes::Function =>{


print(&format!("presplit ncfn: [{}]",&word),"y");

                    let splitword = split(&word,"(");
                    let splitargs = boxsplit(&splitword[0],",");
                    let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::Function,fnname:vec!(Nstring::trimprefix(splitword[0]).into()),fnargs:splitargs};
                    self.parsedfunctions.insert(word.into(), thisfunc);
                }
                NscriptWordTypes::Classfunc =>{

print(&format!("presplit Classfn: [{}]",&word),"y");

                    let splitword = split(&word,"(");
                    let splitname = split(&splitword[0],".");
                    let splitargs = boxsplit(&splitword[0],",");
                    let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::Classfunc,fnname:vec!(Nstring::trimprefix(splitname[0]).into(),splitname[1].into()),fnargs:splitargs};
                    self.parsedfunctions.insert(word.into(), thisfunc);
                }
                NscriptWordTypes::Nestedfunc =>{

                }
                NscriptWordTypes::Structfn =>{

                    print(&format!("presplit structfn: [{}]",&word),"y");
                    let splitword = split(&word,"(");
                    let splitname = split(&splitword[0],"::");
                    let splitargs = boxsplit(&splitword[0],",");
                    let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::Structfn,fnname:vec!(Nstring::trimprefix(splitname[0]).into(),splitname[1].into()),fnargs:splitargs};
                    self.parsedfunctions.insert(word.into(), thisfunc);

                }
                _ =>{

                    println!("nofunc: [{}]",&word);
                }
            }
        }
    }

    // 3.6.106> new cached presplit parsing
   pub fn execute_cachedfunction(&mut self,wordr:&Box<str>, block:&mut NscriptCodeBlock) ->NscriptVar{
        match self.checkwordtype(wordr){
            NscriptWordTypes::Function =>{
                if let Some(var) = self.execute_cachedncfunction(&wordr, block){
                    return var;
                }
                return NscriptVar::newstring("r","".to_owned());
            }
            NscriptWordTypes::Classfunc =>{
                return self.execute_cachedclassfunction(wordr, block);
            }
            NscriptWordTypes::RustFunction =>{
                return self.execute_cachedrustfunction(wordr, block);
            }
            NscriptWordTypes::Structfn =>{
                let fblock =self.getexecutableblock(&block.name);
                return self.execute_ruststructfn(&wordr,&fblock, block);
            }
            _ =>{
                print(&format!("execute_cachedfunction() no cached functions found for [{}] in block: [{}]",&wordr,&block.name),"r");
                return NscriptVar::new("func");
            }
        }
    }
    // used in cachedfunction()
    pub fn execute_cachedrustfunction(&mut self,word:&Box<str>, block:&mut NscriptCodeBlock) ->NscriptVar{
        //let funcname = Nstring::trimprefix(&funcname);
        //let mut varvec : Vec<NscriptVar> = Vec::new();
        if let Some(res) = self.parsedfunctions.get(word.into()){
            // set arguments from the cache and evaluate them into the blck
            let varvec: Vec<&str> = res.fnargs.iter().map(|s| s.as_ref()).collect();

            //let nfn = Nstring::trimprefix(&res.fnname[0]);
            if let Some(rustfn) = self.rustfunctions.get(&res.fnname[0]){
                return rustfn(&varvec,block,&mut self.storage);
            }

        }


        print(&format!("execute_cachedfunction() no cached rust functions found for [{}] in block: [{}]",&word,&block.name),"r");
        return NscriptVar::new("wont");
    }
    pub fn execute_cachedncfunction(&mut self,word:&Box<str>,block:&mut NscriptCodeBlock) ->Option<NscriptVar>{
        let mut varvec : Vec<NscriptVar> = Vec::new();
        if let Some(res) = self.parsedfunctions.get(word.into()){
            // set arguments from the cache and evaluate them into the blck
            for xarg in &res.fnargs{
                varvec.push(self.storage.getvar(&xarg,block));
            }

            let funcname : Box<str> = res.fnname[0].clone().into();
            if let Some(func) = self.userfunctions.get(&funcname.to_string()){
                let mut getblock = func.codeblock.clone();
                let formattedblockfunc = self.getexecutableblock(&getblock.name);//func.formattedcodeblock.clone();
                let ln =res.fnargs.len();
                let ln2 =func.args.len();
                if ln != 0{
                    for xarg in 0..ln{
                        if xarg < ln2{
                            getblock.setvar(&func.args[xarg],self.storage.getvar(&res.fnargs[xarg],block));
                        }
                    }
                }

                return self.executescope(&formattedblockfunc.boxedcode[0],&formattedblockfunc,&mut getblock);
            }else{
                print(&format!("execute_cachedfunction() no cached nc functions found for [{}] in block: [{}]",res.fnname[0],&block.name),"r");
            }
        }

        None
    }
   fn execute_cachedclassfunction(&mut self,word:&Box<str>,block:&mut NscriptCodeBlock) ->NscriptVar{
        //let executablefunc:NscriptExecutableFn;
        let mut varvec : Vec<NscriptVar> = Vec::new();
        let mut class : Box<str> = "".into();
        let mut func : Box<str> = "".into();
        if let Some(res) = self.parsedfunctions.get(word.into()){
            // set arguments from the cache and evaluate them into the blck
            for xarg in &res.fnargs{
                varvec.push(self.storage.getvar(&xarg,block));
            }
            class = res.fnname[0].clone().into();
            func = res.fnname[1].clone().into();
        }

        let class = Nstring::trimprefix(&class);
        let mut getblock: NscriptCodeBlock;
        let  formattedblockfunc: NscriptExecutableCodeBlock;
        // let givenargs = split(&givenargs,",");
        // let mut varvec: Vec<NscriptVar> = Vec::new();
        // for xarg in &givenargs{
        //     varvec.push(self.storage.getvar(&xarg,block));
        // }
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
                formattedblockfunc = thisfunc.executablecodeblock.clone();
                let len = thisfunc.args.len();
                for xarg in 0..varvec.len(){
                    if len > xarg{
                        getblock.setvar(&thisfunc.args[xarg],varvec[xarg].clone());
                    }
                }
                if let Some(resultvar) = self.executescope(&formattedblockfunc.boxedcode[0],&formattedblockfunc,&mut getblock){
                    if resultvar.name == "return".into(){
                        return resultvar;
                    }
                };
                return NscriptVar::new("func");
            }else{
                if funcname != "construct".into() && funcname != "destruct".into() {
                    print(&format!("cant find classfn [{}].[{}]",&funcname,&classname),"r");
                }
                return NscriptVar::new("error");
            }
        };
        return NscriptVar::new("error");
    }

}
