use actix_web::{error, Error};
use opensearch::http::response::Response;

use crate::dto::User;
use serde_json::{json, Value};

// might be moved into separate users.rs & messages.rs
pub async fn parse_user_search(response: Response) -> Result<User, Error> {
    if !response.status_code().is_success() {
        return Err(error::ErrorServiceUnavailable(format!(
            "OS server returned: {}",
            response.status_code().to_string()
        )));
    }

    let json = response.json::<Value>().await;

    match json {
        Ok(mut j) => {
            if j["hits"]["total"]["value"].as_i64().unwrap() == 0 {
                Err(error::ErrorNotFound("User not found")) // error
            } else {
                let user_json = j["hits"]["hits"][0]["_source"].take();
                let user: User = serde_json::from_value(user_json).unwrap();

                Ok(user)
            }
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}

pub async fn parse_message_search(response: Response) -> Result<Vec<Value>, Error> {
    if !response.status_code().is_success() {
        return Err(error::ErrorServiceUnavailable(format!(
            "OS server returned: {}",
            response.status_code().to_string()
        )));
    }

    let mut json: Value = response.json().await.unwrap();
    let num_messages = json["hits"]["total"]["value"].as_u64().unwrap();

    if num_messages == 0 {
        Ok(Vec::new())
    } else {
        let hits = json["hits"]["hits"].as_array_mut().unwrap();
        let transformed: Vec<Value> = hits
            .iter_mut()
            .map(|v| v.as_object_mut().unwrap().remove("_source").unwrap())
            .collect();

        Ok(transformed)
    }

    // return straight Vec of Values to avoid extra deserialize-serialize cycle
}
