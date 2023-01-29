#[macro_use]
extern crate error_chain;

pub mod consts;
pub mod errors;
pub mod macros;

use crate::errors::*;
use bitfinex::{api::Bitfinex, candles::CandleHistoryParams, funding::FundingOfferParams};
use consts::*;
use dotenv::dotenv;
use env_logger;
use log::*;
use std::env;
use std::{process, thread, time};

#[derive(Clone)]
struct CandleParams {
    tf: String,
    n: i32,
    limit: i32,
    period: i32,
}

impl Default for CandleParams {
    fn default() -> CandleParams {
        return CandleParams {
            tf: "15m".to_string(),
            n: 10,
            limit: 96 + 1,
            period: 2,
        };
    }
}

impl From<CandleParams> for CandleHistoryParams {
    fn from(cp: CandleParams) -> Self {
        let mut chp = Self::default();
        chp.limit = Some(cp.limit);
        chp.period = Some(cp.period);
        chp
    }
}

/// Return the "High" value of the {n}th highest candle value in the latest {n} candles of timeframe {tf}
fn get_nth_highest_candle(api: &Bitfinex, currency: &String, params: CandleParams) -> Result<f64> {
    if params.n > params.limit {
        bail!("n cannot be greater than the limit");
    }
    let mut candles = api.candles.history(
        format! {"f{}", currency},
        params.tf.to_string(),
        &(params.clone().into()),
    )?;
    if candles.len() <= params.n as usize {
        bail!("Unexpected error while fetching the total amount of candle. Limit too big ?");
    }
    candles.sort_by(|a, b| a.high.partial_cmp(&b.high).unwrap());
    Ok(candles[candles.len() - params.n as usize].high)
}

fn get_balance(api: &Bitfinex, currency: String) -> Result<(f64, f64)> {
    let wallets = api.account.get_wallets()?;
    let f_wallet = wallets
        .into_iter()
        .find(|x| x.currency == currency && x.wallet_type == "funding");
    if f_wallet.is_none() {
        bail!("No funding balance found !");
    }

    Ok((
        f_wallet.as_ref().unwrap().balance_available.unwrap(),
        f_wallet.as_ref().unwrap().balance,
    ))
}

fn validate_env() -> (String, String) {
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("Missing environment variable API_KEY.");
            process::exit(1);
        }
    };

    let secret_key = match env::var("SECRET_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("Missing environment variable SECRET_KEY.");
            process::exit(1);
        }
    };

    (api_key, secret_key)
}

fn main_loop(currency: &String) {
    let (api_key, secret_key) = validate_env();
    let api = Bitfinex::new(Some(api_key), Some(secret_key));

    let minimum_for_currency = LIMIT_PER_CURRENCY.get(currency).unwrap();

    let mut first_loop = true;

    loop {
        if !first_loop {
            thread::sleep(time::Duration::from_secs(60));
        }

        first_loop = false;

        let balance = get_balance(&api, currency.clone());
        exit_or_unwrap!("Unable to fetch balance.", balance);

        let (avail, total) = balance;

        // we also need to add what is already being offered
        let offers = api.funding.get_active_offers(format! {"f{}", currency});
        exit_or_unwrap!("Unable to fetch offers.", offers);

        // this should never happen, max 1 offer at a time !
        if offers.len() > 1 {
            warn!("Detected multiple available offers! It looks like there was some manual interference, please don't do that! Removing them...");
            let _cancel_all = api.funding.cancel_all_funding_offers(currency.clone());
            exit_or_unwrap!("Unable to cancel all offers", _cancel_all);
        }

        let on_offer: f64 = if offers.len() == 1 {
            offers[0].amount
        } else {
            0.0
        };

        let ratio = (avail + on_offer) / total;

        if avail + on_offer < minimum_for_currency + 1.0 {
            info!(
                "Only {} available but minimum is {}, waiting for more availabilities ...",
                avail + on_offer,
                minimum_for_currency
            );
            continue;
        }

        info!(
            "{}% (${}) of the funds are available and could be lended",
            (ratio * 100.0).to_string(),
            (avail + on_offer).to_string()
        );

        let mut nth15m: Option<Result<f64>> = None;
        let mut period = 0;

        if ratio < 0.5 {
            let mut candle_params = CandleParams::default();
            candle_params.n = 3;
            nth15m = Some(get_nth_highest_candle(&api, currency, candle_params));
            period = 120;
        }

        if ratio >= 0.5
            || (nth15m.is_some()
                && nth15m.as_ref().unwrap().is_ok()
                && *nth15m.as_ref().unwrap().as_ref().unwrap() < 0.0005)
        {
            let mut candle_params = CandleParams::default();
            candle_params.n = 10;
            nth15m = Some(get_nth_highest_candle(&api, currency, candle_params));
            period = 2;
        }

        let nth15m = nth15m.unwrap();
        exit_or_unwrap!("Error fetching candle history!", nth15m);

        let rate = nth15m * 0.99;
        if on_offer > 0.0 && (rate * 1.01 < offers[0].rate || rate * 0.99 > offers[0].rate) {
            let _cancel_all = api.funding.cancel_all_funding_offers(currency.to_string());
            exit_or_unwrap!("Unable to cancel all offers.", _cancel_all);
        } else if on_offer > 0.0 {
            info!("Set offer is good, letting it there and going to sleep.");
            continue;
        }

        // support for wallet below 10x minimum
        // as the bot otherwise try to lend out 10% of the amount and it's below minimum
        let amount: f64 = if total < minimum_for_currency * 10.0 {
            minimum_for_currency + 1.0
        } else {
            f64::min(avail + on_offer, total * 0.1)
        };

        info!(
            "Posted f{} offer for {} at {}% - {} days",
            currency,
            (amount - 1.0).to_string(),
            (rate * 100.0).to_string(),
            period
        );

        let funding_offer = FundingOfferParams {
            t: "LIMIT".to_string(),
            symbol: format!("f{}", currency),
            amount: (amount - 1.0).to_string(), // prevent imprecision problems
            rate: rate.to_string(),
            period,
        };

        let _funding_offer = api.funding.submit_funding_offer(funding_offer);
        exit_or_unwrap!("Unable to post funding offer", _funding_offer);
    }
}

fn help() {
    println!(
        "usage:
    finex-funding-bot <currency>
    Running funding bot for the given currency"
    );
}

fn main() {
    let mut builder = env_logger::builder();
    builder.filter_level(LevelFilter::Info);
    builder.format_timestamp_secs();
    builder.init();
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help();
            error!("No currency provided");
        }
        2 => {
            let currency = &args[1];
            main_loop(currency)
        }
        3 => {
            let currency = &args[2];
            main_loop(currency)
        }
        _ => {
            help();
        }
    }
}
