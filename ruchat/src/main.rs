pub mod client;

use std::collections::HashMap;
use std::error::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::str::FromStr;

use client::Client;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

use std::{
    io::{prelude::*, BufReader}
    //net::{TcpListener, TcpStream},
};


// 라이프타입을 이해하지 못해 이해 가능한 수준만 구현


// 아래 참고
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// https://crates.io/crates/mio


//fn barrow_fn(mut i:&mut str){
//    i = &mut "aa";
//}

fn main()  -> Result<(), Box<dyn Error>> {

    //let i = "aaa".to_owned();
    //barrow_fn(&i);

    //let b = i;

    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(1024);
    let addr = "0.0.0.0:7878".parse()?;
    let mut server = TcpListener::bind(addr)?;


    let mut g_counter: usize = 0;
    let mut g_clients: HashMap<Token, Client> = HashMap::new();

   
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, Token(0), Interest::READABLE)?;


    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {

            match event.token() {
                Token(0) => {
                    loop {
                        match server.accept() {
                            Ok((mut stream,socketaddr)) => {
                                
                                
                                //클라이언트 토큰 생성
                                // counter 는 64테라이고 무제한이라는 가정
                                g_counter += 1;
                                let client_token: Token = Token(g_counter);

                                println!("Connection:!! token {}" , client_token.0 ); 

                                poll.registry()
                                .register(&mut stream, client_token, Interest::READABLE | Interest::WRITABLE)?;

                                // client_token 가 빌림이 아닌데 왜 문제 없는지를 모르겠다.! 
                                let client = Client::new(client_token,stream);

                                g_clients.insert(client_token , client );
                            },
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock =>
                                break,
                            Err(e) => 
                                panic!("Unexpected error: {}", e)
                        }   
                    }                       
                },
                token => {
                    
                    match handle_clients(&token , &mut g_clients) {
                        Ok(_) => {

                        },
                        Err(kind) => {                      
                            println!("Disconnect:!! token {}" , token.0 );       
                            g_clients.remove(&token);
                        }
                        //poll.registry().deregister(source)
                    }
 
                    // Socket associated with token is ready for reading data from it
                }
            }
        }
    }    
/*
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
    */
}


//let mut g_counter: usize = 0;
//let mut g_clients: HashMap<Token, TcpStream> = HashMap::new();

fn handle_clients( token: &Token ,  clients : &mut HashMap<Token, Client>  ) -> Result<usize,ErrorKind>{

    let mut vclient_commands :Vec<String> = Vec::new();
    

    if let Some(client) = clients.get_mut(token) 
    {
        match client.read() {
            Ok(size)   => {
                loop{
                    if let Some(line) = client.get_line() {
                        println!(" {} " , line );

                        //for ( k , v ) in clients.iter_mut() {
                        //}
                        vclient_commands.push(line);                        
                    }else{
                        break;
                    }
                };
                
            }
            ,
            Err(err ) => return Err(err)
        };
        
        for line in vclient_commands.iter() {
            for ( target_token , target_client ) in clients.iter_mut() {
                if target_token != token {
            
                    let new_line = "\r\n".to_string();
                    let send_str = line.clone() + &new_line ;
                   
                    target_client.write( &send_str );
                }
            }
        }

        return Ok(vclient_commands.len()); 
    }else{
            return Err(ErrorKind::NotFound);
    }
}