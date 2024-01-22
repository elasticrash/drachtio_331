use rsip::{prelude::HeadersExt, Response, SipMessage};

pub fn create(response: Response, origin: String, destination: String) -> SipMessage {
    let mut headers: rsip::Headers = Default::default();
    let via = response.via_header().unwrap();
    let local_tag = response.from_header().unwrap().tag().unwrap().unwrap();
    let remote_tag = response.to_header().unwrap().tag().unwrap().unwrap();

    let call_id = response.call_id_header().unwrap();

    let base_uri = rsip::Uri {
        auth: None,
        host_with_port: rsip::Domain::from(format!("sip:{}@{}:{}", origin, "127.0.0.1", 5060))
            .into(),
        ..Default::default()
    };

    headers.push(
        rsip::typed::Via {
            version: rsip::Version::V2,
            transport: rsip::Transport::Udp,
            params: vec![
                rsip::Param::Branch(via.branch().clone().unwrap()),
                rsip::Param::Other("rport".into(), None),
            ],
            uri: via.uri().clone().unwrap(),
        }
        .into(),
    );

    headers.push(
        rsip::typed::From {
            display_name: Some(origin.to_string()),
            uri: base_uri.clone(),
            params: vec![rsip::Param::Tag(rsip::param::Tag::new(local_tag.clone()))],
        }
        .into(),
    );

    headers.push(
        rsip::typed::To {
            display_name: None,
            uri: rsip::Uri {
                auth: None,
                host_with_port: rsip::Domain::from(format!(
                    "sip:{}@{}:{}",
                    destination, "127.0.0.1", 5060
                ))
                .into(),
                ..Default::default()
            },
            params: vec![rsip::Param::Tag(rsip::param::Tag::new(remote_tag))],
        }
        .into(),
    );

    headers.push(rsip::Header::CallId(call_id.clone()).into());
    headers.push(
        rsip::typed::CSeq {
            seq: 1,
            method: rsip::Method::Ack,
        }
        .into(),
    );
    headers.push(rsip::headers::MaxForwards::from(70).into());

    headers.push(rsip::headers::ContentLength::default().into());

    let response: SipMessage = rsip::Request {
        method: rsip::Method::Ack,
        uri: rsip::Uri {
            scheme: Some(rsip::Scheme::Sip),
            host_with_port: rsip::Domain::from(format!("{}@{}:{}", destination, "127.0.0.1", 5060))
                .into(),
            ..Default::default()
        },
        version: rsip::Version::V2,
        headers,
        body: Default::default(),
    }
    .into();

    response
}
