use serde::Serialize;

#[derive(Serialize)]
struct ProviderDigitaloceanData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_wait_max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_wait_min: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spaces_access_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spaces_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spaces_secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderDigitalocean_ {
    data: std::cell::RefCell<ProviderDigitaloceanData>,
}

pub struct ProviderDigitalocean(std::rc::Rc<ProviderDigitalocean_>);

impl ProviderDigitalocean {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "digitalocean", alias)
        } else {
            "digitalocean".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `api_endpoint`.\nThe URL to use for the DigitalOcean API."]
    pub fn set_api_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_retry_max`.\nThe maximum number of retries on a failed API request."]
    pub fn set_http_retry_max(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_retry_max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_retry_wait_max`.\nThe maximum wait time (in seconds) between failed API requests."]
    pub fn set_http_retry_wait_max(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_retry_wait_max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_retry_wait_min`.\nThe minimum wait time (in seconds) between failed API requests."]
    pub fn set_http_retry_wait_min(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_retry_wait_min = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests_per_second`.\nThe rate of requests per second to limit the HTTP client."]
    pub fn set_requests_per_second(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().requests_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spaces_access_id`.\nThe access key ID for Spaces API operations."]
    pub fn set_spaces_access_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().spaces_access_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spaces_endpoint`.\nThe URL to use for the DigitalOcean Spaces API."]
    pub fn set_spaces_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().spaces_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spaces_secret_key`.\nThe secret access key for Spaces API operations."]
    pub fn set_spaces_secret_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().spaces_secret_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token`.\nThe token key for API operations."]
    pub fn set_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderDigitalocean_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "digitalocean".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "digitalocean/digitalocean",
            "version": "2.34.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDigitalocean {}

impl BuildProviderDigitalocean {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderDigitalocean {
        let out =
            ProviderDigitalocean(
                std::rc::Rc::new(ProviderDigitalocean_ { data: std::cell::RefCell::new(ProviderDigitaloceanData {
                    alias: std::option::Option::None,
                    api_endpoint: core::default::Default::default(),
                    http_retry_max: core::default::Default::default(),
                    http_retry_wait_max: core::default::Default::default(),
                    http_retry_wait_min: core::default::Default::default(),
                    requests_per_second: core::default::Default::default(),
                    spaces_access_id: core::default::Default::default(),
                    spaces_endpoint: core::default::Default::default(),
                    spaces_secret_key: core::default::Default::default(),
                    token: core::default::Default::default(),
                }) }),
            );
        stack.add_provider(out.0.clone());
        out
    }
}
