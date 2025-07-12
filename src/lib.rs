#[cxx::bridge(namespace = "kimyo")]
mod ffi {
    struct Server {
        port: u16,
    }

    extern "Rust" {
        fn start(self: &Server);
    }
}

impl ffi::Server{
    pub fn start(&self) {
        println!("Starting server...");
    }
}
