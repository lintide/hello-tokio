mod calc;

use calc::Calc;

#[tokio::main]
async fn main() {
    let mut c1 = Calc::new();
    c1.send(("add".to_owned(), 1));
    c1.send(("add".to_owned(), 2));

    // 这里输出:0, 因为+1, +2还在队列里没有执行完
    print!("value: {}", c1.result()); // vaule: 0
}
