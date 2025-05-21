use futures_util::{stream::StreamExt, SinkExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            line = stdin.next_line() => match line {
                Ok(Some(text)) => {
                    ws_sender.send(Message::text(text)).await?;
                }
                Ok(None) => break,         
                Err(e)   => { eprintln!("stdin error: {}", e); break; }
            },

            frame = ws_receiver.next() => match frame {
                Some(Ok(msg)) if msg.is_text() => {
                    if let Some(text) = msg.as_text() {
                        println!("{}", text);
                    }
                }
                Some(Ok(msg)) if msg.is_close() => break,
                Some(Ok(_)) => { /* ignore binary/ping/etc */ }
                Some(Err(e)) => { eprintln!("websocket error: {}", e); break; }
                None => break,
            },
        }
    }

    Ok(())
}
