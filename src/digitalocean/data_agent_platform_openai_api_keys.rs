#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformOpenaiApiKeysData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataAgentPlatformOpenaiApiKeys_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<std::vec::Vec<DataAgentPlatformOpenaiApiKeys_Sort>>,
    dynamic: DataAgentPlatformOpenaiApiKeysDynamic,
}

struct DataAgentPlatformOpenaiApiKeys_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformOpenaiApiKeysData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformOpenaiApiKeys(std::rc::Rc<DataAgentPlatformOpenaiApiKeys_>);

impl DataAgentPlatformOpenaiApiKeys {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort`.\n"]
    pub fn set_sort(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_Sort>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openai_api_keys` after provisioning.\n"]
    pub fn openai_api_keys(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openai_api_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformOpenaiApiKeys {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformOpenaiApiKeys { }

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys {
    type O = terrars::ListRef<DataAgentPlatformOpenaiApiKeysRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformOpenaiApiKeys_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_openai_api_keys".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys {
    pub tf_id: std::string::String,
}

impl BuildDataAgentPlatformOpenaiApiKeys {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformOpenaiApiKeys {
        let out = DataAgentPlatformOpenaiApiKeys(std::rc::Rc::new(DataAgentPlatformOpenaiApiKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformOpenaiApiKeysData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformOpenaiApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformOpenaiApiKeysRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openai_api_keys` after provisioning.\n"]
    pub fn openai_api_keys(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openai_api_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataAgentPlatformOpenaiApiKeysDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataAgentPlatformOpenaiApiKeys_Filter>>,
    sort: std::option::Option<terrars::DynamicBlock<DataAgentPlatformOpenaiApiKeys_Sort>>,
}

#[derive(Serialize)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {
    type O = terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {}

impl BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {
    pub fn build(self) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_AgreementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_AgreementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_AgreementRef {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_AgreementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_AgreementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    major: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {
    #[doc = "Set the field `major`.\n"]
    pub fn set_major(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.major = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minor`.\n"]
    pub fn set_minor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patch`.\n"]
    pub fn set_patch(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.patch = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {
    type O = terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {}

impl BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {
    pub fn build(self) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions {
            major: core::default::Default::default(),
            minor: core::default::Default::default(),
            patch: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_VersionsRef {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `major` after provisioning.\n"]
    pub fn major(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major", self.base))
    }

    #[doc = "Get a reference to the value of field `minor` after provisioning.\n"]
    pub fn minor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minor", self.base))
    }

    #[doc = "Get a reference to the value of field `patch` after provisioning.\n"]
    pub fn patch(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agreement: std::option::Option<terrars::ListField<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
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
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_complete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usecases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<terrars::ListField<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions>>,
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {
    #[doc = "Set the field `agreement`.\n"]
    pub fn set_agreement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Agreement>>,
    ) -> Self {
        self.agreement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_name`.\n"]
    pub fn set_inference_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inference_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_version`.\n"]
    pub fn set_inference_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inference_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_foundational`.\n"]
    pub fn set_is_foundational(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_foundational = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent_uuid`.\n"]
    pub fn set_parent_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_complete`.\n"]
    pub fn set_upload_complete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.upload_complete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usecases`.\n"]
    pub fn set_usecases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.usecases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_Versions>>,
    ) -> Self {
        self.versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {
    type O = terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {}

impl BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {
    pub fn build(self) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models {
            agreement: core::default::Default::default(),
            created_at: core::default::Default::default(),
            inference_name: core::default::Default::default(),
            inference_version: core::default::Default::default(),
            is_foundational: core::default::Default::default(),
            name: core::default::Default::default(),
            parent_uuid: core::default::Default::default(),
            provider: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            upload_complete: core::default::Default::default(),
            url: core::default::Default::default(),
            usecases: core::default::Default::default(),
            versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_ModelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_ModelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_ModelsRef {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_ModelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_ModelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agreement` after provisioning.\n"]
    pub fn agreement(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_AgreementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agreement", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_name` after provisioning.\n"]
    pub fn inference_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_name", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_version` after provisioning.\n"]
    pub fn inference_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inference_version", self.base))
    }

    #[doc = "Get a reference to the value of field `is_foundational` after provisioning.\n"]
    pub fn is_foundational(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_foundational", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parent_uuid` after provisioning.\n"]
    pub fn parent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_complete` after provisioning.\n"]
    pub fn upload_complete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_complete", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `usecases` after provisioning.\n"]
    pub fn usecases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usecases", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    models: std::option::Option<terrars::ListField<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_by`.\n"]
    pub fn set_created_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deleted_at`.\n"]
    pub fn set_deleted_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deleted_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `models`.\n"]
    pub fn set_models(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_Models>>,
    ) -> Self {
        self.models = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {
    type O = terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {}

impl BuildDataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {
    pub fn build(self) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            models: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef {
        DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformOpenaiApiKeys_OpenaiApiKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc = "Get a reference to the value of field `deleted_at` after provisioning.\n"]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.base))
    }

    #[doc = "Get a reference to the value of field `models` after provisioning.\n"]
    pub fn models(&self) -> terrars::ListRef<DataAgentPlatformOpenaiApiKeys_OpenaiApiKeys_ModelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.models", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformOpenaiApiKeys_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_by: std::option::Option<terrars::PrimField<std::string::String>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformOpenaiApiKeys_Filter {
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_by`.\n"]
    pub fn set_match_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys_Filter {
    type O = terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataAgentPlatformOpenaiApiKeys_Filter {
    pub fn build(self) -> DataAgentPlatformOpenaiApiKeys_Filter {
        DataAgentPlatformOpenaiApiKeys_Filter {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformOpenaiApiKeys_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeys_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformOpenaiApiKeys_FilterRef {
        DataAgentPlatformOpenaiApiKeys_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformOpenaiApiKeys_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_by` after provisioning.\n"]
    pub fn match_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_by", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformOpenaiApiKeys_Sort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
}

impl DataAgentPlatformOpenaiApiKeys_Sort {
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformOpenaiApiKeys_Sort {
    type O = terrars::BlockAssignable<DataAgentPlatformOpenaiApiKeys_Sort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformOpenaiApiKeys_Sort {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformOpenaiApiKeys_Sort {
    pub fn build(self) -> DataAgentPlatformOpenaiApiKeys_Sort {
        DataAgentPlatformOpenaiApiKeys_Sort {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformOpenaiApiKeys_SortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformOpenaiApiKeys_SortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformOpenaiApiKeys_SortRef {
        DataAgentPlatformOpenaiApiKeys_SortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformOpenaiApiKeys_SortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
