use bytes::BytesMut;
use futures::sink::SinkExt;
use futures::StreamExt as FuturesStreamEx;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::codec::{FramedRead, FramedWrite};

struct MyCodec {}

impl Encoder<usize> for MyCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: usize, buf: &mut BytesMut) -> Result<(), Self::Error> {
        // do something here
        todo!();
    }
}

impl Decoder for MyCodec {
    type Item = usize;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // do something with the incoming src bytes
        todo!();
    }
}

#[tokio::main]
async fn main() {}

async fn spawn_read_write_tasks<
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
>(
    producer_rx: R,
    producer_tx: W,
) {
    let codec = MyCodec {};

    let mut writer = FramedWrite::new(producer_tx, codec);

    let (tx1, mut rx1) = unbounded_channel::<usize>();
    let (tx2, mut rx2) = unbounded_channel::<usize>();

    let rx1_stream = UnboundedReceiverStream::new(rx1);
    let rx2_stream = UnboundedReceiverStream::new(rx2);

    let rx_stream = rx1_stream.merge(rx2_stream);

    if let Err(err) = rx_stream.forward(writer).await {
        panic!("Stream ended with error {:?}", err);
    }
}
