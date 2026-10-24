use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct SpacesBucketData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_rule: std::option::Option<std::vec::Vec<SpacesBucketCorsRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_rule: std::option::Option<std::vec::Vec<SpacesBucketLifecycleRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioning: std::option::Option<std::vec::Vec<SpacesBucketVersioningEl>>,
    dynamic: SpacesBucketDynamic,
}

struct SpacesBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpacesBucketData>,
}

#[derive(Clone)]
pub struct SpacesBucket(std::rc::Rc<SpacesBucket_>);

impl SpacesBucket {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `acl`.\nCanned ACL applied on bucket creation"]
    pub fn set_acl(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().acl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\nUnless true, the bucket will only be destroyed if empty"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nBucket region"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors_rule`.\n"]
    pub fn set_cors_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<SpacesBucketCorsRuleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_rule`.\n"]
    pub fn set_lifecycle_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpacesBucketLifecycleRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `versioning`.\n"]
    pub fn set_versioning(self, v: impl std::convert::Into<terrars::BlockAssignable<SpacesBucketVersioningEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\nCanned ACL applied on bucket creation"]
    pub fn acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_domain_name` after provisioning.\nThe FQDN of the bucket"]
    pub fn bucket_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe FQDN of the bucket without the bucket name"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nUnless true, the bucket will only be destroyed if empty"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nBucket name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nBucket region"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the bucket"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_rule` after provisioning.\n"]
    pub fn cors_rule(&self) -> terrars::ListRef<SpacesBucketCorsRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<SpacesBucketLifecycleRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> terrars::ListRef<SpacesBucketVersioningElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }
}

impl terrars::Referable for SpacesBucket {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpacesBucket { }

impl terrars::ToListMappable for SpacesBucket {
    type O = terrars::ListRef<SpacesBucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpacesBucket_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_spaces_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpacesBucket {
    pub tf_id: std::string::String,
    #[doc = "Bucket name"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSpacesBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> SpacesBucket {
        let out = SpacesBucket(std::rc::Rc::new(SpacesBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpacesBucketData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                acl: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                cors_rule: core::default::Default::default(),
                lifecycle_rule: core::default::Default::default(),
                versioning: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpacesBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpacesBucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpacesBucketRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\nCanned ACL applied on bucket creation"]
    pub fn acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_domain_name` after provisioning.\nThe FQDN of the bucket"]
    pub fn bucket_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe FQDN of the bucket without the bucket name"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nUnless true, the bucket will only be destroyed if empty"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nBucket name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nBucket region"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the bucket"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_rule` after provisioning.\n"]
    pub fn cors_rule(&self) -> terrars::ListRef<SpacesBucketCorsRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> terrars::ListRef<SpacesBucketLifecycleRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> terrars::ListRef<SpacesBucketVersioningElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketCorsRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    allowed_methods: terrars::ListField<terrars::PrimField<std::string::String>>,
    allowed_origins: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SpacesBucketCorsRuleEl {
    #[doc =
        "Set the field `allowed_headers`.\nA list of headers that will be included in the CORS preflight request's Access-Control-Request-Headers. A header may contain one wildcard (e.g. x-amz-*)."]
    pub fn set_allowed_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpacesBucketCorsRuleEl {
    type O = terrars::BlockAssignable<SpacesBucketCorsRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketCorsRuleEl {
    #[doc = "A list of HTTP methods (e.g. GET) which are allowed from the specified origin."]
    pub allowed_methods: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "A list of hosts from which requests using the specified methods are allowed. A host may contain one wildcard (e.g. http://*.example.com)."]
    pub allowed_origins: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSpacesBucketCorsRuleEl {
    pub fn build(self) -> SpacesBucketCorsRuleEl {
        SpacesBucketCorsRuleEl {
            allowed_headers: core::default::Default::default(),
            allowed_methods: self.allowed_methods,
            allowed_origins: self.allowed_origins,
            max_age_seconds: core::default::Default::default(),
        }
    }
}

pub struct SpacesBucketCorsRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpacesBucketCorsRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpacesBucketCorsRuleElRef {
        SpacesBucketCorsRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketCorsRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_headers` after provisioning.\nA list of headers that will be included in the CORS preflight request's Access-Control-Request-Headers. A header may contain one wildcard (e.g. x-amz-*)."]
    pub fn allowed_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_methods` after provisioning.\nA list of HTTP methods (e.g. GET) which are allowed from the specified origin."]
    pub fn allowed_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nA list of hosts from which requests using the specified methods are allowed. A host may contain one wildcard (e.g. http://*.example.com)."]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketLifecycleRuleElExpirationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expired_object_delete_marker: std::option::Option<terrars::PrimField<bool>>,
}

impl SpacesBucketLifecycleRuleElExpirationEl {
    #[doc = "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expired_object_delete_marker`.\n"]
    pub fn set_expired_object_delete_marker(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.expired_object_delete_marker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpacesBucketLifecycleRuleElExpirationEl {
    type O = terrars::BlockAssignable<SpacesBucketLifecycleRuleElExpirationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketLifecycleRuleElExpirationEl {}

impl BuildSpacesBucketLifecycleRuleElExpirationEl {
    pub fn build(self) -> SpacesBucketLifecycleRuleElExpirationEl {
        SpacesBucketLifecycleRuleElExpirationEl {
            date: core::default::Default::default(),
            days: core::default::Default::default(),
            expired_object_delete_marker: core::default::Default::default(),
        }
    }
}

pub struct SpacesBucketLifecycleRuleElExpirationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpacesBucketLifecycleRuleElExpirationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpacesBucketLifecycleRuleElExpirationElRef {
        SpacesBucketLifecycleRuleElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketLifecycleRuleElExpirationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `expired_object_delete_marker` after provisioning.\n"]
    pub fn expired_object_delete_marker(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expired_object_delete_marker", self.base))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
}

impl SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    type O = terrars::BlockAssignable<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {}

impl BuildSpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    pub fn build(self) -> SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
        SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl { days: core::default::Default::default() }
    }
}

pub struct SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
        SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpacesBucketLifecycleRuleElDynamic {
    expiration: std::option::Option<terrars::DynamicBlock<SpacesBucketLifecycleRuleElExpirationEl>>,
    noncurrent_version_expiration: std
    ::option
    ::Option<terrars::DynamicBlock<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
}

#[derive(Serialize)]
pub struct SpacesBucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort_incomplete_multipart_upload_days: std::option::Option<terrars::PrimField<f64>>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<std::vec::Vec<SpacesBucketLifecycleRuleElExpirationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noncurrent_version_expiration: std
    ::option
    ::Option<std::vec::Vec<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    dynamic: SpacesBucketLifecycleRuleElDynamic,
}

impl SpacesBucketLifecycleRuleEl {
    #[doc = "Set the field `abort_incomplete_multipart_upload_days`.\n"]
    pub fn set_abort_incomplete_multipart_upload_days(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.abort_incomplete_multipart_upload_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpacesBucketLifecycleRuleElExpirationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expiration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `noncurrent_version_expiration`.\n"]
    pub fn set_noncurrent_version_expiration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.noncurrent_version_expiration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_expiration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpacesBucketLifecycleRuleEl {
    type O = terrars::BlockAssignable<SpacesBucketLifecycleRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketLifecycleRuleEl {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildSpacesBucketLifecycleRuleEl {
    pub fn build(self) -> SpacesBucketLifecycleRuleEl {
        SpacesBucketLifecycleRuleEl {
            abort_incomplete_multipart_upload_days: core::default::Default::default(),
            enabled: self.enabled,
            id: core::default::Default::default(),
            prefix: core::default::Default::default(),
            expiration: core::default::Default::default(),
            noncurrent_version_expiration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SpacesBucketLifecycleRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpacesBucketLifecycleRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpacesBucketLifecycleRuleElRef {
        SpacesBucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketLifecycleRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort_incomplete_multipart_upload_days` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.abort_incomplete_multipart_upload_days", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketVersioningEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl SpacesBucketVersioningEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpacesBucketVersioningEl {
    type O = terrars::BlockAssignable<SpacesBucketVersioningEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketVersioningEl {}

impl BuildSpacesBucketVersioningEl {
    pub fn build(self) -> SpacesBucketVersioningEl {
        SpacesBucketVersioningEl { enabled: core::default::Default::default() }
    }
}

pub struct SpacesBucketVersioningElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpacesBucketVersioningElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpacesBucketVersioningElRef {
        SpacesBucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketVersioningElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpacesBucketDynamic {
    cors_rule: std::option::Option<terrars::DynamicBlock<SpacesBucketCorsRuleEl>>,
    lifecycle_rule: std::option::Option<terrars::DynamicBlock<SpacesBucketLifecycleRuleEl>>,
    versioning: std::option::Option<terrars::DynamicBlock<SpacesBucketVersioningEl>>,
}
