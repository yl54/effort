// Http utility definitions. 
use std::net::{TcpStream};

use httparse::{Error as HttpError, Request, Status, EMPTY_HEADER};

use server::pool::responder::Callback;

#[derive(Clone)]
pub struct HHeader {
    pub key: String,
    pub value: Vec<u8>,
}

impl HHeader {
    pub fn new(k: String, v: Vec<u8>) -> HHeader {
        return HHeader{
            key: k,
            value: v,
        }
    }
}

/*
    HRequests are wrappers for Http Request information.
    TODO: Make the fields private
*/
pub struct HRequest {
    pub method: Option<String>,
    pub path: Option<String>,
    pub version: Option<u8>,
    pub headers: Vec<HHeader>,
    // pub body: Option<String>, // TODO: Get body working
    pub stream: TcpStream,
}

impl HRequest {
    // convert takes an TcpStream and creates an HRequest.
    pub fn convert(req: Request, stream: TcpStream) -> HRequest {
        // Get method
        let method = match req.method {
            Some(m) => Some(m.to_string()),
            None => None,
        };

        // Get path
        let path = match req.path {
            Some(m) => Some(m.to_string()),
            None => None,
        };

        // Get headers
        let mut headers = vec![];
        for header in req.headers.iter() {
            headers.push(HHeader::new(header.name.to_string(), header.value.to_vec()));
        }

        // Return the request
        return HRequest{
            method: method,
            path: path,
            version: req.version,
            headers: headers,
            stream: stream,
        };
    }
}
