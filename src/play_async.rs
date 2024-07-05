#[tokio::main]
async fn main() {
    for i in 1..=100 {
        tokio::spawn(async move { println!("{}", fizzbuzz(i).await) });
    }
}

async fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}
