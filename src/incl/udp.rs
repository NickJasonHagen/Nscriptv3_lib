use crate::*;
use std::{net::UdpSocket };

pub struct NscriptUDP{
    sockets: HashMap<Box<str>,UdpSocket>,
    connections: HashMap<Box<str>,std::net::SocketAddr>,
    buffers: HashMap<Box<str>,String>
}
impl NscriptUDP{
   pub fn new()->NscriptUDP{
        NscriptUDP{
            sockets:HashMap::new(),
            connections:HashMap::new(),
            buffers:HashMap::new(),
        }
    }
    pub fn create(&mut self,port:String){
        // Create a UDP socket
        let socket = UdpSocket::bind(format!("0.0.0.0:{}",&port)).expect("Failed to bind socket");

        // Set the socket to receive packets
        socket.set_nonblocking(true).unwrap();
        self.sockets.insert(port.into(), socket);
    }
    pub fn listen(&mut self,port:&str){
        if let Some(socket) = self.sockets.get_mut(port){

            // Receive data from the client
            let mut buf = [0; 1024];
            let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive packet");
            self.connections.insert(format!("r{}",&port).into(), src);
            self.buffers.insert(format!("msg{}",&port).into(), format!("{}",amt));
            // Handle the received data
            //println!("Received {} bytes from {}", amt, src);

            // Send a response back to the client
            //let mut resp = [0; 1024];
            // ...
            //socket.send_to(&resp[..], src).unwrap();
        }
    }
}
