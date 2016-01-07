extern crate hyper;
extern crate time;
#[macro_use]extern crate prettytable;
extern crate url;
extern crate rustc_serialize;
extern crate jsonway;

mod mtmanager;
pub use mtmanager::MtManager;
pub use mtmanager::MtManagerTrait;
pub use mtmanager::MtModifier;
pub use mtmanager::MtModifierTrait;

#[test]
fn test_basic() {
    use mtmanager::MtManager;
    use mtmanager::MtManagerTrait;
    
    let mut manager = MtManager::new();
    
    manager.set_url("http://www.baidu.com".to_string(), "GET".to_string(), "".to_string())
        .set_seconds(4)
        .set_threads(3)
        .start();
    
    println!("End.");
}
