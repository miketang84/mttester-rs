extern crate mttester;

use mttester::MtManager;
use mttester::MtManagerTrait;
use std::sync::{Arc, Mutex};
use mttester::MtModifierTrait;
use mttester::MtModifier;

fn main() {

    let incr_index = Arc::new(Mutex::new(10));
    
    let mut manager: MtManager<MtModifier> = MtManager::new();
    let c = MtModifier;
    
    manager.set_url("http://www.artselleasy.com/ysxy/api/art/1789".to_string(), "GET".to_string(), "urlencoded".to_string())
        .add_param("foo".to_string(), "bar".to_string())
        .add_closure_param("test".to_string(), Box::new(move || "hello".to_string()) )
        .add_closure_param("pageid".to_string(), Box::new(move || {
            let mut incr = incr_index.lock().unwrap();
            *incr += 1;
            //println!("{}", *incr);
            (*incr).to_string()
        }))
        .add_modifier_param("life".to_string(), c)
        .set_seconds(5)
        .set_threads(5)
        .start();
    
    println!("End.");
}

