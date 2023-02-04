use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    // loop은 무한루프를 생성한다
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Error handling을 위해 expect 호출에서 match 표현식으로 변경한다
        // parse를 Return type을 반환하고 Return은 enum이므로 Ok, Err variant를 가진다
        // 성공한 경우 Ok 안에 parse가 생산한 값인 num이 담길 것이다. 이를 그대로 반환하여 guess에 담는다
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _는 catchall 값으로 Err 안에 어떤 값이 있던지 상관없이 모든 Err variant를 처리한다
            // continue를 통해 loop의 다음 iteration으로 넘어간다
            Err(_) => continue,
            };



        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
