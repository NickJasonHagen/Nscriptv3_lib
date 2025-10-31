
use crate::*;

impl  Nscript{
    pub fn httpexec(&mut self, tocall:&str,args:&Vec<NscriptVar>) -> NscriptVar{
        match tocall{
            "bind" => {
                if args.len() > 1{
                    return self.httpbind(&args[0].stringdata, &args[1].stringdata);
                }else{
                    print("forgotten arguments for http::bind!","r");
                }
            }
            "listen" =>{
                return self.httplisten(&args[0].stringdata);
            }
            _ => {

            }
        }
        let var = NscriptVar::new("httpexec");
        var
    }
    pub fn httpbind(&mut self, ip:&str, port:&str) ->NscriptVar{
        let mut var = NscriptVar::new("httpbind");
        // retrieve the prop's set for class server in nscript:server.nc
        let server_addres_nc = ip.to_string();
        let server_port_nc = port.to_string();
        let name = server_addres_nc.to_string() + &port;
        let listener: TcpListener;
        if server_port_nc != "" && server_addres_nc != "" {
            // when the server.nc is run class server.ip and server.port be checked!
            listener = TcpListener::bind(format!("{}:{}", &server_addres_nc, &server_port_nc)).unwrap();
            println!(
                "Server started at http://{}:{}",
                &server_addres_nc, &server_port_nc
            );
        } else {
            // if missing serverclass or something, use the constants
            listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", 8080)).unwrap();

        }
        #[cfg(not(windows))]
        if let Err(_) = listener.set_nonblocking(true){
            print("error cant set http listener to nonblocking","r");
        };
        var.stringdata = name.to_string();
        self.tcplisteners.insert(name,listener);
        var
    }

