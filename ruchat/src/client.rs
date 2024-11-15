use std::collections::VecDeque;
use std::io::{self, BufReader, ErrorKind, Read, Write};
use std::ops::Index;

use mio::{net::TcpStream, Token};
use mio::{Events, Interest, Poll};


const MAX_BUFSIZE : usize = 1024 * 1024;
pub struct Client {

    pub token:Token,
    pub stream:TcpStream,

    proc_buffer:String,
    proc_deque: VecDeque<String>
}

//함수 구현시
impl Client {
    pub fn new(token:Token , stream :TcpStream)-> Client {
        println!("Client new  token {}" , token.0);
        return Client{
            token : token , 
            stream : stream, 
            proc_buffer : String::new(),
            proc_deque : VecDeque::new()
            
        };
    }

    pub fn read(&mut self) ->Result<usize, ErrorKind>{
        let mut buffer = [0 as u8; 1024];

        loop {               
            match self.stream.read(&mut buffer) 
            {
                Ok(0) => return Err(std::io::ErrorKind::ConnectionReset),
                Ok(len) => {                      
                    let str = String::from_utf8_lossy(&buffer).into_owned();                    
                    self.proc_buffer.push_str(&str);
       
                    if self.proc_buffer.len() > MAX_BUFSIZE {
                        //사이즈 너무 크면 제거
                        return Err(std::io::ErrorKind::InvalidData)
                    }

                    let mut last_line = "".to_owned();
                    let mut lines = self.proc_buffer.lines();
                    //let mut num_line = 0;
                    let num_line = self.proc_buffer.lines().count();
                    let mut cnt_line: usize = 0;
                    
             
                    loop {
                        cnt_line += 1;
                        if let Some(s) = lines.next() {
                           
                            last_line = s.trim().to_owned();
                            if last_line.is_empty() { 
                                break; 
                            };
                            if cnt_line < num_line {
                                self.proc_deque.push_front(last_line.to_owned());
                            }
                        }else{
                            break;
                        }
                    }
                    self.proc_buffer.clear();
                    self.proc_buffer.push_str(&last_line); //엔터값 안들어올경우 처리 
                    //self.proc_buffer.push_str(&self.proc_last);
               
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(0),
                Err(e) => return Err(std::io::ErrorKind::ConnectionAborted)
            }
        }    
    }

    pub fn write(&mut self , str:&String ) ->io::Result<usize>         
    {
        return self.stream.write(str.as_bytes());
    }

    pub fn get_line(&mut self) -> Option<String> {

        return self.proc_deque.pop_front();
    
    }

}