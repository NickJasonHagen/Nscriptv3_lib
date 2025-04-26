use crate::*;
pub struct NscriptTcp{
    pub streammap: HashMap<String,TcpStream>,

    pub listenermap: HashMap<String,TcpListener>,
    pub streamidcounter: u64,

    pub listeneridcounter: u64,
}

impl NscriptTcp{
    pub fn new() -> NscriptTcp {
        NscriptTcp{
            streammap: HashMap::new(),
            listenermap: HashMap::new(),
            streamidcounter: 1,
            listeneridcounter: 1,
        }
    }

    pub fn listener(&mut self,ip:&str,port:&str) -> String{
        let  listener: TcpListener;

        listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
        println!("Server started at http://{}:{}",  ip, port);


        #[cfg(windows)]
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        #[cfg(not(windows))]
        //let newid = String::new();
        match listener.set_nonblocking(true){
            Ok(_) => {
                 self.listeneridcounter = self.listeneridcounter + 1;
                let newid = "nc_listener_".to_owned() + &self.listeneridcounter.to_string();
                self.listenermap.insert(newid.clone(), listener);
                eprintln!("socked ok!");
                return newid;

            }
            Err(e) => {
                eprintln!("Error on NscriptTcp listener, cant set nonblocking tcp code:{}",e);
            }
        };
       return "".to_owned();
    }
    pub fn accept(&mut self,id: &str)->String{
        let g = self.listenermap.get_key_value(id);
        let listener: &TcpListener;
        let  newid = String::new();
        match g {
            None => {
                eprintln!("listener error, does not exist!id={}",id);
                return newid;
            },
            Some((_i, k)) => listener = k,
        }
        match listener.accept() { // add the stream to the map
            Ok((stream, _)) => {
                self.streamidcounter = self.streamidcounter + 1;
                let newid = "nc_stream_".to_owned() + &self.streamidcounter.to_string();
                self.streammap.insert(newid.clone(), stream);
                eprintln!("stream ok!");
                return newid;

            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No incoming connections yet,
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
        return "".to_owned();
    }
    pub fn connect(&mut self,ip: &str,ports:&str)->String{
        //let stream: TcpStream;
        let mut newid = String::new();
        let mut port: u16 = 8888;
        if let Ok(port) = ports.to_owned().parse::<u16>() {
            println!("Parsed port number: {}", port);
            // Use the port number (port) here in your code
        } else {
            port = 8888;
            println!("Failed to parse port number: using port:8888");
            // Handle the case when parsing fails
        }
        match TcpStream::connect((ip, port)){
            Ok(e) => {
                self.streamidcounter = self.streamidcounter + 1;
                newid = "nc_stream_".to_owned() + &self.streamidcounter.to_string();
                self.streammap.insert(newid.clone(), e);


            }
            Err(r) => {
                println!("Error:{}",r);
            }
        };
        newid
    }
        pub fn disconnect(&mut self, id: &str) -> String {
        if id.starts_with("nc_stream_") {
            if let Some((_i, stream)) = self.streammap.remove_entry(id) {
                // Close the stream if needed

                stream.shutdown(Shutdown::Both).ok();
                true.to_string()
            } else {
                false.to_string()
            }
        } else if id.starts_with("nc_listener_") {
            self.listenermap.remove(id).is_some().to_string()
        } else {
            false.to_string()
        }
    }
    pub fn send(&mut self,id:&str,msg: &str)->String{
        let g = self.streammap.get_key_value(id);
        let mut stream: &TcpStream;
        match g {
            None => {
                println!("stream error, does not exist!id={}",id);
                return "error".to_owned();

            },
            Some((_i, k)) => {
                stream = k;
            },


        }
        match stream.write(msg.as_bytes()){
            Ok(_) => {return "ok".to_owned();},
            Err(_) => {return "error".to_string();},
        }

    }
    pub fn receive(&mut self,id:&str)->String{
        let g = self.streammap.get_key_value(id);
        let mut stream: &TcpStream;
        let mut response = String::new();
        match g {
            None => {
                println!("stream error, does not exist!id={}",id);
                return "".to_owned();

            },
            Some((_i, k)) => {
                stream = k;
            },


        }
        let mut buffer = [0; 1024];
        //let timeout = Duration::from_millis(0);
        //stream.set_read_timeout(Some(timeout));
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {

                //println!("\nbytesRead!{}\n",bytes_read);
                let ispacket = String::from_utf8_lossy(&buffer[0..bytes_read]);
                if ispacket == "[/nc]"{}
                response = "".to_owned() + &ispacket;

                // procceed the connection.

            }
            Err(e) => {
                println!("receive error:{}",e);
                // handle OS error on connection-reset

            }
        }
            response
    }
}

