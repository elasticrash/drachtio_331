use chrono::prelude::*;
use rsip::{Request, SipMessage};
use uuid::Uuid;

use crate::util::get_fake_sdp;

pub fn create(origin: String, destination: String) -> SipMessage {
    let now = Utc::now();
    let base_uri = rsip::Uri {
        auth: None,
        host_with_port: rsip::Domain::from(format!("sip:{}@{}:{}", origin, "127.0.0.1", 5060))
            .into(),
        ..Default::default()
    };

    let mut headers: rsip::Headers = Default::default();

    headers.push(
        rsip::typed::Via {
            version: rsip::Version::V2,
            transport: rsip::Transport::Udp,
            uri: rsip::Uri {
                host_with_port: (rsip::Domain::from(format!("{}:{}", "127.0.0.1", 5060))).into(),
                ..Default::default()
            },
            params: vec![rsip::Param::Branch(rsip::param::Branch::new(format!(
                "z9hG4bK{}{}{}{}{}{}",
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
                now.timestamp_millis()
            )))],
        }
        .into(),
    );

    headers.push(
        rsip::typed::From {
            display_name: Some("test".to_string()),
            uri: base_uri,
            params: vec![rsip::Param::Tag(rsip::param::Tag::new(
                Uuid::new_v4().to_string(),
            ))],
        }
        .into(),
    );

    headers.push(
        rsip::typed::To {
            display_name: Some(origin.to_string()),
            uri: rsip::Uri {
                auth: None,
                host_with_port: rsip::Domain::from(format!(
                    "sip:{}@{}:{}",
                    destination, "127.0.0.1", 5060
                ))
                .into(),
                ..Default::default()
            },
            params: Default::default(),
        }
        .into(),
    );

    headers.push(rsip::headers::CallId::from(Uuid::new_v4().to_string()).into());

    headers.push(
        rsip::typed::Contact {
            display_name: Some(origin.to_string()),
            uri: rsip::Uri {
                host_with_port: (rsip::Domain::from(format!(
                    "sip:{}@{}:{}",
                    origin, "127.0.0.1", 5061
                )))
                .into(),
                ..Default::default()
            },
            params: Default::default(),
        }
        .into(),
    );

    headers.push(rsip::headers::MaxForwards::from(70).into());

    headers.push(
        rsip::typed::CSeq {
            seq: 1,
            method: rsip::Method::Invite,
        }
        .into(),
    );

    headers.push(rsip::headers::Allow::from("ACK,BYE,CANCEL,INVITE").into());
    let fake_sdp_body = get_fake_sdp("127.0.0.1");

    headers.push(rsip::headers::ContentType::from("application/sdp").into());
    headers.push(rsip::headers::ContentLength::from(fake_sdp_body.len().to_string()).into());

    let invite: SipMessage = Request {
        method: rsip::Method::Invite,
        uri: rsip::Uri {
            scheme: Some(rsip::Scheme::Sip),
            host_with_port: rsip::Domain::from(format!("{}@{}:{}", origin, "127.0.0.1", 5060))
                .into(),
            ..Default::default()
        },
        version: rsip::Version::V2,
        headers,
        body: fake_sdp_body.as_bytes().to_vec(),
    }
    .into();

    invite
}
