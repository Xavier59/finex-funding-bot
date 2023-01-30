#[macro_export]
macro_rules! exit_or_unwrap {
    ( $err_msg: expr, $api_res: ident) => {
        if $api_res.is_err() {
            log::error!("{}\n{}", $err_msg, $api_res.as_ref().err().unwrap());
            continue;
        }
        let $api_res = $api_res.unwrap();
    };
}