    pub fn httplisten(&mut self,nameid:&str) ->NscriptVar{
        let mut var = NscriptVar::new("listener");
        if let Some(listener) = self.tcplisteners.get_mut(nameid){
            match listener.accept() {
                Ok((stream, _)) => {
                    match stream.peer_addr() {
                        Ok(res) => {
                            let remote_ip = res.ip();
                            let mut socketvar = NscriptVar::new("$socketip");
                            socketvar.stringdata = remote_ip.to_string();
                            self.storage.setglobal("$socketip",socketvar);
                            //let mut block = NscriptCodeBlock::new("httplisten");
                            //let formattedblock = NscriptExecutableCodeBlock::new();//.formattedcode.clone();
                            //let onconnectvar = self.executeword("\\server.onconnect($socketip)",&formattedblock, &mut block);
                            // if onconnectvar.stringdata == "false" {
                            //     var.stringdata = format!("connection server.onconnect($socketip) returned false -> closed");
                            // }
                            // else{
                                self.handle_connection( stream);
                                var.stringdata = format!("connection ok and closed");
                            //}
                            return var;
                        }
                        Err(err) => {
                            var.stringdata = format!("Connection error{}", err).to_string();
                            return var;
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    var.stringdata = "No incoming connections yet".to_string();
                    return var;
                }
                Err(e) => {
                    var.stringdata = format!("Error accepting connection: {}", e);
                    return var;
                }
            }
        }
        var.stringdata = "theres no binding loaded".to_string();
        return var;
    }
    fn httpsetarguments(&mut self,pathparts:&Vec<&str>){
        let mut url_args = Vec::new();
        if pathparts.len() > 1 {
            url_args = split(pathparts[1], "&");
        }
        for i in 1..10 {
            let name = "$param".to_string() + &i.to_string();
            let mut paramvar = NscriptVar::new(&name);
            if url_args.len() > i - 1 {
                paramvar.stringdata = decode_html_url(&url_args[i - 1].to_owned());
            }
            self.storage.setglobal(&name, paramvar);
        }
    }
    fn httprunhttpaccessnc(&mut self,pathparts:&Vec<&str>,webroot:&str) -> bool{
        let mut httpaccessfile = split(&pathparts[0],"/");
        let arglen = httpaccessfile.len();
        httpaccessfile[arglen-1] = "httpaccess.nc";
        let httpa = webroot.to_string() + "/" + &httpaccessfile.join("/");
        if Nfile::checkexists(&httpa) {
            let ret = self.parsefile(&httpa).stringdata.to_string();
            if ret == "false" || ret == "!false"{
                return false;
            }
        }
        return true;
    }
    fn checkstream(&mut self,stream: &mut TcpStream,mut buffer: &mut [u8]) -> bool{
        match stream.read(&mut buffer) {
            Ok(_) => {
                true
            }
            Err(_) => {
                println!("stream read error ! ");
                false
            }
        }
    }

    fn handle_connection(&mut self,mut stream: TcpStream) {
        // this is the webserver part it will take a GET request and handle it.
        // text files are on the main thread for other downloads it goes to a other thread
        // .nc files are being regonised and they will return their return results to the user browser.
        // --------------------------------------------------------------------------------------

        let mut buffer = [0; 1024];
        //stream.read(&mut buffer).unwrap();
        let mut connectionblock = NscriptCodeBlock::new("connection");
        //let  formattedblock = NscriptExecutableCodeBlock::new();
        if self.checkstream(&mut stream,&mut buffer) == false{
            return
        }

        let request = String::from_utf8_lossy(&buffer[..]);
        let mut var = NscriptVar::new("$request");
        var.stringdata = request.to_string();
        self.storage.setglobal("$request",var);
        //vmap.setvar("server.request".to_owned(), &request);
        if Nstring::instring(&request, "B blob data") {
            println!("(debug->returning) Blob data entering: {}", &request);
            return; // prevent errors , return!
        }
        if Nstring::instring(&request, "POST") == false && Nstring::instring(&request, "GET") == false {
            println!("A non POST nor GET packet entered: \n {}", &request);
            return; // clearly we aint gonna handle this (yet)
        }
        let domainname = Nstring::replace(
            &Nstring::stringbetween(&request, "Host: ", "\r\n"),
            "www.",
            "",
        );

        let domainname = split(&domainname, ":")[0];
        let mut var = NscriptVar::new("$domainname");
        var.stringdata = domainname.to_string();
        self.storage.setglobal("$domainname",var);
        let mut block = NscriptCodeBlock::new("httplisten");
        let formattedblock = NscriptExecutableCodeBlock::new();//.formattedcode.clone();
        let onconnectvar = self.executeword("\\server.onconnect($socketip)",&formattedblock, &mut block);
        if onconnectvar.stringdata == "false" {
            //var.stringdata = format!("connection server.onconnect($socketip) returned false -> closed");
            return;
        }
    //}
        let request_parts: Vec<&str> = request.split(" ").collect();
        let mut pathparts = Vec::new();
        let trimmedreq: String;
        if request_parts.len() > 1 {
            if request_parts[1].contains("B blob data") {
                println!("blobdatafound returning");
                return; // Ignore blob data and return without processing
            }
            trimmedreq = Nstring::trimleft(&request_parts[1], 1);
            pathparts = split(&trimmedreq, "?");
        } else {
            pathparts.push("");
        }
        if pathparts[0] == "" {
            pathparts[0] = "index.nc";
        }

        self.httpsetarguments(&pathparts);

        let mut webroot = self.executeword("&server.serverroot",&formattedblock,&mut connectionblock).stringdata.to_string();
        if webroot == "" {
            webroot = "./".to_string();
        }

        let mut file_path = Nstring::replace(
            &format!("{}{}{}", &webroot, "/", &pathparts[0]),
            "/..",
            "",
        );
        if self.httprunhttpaccessnc(&pathparts,&webroot) == false{return;};

        let checkthis = webroot.clone() + "domains/" + &domainname + "/http.nc";
        if Nfile::checkexists(&checkthis) {
            file_path = webroot.clone() + "domains/" + &domainname + "/public/" + &pathparts[0];
        }
        if request_parts[0] == "POST" {
            let mut postdata:String;// String::new();
            let mut postvar = NscriptVar::new("$POSTPACKET");
            postvar.stringdata = Nstring::replace(&request.to_string(), "\0", "");
            self.storage.setglobal(
                "$POSTPACKET",
                postvar,
            );
            let strippostdata = split(&request, "\r\n\r\n");
            if strippostdata.len() > 1 {
                postdata = "".to_owned() + &strippostdata[1..].join("\r\n\r\n"); // used for post buffer data
            } else {
                return; //some jacked up post request being done.
            }
            let receivedcontentlenght = postdata.len();

            if let Some(extension) = Path::new(&file_path)
                .extension()
                .and_then(|os_str| os_str.to_str().map(|s| s.to_owned()))
            {
                if ["nc"].contains(&extension.as_str()) {

                    //println!("Its a Post to Nc");
                    let bsize = nscript_usize(
                        &Nstring::stringbetween(&request, "Content-Length: ", "Cache").trim(),
                    );
                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    if bsize > nscript_usize(&self.executeword("&server.POSTbytesmax",&formattedblock, &mut connectionblock).stringdata) {
                        let response = "SERVERERROR:PostSizeExceedsLimit";
                        stream.write(response.as_bytes()).unwrap();
                        return;
                    }
                    if bsize > receivedcontentlenght {
                        let mut start_time = Instant::now();
                        loop {
                            let end = Instant::now();
                            if (start_time - end).as_millis() >= 1000 {
                                print("closed by timeout", "r");
                                break;
                            }

                            match stream.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    postdata = postdata + &String::from_utf8_lossy(&buffer[0..bytes_read]);
                                    if bytes_read <= 1023 && postdata.len() + 1024 >= bsize{
                                        break;
                                    }
                                    start_time = Instant::now();
                                }
                                Err(e) => {
                                    print!("error nchttp {}", e); // handle OS error on connection-reset)
                                    break;
                                }
                            }
                        }
                    }
                    let mut postvar = NscriptVar::new("$POSTDATA");
                    postvar.stringdata = Nstring::replace(&postdata.trim(), "\0", "");
                    self.storage.setglobal(
                        "$POSTDATA",
                        postvar,
                    );
                    let response = self.parsecode(&Nfile::read(&file_path), &file_path).stringdata.to_string();

                    stream.write(response.as_bytes()).unwrap();
                }
            }
            return;
        }
        if request_parts[0] == "GET" {
            self.storage.setglobal(
                "$POSTDATA",
                NscriptVar::new("$POSTDATA"),
            );
            if let Some(extension) = Path::new(&file_path)
                .extension()
                .and_then(|os_str| os_str.to_str().map(|s| s.to_owned()))
            {

                if ["nc"].contains(&extension.as_str()) {
                    let _ = match File::open(&file_path) {
                        Ok(_) => {}
                        Err(_) => {
                            let mut response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                            let nc404file =
                            webroot.clone() + "/404.nc";
                            println!("404={},", nc404file);
                            if Nfile::checkexists(&nc404file) {
                                let ret = self.parsecode(&Nfile::read(&nc404file),"404").stringdata.to_string();
                                response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                                    "text/html",
                                    &ret.len()
                                );
                                stream.write(response.as_bytes()).unwrap();
                                stream.write(ret.as_bytes()).unwrap();
                                return;
                            } else {
                                stream.write(response.as_bytes()).unwrap();
                                return;
                            }
                        }
                    };
                    //let scriptcode = Nfile::read(&file_path);
                    let ret = self.parsefile(&file_path).stringdata.to_string();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                        "text/html",
                        &ret.len()
                    );

                    if let Err(_) = stream.write(response.as_bytes()) {return;}
                    stream.write(ret.as_bytes()).unwrap();
                    return
                }
                if ["html"].contains(&extension.as_str()) {
                    let _ = match File::open(&file_path) {
                        Ok(_) => {}
                        Err(_) => {
                            let mut response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                            let nc404file =
                            webroot.clone() + "/404.nc";
                            println!("404={},", nc404file);
                            if Nfile::checkexists(&nc404file) {
                                //let compcode = nscript_formatsheet(&read_file_utf8(&nc404file),vmap);
                                let compcode = Nfile::read(&nc404file);
                                let ret = self.parsecode(&compcode,"404").stringdata.to_string();

                                response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                                    "text/html",
                                    &ret.len()
                                );
                                stream.write(response.as_bytes()).unwrap();
                                stream.write(ret.as_bytes()).unwrap();
                                return;
                            } else {
                                stream.write(response.as_bytes()).unwrap();
                                return;
                            }
                        }
                    };
                    let mut scriptcode = Nfile::read(&file_path);
                    loop {
                        let subscope = Nstring::stringbetween(&scriptcode, "<nscript>", "</nscript>");
                        if subscope != ""{
                            let nvar = self.parsecode(&subscope, &file_path);

                            let toremove = "<nscript>".to_string() + &subscope + "</nscript>";
                            if nvar.name == "return"{
                                scriptcode = Nstring::replace(&scriptcode, &toremove, &nvar.stringdata);
                            }
                            else{
                                scriptcode = Nstring::replace(&scriptcode, &toremove,"");
                            }
                        }
                        else{
                            break
                        }
                    }
                    let ret = scriptcode;//.parsefile(&file_path).stringdata.to_string();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                        "text/html",
                        &ret.len()
                    );

