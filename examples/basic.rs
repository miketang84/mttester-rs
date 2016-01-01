extern crate mttester;

use mttester::MtManager;
use mttester::MtManagerTrait;


fn main() {

    
    let mut manager = MtManager::new();
    
    manager.add_url("http://www.artselleasy.com/ysxy/api/art/1789".to_string(), "GET".to_string())
        .set_seconds(5)
        .set_threads(10)
        .start();
    
    println!("End.");
}