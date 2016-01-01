extern crate mttester;

use mttester::MtManager;
use mttester::MtManagerTrait;


fn main() {

    
    let mut manager = MtManager::new();
    
    manager.set_auth_url("http://www.artselleasy.com/ysxy/api/login".to_string(), "POST".to_string(), "json".to_string())
        .set_left_values("pn".to_string(), "psw".to_string(), "accessToken".to_string())
        .add_account("15281020829".to_string(), "123456".to_string())
        .set_url("http://www.artselleasy.com/ysxy/api/getUserInfo".to_string(), "GET".to_string(), "".to_string())
        .set_seconds(10)
        // .set_threads(10)
        .start();
    
    println!("End.");
}
