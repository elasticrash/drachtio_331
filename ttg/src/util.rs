use std::fmt::Write;

pub fn get_fake_sdp(ip: &str) -> String {
    let mut body = "v=0\r\n".to_string();
    let _ = write!(body, "o=tggVCE 226678890 391916715 IN IP4 {}\r\n", ip);
    body.push_str("s=tggVCE Audio Call\r\n");
    let _ = write!(body, "c=IN IP4 {}\r\n", ip);
    body.push_str("t=0 0\r\n");
    body.push_str("m=audio 49152 RTP/AVP 0 8 96\r\n");

    body.push_str("a=rtpmap:0 PCMU/8000\r\n");
    body.push_str("a=rtpmap:8 PCMA/8000\r\n");
    body.push_str("a=rtpmap:96 telephone-event/8000\r\n");
    body.push_str("a=fmtp:96 0-15\r\n");

    body
}
