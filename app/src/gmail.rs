use std::error::Error;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct SimpleMail {
    pub id: String,
    pub subject: Option<String>,
    pub from: Option<String>,
    pub date: Option<String>,
    pub snippet: Option<String>,
}

#[derive(Deserialize)]
struct ListResp {
    messages: Option<Vec<MessageId>>,
}
#[derive(Deserialize)]
struct MessageId {
    id: String,
}

#[derive(Deserialize)]
struct MessageFull {
    id: String,
    snippet: Option<String>,
    payload: Option<Payload>,
}
#[derive(Deserialize)]
struct Payload {
    headers: Option<Vec<Header>>,
}
#[derive(Deserialize)]
struct Header {
    name: String,
    value: String,
}

fn header_value(headers: Option<&Vec<Header>>, name: &str) -> Option<String> {
    headers
        .and_then(|hs| hs.iter().find(|h| h.name.eq_ignore_ascii_case(name)))
        .map(|h| h.value.clone())
}

pub fn fetch_latest(access_token: &str, max_results: usize) -> Result<Vec<SimpleMail>, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let list_url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages?labelIds=INBOX&maxResults={}",
        max_results
    );

    let list_res = client
        .get(&list_url)
        .bearer_auth(access_token)
        .send()?;

    if !list_res.status().is_success() {
        let status = list_res.status();
        let body = list_res.text().unwrap_or_else(|_| "<failed to read body>".into());
        return Err(format!("gmail list API error: {} - {}", status, body).into());
    }
    let list: ListResp = list_res.json()?;

    let mut out = Vec::new();
    if let Some(msgs) = list.messages {
        for m in msgs {
            let msg_url = format!("https://gmail.googleapis.com/gmail/v1/users/me/messages/{}?format=full", m.id);
            let msg_res = client
                .get(&msg_url)
                .bearer_auth(access_token)
                .send()?;
            if !msg_res.status().is_success() {
                let status = msg_res.status();
                let body = msg_res.text().unwrap_or_else(|_| "<failed to read body>".into());
                return Err(format!("gmail get message error: {} - {}", status, body).into());
            }
            let mf: MessageFull = msg_res.json()?;

            let subject = header_value(mf.payload.as_ref().and_then(|p| p.headers.as_ref()), "Subject");
            let from = header_value(mf.payload.as_ref().and_then(|p| p.headers.as_ref()), "From");
            let date = header_value(mf.payload.as_ref().and_then(|p| p.headers.as_ref()), "Date");

            out.push(SimpleMail {
                id: mf.id,
                subject,
                from,
                date,
                snippet: mf.snippet,
            });
        }
    }
    Ok(out)
}
