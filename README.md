#MTtester

Rust http logic tester, easy to use.

This testing tool is designed for simulate logical bussiness, using the thread of a user's action, you can attach a serial actions to one test, including login auth.


## TODO

1. add multiple request support, every request has its own req_content_type, headers, params, method, url;
2. add support for multiple threads per account, if server permit mutliple end login;
3. make API easier, remove obviously .to_string(), to_owned();
4. internal use referrence str to improve performace;
5. add support for cookie type token authorization;

