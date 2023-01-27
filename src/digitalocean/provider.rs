use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderDigitaloceanData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spaces_access_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spaces_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spaces_secret_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

struct ProviderDigitalocean_ {
    data: RefCell<ProviderDigitaloceanData>,
}

pub struct ProviderDigitalocean(Rc<ProviderDigitalocean_>);

impl ProviderDigitalocean {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "digitalocean", alias)
        } else {
            "digitalocean".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `api_endpoint`.\nThe URL to use for the DigitalOcean API."]
    pub fn set_api_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `spaces_access_id`.\nThe access key ID for Spaces API operations."]
    pub fn set_spaces_access_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spaces_access_id = Some(v.into());
        self
    }

    #[doc= "Set the field `spaces_endpoint`.\nThe URL to use for the DigitalOcean Spaces API."]
    pub fn set_spaces_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spaces_endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `spaces_secret_key`.\nThe secret access key for Spaces API operations."]
    pub fn set_spaces_secret_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().spaces_secret_key = Some(v.into());
        self
    }

    #[doc= "Set the field `token`.\nThe token key for API operations."]
    pub fn set_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().token = Some(v.into());
        self
    }
}

impl Provider for ProviderDigitalocean_ {
    fn extract_type_tf_id(&self) -> String {
        "digitalocean".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "digitalocean/digitalocean",
            "version": "2.25.2",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderDigitalocean {}

impl BuildProviderDigitalocean {
    pub fn build(self, stack: &mut Stack) -> ProviderDigitalocean {
        let out = ProviderDigitalocean(Rc::new(ProviderDigitalocean_ { data: RefCell::new(ProviderDigitaloceanData {
            alias: None,
            api_endpoint: core::default::Default::default(),
            spaces_access_id: core::default::Default::default(),
            spaces_endpoint: core::default::Default::default(),
            spaces_secret_key: core::default::Default::default(),
            token: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
