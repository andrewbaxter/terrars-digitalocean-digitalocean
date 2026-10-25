#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct NfsAccessPointData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
    share_id: terrars::PrimField<std::string::String>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_policy: std::option::Option<std::vec::Vec<NfsAccessPoint_AccessPolicy>>,
    dynamic: NfsAccessPointDynamic,
}

struct NfsAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NfsAccessPointData_>,
}

#[derive(Clone)]
pub struct NfsAccessPoint(std::rc::Rc<NfsAccessPoint_>);

impl NfsAccessPoint {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_policy`.\n"]
    pub fn set_access_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NfsAccessPoint_AccessPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_id` after provisioning.\n"]
    pub fn share_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_policy` after provisioning.\n"]
    pub fn access_policy(&self) -> terrars::ListRef<NfsAccessPoint_AccessPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_policy", self.extract_ref()))
    }
}

impl terrars::Referable for NfsAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NfsAccessPoint { }

impl terrars::ToListMappable for NfsAccessPoint {
    type O = terrars::ListRef<NfsAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NfsAccessPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_nfs_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNfsAccessPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub share_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildNfsAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> NfsAccessPoint {
        let out = NfsAccessPoint(std::rc::Rc::new(NfsAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NfsAccessPointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                path: self.path,
                share_id: self.share_id,
                vpc_id: self.vpc_id,
                access_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NfsAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NfsAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NfsAccessPointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `share_id` after provisioning.\n"]
    pub fn share_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_policy` after provisioning.\n"]
    pub fn access_policy(&self) -> terrars::ListRef<NfsAccessPoint_AccessPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_policy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NfsAccessPointDynamic {
    access_policy: std::option::Option<terrars::DynamicBlock<NfsAccessPoint_AccessPolicy>>,
}

#[derive(Serialize)]
pub struct NfsAccessPoint_AccessPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anongid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonuid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_enforcement_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    squash_config: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NfsAccessPoint_AccessPolicy {
    #[doc = "Set the field `anongid`.\n"]
    pub fn set_anongid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anongid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anonuid`.\n"]
    pub fn set_anonuid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.anonuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_enforcement_enabled`.\n"]
    pub fn set_identity_enforcement_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.identity_enforcement_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `squash_config`.\n"]
    pub fn set_squash_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.squash_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NfsAccessPoint_AccessPolicy {
    type O = terrars::BlockAssignable<NfsAccessPoint_AccessPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNfsAccessPoint_AccessPolicy {}

impl BuildNfsAccessPoint_AccessPolicy {
    pub fn build(self) -> NfsAccessPoint_AccessPolicy {
        NfsAccessPoint_AccessPolicy {
            anongid: core::default::Default::default(),
            anonuid: core::default::Default::default(),
            identity_enforcement_enabled: core::default::Default::default(),
            protocols: core::default::Default::default(),
            squash_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NfsAccessPoint_AccessPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NfsAccessPoint_AccessPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NfsAccessPoint_AccessPolicyRef {
        NfsAccessPoint_AccessPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NfsAccessPoint_AccessPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `anongid` after provisioning.\n"]
    pub fn anongid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anongid", self.base))
    }

    #[doc = "Get a reference to the value of field `anonuid` after provisioning.\n"]
    pub fn anonuid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.anonuid", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_enforcement_enabled` after provisioning.\n"]
    pub fn identity_enforcement_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_enforcement_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `squash_config` after provisioning.\n"]
    pub fn squash_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash_config", self.base))
    }
}
