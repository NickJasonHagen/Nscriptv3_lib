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

        match self.checkwordtype(word){
            NscriptWordTypes::RustFunction =>{
                let splitword = split(&word,"(");
                let splitargs = boxsplit(&splitword[0],",");
                let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::RustFunction,fnname:vec!(splitword[0].into()),fnargs:splitargs};
                self.parsedfunctions.insert(word.into(), thisfunc);
            }
            NscriptWordTypes::Function =>{
                let splitword = split(&word,"(");
                let splitargs = boxsplit(&splitword[0],",");
                let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::Function,fnname:vec!(splitword[0].into()),fnargs:splitargs};
                self.parsedfunctions.insert(word.into(), thisfunc);
            }
            NscriptWordTypes::Classfunc =>{
                let splitword = split(&word,"(");
                let splitname = split(&splitword[0],".");
                let splitargs = boxsplit(&splitword[0],",");
                let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::Function,fnname:vec!(splitname[0].into(),splitname[1].into()),fnargs:splitargs};
                self.parsedfunctions.insert(word.into(), thisfunc);
            }
            NscriptWordTypes::Nestedfunc =>{

            }
            NscriptWordTypes::Structfn =>{
                let splitword = split(&word,"(");
                let splitname = split(&splitword[0],"::");
                let splitargs = boxsplit(&splitword[0],",");
                let thisfunc = NscriptExecutableFn{fntype:NscriptWordTypes::Function,fnname:vec!(splitname[0].into(),splitname[1].into()),fnargs:splitargs};
                self.parsedfunctions.insert(word.into(), thisfunc);

            }
            _ =>{

            }
        }
    }
}
