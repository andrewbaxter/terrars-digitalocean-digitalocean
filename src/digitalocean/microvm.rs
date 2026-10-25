#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct MicrovmData_ {
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
    auto_resume: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networking: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_pause: std::option::Option<std::vec::Vec<Microvm_AutoPause>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<std::vec::Vec<Microvm_Size>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<Microvm_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Microvm_Timeouts>,
    dynamic: MicrovmDynamic,
}

struct Microvm_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MicrovmData_>,
}

#[derive(Clone)]
pub struct Microvm(std::rc::Rc<Microvm_>);

impl Microvm {
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

    #[doc =
        "Set the field `auto_resume`.\nWhether the MicroVM should auto-resume on request. Forces recreation on change: the MicroVMs API has no in-place update path for auto_resume."]
    pub fn set_auto_resume(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_resume = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\nEnvironment variables passed to the MicroVM"]
    pub fn set_environment(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_port`.\nPort the MicroVM exposes over HTTP"]
    pub fn set_http_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_protocol`.\nHTTP protocol: 'http' or 'http2'"]
    pub fn set_http_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().http_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `networking`.\nNetworking mode: 'public' or 'vpc'"]
    pub fn set_networking(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\nGuest ports open for ingress. Defaults to just http_port when omitted."]
    pub fn set_ports(self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().ports = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nDigitalOcean region slug. Required when creating from oci_ref; optional when restoring from a checkpoint (inherited)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nDesired lifecycle state: 'running' or 'paused'. Changes are applied by calling the microvm pause / resume action endpoints."]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\nUUID of the VPC to attach the MicroVM to. Only valid when networking is 'vpc'."]
    pub fn set_vpc_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_pause`.\n"]
    pub fn set_auto_pause(self, v: impl std::convert::Into<terrars::BlockAssignable<Microvm_AutoPause>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_pause = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_pause = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(self, v: impl std::convert::Into<terrars::BlockAssignable<Microvm_Size>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().size = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.size = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<Microvm_Source>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Microvm_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the MicroVM"]
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
        "Get a reference to the value of field `state` after provisioning.\nDesired lifecycle state: 'running' or 'paused'. Changes are applied by calling the microvm pause / resume action endpoints."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urls` after provisioning.\nIngress URLs for the MicroVM"]
    pub fn urls(&self) -> terrars::ListRef<Microvm_UrlsRef> {
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

    #[doc = "Get a reference to the value of field `auto_pause` after provisioning.\n"]
    pub fn auto_pause(&self) -> terrars::ListRef<Microvm_AutoPauseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_pause", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::ListRef<Microvm_SizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<Microvm_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Microvm_TimeoutsRef {
        Microvm_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Microvm {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Microvm { }

impl terrars::ToListMappable for Microvm {
    type O = terrars::ListRef<MicrovmRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Microvm_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_microvm".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMicrovm {
    pub tf_id: std::string::String,
    #[doc = "Name of the MicroVM"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMicrovm {
    pub fn build(self, stack: &mut terrars::Stack) -> Microvm {
        let out = Microvm(std::rc::Rc::new(Microvm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MicrovmData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_resume: core::default::Default::default(),
                environment: core::default::Default::default(),
                http_port: core::default::Default::default(),
                http_protocol: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                networking: core::default::Default::default(),
                ports: core::default::Default::default(),
                region: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_uuid: core::default::Default::default(),
                auto_pause: core::default::Default::default(),
                size: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MicrovmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MicrovmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MicrovmRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the MicroVM"]
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
        "Get a reference to the value of field `state` after provisioning.\nDesired lifecycle state: 'running' or 'paused'. Changes are applied by calling the microvm pause / resume action endpoints."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urls` after provisioning.\nIngress URLs for the MicroVM"]
    pub fn urls(&self) -> terrars::ListRef<Microvm_UrlsRef> {
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

    #[doc = "Get a reference to the value of field `auto_pause` after provisioning.\n"]
    pub fn auto_pause(&self) -> terrars::ListRef<Microvm_AutoPauseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_pause", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::ListRef<Microvm_SizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<Microvm_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Microvm_TimeoutsRef {
        Microvm_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Microvm_Urls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Microvm_Urls {
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

impl terrars::ToListMappable for Microvm_Urls {
    type O = terrars::BlockAssignable<Microvm_Urls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMicrovm_Urls {}

impl BuildMicrovm_Urls {
    pub fn build(self) -> Microvm_Urls {
        Microvm_Urls {
            default: core::default::Default::default(),
            hostname: core::default::Default::default(),
            port: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Microvm_UrlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Microvm_UrlsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Microvm_UrlsRef {
        Microvm_UrlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Microvm_UrlsRef {
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

#[derive(Serialize)]
pub struct Microvm_AutoPause {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Microvm_AutoPause {
    #[doc =
        "Set the field `idle_timeout`.\nIdle timeout as a Go duration string (e.g. '5m', '30s'). Forces recreation on change (no in-place API path)."]
    pub fn set_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Microvm_AutoPause {
    type O = terrars::BlockAssignable<Microvm_AutoPause>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMicrovm_AutoPause {
    #[doc = "Whether auto-pause is enabled. Forces recreation on change (no in-place API path)."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildMicrovm_AutoPause {
    pub fn build(self) -> Microvm_AutoPause {
        Microvm_AutoPause {
            enabled: self.enabled,
            idle_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Microvm_AutoPauseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Microvm_AutoPauseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Microvm_AutoPauseRef {
        Microvm_AutoPauseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Microvm_AutoPauseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether auto-pause is enabled. Forces recreation on change (no in-place API path)."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `idle_timeout` after provisioning.\nIdle timeout as a Go duration string (e.g. '5m', '30s'). Forces recreation on change (no in-place API path)."]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct Microvm_Size {
    cpu: terrars::PrimField<f64>,
    memory: terrars::PrimField<f64>,
}

impl Microvm_Size { }

impl terrars::ToListMappable for Microvm_Size {
    type O = terrars::BlockAssignable<Microvm_Size>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMicrovm_Size {
    #[doc = "Number of vCPUs"]
    pub cpu: terrars::PrimField<f64>,
    #[doc = "Memory in MiB"]
    pub memory: terrars::PrimField<f64>,
}

impl BuildMicrovm_Size {
    pub fn build(self) -> Microvm_Size {
        Microvm_Size {
            cpu: self.cpu,
            memory: self.memory,
        }
    }
}

#[allow(dead_code)]
pub struct Microvm_SizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Microvm_SizeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Microvm_SizeRef {
        Microvm_SizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Microvm_SizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nNumber of vCPUs"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk` after provisioning.\nAttached disk in GB (provisioned with the size)"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\nMemory in MiB"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct Microvm_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    checkpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_ref: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Microvm_Source {
    #[doc = "Set the field `checkpoint_id`.\nCheckpoint UUID to restore"]
    pub fn set_checkpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.checkpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_ref`.\nOCI reference for the workload container"]
    pub fn set_oci_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oci_ref = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Microvm_Source {
    type O = terrars::BlockAssignable<Microvm_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMicrovm_Source {}

impl BuildMicrovm_Source {
    pub fn build(self) -> Microvm_Source {
        Microvm_Source {
            checkpoint_id: core::default::Default::default(),
            oci_ref: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Microvm_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Microvm_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Microvm_SourceRef {
        Microvm_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Microvm_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `checkpoint_id` after provisioning.\nCheckpoint UUID to restore"]
    pub fn checkpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checkpoint_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oci_ref` after provisioning.\nOCI reference for the workload container"]
    pub fn oci_ref(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_ref", self.base))
    }
}

#[derive(Serialize)]
pub struct Microvm_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Microvm_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Microvm_Timeouts {
    type O = terrars::BlockAssignable<Microvm_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMicrovm_Timeouts {}

impl BuildMicrovm_Timeouts {
    pub fn build(self) -> Microvm_Timeouts {
        Microvm_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Microvm_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Microvm_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Microvm_TimeoutsRef {
        Microvm_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Microvm_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct MicrovmDynamic {
    auto_pause: std::option::Option<terrars::DynamicBlock<Microvm_AutoPause>>,
    size: std::option::Option<terrars::DynamicBlock<Microvm_Size>>,
    source: std::option::Option<terrars::DynamicBlock<Microvm_Source>>,
}
