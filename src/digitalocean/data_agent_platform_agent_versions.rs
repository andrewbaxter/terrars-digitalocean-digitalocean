#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformAgentVersionsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    agent_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataAgentPlatformAgentVersions_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<std::vec::Vec<DataAgentPlatformAgentVersions_Sort>>,
    dynamic: DataAgentPlatformAgentVersionsDynamic,
}

struct DataAgentPlatformAgentVersions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformAgentVersionsData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformAgentVersions(std::rc::Rc<DataAgentPlatformAgentVersions_>);

impl DataAgentPlatformAgentVersions {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgentVersions_Filter>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgentVersions_Sort>>,
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

    #[doc =
        "Get a reference to the value of field `agent_id` after provisioning.\nThe ID of the agent to fetch versions for"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_versions` after provisioning.\n"]
    pub fn agent_versions(&self) -> terrars::ListRef<DataAgentPlatformAgentVersions_AgentVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataAgentPlatformAgentVersions_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformAgentVersions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformAgentVersions { }

impl terrars::ToListMappable for DataAgentPlatformAgentVersions {
    type O = terrars::ListRef<DataAgentPlatformAgentVersionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformAgentVersions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_agent_versions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformAgentVersions {
    pub tf_id: std::string::String,
    #[doc = "The ID of the agent to fetch versions for"]
    pub agent_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformAgentVersions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformAgentVersions {
        let out = DataAgentPlatformAgentVersions(std::rc::Rc::new(DataAgentPlatformAgentVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformAgentVersionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                agent_id: self.agent_id,
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

pub struct DataAgentPlatformAgentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformAgentVersionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `agent_id` after provisioning.\nThe ID of the agent to fetch versions for"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_versions` after provisioning.\n"]
    pub fn agent_versions(&self) -> terrars::ListRef<DataAgentPlatformAgentVersions_AgentVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataAgentPlatformAgentVersions_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    child_agent_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    if_case: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {
    #[doc = "Set the field `agent_name`.\n"]
    pub fn set_agent_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `child_agent_uuid`.\n"]
    pub fn set_child_agent_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.child_agent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `if_case`.\n"]
    pub fn set_if_case(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.if_case = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_deleted`.\n"]
    pub fn set_is_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_name`.\n"]
    pub fn set_route_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {}

impl BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {
    pub fn build(self) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents {
            agent_name: core::default::Default::default(),
            child_agent_uuid: core::default::Default::default(),
            if_case: core::default::Default::default(),
            is_deleted: core::default::Default::default(),
            route_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgentsRef {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_name` after provisioning.\n"]
    pub fn agent_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_name", self.base))
    }

    #[doc = "Get a reference to the value of field `child_agent_uuid` after provisioning.\n"]
    pub fn child_agent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.child_agent_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `if_case` after provisioning.\n"]
    pub fn if_case(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.if_case", self.base))
    }

    #[doc = "Get a reference to the value of field `is_deleted` after provisioning.\n"]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `route_name` after provisioning.\n"]
    pub fn route_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faas_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faas_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faas_name`.\n"]
    pub fn set_faas_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.faas_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faas_namespace`.\n"]
    pub fn set_faas_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.faas_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_deleted`.\n"]
    pub fn set_is_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {}

impl BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {
    pub fn build(self) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions {
            description: core::default::Default::default(),
            faas_name: core::default::Default::default(),
            faas_namespace: core::default::Default::default(),
            is_deleted: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctionsRef {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `faas_name` after provisioning.\n"]
    pub fn faas_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.faas_name", self.base))
    }

    #[doc = "Get a reference to the value of field `faas_namespace` after provisioning.\n"]
    pub fn faas_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.faas_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `is_deleted` after provisioning.\n"]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {
    #[doc = "Set the field `is_deleted`.\n"]
    pub fn set_is_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {}

impl BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {
    pub fn build(self) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails {
            is_deleted: core::default::Default::default(),
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrailsRef {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_deleted` after provisioning.\n"]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {
    #[doc = "Set the field `is_deleted`.\n"]
    pub fn set_is_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {}

impl BuildDataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {
    pub fn build(self) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases {
            is_deleted: core::default::Default::default(),
            name: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBasesRef {
        DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_deleted` after provisioning.\n"]
    pub fn is_deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgentVersions_AgentVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_child_agents: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_functions: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_guardrails: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_knowledge_bases: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    can_rollback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currently_applied: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provide_citations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_hash: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgentVersions_AgentVersions {
    #[doc = "Set the field `agent_uuid`.\n"]
    pub fn set_agent_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_child_agents`.\n"]
    pub fn set_attached_child_agents(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgents>>,
    ) -> Self {
        self.attached_child_agents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_functions`.\n"]
    pub fn set_attached_functions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctions>>,
    ) -> Self {
        self.attached_functions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_guardrails`.\n"]
    pub fn set_attached_guardrails(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrails>>,
    ) -> Self {
        self.attached_guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_knowledge_bases`.\n"]
    pub fn set_attached_knowledge_bases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBases>>,
    ) -> Self {
        self.attached_knowledge_bases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `can_rollback`.\n"]
    pub fn set_can_rollback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.can_rollback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_by_email`.\n"]
    pub fn set_created_by_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_by_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `currently_applied`.\n"]
    pub fn set_currently_applied(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.currently_applied = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `k`.\n"]
    pub fn set_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.k = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provide_citations`.\n"]
    pub fn set_provide_citations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.provide_citations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieval_method`.\n"]
    pub fn set_retrieval_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retrieval_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trigger_action`.\n"]
    pub fn set_trigger_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trigger_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_hash`.\n"]
    pub fn set_version_hash(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_hash = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_AgentVersions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_AgentVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_AgentVersions {}

impl BuildDataAgentPlatformAgentVersions_AgentVersions {
    pub fn build(self) -> DataAgentPlatformAgentVersions_AgentVersions {
        DataAgentPlatformAgentVersions_AgentVersions {
            agent_uuid: core::default::Default::default(),
            attached_child_agents: core::default::Default::default(),
            attached_functions: core::default::Default::default(),
            attached_guardrails: core::default::Default::default(),
            attached_knowledge_bases: core::default::Default::default(),
            can_rollback: core::default::Default::default(),
            created_at: core::default::Default::default(),
            created_by_email: core::default::Default::default(),
            currently_applied: core::default::Default::default(),
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            instruction: core::default::Default::default(),
            k: core::default::Default::default(),
            max_tokens: core::default::Default::default(),
            model_name: core::default::Default::default(),
            name: core::default::Default::default(),
            provide_citations: core::default::Default::default(),
            retrieval_method: core::default::Default::default(),
            tags: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
            trigger_action: core::default::Default::default(),
            version_hash: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_AgentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_AgentVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgentVersions_AgentVersionsRef {
        DataAgentPlatformAgentVersions_AgentVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_AgentVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_uuid` after provisioning.\n"]
    pub fn agent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_child_agents` after provisioning.\n"]
    pub fn attached_child_agents(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgentVersions_AgentVersions_AttachedChildAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_child_agents", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_functions` after provisioning.\n"]
    pub fn attached_functions(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgentVersions_AgentVersions_AttachedFunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_functions", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_guardrails` after provisioning.\n"]
    pub fn attached_guardrails(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgentVersions_AgentVersions_AttachedGuardrailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_guardrails", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_knowledge_bases` after provisioning.\n"]
    pub fn attached_knowledge_bases(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgentVersions_AgentVersions_AttachedKnowledgeBasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_knowledge_bases", self.base))
    }

    #[doc = "Get a reference to the value of field `can_rollback` after provisioning.\n"]
    pub fn can_rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_rollback", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_by_email` after provisioning.\n"]
    pub fn created_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by_email", self.base))
    }

    #[doc = "Get a reference to the value of field `currently_applied` after provisioning.\n"]
    pub fn currently_applied(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currently_applied", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `k` after provisioning.\n"]
    pub fn k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `provide_citations` after provisioning.\n"]
    pub fn provide_citations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provide_citations", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval_method` after provisioning.\n"]
    pub fn retrieval_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieval_method", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_action` after provisioning.\n"]
    pub fn trigger_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_action", self.base))
    }

    #[doc = "Get a reference to the value of field `version_hash` after provisioning.\n"]
    pub fn version_hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_hash", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgentVersions_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_by: std::option::Option<terrars::PrimField<std::string::String>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgentVersions_Filter {
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

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_Filter {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataAgentPlatformAgentVersions_Filter {
    pub fn build(self) -> DataAgentPlatformAgentVersions_Filter {
        DataAgentPlatformAgentVersions_Filter {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgentVersions_FilterRef {
        DataAgentPlatformAgentVersions_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_FilterRef {
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
pub struct DataAgentPlatformAgentVersions_Sort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
}

impl DataAgentPlatformAgentVersions_Sort {
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgentVersions_Sort {
    type O = terrars::BlockAssignable<DataAgentPlatformAgentVersions_Sort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgentVersions_Sort {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformAgentVersions_Sort {
    pub fn build(self) -> DataAgentPlatformAgentVersions_Sort {
        DataAgentPlatformAgentVersions_Sort {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgentVersions_SortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentVersions_SortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgentVersions_SortRef {
        DataAgentPlatformAgentVersions_SortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgentVersions_SortRef {
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

#[derive(Serialize, Default)]
struct DataAgentPlatformAgentVersionsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgentVersions_Filter>>,
    sort: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgentVersions_Sort>>,
}
