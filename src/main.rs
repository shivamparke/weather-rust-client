use std::collections::HashMap;
use reqwest::header::AUTHORIZATION;

mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}\n", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //

    // let client = reqwest::Client::new();
    //
    // let response = client
    //     .get("https://shvm-aws-cloud-labs.s3.amazonaws.com/response.json")
    //     .send()
    //     .await?;
    //
    // let weather2 = response
    //     .json::<model::Weather>()
    //     .await?;
    //
    // println!("\nWeather from openweathermap.org:\n {:?}", weather2);

    // This will POST a body of `{"lang":"rust","body":"json"}`
    // For sending username and password for authentication
    let mut request_body = HashMap::new();
    request_body.insert("username", "shivam");
    request_body.insert("password", "abcd1234");
    let client2 = reqwest::Client::new();
    let auth_response = client2.post("http://localhost:3000/v1/auth")
        .json(&request_body)
        .send()
        .await?;
    let jwt_token = auth_response
        .json::<model::AuthModel>()
        .await?;
    println!("The token is: {:?}", jwt_token.access_token);


    let header_str = "Bearer ".to_owned() + &jwt_token.access_token;


    let greeting_response = client2.get("http://localhost:3000/v1/hello")
        .header(AUTHORIZATION, header_str.clone())
        .send()
        .await?;
    let received_greeting = greeting_response
        .json::<model::HelloModel>()
        .await?;
    println!("The greeting is: {:?}", received_greeting.greeting);


    let weather_response = client2.get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, &header_str)
        .send()
        .await?;
    let received_weather = weather_response
        .json::<model::Weather>()
        .await?;
    println!("The temperature is: {:?}", received_weather.main.temp);


    Ok(())
}
