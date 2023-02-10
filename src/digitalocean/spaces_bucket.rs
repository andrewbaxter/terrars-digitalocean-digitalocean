use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct SpacesBucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_rule: Option<Vec<SpacesBucketCorsRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_rule: Option<Vec<SpacesBucketLifecycleRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versioning: Option<Vec<SpacesBucketVersioningEl>>,
    dynamic: SpacesBucketDynamic,
}

struct SpacesBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpacesBucketData>,
}

#[derive(Clone)]
pub struct SpacesBucket(Rc<SpacesBucket_>);

impl SpacesBucket {
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

    #[doc= "Set the field `acl`.\nCanned ACL applied on bucket creation"]
    pub fn set_acl(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().acl = Some(v.into());
        self
    }

    #[doc= "Set the field `force_destroy`.\nUnless true, the bucket will only be destroyed if empty"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nBucket region"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `cors_rule`.\n"]
    pub fn set_cors_rule(self, v: impl Into<BlockAssignable<SpacesBucketCorsRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `lifecycle_rule`.\n"]
    pub fn set_lifecycle_rule(self, v: impl Into<BlockAssignable<SpacesBucketLifecycleRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `versioning`.\n"]
    pub fn set_versioning(self, v: impl Into<BlockAssignable<SpacesBucketVersioningEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().versioning = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.versioning = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `acl` after provisioning.\nCanned ACL applied on bucket creation"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_domain_name` after provisioning.\nThe FQDN of the bucket"]
    pub fn bucket_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nThe FQDN of the bucket without the bucket name"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_destroy` after provisioning.\nUnless true, the bucket will only be destroyed if empty"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nBucket name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nBucket region"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the bucket"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cors_rule` after provisioning.\n"]
    pub fn cors_rule(&self) -> ListRef<SpacesBucketCorsRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> ListRef<SpacesBucketLifecycleRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> ListRef<SpacesBucketVersioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }
}

impl Resource for SpacesBucket {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for SpacesBucket {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for SpacesBucket {
    type O = ListRef<SpacesBucketRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for SpacesBucket_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_spaces_bucket".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpacesBucket {
    pub tf_id: String,
    #[doc= "Bucket name"]
    pub name: PrimField<String>,
}

impl BuildSpacesBucket {
    pub fn build(self, stack: &mut Stack) -> SpacesBucket {
        let out = SpacesBucket(Rc::new(SpacesBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpacesBucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acl: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                cors_rule: core::default::Default::default(),
                lifecycle_rule: core::default::Default::default(),
                versioning: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpacesBucketRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpacesBucketRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `acl` after provisioning.\nCanned ACL applied on bucket creation"]
    pub fn acl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_domain_name` after provisioning.\nThe FQDN of the bucket"]
    pub fn bucket_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nThe FQDN of the bucket without the bucket name"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `force_destroy` after provisioning.\nUnless true, the bucket will only be destroyed if empty"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nBucket name"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nBucket region"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the bucket"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cors_rule` after provisioning.\n"]
    pub fn cors_rule(&self) -> ListRef<SpacesBucketCorsRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_rule` after provisioning.\n"]
    pub fn lifecycle_rule(&self) -> ListRef<SpacesBucketLifecycleRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `versioning` after provisioning.\n"]
    pub fn versioning(&self) -> ListRef<SpacesBucketVersioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.versioning", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketCorsRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_headers: Option<ListField<PrimField<String>>>,
    allowed_methods: ListField<PrimField<String>>,
    allowed_origins: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_seconds: Option<PrimField<f64>>,
}

impl SpacesBucketCorsRuleEl {
    #[doc= "Set the field `allowed_headers`.\nA list of headers that will be included in the CORS preflight request's Access-Control-Request-Headers. A header may contain one wildcard (e.g. x-amz-*)."]
    pub fn set_allowed_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for SpacesBucketCorsRuleEl {
    type O = BlockAssignable<SpacesBucketCorsRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketCorsRuleEl {
    #[doc= "A list of HTTP methods (e.g. GET) which are allowed from the specified origin."]
    pub allowed_methods: ListField<PrimField<String>>,
    #[doc= "A list of hosts from which requests using the specified methods are allowed. A host may contain one wildcard (e.g. http://*.example.com)."]
    pub allowed_origins: ListField<PrimField<String>>,
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
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketCorsRuleElRef {
    fn new(shared: StackShared, base: String) -> SpacesBucketCorsRuleElRef {
        SpacesBucketCorsRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketCorsRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_headers` after provisioning.\nA list of headers that will be included in the CORS preflight request's Access-Control-Request-Headers. A header may contain one wildcard (e.g. x-amz-*)."]
    pub fn allowed_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_methods` after provisioning.\nA list of HTTP methods (e.g. GET) which are allowed from the specified origin."]
    pub fn allowed_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_origins` after provisioning.\nA list of hosts from which requests using the specified methods are allowed. A host may contain one wildcard (e.g. http://*.example.com)."]
    pub fn allowed_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketLifecycleRuleElExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expired_object_delete_marker: Option<PrimField<bool>>,
}

impl SpacesBucketLifecycleRuleElExpirationEl {
    #[doc= "Set the field `date`.\n"]
    pub fn set_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.date = Some(v.into());
        self
    }

    #[doc= "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }

    #[doc= "Set the field `expired_object_delete_marker`.\n"]
    pub fn set_expired_object_delete_marker(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.expired_object_delete_marker = Some(v.into());
        self
    }
}

impl ToListMappable for SpacesBucketLifecycleRuleElExpirationEl {
    type O = BlockAssignable<SpacesBucketLifecycleRuleElExpirationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketLifecycleRuleElExpirationElRef {
    fn new(shared: StackShared, base: String) -> SpacesBucketLifecycleRuleElExpirationElRef {
        SpacesBucketLifecycleRuleElExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketLifecycleRuleElExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `date` after provisioning.\n"]
    pub fn date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date", self.base))
    }

    #[doc= "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc= "Get a reference to the value of field `expired_object_delete_marker` after provisioning.\n"]
    pub fn expired_object_delete_marker(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.expired_object_delete_marker", self.base))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<PrimField<f64>>,
}

impl SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    #[doc= "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days = Some(v.into());
        self
    }
}

impl ToListMappable for SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl {
    type O = BlockAssignable<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    fn new(shared: StackShared, base: String) -> SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
        SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketLifecycleRuleElNoncurrentVersionExpirationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpacesBucketLifecycleRuleElDynamic {
    expiration: Option<DynamicBlock<SpacesBucketLifecycleRuleElExpirationEl>>,
    noncurrent_version_expiration: Option<DynamicBlock<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
}

#[derive(Serialize)]
pub struct SpacesBucketLifecycleRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort_incomplete_multipart_upload_days: Option<PrimField<f64>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<Vec<SpacesBucketLifecycleRuleElExpirationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noncurrent_version_expiration: Option<Vec<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    dynamic: SpacesBucketLifecycleRuleElDynamic,
}

impl SpacesBucketLifecycleRuleEl {
    #[doc= "Set the field `abort_incomplete_multipart_upload_days`.\n"]
    pub fn set_abort_incomplete_multipart_upload_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.abort_incomplete_multipart_upload_days = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `expiration`.\n"]
    pub fn set_expiration(mut self, v: impl Into<BlockAssignable<SpacesBucketLifecycleRuleElExpirationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expiration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `noncurrent_version_expiration`.\n"]
    pub fn set_noncurrent_version_expiration(
        mut self,
        v: impl Into<BlockAssignable<SpacesBucketLifecycleRuleElNoncurrentVersionExpirationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.noncurrent_version_expiration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.noncurrent_version_expiration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for SpacesBucketLifecycleRuleEl {
    type O = BlockAssignable<SpacesBucketLifecycleRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketLifecycleRuleEl {
    #[doc= ""]
    pub enabled: PrimField<bool>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct SpacesBucketLifecycleRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketLifecycleRuleElRef {
    fn new(shared: StackShared, base: String) -> SpacesBucketLifecycleRuleElRef {
        SpacesBucketLifecycleRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketLifecycleRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `abort_incomplete_multipart_upload_days` after provisioning.\n"]
    pub fn abort_incomplete_multipart_upload_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.abort_incomplete_multipart_upload_days", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketVersioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl SpacesBucketVersioningEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for SpacesBucketVersioningEl {
    type O = BlockAssignable<SpacesBucketVersioningEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketVersioningElRef {
    fn new(shared: StackShared, base: String) -> SpacesBucketVersioningElRef {
        SpacesBucketVersioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketVersioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpacesBucketDynamic {
    cors_rule: Option<DynamicBlock<SpacesBucketCorsRuleEl>>,
    lifecycle_rule: Option<DynamicBlock<SpacesBucketLifecycleRuleEl>>,
    versioning: Option<DynamicBlock<SpacesBucketVersioningEl>>,
}
