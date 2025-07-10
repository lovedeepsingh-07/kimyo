#[cxx::bridge(namespace = "kimyo")]
mod ffi {
    struct Server {
        address:  String,
        port: u16,
    }

    extern "Rust" {
        fn start(self: &Server) -> String;
    }
}

impl ffi::Server{
    pub fn start(&self) -> String {
        return format!("on the behalf of rust, I would like to say, you address is {:#?} and your port is {:#?}",self.address,self.port.to_string()).to_string();
    }
}
