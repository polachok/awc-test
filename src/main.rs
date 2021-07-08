use actix_rt::time::sleep;
use awc::Client;
use std::time::Duration;
use std::time::Instant;

#[actix_rt::main]
async fn main() {
    let mut client = Client::new();

    let mut times = Vec::new();

    for i in 0..20 {
        let before = Instant::now();
        send(&mut client).await;
        times.push(before.elapsed());
        print!(".");
        if i % 10 == 0 {
            println!("");
        }
        sleep(Duration::from_millis(300)).await;
    }
    let mean: Duration = times.iter().sum::<Duration>() / times.len() as u32;
    println!("mean time: {:?}", mean);
}

async fn send(client: &mut Client) {
    let body = r#"{"hello": "world"}"#;
    client
        .post("http://httpbin.org/post")
        .send_body(body)
        .await
        .unwrap();
}
