// prelude: 모든 프로그램에서 접근가능한 Rust standard library
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;

// main 함수가 program entrypoint
fn main() {
    // !가 붙으면 rust macro를 호출하는 것. 만일 function을 호출하는 것이었으면 println이엇을 것임
    println!("Guess the number!");
    println!("Please input your guess.");

    // let 키워드로 변수 선언. 변수는 기본적으로 immutable
    // let apples = 5;
    // mutable 변수를 선언하려면 mut 키워드 사용
    // :: 연산자는 new 함수가 String type의 associated function임을 나타냄
    // associated function은 type자체에 구현된 function
    // the line has created a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new();

    // use를 첫줄에 import하지 않았다면 std::io::stdin으로 사용할 수도 있음
    // stdin 함수는 std::io::Stdin type의 instance를 return (첫째문자가 대문자)
    // 이 Stdin은 terminal standard input의 handle을 나타내는 type임
    io::stdin()
        // read_line은 사용자가 terminal에 stdin으로 입력하는 모든 것을 받아서 string에 append 하는 것임 (overwrite하지 않음)
        // 따라서 대상이 될 string을 인자로 넘겨야 하고, 그 string 인자는 mutable이어야 함
        // & 는 이 인자가 reference값임을 나타냄. 코드의 여러 부분에서 같은 데이터에 접근할 수 있도록 함.
        // 이를 통해 메모리에 여러번 동일한 데이터를 복사할 필요가 없음
        // reference는 기본적으로 immutable이므로 mutable하게 만들려면 mut 키워드를 사용해야 함
        .read_line(&mut guess)
        // read_line은 io::Result type을 반환함
        // Result는 enum임. enum은 여러 state를 가질 수 있고, 이러한 state를 variant라고 함
        // Result는 Ok와 Err 두가지 variant를 가짐
        // expect는 Result가 Err variant를 가지고 있다면 프로그램을 crash시키면서 msg를 보여주고, Ok variant를 가지고 있다면 Ok variant에 담긴 값을 return함
        .expect("Failed to read line");

    // {}를 사용하여 변수를 string에 넣을 수 있음
    // 예시코드
    /*
        let x = 5;
        let y = 10;
        println!("x = {x} and y + 2 = {}", y + 2);  // 5, 12
    */
    println!("You guessed: {guess}");
    // println!("You guessed: {}", guess);
}
