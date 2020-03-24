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

    tokio::spawn(async move {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let (ctrl_c_tx, mut ctrl_c_rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Ok(()) = cross_rx.recv() {
                tx.send(()).unwrap();
            }
        });

        tokio::spawn(async move {
            if let Ok(()) = tokio::signal::ctrl_c().await {
                println!("send");
                ctrl_c_tx.send(()).unwrap();
            }
        });

        loop {
            tokio::select! {
                _ = rx.recv() => {  println!("recv"); }
                _ = ctrl_c_rx.recv() => {
                    println!("ctrl-c");
                    std::process::exit(0);
                },
            }
        }
    })
    .await
}
