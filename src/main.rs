
struct Exchange {
	usd_btc: f32,
	btc_eth: f32,
	eth_usd: f32,
}

fn main() {
    println!("Hello, world!");

    let ex1 = Exchange {
        usd_btc: 1.0,
	btc_eth: 1.0,
	eth_usd: 1.0,
    };
}

fn getEx() -> Exchange {
     Exchange {
        usd_btc: 1.0,
        btc_eth: 1.0,
        eth_usd: 1.0,
     }	
}

impl Exchange {
    pub fn fetch(&mut self){
        
    }



}
