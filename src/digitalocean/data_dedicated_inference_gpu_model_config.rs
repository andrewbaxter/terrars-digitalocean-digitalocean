#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDedicatedInferenceGpuModelConfigData_ {
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
}

struct DataDedicatedInferenceGpuModelConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDedicatedInferenceGpuModelConfigData_>,
}

#[derive(Clone)]
pub struct DataDedicatedInferenceGpuModelConfig(std::rc::Rc<DataDedicatedInferenceGpuModelConfig_>);

impl DataDedicatedInferenceGpuModelConfig {
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

    #[doc =
        "Get a reference to the value of field `gpu_model_configs` after provisioning.\nThe list of supported GPU and model combinations for dedicated inference endpoints."]
    pub fn gpu_model_configs(&self) -> terrars::ListRef<DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_model_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataDedicatedInferenceGpuModelConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDedicatedInferenceGpuModelConfig { }

impl terrars::ToListMappable for DataDedicatedInferenceGpuModelConfig {
    type O = terrars::ListRef<DataDedicatedInferenceGpuModelConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDedicatedInferenceGpuModelConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_dedicated_inference_gpu_model_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDedicatedInferenceGpuModelConfig {
    pub tf_id: std::string::String,
}

impl BuildDataDedicatedInferenceGpuModelConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDedicatedInferenceGpuModelConfig {
        let out = DataDedicatedInferenceGpuModelConfig(std::rc::Rc::new(DataDedicatedInferenceGpuModelConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDedicatedInferenceGpuModelConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDedicatedInferenceGpuModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceGpuModelConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDedicatedInferenceGpuModelConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `gpu_model_configs` after provisioning.\nThe list of supported GPU and model combinations for dedicated inference endpoints."]
    pub fn gpu_model_configs(&self) -> terrars::ListRef<DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_model_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceGpuModelConfig_GpuModelConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_slugs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_model_gated: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_slug: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInferenceGpuModelConfig_GpuModelConfigs {
    #[doc = "Set the field `gpu_slugs`.\n"]
    pub fn set_gpu_slugs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gpu_slugs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_model_gated`.\n"]
    pub fn set_is_model_gated(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_model_gated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_slug`.\n"]
    pub fn set_model_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_slug = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceGpuModelConfig_GpuModelConfigs {
    type O = terrars::BlockAssignable<DataDedicatedInferenceGpuModelConfig_GpuModelConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceGpuModelConfig_GpuModelConfigs {}

impl BuildDataDedicatedInferenceGpuModelConfig_GpuModelConfigs {
    pub fn build(self) -> DataDedicatedInferenceGpuModelConfig_GpuModelConfigs {
        DataDedicatedInferenceGpuModelConfig_GpuModelConfigs {
            gpu_slugs: core::default::Default::default(),
            is_model_gated: core::default::Default::default(),
            model_name: core::default::Default::default(),
            model_slug: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef {
        DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceGpuModelConfig_GpuModelConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gpu_slugs` after provisioning.\n"]
    pub fn gpu_slugs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu_slugs", self.base))
    }

    #[doc = "Get a reference to the value of field `is_model_gated` after provisioning.\n"]
    pub fn is_model_gated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_model_gated", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `model_slug` after provisioning.\n"]
    pub fn model_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_slug", self.base))
    }
}
