use nestle::{core::Nestle, derives::Nestle};

#[derive(Debug, Nestle)]
#[nestle(width = 64)]
#[repr(i16)]
enum Instrument {
    Spot(CurrencyPair) = 1,
    Options(Options) = 2,
    PerpetualFutures(CurrencyPair) = 3,
}

#[derive(Debug, Nestle)]
#[nestle(width = 48)]
struct Options {
    expiry: u16,
    pair: CurrencyPair,
}

#[derive(Debug, Nestle)]
#[nestle(width = 32)]
struct CurrencyPair {
    base: Currency,
    quote: Currency,
}

#[derive(Debug, Nestle)]
#[repr(i16)]
enum Currency {
    Btc = 1,
    Eth = 2,
    Doge = 3,
    Usd = 4,
}

fn main() {
    println!("Crypto instrument Ids and Rust types:\n");
    let spot = Instrument::Spot(CurrencyPair {
        base: Currency::Btc,
        quote: Currency::Usd,
    });
    println!("{} -> {:?}", spot.encode().unwrap(), spot);
    let option = Instrument::Options(Options {
        expiry: 365,
        pair: CurrencyPair {
            base: Currency::Eth,
            quote: Currency::Usd,
        },
    });
    println!("{} -> {:?}", option.encode().unwrap(), option);
    let perpetual = Instrument::PerpetualFutures(CurrencyPair {
        base: Currency::Btc,
        quote: Currency::Usd,
    });
    println!("{} -> {:?}", perpetual.encode().unwrap(), perpetual);
}
