
1. 기본 문법 블럭
```
fn main() {
    // 주석은 C & C++ 과 동일
    /* 
    * println! 함수가 아니고 매크로
    */
    println!("Hello, world!"); 
}
```
2. 컴파일 
```
$ rustc main.rs
$ ./main
Hello, world!
```

3. use 키워드 사용
- include , import 역할을 하는 키워드
```
/// 기본적으로 아래와 같이
use std::io::prelude::*;
use std::io::BufReader;
/// 묶어서 선언도 가능
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
```

3. Result , Option enum 을 주로 사용하고 unwrap로 값에 접근 

4. defendense 추가시 cargo 
```
# 기본 mio 추가 및 feature 추가 
cargo add mio -F os-poll -F net
```

5. ? 연산자  : Result<R,E> , Option(R) 만 가능 unwrap 기능 차이점은 Result 의 경우 실패시 리턴값으로 전달한다.
```
let a = Some(1); // => a is Some(1)
let b = Some(1)?; // => b is 1
/////

fn test_question() -> Result<i32,String>
{
    let err = Err("error")?;
    Ok(0)
}

```