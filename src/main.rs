//use std::io::{stdout, Write};
use std:: io::Write;

extern crate curl;
extern crate json;
//use json;
use std::string::String;
use curl::easy::Easy;
use std::str;
use std::f32;
use std::vec::Vec;

// Sandbox versions
//const gemini_btcusd_s: &'static str = "https://api.sandbox.gemini.com/v1/pubticker/btcusd";
//const gemini_ethbtc_s: &'static str = "https://api.sandbox.gemini.com/v1/pubticker/ethbtc";
//const gemini_ethusd_s: &'static str = "https://api.sandbox.gemini.com/v1/pubticker/ethusd";
//const gemini_key_s: &'static str = "6NS4aDoyt1ySJ11YAsYH";
//const gemini_secret_s: &'static str = "2fzP7TNxwpx53TLVmTMnGVxXSHBJ";

// Real versions. (use these to make money)
const GEMINI_BTCUSD: &'static str = "https://api.gemini.com/v1/pubticker/btcusd";
const GEMINI_ETHBTC: &'static str = "https://api.gemini.com/v1/pubticker/ethbtc";
const GEMINI_ETHUSD: &'static str = "https://api.gemini.com/v1/pubticker/ethusd";
const GEMINI_KEY: &'static str = "dYwRj7wseEhPfP7pceJ3";
const GEMINI_SECRET: &'static str = "3iDMwgnk3QJNT1686Vitr9xiaR5z";



struct Exchange {
	usd_btc_bid: f32,
    usd_btc_ask: f32,
	btc_eth_bid: f32,
    btc_eth_ask: f32,
	eth_usd_bid: f32,
    eth_usd_ask: f32,
}

fn main() {

    let mut ex1 = Exchange {
        usd_btc_bid: 1.0,
        usd_btc_ask: 1.0,
        btc_eth_bid: 1.0,
    	btc_eth_ask: 1.0,
	    eth_usd_bid: 1.0,
        eth_usd_ask: 1.0,
    };
    ex1.fetch();
    let multiplier = ex1.eval_cycle();
    println!("{}", multiplier);
    ex1.print();

    while true {
        ex1.fetch();
        let r = ex1.eval_cycle();
        println!("{}", r);
    }
}

/*
fn get_ex() -> Exchange {
     Exchange {
        usd_btc: 1.0,
        btc_eth: 1.0,
        eth_usd: 1.0,
     }	
}
*/

impl Exchange {
    pub fn fetch(&mut self){

        let mut u = Vec::new();
        
        {
        let mut easy = Easy::new();
        easy.url(GEMINI_BTCUSD).unwrap();
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {Ok(u.write(data).unwrap())});
        transfer.perform().unwrap();
        }
       
        {
        let mut res = String::from_utf8_lossy(&mut u);
        let parsed_res = json::parse(&res);
        if parsed_res.is_ok() {
            let parsed = parsed_res.unwrap();
            
            let val = &parsed["bid"];
            if val.is_string(){
                let mut w = Vec::new();
                val.write(&mut w);
                let ln = w.len();
                w.remove(ln-1);
                w.remove(0);
                let writer = String::from_utf8_lossy(&w);
                self.usd_btc_bid = writer.parse().unwrap();
            }

            let val = &parsed["ask"];
            if val.is_string(){
                let mut w = Vec::new();
                val.write(&mut w);
                let ln = w.len();
                w.remove(ln-1);
                w.remove(0);
                let writer = String::from_utf8_lossy(&w);
                self.usd_btc_ask = writer.parse().unwrap();
            }
        }
        }

        drop(u);
        let mut u = Vec::new();

        {
        let mut easy = Easy::new();
        easy.url(GEMINI_ETHBTC).unwrap();
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {Ok(u.write(data).unwrap())});
        transfer.perform().unwrap();
        }

        {
        let mut res = String::from_utf8_lossy(&mut u);
        let parsed_res = json::parse(&res);
        if parsed_res.is_ok() {
            let parsed = parsed_res.unwrap();
            
            let val = &parsed["bid"];
            if val.is_string(){
                let mut w = Vec::new();
                val.write(&mut w);
                let ln = w.len();
                w.remove(ln-1);
                w.remove(0);
                let writer = String::from_utf8_lossy(&w);
                self.btc_eth_bid = writer.parse().unwrap();
            }

            let val = &parsed["ask"];
            if val.is_string(){
                let mut w = Vec::new();
                val.write(&mut w);
                let ln = w.len();
                w.remove(ln-1);
                w.remove(0);
                let writer = String::from_utf8_lossy(&w);
                self.btc_eth_ask = writer.parse().unwrap();
            }
        }
        }

        drop(u);
        let mut u = Vec::new();

        {
        let mut easy = Easy::new();
        easy.url(GEMINI_ETHUSD).unwrap();
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {Ok(u.write(data).unwrap())});
        transfer.perform().unwrap();
        }

        let mut res = String::from_utf8_lossy(&mut u);
        let parsed_res = json::parse(&res);
        if parsed_res.is_ok() {
            let parsed = parsed_res.unwrap();
            
            let val = &parsed["bid"];
            if val.is_string(){
                let mut w = Vec::new();
                val.write(&mut w);
                let ln = w.len();
                w.remove(ln-1);
                w.remove(0);
                let writer = String::from_utf8_lossy(&w);
                let eth: f32 = writer.parse().unwrap();
                self.eth_usd_bid = eth.recip();
            }

            let val = &parsed["ask"];
            if val.is_string(){
                let mut w = Vec::new();
                val.write(&mut w);
                let ln = w.len();
                w.remove(ln-1);
                w.remove(0);
                let writer = String::from_utf8_lossy(&w);
                let eth: f32 = writer.parse().unwrap();
                self.eth_usd_ask = eth.recip();
            }
        }
    }

    pub fn eval_cycle(&mut self) -> f32 {
        self.usd_btc_ask * self.btc_eth_ask * self.eth_usd_bid * 0.99251873437
    }


    pub fn print(&mut self){
        println!("{}", self.usd_btc_bid);
        println!("{}", self.usd_btc_ask);
        println!("{}", self.btc_eth_bid);
        println!("{}", self.btc_eth_ask);
        println!("{}", self.eth_usd_bid);
        println!("{}", self.eth_usd_ask); 
    }
}