                    stream.write(response.as_bytes()).unwrap();
                    stream.write(ret.as_bytes()).unwrap();

                    return;
                }
                let file_path_clone = file_path.clone(); // clone file_path
                thread::spawn(move || {
                    let mut file = match File::open(&file_path_clone) {
                        Ok(file) => file,
                        Err(_) => {
                            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\nPageNotFound");
                            stream.write(response.as_bytes()).unwrap();
                            return;
                        }
                    };
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents).unwrap();
                    let content_type = match extension.as_str() {
                        "jpg" | "jpeg" => "image/jpeg",
                        "png" => "image/png",
                        "gif" => "image/gif",
                        "js" => "application/javascript",
                        "txt" => "text/plain",
                        "html" => "text/html",
                        "css" => "text/css",
                        _ => "application/octet-stream",
                    };
                    let response = format!(
                        "HTTP/2.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                        content_type,
                        contents.len()
                    );

                    if let Err(_) = stream.write(response.as_bytes()) {return;}
                    if let Err(_) = stream.write(&contents) {return;}

                });
                return;
            }
        }
    }

}


pub struct NscriptPostHandle{
    pub scriptpath: String,
    pub params: Vec<NscriptVar>,
    pub postdata: NscriptVar,
    pub status: String,
    pub httppostthreadsreceiver: mpsc::Receiver<NscriptVar>,
    pub httppostthreadssenders: mpsc::Sender<NscriptVar>,
}

