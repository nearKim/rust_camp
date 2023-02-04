use std::io;
// Ordering 타입은 enum이고 Less, Greater, Equal 세가지 variant를 가진다
use std::cmp::Ordering;
// Rng는 random number generator가 implement해야 할 trait을 정의한다
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // rand::thread_rng 함수는 현재 실행되는 thread에 localgks random number generator를 생성한다
    let secret_number = rand::thread_rng()
        // get_range는 range expression을 input으로 받는다: start..=end
        .gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // secret_number는 i32 타입이고 guess는 String 타입이다. 이에 따라 밑에서 비교를 위해서는 형변환을 해줘야 한다.
    // 위의 guess와 변수명이 겹치지만 알아서 shadowing을 통해 재사용한다 -> guess_str, guess_int와 같은 불필요한 변수명을 사용하지 않아도 된다.
    //
    let guess: u32 = guess.trim()
        // strings의 parse() 메소드는 string을 다른 type으로 변환한다
        // guess 뒤에 있는 : u32는 parse() 메소드가 반환하는 type을 명시한다
        // 추가적으로 u32 annotation은 i32 secret_number 비교를 통해 Rust가 secret_number를 u32로 취급함을 의미한다
        // parse 메소드는 실패할 수 있기 때문에 Result type을 반환한다. -> read_line과 비슷함
        .parse()
        .expect("Please type a number!");


    println!("You guessed: {guess}");

    // match 표현식은 arm들로 이뤄진다
    // arm은 pattern과 match에 주어진 값이 pattern에 맞으면 실행할 code 블록으로 이뤄진다
    // match 표현식은 첫번째 성공적인 match이후 종료된다
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
