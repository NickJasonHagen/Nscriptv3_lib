use crate::*;
#[cfg(not(windows))]
use termion::{
    event::{ Key},
    input::TermRead,
    raw::{IntoRawMode},
    cursor,
    clear,
};
use crossterm::{
    terminal
};
pub struct Nterminal{

}


impl Nterminal{
    pub fn enableraw(){
        let _ = terminal::enable_raw_mode();
    }
    pub fn disableraw(){
        let _ = terminal::disable_raw_mode();
    }
    #[cfg(windows)]
    pub fn updatedterminal(printframe:&str){
        cwrite(printframe,"");
    }

    #[cfg(not(windows))]

    pub fn updateterminal(printframe:&str){
        // Set up the terminal
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = io::BufWriter::new(stdout);
        //let stdin = io::stdin();

        print!("{}{}", clear::All, cursor::Hide);
        stdout.flush().unwrap();
        let mut i =  1;

        for line in split(printframe,"\n"){
            let mut beginline = 1;
            for subline in split(line,"|||"){
                let checkcolor = split(subline,"&printcolor=");
                if checkcolor.len() > 1{
                    Nterminal::print(checkcolor[0],checkcolor[1],beginline,i);
                    let lenght: u16 =  split(checkcolor[0],"").len() as u16;
                    beginline = beginline + lenght ;


                }
                else{
                    print!(
                        "{}{}",
                        cursor::Goto(1,i),
                        line
                    );
                }


            }
            i = i +1;
        }
        //thread::sleep(Duration::from_secs(1));
        // Restore the terminal state
    }
    #[cfg(windows)]
    pub fn flush()->String{
        return "error_nonwinfunction".to_owned();
    }

