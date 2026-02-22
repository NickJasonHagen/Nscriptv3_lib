use crate::*;
use std::{net::UdpSocket };

pub struct NscriptUDP{
    sockets: HashMap<Box<str>,UdpSocket>,
    connections: HashMap<Box<str>,std::net::SocketAddr>,
}
impl NscriptUDP{
   pub fn new()->NscriptUDP{
        NscriptUDP{
            sockets:HashMap::new(),
            connections:HashMap::new(),
        }
    }
    pub fn bind(&mut self,ip:&str, port:&str)->NscriptVar{
        // Create a UDP socket
        let socket = UdpSocket::bind(format!("{}:{}",&ip,&port)).expect("Failed to bind socket");

        // Set the socket to receive packets
        socket.set_nonblocking(true).unwrap();
        self.sockets.insert(port.into(), socket);
        // return the socket object for nscript to work with
        NscriptVar::newstring("r", format!("udp:{}:{}",&ip,&port))
    }
    pub fn listen(&mut self,bindsocket:&str)->NscriptVar{
        if let Some(socket) = self.sockets.get_mut(bindsocket){

            // Receive data from the client
            let mut buf = [0; 1024];
            let (_, src) = socket.recv_from(&mut buf).expect("Failed to receive packet");
            let string = String::from_utf8_lossy(&buf).to_string();
            let addressname = format!("updclient:{}",&src);
            self.connections.insert(addressname.to_string().into(), src);
            // return a nscript vector with [0] for the returning socket and [1] with the data
            return  NscriptVar { name:"r".into(),stringdata:"".to_string() , stringvec:vec!(addressname,string) };
        }

        NscriptVar::new("r")
    }
    pub fn send(&mut self,socketaddr:&str,clientsocket:&str,msg:&str)->NscriptVar{
        if let Some(s) = self.sockets.get(socketaddr){
            if let Some(sock) = self.connections.get(clientsocket){
                if let Ok(e) = s.send_to(msg.as_bytes(),sock){
                    return NscriptVar::newstring("r", format!("{}",e));
                }
            }
        }
        return NscriptVar::newstring("r", "ERROR".to_string());
    }
}
pub fn nscriptfn_udpbind(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    if args.len() > 1{
        let  ip = storage.getargstring(args[0], block);
        let  port = storage.getargstring(args[0], block);
        return storage.udp.bind(&ip, &port);

    }else{
        print("error: udpbind(IP,PORT) // invalid arguments given ","r");
    }
    return NscriptVar::new("res");
}
pub fn nscriptfn_udplisten(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
        let  socketid = storage.getargstring(args[0], block);
        return storage.udp.listen(&socketid);
}
pub fn nscriptfn_udpsend(args:&Vec<&str>,block :&mut NscriptCodeBlock , storage :&mut NscriptStorage) ->NscriptVar{
    let socketid = storage.getargstring(args[0], block);
    let clientsocket = storage.getargstring(args[1], block);
    let msg = storage.getargstring(args[2], block);
    return storage.udp.send(&socketid,&clientsocket,&msg);
}
