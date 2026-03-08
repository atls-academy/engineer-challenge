use crate::infrastructure::adapters::kratos::models::errors::KratosFlowError;
use crate::infrastructure::adapters::kratos::models::flows::{FlowResult, PostFlowResult};
use reqwest::{Client, StatusCode, header};

pub async fn fetch_flow(
    client: &Client,
    public_url: &str,
    endpoint: &str,
    cookie: Option<&str>,
) -> Result<FlowResult, Box<dyn std::error::Error>> {
    let url = format!("{}/self-service/{}/browser", public_url, endpoint)
        .replace("localhost", "127.0.0.1");

    let mut request = client.get(&url);
    if let Some(cookie_value) = cookie {
        request = request.header(header::COOKIE, cookie_value);
    }

    let response = request.send().await.map_err(|e| {
        format!(
            "Failed to connect to Kratos at {}: {}. Make sure Kratos is running.",
            url, e
        )
    })?;

    let status = response.status();
    let flow_cookies = extract_cookies(&response);

    if status == StatusCode::SEE_OTHER || status == StatusCode::FOUND {
        return handle_redirect(client, public_url, endpoint, response, flow_cookies, cookie).await;
    }

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!(
            "Failed to fetch {} flow (status {}): {}",
            endpoint, status, error_text
        )
        .into());
    }

    let flow: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse {} flow response: {}", endpoint, e))?;

    let csrf_token = extract_csrf_token(&flow)?;
    let mut all_cookies = cookie.map(|c| vec![c.to_string()]).unwrap_or_default();
    all_cookies.extend(flow_cookies);

    Ok(FlowResult {
        flow,
        csrf_token,
        cookies: all_cookies,
    })
}

pub async fn post_flow(
    client: &Client,
    public_url: &str,
    endpoint: &str,
    flow_id: &str,
    data: serde_json::Value,
    cookies: &[String],
) -> Result<PostFlowResult, KratosFlowError> {
    let url = format!("{}/self-service/{}?flow={}", public_url, endpoint, flow_id)
        .replace("localhost", "127.0.0.1");

    let response = client
        .post(&url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::COOKIE, cookies.join("; "))
        .json(&data)
        .send()
        .await
        .map_err(KratosFlowError::network)?;

    let response_cookies = extract_cookies(&response);
    let status = response.status();

    if !status.is_success() {
        let body = response
            .json::<serde_json::Value>()
            .await
            .unwrap_or_else(|_| serde_json::json!({}));
        return Err(KratosFlowError { status, body });
    }

    let data = response
        .json::<serde_json::Value>()
        .await
        .map_err(KratosFlowError::network)?;

    Ok(PostFlowResult {
        data,
        cookies: response_cookies,
    })
}

fn extract_cookies(response: &reqwest::Response) -> Vec<String> {
    response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .collect()
}

fn extract_csrf_token(flow: &serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    flow["ui"]["nodes"]
        .as_array()
        .and_then(|nodes| {
            nodes
                .iter()
                .find(|node| node["attributes"]["name"].as_str() == Some("csrf_token"))
        })
        .and_then(|node| node["attributes"]["value"].as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "CSRF token not found in flow response".into())
}

async fn handle_redirect(
    client: &Client,
    public_url: &str,
    endpoint: &str,
    response: reqwest::Response,
    flow_cookies: Vec<String>,
    cookie: Option<&str>,
) -> Result<FlowResult, Box<dyn std::error::Error>> {
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|h| h.to_str().ok())
        .ok_or("No redirect location found")?;

    let flow_id = location
        .split("flow=")
        .nth(1)
        .ok_or(format!("Flow ID not found in redirect URL: {}", location))?
        .split('&')
        .next()
        .ok_or(format!("Flow ID not found in redirect URL: {}", location))?;

    let flow_url = format!(
        "{}/self-service/{}/flows?id={}",
        public_url.replace("localhost", "127.0.0.1"),
        endpoint,
        flow_id
    );

    let mut flow_request = client.get(&flow_url);
    if !flow_cookies.is_empty() {
        flow_request = flow_request.header(header::COOKIE, flow_cookies.join("; "));
    } else if let Some(cookie_value) = cookie {
        flow_request = flow_request.header(header::COOKIE, cookie_value);
    }

    let flow_response = flow_request.send().await?;
    if !flow_response.status().is_success() {
        let status = flow_response.status();
        let error_text = flow_response.text().await.unwrap_or_default();
        return Err(format!(
            "Failed to fetch {} flow (status {}): {}",
            endpoint, status, error_text
        )
        .into());
    }

    let flow: serde_json::Value = flow_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse {} flow response: {}", endpoint, e))?;

    let csrf_token = extract_csrf_token(&flow)?;
    let mut all_cookies = cookie.map(|c| vec![c.to_string()]).unwrap_or_default();
    all_cookies.extend(flow_cookies);

    Ok(FlowResult {
        flow,
        csrf_token,
        cookies: all_cookies,
    })
}
