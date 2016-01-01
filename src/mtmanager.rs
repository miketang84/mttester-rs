
use std::collections::HashMap;
use std::path::Path;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use hyper;
use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::Headers;
use time;
use std::io::Read;


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
    output_file: Option<String>,
    
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    // accounts to simulate
    // <account_name, password>
    accounts: Vec<(String, String)>,
    
    req_content_type: String,
    need_auth: bool
    
}

pub trait MtManagerTrait {
    fn set_auth_url(&mut self, url: String, method: String) -> &mut Self;
    fn add_url(&mut self, url: String, method: String) -> &mut Self;
    fn set_seconds(&mut self, s: i64) -> &mut Self;
    fn set_threads(&mut self, n: i64) -> &mut Self;
    fn set_threads_per_account(&mut self, n: i64) -> &mut Self;
    fn add_account(&mut self, account: String, password: String) -> &mut Self;
    fn add_header(&mut self, key: String, value: String) -> &mut Self;
    fn add_param(&mut self, key: String, value: String) -> &mut Self;
    // set the request content data type, default is www-form-urlencoded, you can set "urlencoded", or "json" now
    fn set_param_type(&mut self, ctype: String) -> &mut Self;
    fn output_file(&mut self, path: String) -> &mut Self;
    fn start(&mut self);
}


impl MtManager {
    
    pub fn new() -> MtManager {
        let mt: MtManager = Default::default();
        mt
    }
}


impl MtManagerTrait for MtManager {
    fn set_auth_url(&mut self, url: String, method: String) -> &mut Self {
        self.auth_url = (url, method);
        self
    }
    
    fn add_url(&mut self, url: String, method: String) -> &mut Self {
        self.url = (url, method);
        self
    }
    
    fn set_seconds(&mut self, s: i64) -> &mut Self {
        self.time_seconds = s;
        self
    }
    
    fn set_threads(&mut self, n: i64) -> &mut Self {
        self.threads = n;
        self
    }
    
    fn set_threads_per_account(&mut self, n: i64) -> &mut Self {
        self.threads_per_account = n;
        self
    }
    
    fn add_account(&mut self, account: String, password: String) -> &mut Self {
        self.accounts.push((account, password));
        self
    }
    
    fn add_header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }
    
    fn add_param(&mut self, key: String, value: String) -> &mut Self {
        self.params.insert(key, value);
        self
    }
    
    // set the request content data type, default is www-form-urlencoded, you can set "urlencoded", or "json" now
    fn set_param_type(&mut self, ctype: String) -> &mut Self {
        self.req_content_type = ctype;
        self
    }
    
    fn output_file(&mut self, path: String) -> &mut Self {
        // create path
        // let path_obj = Path::new(path);
        self.output_file = Some(path);
        self
    }
    
    
    fn start(&mut self) {
        // here now, self has been filled enough fileds to start with
        self.need_auth = if self.accounts.len() == 0 {
            false
        }
        else {
            true
        };
        
        let (tx, rx) = channel();
        
        if !self.need_auth {
            // consider no auth first
            for i in 0..self.threads {
                // prepare bindings and channel
                let (url, method) = self.url.clone();
                let time_seconds = self.time_seconds;
                let thread_tx = tx.clone();
                
                // create self.threads threads, do loop in every thread
                thread::spawn ( move || {
                    
                    _doreq(
                            thread_tx, 
                            method, 
                            url, 
                            HashMap::new(), 
                            HashMap::new(), 
                            time_seconds,
                            "urlencoded".to_string(),
                            i
                    );
                    
                });
                
            }
        }
        else {
            // TODO: consider need auth
        }
        
        
        // in main thread, collect the return result, and calculate
        let mut collectors = Vec::with_capacity(self.threads as usize);
        for i in 0..self.threads {
            collectors.push(rx.recv().unwrap());
        }
        
        // println!("Test Result: {:?}", collectors);
        let total_requests = collectors.iter().fold(0, |acc, ref item| acc + item.len());
        
        // =================================================
        // println!("URLs: {} {}", self.url.1, self.url.0);
        // println!("Time Last: {}", self.time_seconds);
        // println!("Users: {}", self.threads);
        // println!("Total RPS: {:.2}", total_requests as f64 / self.time_seconds as f64 );
        let table = table!(
            ["URLs", format!("{} {}", self.url.1, self.url.0)],
            ["Time Last", self.time_seconds],
            ["Users", self.threads],
            ["Total RPS", format!("{:.2}", total_requests as f64 / self.time_seconds as f64)]
        );
        table.printstd();
        
        thread::sleep_ms(1000);
        
    }
}

#[derive(Debug)]
struct ReqResult {
    // response status
    status: hyper::status::StatusCode,
    // response body length
    body_length: i64,
    // req->res duration, m seconds
    time_last: f64,
    
}

fn _doreq (
        thread_tx: Sender<Vec<ReqResult>>,
        method: String, 
        url: String, 
        headers: HashMap<String, String>, 
        params: HashMap<String, String>, 
        time_seconds: i64,
        req_content_type: String, 
        thread_i: i64) {
    let mut bench_result: Vec<ReqResult> = vec![];
    
    // using hyper to do http client request
    let mut client = Client::new();
    if method == "GET".to_owned() {
        // calculate current timestamp;
        let start_t = time::precise_time_s();
        
        loop {
            let per_start = time::precise_time_ns();
            // fill neccessary headers
            let mut headers = Headers::new();
            
            let mut cres = client.get(&url)
                .headers(headers)
                .send().unwrap();
            
            let mut body = String::new();
            cres.read_to_string(&mut body).unwrap();
            // println!("ret value: {}", body);
            
            let per_end = time::precise_time_ns();
            
            assert_eq!(cres.status, hyper::Ok);
            // make ReqResult instance
            let req_result = ReqResult {
                status: cres.status,
                body_length: body.len() as i64,
                time_last: (per_end - per_start) as f64 / 1000000.0
            };
            println!("{:?}", req_result);
            bench_result.push(req_result);
            
            
            let end_t = time::precise_time_s ();
            let delta = end_t - start_t;
            // check the time duration, if exceed, jump out
            if delta >= time_seconds as f64 {
                // send bench_result to main thread using channel
                thread_tx.send(bench_result).unwrap();
                println!("thread {} finished.", thread_i);
                // jump out
                break;
            }
        
        }
    
    }
    else if method == "POST".to_owned() {
        
    }
}
