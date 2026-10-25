#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct VectorDatabaseData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    size: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<VectorDatabase_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VectorDatabase_Timeouts>,
    dynamic: VectorDatabaseDynamic,
}

struct VectorDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VectorDatabaseData_>,
}

#[derive(Clone)]
pub struct VectorDatabase(std::rc::Rc<VectorDatabase_>);

impl VectorDatabase {
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

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<VectorDatabase_Config>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VectorDatabase_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<VectorDatabase_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_uuid` after provisioning.\n"]
    pub fn owner_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<VectorDatabase_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorDatabase_TimeoutsRef {
        VectorDatabase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VectorDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VectorDatabase { }

impl terrars::ToListMappable for VectorDatabase {
    type O = terrars::ListRef<VectorDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VectorDatabase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_vector_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVectorDatabase {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<std::string::String>,
}

impl BuildVectorDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> VectorDatabase {
        let out = VectorDatabase(std::rc::Rc::new(VectorDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VectorDatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project_id: core::default::Default::default(),
                region: self.region,
                size: self.size,
                tags: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VectorDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VectorDatabaseRef {
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

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<VectorDatabase_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_uuid` after provisioning.\n"]
    pub fn owner_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<VectorDatabase_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorDatabase_TimeoutsRef {
        VectorDatabase_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VectorDatabaseDynamic {
    config: std::option::Option<terrars::DynamicBlock<VectorDatabase_Config>>,
}

#[derive(Serialize)]
pub struct VectorDatabase_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorDatabase_Endpoints {
    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http`.\n"]
    pub fn set_http(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VectorDatabase_Endpoints {
    type O = terrars::BlockAssignable<VectorDatabase_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorDatabase_Endpoints {}

impl BuildVectorDatabase_Endpoints {
    pub fn build(self) -> VectorDatabase_Endpoints {
        VectorDatabase_Endpoints {
            grpc: core::default::Default::default(),
            http: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorDatabase_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorDatabase_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorDatabase_EndpointsRef {
        VectorDatabase_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorDatabase_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http` after provisioning.\n"]
    pub fn http(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorDatabase_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_quantization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_auto_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weaviate_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorDatabase_Config {
    #[doc = "Set the field `default_quantization`.\n"]
    pub fn set_default_quantization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_quantization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_auto_schema`.\n"]
    pub fn set_enable_auto_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_auto_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weaviate_version`.\n"]
    pub fn set_weaviate_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weaviate_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VectorDatabase_Config {
    type O = terrars::BlockAssignable<VectorDatabase_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorDatabase_Config {}

impl BuildVectorDatabase_Config {
    pub fn build(self) -> VectorDatabase_Config {
        VectorDatabase_Config {
            default_quantization: core::default::Default::default(),
            enable_auto_schema: core::default::Default::default(),
            weaviate_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorDatabase_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorDatabase_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorDatabase_ConfigRef {
        VectorDatabase_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorDatabase_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_quantization` after provisioning.\n"]
    pub fn default_quantization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_quantization", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_auto_schema` after provisioning.\n"]
    pub fn enable_auto_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `weaviate_version` after provisioning.\n"]
    pub fn weaviate_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weaviate_version", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorDatabase_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorDatabase_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VectorDatabase_Timeouts {
    type O = terrars::BlockAssignable<VectorDatabase_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorDatabase_Timeouts {}

impl BuildVectorDatabase_Timeouts {
    pub fn build(self) -> VectorDatabase_Timeouts {
        VectorDatabase_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VectorDatabase_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorDatabase_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorDatabase_TimeoutsRef {
        VectorDatabase_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorDatabase_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
