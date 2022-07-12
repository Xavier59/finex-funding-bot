error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        Internal(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }
    }

    links {
        BitfinexError(bitfinex::errors::Error, bitfinex::errors::ErrorKind);
    }

}