    #[cfg(not(windows))]
    pub fn flush(){
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = io::BufWriter::new(stdout);
        print!("{}{}", clear::All, cursor::Hide);
        stdout.flush().unwrap();
    }
    #[cfg(windows)]
    pub fn terminalkey()->String{
        return "error_nonwinfunction".to_owned();
    }
    #[cfg(windows)]
    pub fn resetterminal(){
        //return "error-unixonly".to_owned();

    }
    #[cfg(windows)]
    pub fn print(color:&str,s:&str,i:u16,x:u16){
        //return "error-unixonly".to_owned();

    }
    #[cfg(not(windows))]
    pub fn resetterminal(){
        print!("{}{}", cursor::Show, clear::All);

    }
    #[cfg(not(windows))]
    pub fn terminalkey()->String{
        // Listen for keyboard input in the main thread
        let mut ret = "".to_string();

        //let soundthread = thread::spawn(move || {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = io::BufWriter::new(stdout);
        let stdin = io::stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('a') =>{
                    ret = "a".to_owned();
                    break
                },
                Key::Char('b') =>{
                    ret = "b".to_owned();
                    break
                },
                Key::Char('c') =>{
                    ret = "c".to_owned();
                    break
                },
                Key::Char('d') =>{
                    ret = "d".to_owned();
                    break
                },
                Key::Char('e') =>{
                    ret = "e".to_owned();
                    break
                },
                Key::Char('f') =>{
                    ret = "f".to_owned();
                    break
                },
                Key::Char('g') =>{
                    ret = "g".to_owned();
                    break
                },
                Key::Char('h') =>{
                    ret = "h".to_owned();
                    break
                },
                Key::Char('i') =>{
                    ret = "i".to_owned();
                    break
                },
                Key::Char('j') =>{
                    ret = "j".to_owned();
                    break
                },
                Key::Char('k') =>{
                    ret = "k".to_owned();
                    break
                },
                Key::Char('l') =>{
                    ret = "l".to_owned();
                    break
                },
                Key::Char('m') =>{
                    ret = "m".to_owned();
                    break
                },
                Key::Char('n') =>{
                    ret = "n".to_owned();
                    break
                },
                Key::Char('o') =>{
                    ret = "o".to_owned();
                    break
                },
                Key::Char('p') =>{
                    ret = "p".to_owned();
                    break
                },
                Key::Char('q') =>{
                    ret = "q".to_owned();
                    break
                },
                Key::Char('r') =>{
                    ret = "r".to_owned();
                    break
                },
                Key::Char('s') =>{
                    ret = "s".to_owned();
                    break
                },
                Key::Char('t') =>{
                    ret = "t".to_owned();
                    break
                },
                Key::Char('v') =>{
                    ret = "v".to_owned();
                    break
                },
                Key::Char('w') =>{
                    ret = "w".to_owned();
                    break
                },
                Key::Char('x') =>{
                    ret = "x".to_owned();
                    break
                },
                Key::Char('u') =>{
                    ret = "u".to_owned();
                    break
                },

                Key::Char('y') =>{
                    ret = "y".to_owned();
                    break
                },
                Key::Char('z') =>{
                    ret = "z".to_owned();
                    break
                },
                Key::Char('\n') =>{
                    ret = "enter".to_owned();
                    break
                },
                Key::Char(' ') =>{
                    ret = "space".to_owned();
                    break
                },
                Key::Char('.') =>{
                    ret = ".".to_owned();
                    break
                },
                Key::Char(',') =>{
                    ret = ",".to_owned();
                    break
                },
                Key::Char('(') =>{
                    ret = "(".to_owned();
                    break
                },
                Key::Char(')') =>{
                    ret = ")".to_owned();
                    break
                },
                Key::Char('"') =>{
                    ret = "".to_owned();
                    break
                },
                Key::Char('!') =>{
                    ret = "!".to_owned();
                    break
                },
                Key::Char('@') =>{
                    ret = "@".to_owned();
                    break
                },
                Key::Char('#') =>{
                    ret = "#".to_owned();
                    break
                },
                Key::Char('%') =>{
                    ret = "%".to_owned();
                    break
                },
                Key::Char('^') =>{
                    ret = "^".to_owned();
                    break
                },
                Key::Char('&') =>{
                    ret = "&".to_owned();
                    break
                },
                Key::Char('*') =>{
                    ret = "*".to_owned();
                    break
                },

                Key::Char('1') =>{
                    ret = "1".to_owned();
                    break
                },
                Key::Char('2') =>{
                    ret = "2".to_owned();
                    break
                },
                Key::Char('3') =>{
                    ret = "3".to_owned();
                    break
                },
                Key::Char('4') =>{
                    ret = "4".to_owned();
                    break
                },
                Key::Char('5') =>{
                    ret = "5".to_owned();
                    break
                },
                Key::Char('6') =>{
                    ret = "6".to_owned();
                    break
                },
                Key::Char('7') =>{
                    ret = "7".to_owned();
                    break
                },
                Key::Char('8') =>{
                    ret = "8".to_owned();
                    break
                },
                Key::Char('9') =>{
                    ret = "9".to_owned();
                    break
                },
                Key::Char('0') =>{
                    ret = "0".to_owned();
                    break
                },
                Key::Up =>{
                    ret =  "up".to_owned();
                    break
                },
                Key::Down =>{
                    ret =  "down".to_owned();
                    break
                },
                Key::Left =>{
                    ret =  "left".to_owned();
                    break
                },
                Key::Right =>{
                    ret =  "right".to_owned();
                    break
                },

                Key::BackTab=>{
                    ret =  "tab".to_owned();
                    break
                },
                Key::Ctrl('c')=>{
                    ret =  "ctrlc".to_owned();
                    break
                },
                Key::Alt('r')=>{
                    ret =  "altr".to_owned();
                    break
                },
                Key::F(1)=>{
                    ret =  "F1".to_owned();
                    break
                },
                Key::F(2)=>{
                    ret =  "F2".to_owned();
                    break
                },
                Key::F(3)=>{
                    ret =  "F3".to_owned();
                    break
                },
                Key::F(4)=>{
                    ret =  "F4".to_owned();
                    break
                },
                Key::F(5)=>{
                    ret =  "F5".to_owned();
                    break
                },
                Key::F(6)=>{
                    ret =  "F6".to_owned();
                    break
                },
                Key::F(7)=>{
                    ret =  "F7".to_owned();
                    break
                },
                Key::F(8)=>{
                    ret =  "F8".to_owned();
                    break
                },
                Key::F(9)=>{
                    ret =  "F9".to_owned();
                    break
                },
                Key::F(10)=>{
                    ret =  "F10".to_owned();
                    break
                },
                Key::F(11)=>{
                    ret =  "F11".to_owned();
                    break
                },
                Key::F(12)=>{
                    ret =  "F12".to_owned();
                    break
                },

                Key::Backspace =>{
                    ret =  "backspace".to_owned();
                    break
                },
                Key::Esc =>{
                    ret =  "esc".to_owned();
                    break
                },
                _ => {
                    break
                }
            }
            //stdout.flush().unwrap();
        }
        stdout.flush().unwrap();
        //});
        return ret;

    }


    #[cfg(not(windows))]
    pub fn print(m:&str,color:&str,x:u16,i:u16){
        print!(
            "{}{}",
            cursor::Goto(x,i),
             nscriptgetprintingcolor(m,color)
        );
    }
}
pub fn nscriptfn_printpos(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    let x:u16 = match storage.getargstring(&args[2], block).parse::<usize>(){
        Ok(res) =>{
            res.try_into().unwrap_or(1)
        }
        Err(_) =>{
            1
        }
    };
    let y:u16 = match storage.getargstring(&args[3], block).parse::<usize>(){
        Ok(res) =>{
            res.try_into().unwrap_or(1)
        }
        Err(_) =>{
            1
        }
    };
    Nterminal::print(&storage.getargstring(&args[0], block),&storage.getargstring(&args[1], block),x,y);
    NscriptVar::new("r")
}
pub fn nscriptfn_terminalkey(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar  {
    NscriptVar::newstring("r",Nterminal::terminalkey())
}
pub fn nscriptfn_updateterminal(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar  {
    Nterminal::updateterminal(&storage.getargstring(args[0],block));
    NscriptVar::new("r")
}
pub fn nscriptfn_terminalflush(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar  {
    Nterminal::flush();
    NscriptVar::new("r")
}
pub fn nscriptfn_terminalenableraw(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar  {
    Nterminal::enableraw();
    NscriptVar::new("r")
}
pub fn nscriptfn_terminaldisableraw(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar  {
    Nterminal::disableraw();
    NscriptVar::new("r")
}
pub fn nscriptfn_terminalreset(_args:&Vec<&str>,_block :&mut NscriptCodeBlock , _storage :&mut NscriptStorage) -> NscriptVar  {
    Nterminal::resetterminal();
    NscriptVar::new("r")
}

