
use std::path::Path;
use std::thread;
use hyper::Client;

#[derive(Debug, Default)]
pub struct MtManager {
    // time to test, unit is second
    time_seconds: i64,
    // test (url, method)
    url: (String, String),
    // login, auth (url, method)
    auth_url: (String, String),
    // how many threads to test
    threads: i64,
    // how many threads per account use to test
    // total threads number is threads_per_account*accounts.len()
    threads_per_account: i64,
    // the result output to this path file
    output_file: Option<Path>,
    
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    // accounts to simulate
    // <account_name, password>
    accounts: Vec<(String, String)>,
    
    req_content_type: String,
    need_auth: bool
    
}

pub trait MtManagerTrait {
    pub fn set_auth_url(&self, url: String, method: String) -> &self;
    
    pub fn set_url(&self, url: String, method: String) -> &self;
    
    pub fn set_seconds(s: i64) -> &self ;
    pub fn set_threads(s: i64) -> &self ;
    pub fn set_threads_per_account(s: i64) -> &self ;
    pub fn add_account(s: i64) -> &self ;
    pub fn add_header(s: i64) -> &self ;
    pub fn add_param(s: i64) -> &self ;
    // set the request content data type, default is www-form-urlencoded, you can set "urlencoded", or "json" now
    pub fn set_param_type(ptype: String) -> &self ;
    pub fn output_file(path: String) -> &self ;
    
    
    pub fn start(&self);
}


impl MtManager {
    
    pub fn new() -> MtManager {
        Default::default();
    }
}


impl MtManagerTrait for MtManager {
    pub fn set_auth_url(&mut self, url: String, method: String) -> &mut Self {
        self.auth_url = (url, method);
        self
    }
    
    pub fn set_url(&mut self, url: String, method: String) -> &mut Self {
        self.url = (url, method);
        self
    }
    
    pub fn set_seconds(&mut self, s: i64) -> &mut Self {
        self.time_seconds = s;
        self
    }
    
    pub fn set_threads(&mut self, n: i64) -> &mut Self {
        self.threads = n;
        self
    }
    
    pub fn set_threads_per_account(&mut self, n: i64) -> &mut Self {
        self.threads_per_account = n;
        self
    }
    
    pub fn add_account(&mut self, account: String, password: String) -> &mut Self {
        self.accounts.push((account, password));
        self
    }
    
    pub fn add_header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn add_param(&mut self, key: String, value: String) -> &mut Self {
        self.params.insert(key, value);
        self
    }
    
    // set the request content data type, default is www-form-urlencoded, you can set "urlencoded", or "json" now
    pub fn set_param_type(&mut self, ctype: String) -> &mut Self {
        self.req_content_type = ctype;
        self
    }
    
    pub fn output_file(&mut self, path: String) -> &mut Self {
        // create path
        let path_obj = Path::new(path);
        self.output_file = Some(path_obj);
        self
    }
    
    
    pub fn start(&mut self) {
        // here now, self has been filled enough fileds to start with
        self.need_auth = if self.accounts.len() == 0 {
            false
        }
        else {
            true
        }
        
        // consider no auth first
        for i in range(0..self.threads) {
            // prepare bindings and channel
            let (url, method) = self.url;
            let time_seconds = self.time_seconds;
            
            // create self.threads threads, do loop in every thread
            Thread::new( move || {
                let mut bench_result: Vec<ReqResult> = vec![];
                
                // using hyper to do http client request
                let mut client = Client::new();
                if method == "GET".to_owned() {
                    // calculate current timestamp;
                    let start_t = ...;
                    let mut delta = 0;
                    
                    loop {
                        let mut cres = client.get(&url)
                            .headers()
                            .send().unwrap();
                        
                        assert_eq!(cres.status, hyper::Ok);
                        // make ReqResult instance
                        let req_result = ...;
                        bench_result.push(req_result);
                        
                        
                        let end_t = ...;
                        delta = end_t - start_t;
                        // check the time duration, if long enough, jump out
                        if delta >= time_seconds {
                            // send bench_result to main thread using channel
                            
                            // jump out
                            break;
                        }
                    
                    }
                
                }
                
                
            });
            
        }
        
        
        
    }
}


struct ReqResult {
    // response status
    status: hyper::Status,
    // response body length
    body_length: i64,
    // req->res duration, seconds
    time_last: f64,
    
}

