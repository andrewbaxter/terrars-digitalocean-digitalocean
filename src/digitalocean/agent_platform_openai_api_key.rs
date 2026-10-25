#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AgentPlatformOpenaiApiKeyData_ {
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
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<std::vec::Vec<AgentPlatformOpenaiApiKey_Model>>,
    dynamic: AgentPlatformOpenaiApiKeyDynamic,
}

struct AgentPlatformOpenaiApiKey_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgentPlatformOpenaiApiKeyData_>,
}

#[derive(Clone)]
pub struct AgentPlatformOpenaiApiKey(std::rc::Rc<AgentPlatformOpenaiApiKey_>);

impl AgentPlatformOpenaiApiKey {
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

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformOpenaiApiKey_Model>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nThe OpenAI API key."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nWhen the API key was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\nWho created the API key."]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deleted_at` after provisioning.\nWhen the API key was deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA name for the API key."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nWhen the API key was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nThe UUID of the API key."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<AgentPlatformOpenaiApiKey_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }
}

impl terrars::Referable for AgentPlatformOpenaiApiKey {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgentPlatformOpenaiApiKey { }

impl terrars::ToListMappable for AgentPlatformOpenaiApiKey {
    type O = terrars::ListRef<AgentPlatformOpenaiApiKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgentPlatformOpenaiApiKey_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_openai_api_key".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgentPlatformOpenaiApiKey {
    pub tf_id: std::string::String,
    #[doc = "The OpenAI API key."]
    pub api_key: terrars::PrimField<std::string::String>,
    #[doc = "A name for the API key."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAgentPlatformOpenaiApiKey {
    pub fn build(self, stack: &mut terrars::Stack) -> AgentPlatformOpenaiApiKey {
        let out = AgentPlatformOpenaiApiKey(std::rc::Rc::new(AgentPlatformOpenaiApiKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgentPlatformOpenaiApiKeyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_key: self.api_key,
                id: core::default::Default::default(),
                name: self.name,
                model: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgentPlatformOpenaiApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformOpenaiApiKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgentPlatformOpenaiApiKeyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nThe OpenAI API key."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nWhen the API key was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\nWho created the API key."]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deleted_at` after provisioning.\nWhen the API key was deleted."]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nA name for the API key."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nWhen the API key was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nThe UUID of the API key."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<AgentPlatformOpenaiApiKey_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AgentPlatformOpenaiApiKeyDynamic {
    model: std::option::Option<terrars::DynamicBlock<AgentPlatformOpenaiApiKey_Model>>,
}