impl Nscript{

    pub fn handleconnections(&mut self){

    }
    pub fn postthreadcall(&mut self,threadname:&str) -> NscriptVar{
        if let Some(thishandle) = self.httpposthandles.get_mut(&threadname.to_string()){
            match thishandle.httppostthreadssenders.send(NscriptVar::newstring("str", "check".to_string())){
                Ok(_)=>{
                    let msg: NscriptVar = match thishandle.httppostthreadsreceiver.try_recv(){
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
                Err(_)=>{
                    return NscriptVar::new("error");
                }
            };
            return NscriptVar::new("ok");
        };

        NscriptVar::new(".")
    }
    pub fn addnew(&mut self,mut stream:TcpStream,threadname:&str,bufferinit :Vec<u8>,mut postdata:String,bsize:usize,scripttocall: &str,args:Vec<NscriptVar>){
        let mut buffer = bufferinit.to_owned();
        let (main_to_worker_tx, main_to_worker_rx) = mpsc::channel();
        let (worker_to_main_tx, worker_to_main_rx) = mpsc::channel();
        let thishandle = NscriptPostHandle{
            scriptpath:scripttocall.to_string(),
            params:args,
            postdata:NscriptVar::new("."),
            status:"init".to_string(),
            httppostthreadsreceiver:worker_to_main_rx,
            httppostthreadssenders:main_to_worker_tx
        };
        self.httpposthandles.insert(threadname.to_string(),thishandle);
        let worker_to_main_tx = Arc::new(Mutex::new(worker_to_main_tx));
        thread::spawn(move || {
            let mut streamready = false;
            let mut  start_time = Instant::now();
            loop {
                let end = Instant::now();
                if (start_time - end).as_millis() >= 1000 {
                    print("closed by timeout", "r");
                    break;
                }
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        postdata = postdata + &String::from_utf8_lossy(&buffer[0..bytes_read]);
                        if bytes_read <= 1023 && postdata.len() + 1024 >= bsize{
                            streamready = true;
                            break;
                        }
                        start_time = Instant::now();
                    }
                    Err(e) => {
                        print!("error nchttp {}", e); // handle OS error on connection-reset)
                        break;
                    }
                }
            }


            loop {
                let sender = worker_to_main_tx.lock().unwrap();
                let received_message: NscriptVar = match main_to_worker_rx.try_recv(){
                    Ok(rmsg) => {
                        rmsg
                    }
                    Err(_)=>{
                        NscriptVar::new("error")
                    }
                };
                if received_message.name == "close"{// when alls handled, mainthread signals close
                    break;
                }
                if received_message.name != "error"{
                    if streamready{
                        match sender.send(NscriptVar::newstring("some",postdata.to_string())){
                            Ok(_)=>{},
                            Err(_)=>{},
                        };
                    }
               }
            }
        });

    }
}
// pub struct NscriptPostHandle{
//     stream: TcpStream,
// }
// impl NscriptPostHandle{
//     pub fn new(streamhandle:TcpStream) -> NscriptPostHandle{
//         NscriptPostHandle{stream:streamhandle}
//     }
// }
pub fn nscriptfn_decode_html_url(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar{
    let mut nvar = NscriptVar::new("decode");
    nvar.stringdata = decode_html_url(&storage.getargstring(&args[0], block));
    nvar
}
pub fn decode_html_url(url: &str) -> String {
    let entities = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&apos;", "'"),
    ];

    let mut decoded = String::new();
    let mut xurl = Nstring::replace(&url, "+", " ");
    xurl = Nstring::replace(&xurl, "%0D", "\n");
    xurl = Nstring::replace(&xurl, "%40", "@");

    let mut iter = xurl.chars().peekable();

    while let Some(ch) = iter.next() {
        if ch == '%' {
            // Check if it's a valid percent-encoded sequence
            if let (Some(h1), Some(h2)) = (iter.next(), iter.next()) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                    if &h1.to_string() != "0" && &h2.to_string() != "0" {
                        decoded.push(byte as char);
                    }
                    continue;
                }
            }
        }

        decoded.push(ch);
    }

    for (entity, replacement) in &entities {
        decoded = decoded.replace(entity, replacement);
    }

    decoded
}
pub fn html_encode(s_txt: &str) -> String {
    let entities: [(u32, &str); 246] = [
        (34, "quot"),
        (38, "amp"),
        (39, "apos"),
        (60, "lt"),
        (62, "gt"),
        (160, "nbsp"),
        (161, "iexcl"),
        (162, "cent"),
        (163, "pound"),
        (164, "curren"),
        (165, "yen"),
        (166, "brvbar"),
        (167, "sect"),
        (168, "uml"),
        (169, "copy"),
        (170, "ordf"),
        (171, "laquo"),
        (172, "not"),
        (173, "shy"),
        (174, "reg"),
        (175, "macr"),
        (176, "deg"),
        (177, "plusmn"),
        (180, "acute"),
        (181, "micro"),
        (182, "para"),
        (183, "middot"),
        (184, "cedil"),
        (186, "ordm"),
        (187, "raquo"),
        (191, "iquest"),
        (192, "Agrave"),
        (193, "Aacute"),
        (194, "Acirc"),
        (195, "Atilde"),
        (196, "Auml"),
        (197, "Aring"),
        (198, "AElig"),
        (199, "Ccedil"),
        (200, "Egrave"),
        (201, "Eacute"),
        (202, "Ecirc"),
        (203, "Euml"),
        (204, "Igrave"),
        (205, "Iacute"),
        (206, "Icirc"),
        (207, "Iuml"),
        (208, "ETH"),
        (209, "Ntilde"),
        (210, "Ograve"),
        (211, "Oacute"),
        (212, "Ocirc"),
        (213, "Otilde"),
        (214, "Ouml"),
        (215, "times"),
        (216, "Oslash"),
        (217, "Ugrave"),
        (218, "Uacute"),
        (219, "Ucirc"),
        (220, "Uuml"),
        (221, "Yacute"),
        (222, "THORN"),
        (223, "szlig"),
        (224, "agrave"),
        (225, "aacute"),
        (226, "acirc"),
        (227, "atilde"),
        (228, "auml"),
        (229, "aring"),
        (230, "aelig"),
        (231, "ccedil"),
        (232, "egrave"),
        (233, "eacute"),
        (234, "ecirc"),
        (235, "euml"),
        (236, "igrave"),
        (237, "iacute"),
        (238, "icirc"),
        (239, "iuml"),
        (240, "eth"),
        (241, "ntilde"),
        (242, "ograve"),
        (243, "oacute"),
        (244, "ocirc"),
        (245, "otilde"),
        (246, "ouml"),
        (247, "divide"),
        (248, "oslash"),
        (249, "ugrave"),
        (250, "uacute"),
        (251, "ucirc"),
        (252, "uuml"),
        (253, "yacute"),
        (254, "thorn"),
        (255, "yuml"),
        (338, "OElig"),
        (339, "oelig"),
        (352, "Scaron"),
        (353, "scaron"),
        (376, "Yuml"),
        (402, "fnof"),
        (710, "circ"),
        (732, "tilde"),
        (913, "Alpha"),
        (914, "Beta"),
        (915, "Gamma"),
        (916, "Delta"),
        (917, "Epsilon"),
        (918, "Zeta"),
        (919, "Eta"),
        (920, "Theta"),
        (921, "Iota"),
        (922, "Kappa"),
        (923, "Lambda"),
        (924, "Mu"),
        (925, "Nu"),
        (926, "Xi"),
        (927, "Omicron"),
        (928, "Pi"),
        (929, "Rho"),
        (931, "Sigma"),
        (932, "Tau"),
        (933, "Upsilon"),
        (934, "Phi"),
        (935, "Chi"),
        (936, "Psi"),
        (937, "Omega"),
        (945, "alpha"),
        (946, "beta"),
        (947, "gamma"),
        (948, "delta"),
        (949, "epsilon"),
        (950, "zeta"),
        (951, "eta"),
        (952, "theta"),
        (953, "iota"),
        (954, "kappa"),
        (955, "lambda"),
        (956, "mu"),
        (957, "nu"),
        (958, "xi"),
        (959, "omicron"),
        (960, "pi"),
        (961, "rho"),
        (962, "sigmaf"),
        (963, "sigma"),
        (964, "tau"),
        (965, "upsilon"),
        (966, "phi"),
        (967, "chi"),
        (968, "psi"),
        (969, "omega"),
        (977, "thetasym"),
        (978, "upsih"),
        (982, "piv"),
        (8194, "ensp"),
        (8195, "emsp"),
        (8201, "thinsp"),
        (8204, "zwnj"),
        (8205, "zwj"),
        (8206, "lrm"),
        (8207, "rlm"),
        (8211, "ndash"),
        (8212, "mdash"),
        (8216, "lsquo"),
        (8217, "rsquo"),
        (8218, "sbquo"),
        (8220, "ldquo"),
        (8221, "rdquo"),
        (8222, "bdquo"),
        (8224, "dagger"),
        (8225, "Dagger"),
        (8226, "bull"),
        (8230, "hellip"),
        (8240, "permil"),
        (8242, "prime"),
        (8243, "Prime"),
        (8249, "lsaquo"),
        (8250, "rsaquo"),
        (8254, "oline"),
        (8260, "frasl"),
        (8364, "euro"),
        (8465, "image"),
        (8472, "weierp"),
        (8476, "real"),
        (8482, "trade"),
        (8501, "alefsym"),
        (8592, "larr"),
        (8593, "uarr"),
        (8594, "rarr"),
        (8595, "darr"),
        (8596, "harr"),
        (8629, "crarr"),
        (8656, "lArr"),
        (8657, "uArr"),
        (8658, "rArr"),
        (8659, "dArr"),
        (8660, "hArr"),
        (8704, "forall"),
        (8706, "part"),
        (8707, "exist"),
        (8709, "empty"),
        (8711, "nabla"),
        (8712, "isin"),
        (8713, "notin"),
        (8715, "ni"),
        (8719, "prod"),
        (8721, "sum"),
        (8722, "minus"),
        (8727, "lowast"),
        (8730, "radic"),
        (8733, "prop"),
        (8734, "infin"),
        (8736, "ang"),
        (8743, "and"),
        (8744, "or"),
        (8745, "cap"),
        (8746, "cup"),
        (8747, "int"),
        (8764, "sim"),
        (8773, "cong"),
        (8776, "asymp"),
        (8800, "ne"),
        (8801, "equiv"),
        (8804, "le"),
        (8805, "ge"),
        (8834, "sub"),
        (8835, "sup"),
        (8836, "nsub"),
        (8838, "sube"),
        (8839, "supe"),
        (8853, "oplus"),
        (8855, "otimes"),
        (8869, "perp"),
        (8901, "sdot"),
        (8968, "lceil"),
        (8969, "rceil"),
        (8970, "lfloor"),
        (8971, "rfloor"),
        (9001, "lang"),
        (9002, "rang"),
        (9674, "loz"),
        (9824, "spades"),
        (9827, "clubs"),
        (9829, "hearts"),
        (9830, "diams"),
    ];

    let mut s_txt_encoded = String::new();
    for c in s_txt.chars() {
        let entity = entities.iter().find(|&&(code, _)| code == c as u32);
        if let Some((_, name)) = entity {
            s_txt_encoded.push_str(&format!("&{};", name));
        } else {
            s_txt_encoded.push(c);
        }
    }
    s_txt_encoded
}


