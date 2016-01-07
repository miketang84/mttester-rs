extern crate mttester;


use mttester::MtManager;
use mttester::MtManagerTrait;
use std::sync::{Arc, Mutex};
use mttester::MtModifierTrait;
use mttester::MtModifier;

use std::thread;

#[derive(Clone, Default)]
struct Custom;

impl MtModifierTrait for Custom {
    fn trans(&self, index: i64) -> String {
        thread::sleep_ms(0);
        (2300+index).to_string()
    }
}

fn main() {

    let incr_index = Arc::new(Mutex::new(10));
    
    let mut manager: MtManager<Custom> = MtManager::new();
    let c = Custom;
    
    // 748
    
    manager.set_auth_url("http://www.artselleasy.com/ysxy/api/login".to_string(), "POST".to_string(), "json".to_string())
        .set_left_values("pn".to_string(), "psw".to_string(), "accessToken".to_string())
        .add_account("xxxx".to_string(), "pwd".to_string())

        
        .set_url("http://www.artselleasy.com/ysxy/api/bid".to_string(), "POST".to_string(), "json".to_string())
        .add_param("auctionId".to_string(), "748".to_string())
        .add_modifier_param("price".to_string(), c)
        .set_seconds(30)
        .start();
    
    println!("End.");
}

