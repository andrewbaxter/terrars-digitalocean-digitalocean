use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccountData>,
}

#[derive(Clone)]
pub struct DataAccount(Rc<DataAccount_>);

impl DataAccount {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `droplet_limit` after provisioning.\nThe total number of Droplets current user or team may have active at one time."]
    pub fn droplet_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe email address used by the current user to register for DigitalOcean."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email_verified` after provisioning.\nIf true, the user has verified their account via email. False otherwise."]
    pub fn email_verified(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_verified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `floating_ip_limit` after provisioning.\nThe total number of Floating IPs the current user or team may have."]
    pub fn floating_ip_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.floating_ip_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThis value is one of \"active\", \"warning\" or \"locked\"."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status_message` after provisioning.\nA human-readable message giving more details about the status of the account."]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\nThe unique universal identifier for the current user."]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

impl Datasource for DataAccount {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataAccount {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataAccount {
    type O = ListRef<DataAccountRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAccount_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_account".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccount {
    pub tf_id: String,
}

impl BuildDataAccount {
    pub fn build(self, stack: &mut Stack) -> DataAccount {
        let out = DataAccount(Rc::new(DataAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAccountData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `droplet_limit` after provisioning.\nThe total number of Droplets current user or team may have active at one time."]
    pub fn droplet_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe email address used by the current user to register for DigitalOcean."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `email_verified` after provisioning.\nIf true, the user has verified their account via email. False otherwise."]
    pub fn email_verified(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.email_verified", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `floating_ip_limit` after provisioning.\nThe total number of Floating IPs the current user or team may have."]
    pub fn floating_ip_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.floating_ip_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThis value is one of \"active\", \"warning\" or \"locked\"."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status_message` after provisioning.\nA human-readable message giving more details about the status of the account."]
    pub fn status_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\nThe unique universal identifier for the current user."]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}