/// mapped as httpget()
pub fn nscriptfn_get_http_content(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) -> NscriptVar {
    let mut var = NscriptVar::new("httpget");
    if args.len() < 3 {
        return var;
    }
    let hstring = storage.getargstring(&args[0], block);
    let portstring = storage.getargstring(&args[1], block);
    let pathstring = storage.getargstring(&args[2], block);
    let host = hstring.as_str();
    let port = portstring.as_str().parse::<u16>().unwrap_or(0);
    let path = &pathstring;
    let mut stream : TcpStream;
    if let Ok(streamtry) = TcpStream::connect((host, port)){
        stream = streamtry;
    }
    else{
        return var;
    }
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
        path, port, host);
    if let Err(_) = stream.write_all(request.as_bytes()){
        return var;
    };
    let string: String;// "".to_string();
    let mut response = Vec::new();
    if let Err(_) = stream.read_to_end(&mut response){
        return var;
    };
    // Find the position of the double CRLF (indicating the end of headers)
    if let Some(index) = response.windows(4).position(|window| window == b"\r\n\r\n") {
        // Skip the headers and extract the content
        let content = response.split_off(index + 4);
        string = String::from_utf8(content).unwrap_or("".to_string());
    }else{
        string = String::from_utf8(response).unwrap_or("".to_string());
    }
    var.stringdata = string;
    var
}
fn nscript_usize(intasstr: &str)-> usize{
    let selected = match intasstr.parse::<usize>(){
        Ok(res) =>{
            res
        }
        Err(_) =>{
            0
        }
    };
    return selected;
}
