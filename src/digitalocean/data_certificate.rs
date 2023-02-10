use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataCertificateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataCertificate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCertificateData>,
}

#[derive(Clone)]
pub struct DataCertificate(Rc<DataCertificate_>);

impl DataCertificate {
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

    #[doc= "Get a reference to the value of field `domains` after provisioning.\ndomains for which the certificate was issued"]
    pub fn domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the certificate"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_after` after provisioning.\nexpiration date and time of the certificate"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha1_fingerprint` after provisioning.\nSHA1 fingerprint of the certificate"]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha1_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\ncurrent state of the certificate"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\ntype of the certificate"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\nuuid of the certificate"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

impl Datasource for DataCertificate {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataCertificate {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataCertificate {
    type O = ListRef<DataCertificateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataCertificate_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_certificate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCertificate {
    pub tf_id: String,
    #[doc= "name of the certificate"]
    pub name: PrimField<String>,
}

impl BuildDataCertificate {
    pub fn build(self, stack: &mut Stack) -> DataCertificate {
        let out = DataCertificate(Rc::new(DataCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCertificateData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCertificateRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCertificateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCertificateRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\ndomains for which the certificate was issued"]
    pub fn domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the certificate"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `not_after` after provisioning.\nexpiration date and time of the certificate"]
    pub fn not_after(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sha1_fingerprint` after provisioning.\nSHA1 fingerprint of the certificate"]
    pub fn sha1_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha1_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\ncurrent state of the certificate"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\ntype of the certificate"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\nuuid of the certificate"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}
