extern crate mttester;

use mttester::MtManager;
use mttester::MtManagerTrait;


fn main() {

    
    let mut manager = MtManager::new();
    
    manager.add_url("http://www.artselleasy.com/ysxy/share/art.html?artId=1187".to_string(), "GET".to_string())
        .set_seconds(30)
        .set_threads(100)
        .start();
    
    println!("End.");
}
