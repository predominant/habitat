fn main() { inner::main() }

mod inner {
    use std::{env,
              fs,
              path::Path};

    pub fn main() {
        let src = match env::var("SSL_CERT_FILE") {
            Ok(s) => s,
            Err(_) => panic!("Missing SSL_CERT_FILE environment variable.")
        };
        let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("cacert.pem");
        if !dst.exists() {
            fs::copy(src, dst).unwrap();
        }
    }
}
