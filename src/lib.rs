extern crate hyper;
extern crate time;
#[macro_use]extern crate prettytable;

mod mtmanager;
pub use mtmanager::MtManager;
pub use mtmanager::MtManagerTrait;

#[test]
fn test_basic() {
    use mtmanager::MtManager;
    use mtmanager::MtManagerTrait;
    
    let mut manager = MtManager::new();
    
    manager.add_url("http://www.baidu.com".to_string(), "GET".to_string())
        .set_seconds(4)
        .set_threads(3)
        .start();
    
    println!("End.");
}
