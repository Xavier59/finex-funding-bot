# Bitfinex Funding Bot

Implement a basic strategy for lending USD on bitfinex.  
The bot fetches all the 15m candles of the last 24h and try to lend for 120 days 1% below the 3rd highest candle of the day.  
If the rate is too low ( <0.05% / day), only lend for 2 days.  
The bot lend in batch of maximum 10% of the total lending balance to avoid any prolonged period without any lending activity.