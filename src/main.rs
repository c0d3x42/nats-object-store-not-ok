use async_nats::jetstream::object_store;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    env_logger::init();

   /*
    let nats_connection = async_nats::ConnectOptions::with_credentials_file("creds".clone().into())
        .await
        .unwrap();

    let nc = nats_connection
        .connect("nats://localhost:4222")
        .await
        .unwrap();
    */ 

     
    let nc = async_nats::connect("demo.nats.io").await.unwrap();
    let jetstream = async_nats::jetstream::new(nc);

    //let bucket = jetstream.get_object_store("store").await.unwrap();
    let bucket = jetstream
        .create_object_store(object_store::Config {
            bucket: "store".to_string(),
            num_replicas: 1,
            ..Default::default()
        })
        .await
        .unwrap();

    let bucket2 = bucket.clone();
    let bucket3 = bucket.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;
            bucket3
                .put("f", &mut std::io::Cursor::new(vec![1, 2, 3, 4]))
                .await
                .unwrap();
            log::warn!("PUT to f");
        }

        ()
    });

    let wh = tokio::spawn(async move {
        let mut watcher = bucket2.watch().await.unwrap();

        loop {
            let s = watcher.next().await;

            log::info!("watcher got {:#?}", s);
        }
        ()
    });

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
            match bucket.get("f").await {
                Ok(obj) => {
                    log::info!("get f, {:#?}", obj.info());
                }
                Err(err) => {
                    log::error!("GOT NOTHING {}", err);
                }
            }
        }

        ()
    });

    wh.await;
}
