extern crate mttester;
extern crate rustc_serialize;
#[macro_use]extern crate prettytable;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use rustc_serialize::json::Json;


use mttester::MtManager;
use mttester::MtManagerTrait;
use std::sync::{Arc, Mutex};
use mttester::MtModifierTrait;
use mttester::MtModifier;
use std::thread;
use std::cmp::Ord;
use std::collections::HashMap;


#[derive(Clone, Default)]
struct Custom;

impl MtModifierTrait for Custom {
    fn before(&self, index: i64) -> String {
        
        index.to_string()
    }
    fn after(&self, index: i64, res: &String) -> String {
        //println!("{}", res);
        
        
        let ret_data = Json::from_str(res).unwrap();
        
        let bid_list = ret_data.find("content").expect("err 1").as_array().expect("err 2");
        let mut result = vec![];
        for bid in bid_list {
            //println!("{:?}", bid);
            let item_id = bid.find_path(&["item", "id"]).expect("err 3").as_u64().expect("err 3.1");
            let item_name = bid.find_path(&["item", "name"]).expect("err 4").as_string().expect("err 4.1");
            let bid_person = bid.find_path(&["buyer", "loginName"]).expect("err 6").as_string().expect("err 6.1");
            let bid_person = bid.find_path(&["buyer", "name"]).expect("err 8").as_string().expect("err 8.1");
            let bid_price =  bid.find("price").expect("err 5").as_f64().expect("err 5.1");
            let bid_date =  bid.find("date").expect("err 7").as_u64().expect("err 7.1");
            
            //println!("{} {} {} {}", item_id, item_name, bid_price, bid_person);
            result.push((item_id, item_name, bid_person, bid_price as u64, bid_date/1000));
        }
        
        // let mut grouped: HashMap<u64, Vec<(u64, &str, &str, u64, u64)>> = HashMap::new();
        // for item in result {
        //     if let Some(mut val) = grouped.get_mut(&item.0) {
        //         val.push(item);
        //     }
        //     else {
        //         grouped.insert(item.0, vec![item]);
        //     }
        // }
        
        // for (key, mut val) in &mut grouped {
        //     val.sort_by(|a, b| (b.3).cmp(&a.3));
            
        //     println!("{:#?}", val);
        // }
        
        
        result.sort();
        result.sort_by(|a, b| (b.3).cmp(&a.3));
        
        let mut table = Table::new();
        for v in result {
            table.add_row(row![v.0, v.1, v.2, v.3, v.4]);
        }
        table.printstd();
        
        
        thread::sleep_ms(10000);
        "".to_string()
    }
}

fn main() {

    
    let mut manager: MtManager<Custom> = MtManager::new();
    let c = Custom;
    
    // 748
    
    manager.set_auth_url("http://www.artselleasy.com/ysxy/api/login".to_string(), "POST".to_string(), "json".to_string())
        .set_left_values("pn".to_string(), "psw".to_string(), "accessToken".to_string())
        .add_account("13900000000".to_string(), "111111".to_string())

        
        .set_url("http://www.artselleasy.com/ysxy/api/procurementBidList".to_string(), "GET".to_string(), "".to_string())
        .add_param("buyerOrSeller".to_string(), "0".to_string())
        .add_modifier_param("_".to_string(), c)
        .set_seconds(12000)
        .start();
    
    println!("End.");
}

