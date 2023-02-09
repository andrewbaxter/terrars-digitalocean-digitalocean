use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct CdnData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    origin: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}

struct Cdn_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CdnData>,
}

#[derive(Clone)]
pub struct Cdn(Rc<Cdn_>);

impl Cdn {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `certificate_id`.\nID of a DigitalOcean managed TLS certificate for use with custom domains"]
    pub fn set_certificate_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_id = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate_name`.\n"]
    pub fn set_certificate_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_name = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_domain`.\nfully qualified domain name (FQDN) for custom subdomain, (requires certificate_id)"]
    pub fn set_custom_domain(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_domain = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ttl`.\nThe amount of time the content is cached in the CDN"]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `certificate_id` after provisioning.\nID of a DigitalOcean managed TLS certificate for use with custom domains"]
    pub fn certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date and time (ISO8601) of when the CDN endpoint was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_domain` after provisioning.\nfully qualified domain name (FQDN) for custom subdomain, (requires certificate_id)"]
    pub fn custom_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nfully qualified domain name (FQDN) to serve the CDN content"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin` after provisioning.\nfully qualified domain name (FQDN) for the origin server"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nThe amount of time the content is cached in the CDN"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}

impl Resource for Cdn {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Cdn {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Cdn {
    type O = ListRef<CdnRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Cdn_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_cdn".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCdn {
    pub tf_id: String,
    #[doc= "fully qualified domain name (FQDN) for the origin server"]
    pub origin: PrimField<String>,
}

impl BuildCdn {
    pub fn build(self, stack: &mut Stack) -> Cdn {
        let out = Cdn(Rc::new(Cdn_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CdnData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                certificate_id: core::default::Default::default(),
                certificate_name: core::default::Default::default(),
                custom_domain: core::default::Default::default(),
                id: core::default::Default::default(),
                origin: self.origin,
                ttl: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CdnRef {
    shared: StackShared,
    base: String,
}

impl Ref for CdnRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CdnRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate_id` after provisioning.\nID of a DigitalOcean managed TLS certificate for use with custom domains"]
    pub fn certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date and time (ISO8601) of when the CDN endpoint was created."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_domain` after provisioning.\nfully qualified domain name (FQDN) for custom subdomain, (requires certificate_id)"]
    pub fn custom_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nfully qualified domain name (FQDN) to serve the CDN content"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `origin` after provisioning.\nfully qualified domain name (FQDN) for the origin server"]
    pub fn origin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nThe amount of time the content is cached in the CDN"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
}