#[derive(Serialize)]
pub struct AgentPlatformOpenaiApiKey_Model_Agreement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformOpenaiApiKey_Model_Agreement {
    #[doc = "Set the field `description`.\nDescription of the agreement"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the agreement"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nURL of the agreement"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nUUID of the agreement"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformOpenaiApiKey_Model_Agreement {
    type O = terrars::BlockAssignable<AgentPlatformOpenaiApiKey_Model_Agreement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformOpenaiApiKey_Model_Agreement {}

impl BuildAgentPlatformOpenaiApiKey_Model_Agreement {
    pub fn build(self) -> AgentPlatformOpenaiApiKey_Model_Agreement {
        AgentPlatformOpenaiApiKey_Model_Agreement {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformOpenaiApiKey_Model_AgreementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformOpenaiApiKey_Model_AgreementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformOpenaiApiKey_Model_AgreementRef {
        AgentPlatformOpenaiApiKey_Model_AgreementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformOpenaiApiKey_Model_AgreementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the agreement"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the agreement"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL of the agreement"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the agreement"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformOpenaiApiKey_Model_Versions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    major: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch: std::option::Option<terrars::PrimField<f64>>,
}

impl AgentPlatformOpenaiApiKey_Model_Versions {
    #[doc = "Set the field `major`.\nMajor version of the model"]
    pub fn set_major(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.major = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minor`.\nMinor version of the model"]
    pub fn set_minor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patch`.\nPatch version of the model"]
    pub fn set_patch(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.patch = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformOpenaiApiKey_Model_Versions {
    type O = terrars::BlockAssignable<AgentPlatformOpenaiApiKey_Model_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformOpenaiApiKey_Model_Versions {}

impl BuildAgentPlatformOpenaiApiKey_Model_Versions {
    pub fn build(self) -> AgentPlatformOpenaiApiKey_Model_Versions {
        AgentPlatformOpenaiApiKey_Model_Versions {
            major: core::default::Default::default(),
            minor: core::default::Default::default(),
            patch: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformOpenaiApiKey_Model_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformOpenaiApiKey_Model_VersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformOpenaiApiKey_Model_VersionsRef {
        AgentPlatformOpenaiApiKey_Model_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformOpenaiApiKey_Model_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `major` after provisioning.\nMajor version of the model"]
    pub fn major(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major", self.base))
    }

    #[doc = "Get a reference to the value of field `minor` after provisioning.\nMinor version of the model"]
    pub fn minor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minor", self.base))
    }

    #[doc = "Get a reference to the value of field `patch` after provisioning.\nPatch version of the model"]
    pub fn patch(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgentPlatformOpenaiApiKey_ModelDynamic {
    agreement: std::option::Option<terrars::DynamicBlock<AgentPlatformOpenaiApiKey_Model_Agreement>>,
    versions: std::option::Option<terrars::DynamicBlock<AgentPlatformOpenaiApiKey_Model_Versions>>,
}

#[derive(Serialize)]
pub struct AgentPlatformOpenaiApiKey_Model {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_foundational: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_complete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usecases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agreement: std::option::Option<std::vec::Vec<AgentPlatformOpenaiApiKey_Model_Agreement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<std::vec::Vec<AgentPlatformOpenaiApiKey_Model_Versions>>,
    dynamic: AgentPlatformOpenaiApiKey_ModelDynamic,
}

impl AgentPlatformOpenaiApiKey_Model {
    #[doc = "Set the field `inference_name`.\nInference name of the model"]
    pub fn set_inference_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inference_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_version`.\nInfernce version of the model"]
    pub fn set_inference_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inference_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_foundational`.\nIndicates if the Model Base is foundational"]
    pub fn set_is_foundational(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_foundational = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the Knowledge Base"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent_uuid`.\nParent UUID of the Model"]
    pub fn set_parent_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider`.\nProvider of the Model"]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_complete`.\nIndicates if the Model upload is complete"]
    pub fn set_upload_complete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.upload_complete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nURL of the Model"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usecases`.\nList of Usecases for the Model"]
    pub fn set_usecases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.usecases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agreement`.\n"]
    pub fn set_agreement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformOpenaiApiKey_Model_Agreement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agreement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agreement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformOpenaiApiKey_Model_Versions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.versions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.versions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgentPlatformOpenaiApiKey_Model {
    type O = terrars::BlockAssignable<AgentPlatformOpenaiApiKey_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformOpenaiApiKey_Model {}

impl BuildAgentPlatformOpenaiApiKey_Model {
    pub fn build(self) -> AgentPlatformOpenaiApiKey_Model {
        AgentPlatformOpenaiApiKey_Model {
            inference_name: core::default::Default::default(),
            inference_version: core::default::Default::default(),
            is_foundational: core::default::Default::default(),
            name: core::default::Default::default(),
            parent_uuid: core::default::Default::default(),
            provider: core::default::Default::default(),
            upload_complete: core::default::Default::default(),
            url: core::default::Default::default(),
            usecases: core::default::Default::default(),
            agreement: core::default::Default::default(),
            versions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformOpenaiApiKey_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformOpenaiApiKey_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformOpenaiApiKey_ModelRef {
        AgentPlatformOpenaiApiKey_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformOpenaiApiKey_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Knowledge Base"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_name` after provisioning.\nInference name of the model"]
    pub fn inference_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inference_version` after provisioning.\nInfernce version of the model"]
    pub fn inference_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_foundational` after provisioning.\nIndicates if the Model Base is foundational"]
    pub fn is_foundational(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_foundational", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Knowledge Base"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parent_uuid` after provisioning.\nParent UUID of the Model"]
    pub fn parent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\nProvider of the Model"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Knowledge Base was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upload_complete` after provisioning.\nIndicates if the Model upload is complete"]
    pub fn upload_complete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_complete", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL of the Model"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `usecases` after provisioning.\nList of Usecases for the Model"]
    pub fn usecases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usecases", self.base))
    }

    #[doc = "Get a reference to the value of field `agreement` after provisioning.\n"]
    pub fn agreement(&self) -> terrars::ListRef<AgentPlatformOpenaiApiKey_Model_AgreementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agreement", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<AgentPlatformOpenaiApiKey_Model_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}
