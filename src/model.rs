//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub(crate) main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub(crate) temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthModel {
    #[serde(rename = "access-token")]
    pub(crate) access_token: String,
    expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloModel {
    pub(crate) greeting: String,
}

