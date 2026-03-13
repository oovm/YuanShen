use bytes::{Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use ys_protocol::git::{PktLine, PktLineCodec};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn integration_test_pkt_line_all_types() {
    let mut codec = PktLineCodec;
    let mut dst = BytesMut::new();

    let test_cases = vec![
        PktLine::Flush,
        PktLine::Delim,
        PktLine::ResponseEnd,
        PktLine::Data(Bytes::from("Hello, World!")),
        PktLine::Data(Bytes::from("Test data 123")),
    ];

    for pkt in test_cases {
        dst.clear();
        codec.encode(pkt.clone(), &mut dst).unwrap();
        let mut src = BytesMut::from(dst.as_ref());
        let decoded = codec.decode(&mut src).unwrap().unwrap();
        assert_eq!(decoded, pkt);
    }
}

#[test]
fn integration_test_pkt_line_series() {
    let mut codec = PktLineCodec;
    let mut dst = BytesMut::new();

    let packets = vec![
        PktLine::Data(Bytes::from("first")),
        PktLine::Data(Bytes::from("second")),
        PktLine::Flush,
        PktLine::Data(Bytes::from("third")),
        PktLine::Delim,
        PktLine::ResponseEnd,
    ];

    for pkt in packets.clone() {
        codec.encode(pkt, &mut dst).unwrap();
    }

    let mut src = BytesMut::from(dst.as_ref());
    for expected in packets {
        let decoded = codec.decode(&mut src).unwrap().unwrap();
        assert_eq!(decoded, expected);
    }
}
