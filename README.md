# Bitfinex Funding Bot

Implement a basic strategy for lending USD on bitfinex.  
The bot fetches all the 15m candles of the last 24h and try to lend for 120 days 1% below the 3rd highest candle of the day.  
If the rate is too low ( <0.05% / day), only lend for 2 days.  
The bot lend in batch of maximum 10% of the total lending balance (or the minimum required to lend by bitfinex) to avoid any prolonged period without any lending activity.

## Configuration

- Generate Bitfinex api keys for your account with "*Get funding statuses and info*", "*Offer, cancel and close funding*" and "*Get wallet balances and addresses*" rights. Then create a copy of `.env.example` called `.env` and fill `API_KEY` and `API_SECRET`.
- Currently supports `USD`, `BTC` and `SOL` but more can easily be added writing to the [configuration file](https://github.com/Xavier59/finex-funding-bot/blob/master/src/consts.rs)
## Run with Docker

1) Build docker image:

`docker build -t finex-funding-bot .`

2) Run container:

`docker run -itd -e API_KEY=<your-api-key> -e SECRET_KEY=<your-api-secret> finex-funding-bot`