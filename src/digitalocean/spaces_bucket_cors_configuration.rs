use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct SpacesBucketCorsConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_rule: Option<Vec<SpacesBucketCorsConfigurationCorsRuleEl>>,
    dynamic: SpacesBucketCorsConfigurationDynamic,
}

struct SpacesBucketCorsConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpacesBucketCorsConfigurationData>,
}

#[derive(Clone)]
pub struct SpacesBucketCorsConfiguration(Rc<SpacesBucketCorsConfiguration_>);

impl SpacesBucketCorsConfiguration {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `cors_rule`.\n"]
    pub fn set_cors_rule(self, v: impl Into<BlockAssignable<SpacesBucketCorsConfigurationCorsRuleEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\nBucket ID"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for SpacesBucketCorsConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for SpacesBucketCorsConfiguration { }

impl ToListMappable for SpacesBucketCorsConfiguration {
    type O = ListRef<SpacesBucketCorsConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SpacesBucketCorsConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_spaces_bucket_cors_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpacesBucketCorsConfiguration {
    pub tf_id: String,
    #[doc= "Bucket ID"]
    pub bucket: PrimField<String>,
    #[doc= ""]
    pub region: PrimField<String>,
}

impl BuildSpacesBucketCorsConfiguration {
    pub fn build(self, stack: &mut Stack) -> SpacesBucketCorsConfiguration {
        let out = SpacesBucketCorsConfiguration(Rc::new(SpacesBucketCorsConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpacesBucketCorsConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket: self.bucket,
                id: core::default::Default::default(),
                region: self.region,
                cors_rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpacesBucketCorsConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketCorsConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpacesBucketCorsConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\nBucket ID"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpacesBucketCorsConfigurationCorsRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_headers: Option<SetField<PrimField<String>>>,
    allowed_methods: SetField<PrimField<String>>,
    allowed_origins: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_seconds: Option<PrimField<f64>>,
}

impl SpacesBucketCorsConfigurationCorsRuleEl {
    #[doc= "Set the field `allowed_headers`.\n"]
    pub fn set_allowed_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for SpacesBucketCorsConfigurationCorsRuleEl {
    type O = BlockAssignable<SpacesBucketCorsConfigurationCorsRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpacesBucketCorsConfigurationCorsRuleEl {
    #[doc= ""]
    pub allowed_methods: SetField<PrimField<String>>,
    #[doc= ""]
    pub allowed_origins: SetField<PrimField<String>>,
}

impl BuildSpacesBucketCorsConfigurationCorsRuleEl {
    pub fn build(self) -> SpacesBucketCorsConfigurationCorsRuleEl {
        SpacesBucketCorsConfigurationCorsRuleEl {
            allowed_headers: core::default::Default::default(),
            allowed_methods: self.allowed_methods,
            allowed_origins: self.allowed_origins,
            expose_headers: core::default::Default::default(),
            id: core::default::Default::default(),
            max_age_seconds: core::default::Default::default(),
        }
    }
}

pub struct SpacesBucketCorsConfigurationCorsRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SpacesBucketCorsConfigurationCorsRuleElRef {
    fn new(shared: StackShared, base: String) -> SpacesBucketCorsConfigurationCorsRuleElRef {
        SpacesBucketCorsConfigurationCorsRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpacesBucketCorsConfigurationCorsRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_headers` after provisioning.\n"]
    pub fn allowed_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_origins` after provisioning.\n"]
    pub fn allowed_origins(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpacesBucketCorsConfigurationDynamic {
    cors_rule: Option<DynamicBlock<SpacesBucketCorsConfigurationCorsRuleEl>>,
}
