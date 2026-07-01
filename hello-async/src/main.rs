use std::thread;
use std::time::Duration;
use std::pin::{pin, Pin};
use trpl::{Either, Html, StreamExt};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title =Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);
        
        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("UR: {} return first", url);
        match maybe_title {
            Some(title) => println!("Page title: {}", title),
            None => println!("no title for {}", url),
        }
    });

    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..5 {
                println!("hi number {i} from first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..3 {
            println!("hi number {i} from second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
        

        let fut1 = async {
            for i in 1..5 {
                println!("hi number {i} from third task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..3 {
                println!("hi number {i} from fourth task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx2 = tx.clone();

        let tx_fut = async move {
            let vals = vec!["hi", "from", "the", "future"];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let tx2_fut = async move {
            let vals = vec!["more", "msg", "for", "you"];
            for val in vals {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("got: {val}");
            }
        };

        trpl::join!(tx_fut, tx2_fut, rx_fut);
    });

    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("a started");
            slow("a", 30);
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            println!("a done");
        };

        let b = async {
            println!("b started");
            trpl::yield_now().await;
            slow("b", 10);
            slow("b", 15);
            println!("b done");
        };

        trpl::select(a, b).await;
    });

    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(2)).await;
            "finished"
        };

        match timeout(slow, Duration::from_secs(1)).await {
            Ok(message) => println!("slow task completed: {message}"),
            Err(duration) => println!("failed after: {} seconds", duration.as_secs()),
        }
    });

    trpl::block_on(async {
        let values = [1, 2, 3];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });

    trpl::block_on(async {
        let tx1_fut = pin!(async move {});
        let rx_fut = pin!(async move {});
        let tx2_fut = pin!(async move {});

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx2_fut];

        trpl::join_all(futures).await;
    });

    let (tx9, mut rx9) = trpl::channel();
    thread::spawn(move || {
        for i in 1..3 {
            tx9.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(msg) = rx9.recv().await {
            println!("got msg: {msg}");
        }
    })
}

async fn timeout<F: Future>(
    future: F, max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(__) => Err(max_time),
    }
} 
