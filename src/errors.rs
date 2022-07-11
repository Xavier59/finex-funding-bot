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

#[macro_export]
macro_rules! exit_or_unwrap {
    ( $err_msg: expr, $api_res: ident) => {
        if $api_res.is_err() {
            println!("{}\n{}", $err_msg, $api_res.as_ref().err().unwrap());
            continue;
        }
        let $api_res = $api_res.unwrap();
    };
}
