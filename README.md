# Bitfinex Funding Bot

Implement a basic strategy for lending USD on bitfinex.  
The bot fetches all the 15m candles of the last 24h and try to lend for 120 days 1% below the 3rd highest candle of the day.  
If the rate is too low ( <0.05% / day), only lend for 2 days.  
The bot lend in batch of maximum 10% of the total lending balance (or the minimum required to lend by bitfinex) to avoid any prolonged period without any lending activity.

## Configuration
| Key            | Description                                             | Default |
|----------------|---------------------------------------------------------|---------|
| API_KEY        | Your bitfinex api key                                   | -       |
| API_SECRET     | Your bitfinex api secret                                | -       |
| THRESHOLD_RATE | Threshold rate for lending period of 2 days or 120 days | 0.0005  |
| CANDLE_LIMIT   | Number of 15m candles to fetch                          | 97      |

## Run with Docker

Build docker image:

`docker build -t finex-funding-bot .`

Run container:

`docker run -itd -e API_KEY=<your-api-key> -e SECRET_KEY=<your-api-secret> finex-funding-bot`

**TODO 1**: work for any currency
