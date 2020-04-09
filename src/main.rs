use serde::{Deserialize, Serialize};
use reqwest;
use reqwest::StatusCode;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use std::collections::HashMap;
use std::borrow::Borrow;
use std::ops::Add;


#[derive(Debug, Deserialize)]
struct Person {
    id: String,
    employee_name: String,
    employee_salary: String,
    employee_age: String,
    profile_image: String
}

#[derive(Debug, Deserialize)]
struct HttpResponse {
    status: String,
    data: Vec<Person>
}


#[derive(Debug, Serialize)]
struct AuthPostData {
    username: String,
    password: String
}


struct DeviceAPIClient {
    simulation_id: String,
    device_id: String,
    domain_name: String,
    jwt_token: String,
    url_prefix: String,
    client: reqwest::blocking::Client
}


impl DeviceAPIClient {
    fn new(simulation_id: String, device_id: String, domain_name: String,
           username: String, password: String) -> DeviceAPIClient {
        let request_url = format!("{}/external-connection/api/{}/{}/",
                                  domain_name, simulation_id, device_id);
        let login_url = format!("{}/api-token-auth/", domain_name);
        let credentials = AuthPostData{username, password};
        let client = reqwest::blocking::Client::new();
        let auth_resp: HashMap<String, String> = client.post(&login_url).json(&credentials).
            send().expect("POST request to api-token-auth failed.").
            json().expect("POST request body cannot be serialized.");
        let jwt_token = auth_resp.get("token").
            expect("'token' member does not exist on the authentication response.").clone();

        DeviceAPIClient {
            simulation_id, device_id, domain_name, jwt_token, url_prefix: request_url, client: client
        }
    }


    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let jwt_header = format!("JWT {}", self.jwt_token);
        headers.insert("Authorization", HeaderValue::from_str(&jwt_header).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        headers
    }

    fn post(&self, endpoint_suffix: String) {
        let mut endpoint_url = self.url_prefix.clone();
        endpoint_url.push_str(&endpoint_suffix);
        println!("Endpoint url {}",endpoint_url);
        let mut map: HashMap<String, String> = HashMap::new();
        let response = self.client.post(&endpoint_url).
            headers(self.construct_headers()).
            json(&map).
            send().expect(&format!("Get request failed for endpoint {}", endpoint_suffix));
        match response.status() {
            StatusCode::OK => println!("success!"),
            s => println!("Received response status: {:?}", s),
        };
//            text().expect(&format!("Response body for request {} could not be parsed.", endpoint_suffix));
        println!("Response body {}", response.text().expect("failed"));
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let resp = reqwest::blocking::get(
//        "http://dummy.restapiexample.com/api/v1/employees")?.text()?;
//    let serde_resp: HttpResponse = serde_json::from_str(&resp)?;

    let api_client = DeviceAPIClient::new(
        String::from("00555be7-bcc6-4b9b-bb9b-5150300b5f9d"),
        String::from("fde7c16f-f885-4ed8-ab3e-a1bbebeb188f"),
        String::from("https://d3aweb-dev.gridsingularity.com"),
        String::from("researcher-hack@gridsingularity.com"),
        String::from("researchertest321*"));
    println!("{:#?}", api_client.jwt_token);
    api_client.post("register".to_string());
    Ok(())
}