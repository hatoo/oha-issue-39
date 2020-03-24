#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    let (cross_tx, cross_rx) = crossbeam::channel::unbounded();

    for _ in 0..1 {
        let cross_tx = cross_tx.clone();
        tokio::spawn(async move {
            for _ in 0.. {
                cross_tx.send(()).unwrap();
                tokio::time::delay_for(std::time::Duration::from_millis(10)).await;
            }
        });
    }

    // This spawn is necessary to reproduce the symptoms.
    tokio::spawn(async move {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let (mut ctrl_c_tx, mut ctrl_c_rx) = tokio::sync::mpsc::channel(1);
        // Using crossbeam::channel and tokio's channel in same app looks odd
        // but actually `oha` is built like that due to performance.
        tokio::spawn(async move {
            while let Ok(()) = cross_rx.recv() {
                tx.send(()).unwrap();
            }
        });

        tokio::spawn(async move {
            // We can't catch signal inner loop directly on Windows.
            // So it uses channel here.
            if let Ok(()) = tokio::signal::ctrl_c().await {
                println!("send ctrl-c event");
                ctrl_c_tx.send(()).await.unwrap();
            }
        });

        loop {
            tokio::select! {
                _ = rx.recv() => { println!("recv"); }
                // I assume that this arm is fired when user press ctrl-c
                _ = ctrl_c_rx.recv() => {
                    println!("ctrl-c");
                    std::process::exit(0);
                },
            }
        }
    })
    .await
}
