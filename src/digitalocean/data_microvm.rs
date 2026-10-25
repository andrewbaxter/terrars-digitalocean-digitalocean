#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataMicrovmData_ {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMicrovm_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMicrovmData_>,
}

#[derive(Clone)]
pub struct DataMicrovm(std::rc::Rc<DataMicrovm_>);

impl DataMicrovm {
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

    #[doc = "Set the field `id`.\nMicroVM ID"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nMicroVM name"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `auto_pause` after provisioning.\nAuto-pause configuration. Forces recreation on change: the MicroVMs API has no in-place update path for auto_pause."]
    pub fn auto_pause(&self) -> terrars::ListRef<DataMicrovm_AutoPauseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_pause", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_resume` after provisioning.\nWhether the MicroVM should auto-resume on request. Forces recreation on change: the MicroVMs API has no in-place update path for auto_resume."]
    pub fn auto_resume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_resume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe creation timestamp for the MicroVM"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_state` after provisioning.\nObserved lifecycle state of the MicroVM"]
    pub fn current_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nEnvironment variables passed to the MicroVM"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nHuman-readable explanation when current_state is failed"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\nPort the MicroVM exposes over HTTP"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_protocol` after provisioning.\nHTTP protocol: 'http' or 'http2'"]
    pub fn http_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nMicroVM ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nMicroVM name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networking` after provisioning.\nNetworking mode: 'public' or 'vpc'"]
    pub fn networking(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nGuest ports open for ingress. Defaults to just http_port when omitted."]
    pub fn ports(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nDigitalOcean region slug. Required when creating from oci_ref; optional when restoring from a checkpoint (inherited)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `size` after provisioning.\nCompute size. Required when creating from oci_ref; optional when restoring from a checkpoint (inherited)."]
    pub fn size(&self) -> terrars::ListRef<DataMicrovm_SizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nWorkload source. Exactly one of oci_ref or checkpoint_id must be set."]
    pub fn source(&self) -> terrars::ListRef<DataMicrovm_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDesired lifecycle state: 'running' or 'paused'. Changes are applied by calling the microvm pause / resume action endpoints."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urls` after provisioning.\nIngress URLs for the MicroVM"]
    pub fn urls(&self) -> terrars::ListRef<DataMicrovm_UrlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource name (URN) for the MicroVM"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC to attach the MicroVM to. Only valid when networking is 'vpc'."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataMicrovm {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMicrovm { }

impl terrars::ToListMappable for DataMicrovm {
    type O = terrars::ListRef<DataMicrovmRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMicrovm_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_microvm".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMicrovm {
    pub tf_id: std::string::String,
}

impl BuildDataMicrovm {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMicrovm {
        let out = DataMicrovm(std::rc::Rc::new(DataMicrovm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMicrovmData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMicrovmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMicrovmRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `auto_pause` after provisioning.\nAuto-pause configuration. Forces recreation on change: the MicroVMs API has no in-place update path for auto_pause."]
    pub fn auto_pause(&self) -> terrars::ListRef<DataMicrovm_AutoPauseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_pause", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_resume` after provisioning.\nWhether the MicroVM should auto-resume on request. Forces recreation on change: the MicroVMs API has no in-place update path for auto_resume."]
    pub fn auto_resume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_resume", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe creation timestamp for the MicroVM"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_state` after provisioning.\nObserved lifecycle state of the MicroVM"]
    pub fn current_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nEnvironment variables passed to the MicroVM"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_reason` after provisioning.\nHuman-readable explanation when current_state is failed"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\nPort the MicroVM exposes over HTTP"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_protocol` after provisioning.\nHTTP protocol: 'http' or 'http2'"]
    pub fn http_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nMicroVM ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nMicroVM name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networking` after provisioning.\nNetworking mode: 'public' or 'vpc'"]
    pub fn networking(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.networking", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nGuest ports open for ingress. Defaults to just http_port when omitted."]
    pub fn ports(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ports", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nDigitalOcean region slug. Required when creating from oci_ref; optional when restoring from a checkpoint (inherited)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `size` after provisioning.\nCompute size. Required when creating from oci_ref; optional when restoring from a checkpoint (inherited)."]
    pub fn size(&self) -> terrars::ListRef<DataMicrovm_SizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nWorkload source. Exactly one of oci_ref or checkpoint_id must be set."]
    pub fn source(&self) -> terrars::ListRef<DataMicrovm_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDesired lifecycle state: 'running' or 'paused'. Changes are applied by calling the microvm pause / resume action endpoints."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urls` after provisioning.\nIngress URLs for the MicroVM"]
    pub fn urls(&self) -> terrars::ListRef<DataMicrovm_UrlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource name (URN) for the MicroVM"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC to attach the MicroVM to. Only valid when networking is 'vpc'."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMicrovm_AutoPause {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMicrovm_AutoPause {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMicrovm_AutoPause {
    type O = terrars::BlockAssignable<DataMicrovm_AutoPause>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovm_AutoPause {}

impl BuildDataMicrovm_AutoPause {
    pub fn build(self) -> DataMicrovm_AutoPause {
        DataMicrovm_AutoPause {
            enabled: core::default::Default::default(),
            idle_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovm_AutoPauseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovm_AutoPauseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovm_AutoPauseRef {
        DataMicrovm_AutoPauseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovm_AutoPauseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMicrovm_Size {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMicrovm_Size {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMicrovm_Size {
    type O = terrars::BlockAssignable<DataMicrovm_Size>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovm_Size {}

impl BuildDataMicrovm_Size {
    pub fn build(self) -> DataMicrovm_Size {
        DataMicrovm_Size {
            cpu: core::default::Default::default(),
            disk: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovm_SizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovm_SizeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovm_SizeRef {
        DataMicrovm_SizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovm_SizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMicrovm_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_ref: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMicrovm_Source {
    #[doc = "Set the field `checkpoint_id`.\n"]
    pub fn set_checkpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.checkpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_ref`.\n"]
    pub fn set_oci_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oci_ref = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMicrovm_Source {
    type O = terrars::BlockAssignable<DataMicrovm_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovm_Source {}

impl BuildDataMicrovm_Source {
    pub fn build(self) -> DataMicrovm_Source {
        DataMicrovm_Source {
            checkpoint_id: core::default::Default::default(),
            oci_ref: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovm_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovm_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovm_SourceRef {
        DataMicrovm_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovm_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_id` after provisioning.\n"]
    pub fn checkpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checkpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_ref` after provisioning.\n"]
    pub fn oci_ref(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_ref", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMicrovm_Urls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMicrovm_Urls {
    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMicrovm_Urls {
    type O = terrars::BlockAssignable<DataMicrovm_Urls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovm_Urls {}

impl BuildDataMicrovm_Urls {
    pub fn build(self) -> DataMicrovm_Urls {
        DataMicrovm_Urls {
            default: core::default::Default::default(),
            hostname: core::default::Default::default(),
            port: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovm_UrlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovm_UrlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovm_UrlsRef {
        DataMicrovm_UrlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovm_UrlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
