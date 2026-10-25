#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformAgentsData_ {
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
    only_deployed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataAgentPlatformAgents_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<std::vec::Vec<DataAgentPlatformAgents_Sort>>,
    dynamic: DataAgentPlatformAgentsDynamic,
}

struct DataAgentPlatformAgents_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformAgentsData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformAgents(std::rc::Rc<DataAgentPlatformAgents_>);

impl DataAgentPlatformAgents {
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

    #[doc = "Set the field `only_deployed`.\n"]
    pub fn set_only_deployed(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().only_deployed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgents_Filter>>,
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
    pub fn set_sort(self, v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgents_Sort>>) -> Self {
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

    #[doc = "Get a reference to the value of field `agents` after provisioning.\n"]
    pub fn agents(&self) -> terrars::ListRef<DataAgentPlatformAgents_AgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `only_deployed` after provisioning.\n"]
    pub fn only_deployed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_deployed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataAgentPlatformAgents_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformAgents {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformAgents { }

impl terrars::ToListMappable for DataAgentPlatformAgents {
    type O = terrars::ListRef<DataAgentPlatformAgentsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformAgents_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_agents".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformAgents {
    pub tf_id: std::string::String,
}

impl BuildDataAgentPlatformAgents {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformAgents {
        let out = DataAgentPlatformAgents(std::rc::Rc::new(DataAgentPlatformAgents_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformAgentsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                only_deployed: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformAgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformAgentsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `agents` after provisioning.\n"]
    pub fn agents(&self) -> terrars::ListRef<DataAgentPlatformAgents_AgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `only_deployed` after provisioning.\n"]
    pub fn only_deployed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_deployed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataAgentPlatformAgents_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_AgentGuardrail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_attached: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_AgentGuardrail {
    #[doc = "Set the field `agent_uuid`.\n"]
    pub fn set_agent_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_response`.\n"]
    pub fn set_default_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_uuid`.\n"]
    pub fn set_guardrail_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guardrail_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_attached`.\n"]
    pub fn set_is_attached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_attached = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_default`.\n"]
    pub fn set_is_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_default = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_AgentGuardrail {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_AgentGuardrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_AgentGuardrail {}

impl BuildDataAgentPlatformAgents_Agents_AgentGuardrail {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_AgentGuardrail {
        DataAgentPlatformAgents_Agents_AgentGuardrail {
            agent_uuid: core::default::Default::default(),
            created_at: core::default::Default::default(),
            default_response: core::default::Default::default(),
            description: core::default::Default::default(),
            guardrail_uuid: core::default::Default::default(),
            is_attached: core::default::Default::default(),
            is_default: core::default::Default::default(),
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_AgentGuardrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_AgentGuardrailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_AgentGuardrailRef {
        DataAgentPlatformAgents_Agents_AgentGuardrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_AgentGuardrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_uuid` after provisioning.\n"]
    pub fn agent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `default_response` after provisioning.\n"]
    pub fn default_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_response", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_uuid` after provisioning.\n"]
    pub fn guardrail_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `is_attached` after provisioning.\n"]
    pub fn is_attached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_attached", self.base))
    }

    #[doc = "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
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
pub struct DataAgentPlatformAgents_Agents_AnthropicApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_AnthropicApiKey {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_AnthropicApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_AnthropicApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_AnthropicApiKey {}

impl BuildDataAgentPlatformAgents_Agents_AnthropicApiKey {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_AnthropicApiKey {
        DataAgentPlatformAgents_Agents_AnthropicApiKey {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_AnthropicApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_AnthropicApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_AnthropicApiKeyRef {
        DataAgentPlatformAgents_Agents_AnthropicApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_AnthropicApiKeyRef {
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
pub struct DataAgentPlatformAgents_Agents_ApiKeyInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ApiKeyInfos {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_key`.\n"]
    pub fn set_secret_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ApiKeyInfos {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ApiKeyInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ApiKeyInfos {}

impl BuildDataAgentPlatformAgents_Agents_ApiKeyInfos {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ApiKeyInfos {
        DataAgentPlatformAgents_Agents_ApiKeyInfos {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            name: core::default::Default::default(),
            secret_key: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ApiKeyInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ApiKeyInfosRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_ApiKeyInfosRef {
        DataAgentPlatformAgents_Agents_ApiKeyInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ApiKeyInfosRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ApiKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ApiKeys {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ApiKeys {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ApiKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ApiKeys {}

impl BuildDataAgentPlatformAgents_Agents_ApiKeys {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ApiKeys {
        DataAgentPlatformAgents_Agents_ApiKeys { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ApiKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_ApiKeysRef {
        DataAgentPlatformAgents_Agents_ApiKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ApiKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Chatbot {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button_background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Chatbot {
    #[doc = "Set the field `button_background_color`.\n"]
    pub fn set_button_background_color(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.button_background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo`.\n"]
    pub fn set_logo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_color`.\n"]
    pub fn set_primary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_color`.\n"]
    pub fn set_secondary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_message`.\n"]
    pub fn set_starting_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Chatbot {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Chatbot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Chatbot {}

impl BuildDataAgentPlatformAgents_Agents_Chatbot {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Chatbot {
        DataAgentPlatformAgents_Agents_Chatbot {
            button_background_color: core::default::Default::default(),
            logo: core::default::Default::default(),
            name: core::default::Default::default(),
            primary_color: core::default::Default::default(),
            secondary_color: core::default::Default::default(),
            starting_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChatbotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChatbotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_ChatbotRef {
        DataAgentPlatformAgents_Agents_ChatbotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChatbotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `button_background_color` after provisioning.\n"]
    pub fn button_background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_background_color", self.base))
    }

    #[doc = "Get a reference to the value of field `logo` after provisioning.\n"]
    pub fn logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_color` after provisioning.\n"]
    pub fn primary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_color", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_color` after provisioning.\n"]
    pub fn secondary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_color", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_message` after provisioning.\n"]
    pub fn starting_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChatbotIdentifiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChatbotIdentifiers {
    #[doc = "Set the field `chatbot_id`.\n"]
    pub fn set_chatbot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chatbot_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChatbotIdentifiers {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChatbotIdentifiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChatbotIdentifiers {}

impl BuildDataAgentPlatformAgents_Agents_ChatbotIdentifiers {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChatbotIdentifiers {
        DataAgentPlatformAgents_Agents_ChatbotIdentifiers { chatbot_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChatbotIdentifiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChatbotIdentifiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChatbotIdentifiersRef {
        DataAgentPlatformAgents_Agents_ChatbotIdentifiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChatbotIdentifiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chatbot_id` after provisioning.\n"]
    pub fn chatbot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chatbot_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {
        DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKeyRef {
        DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKeyRef {
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
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_key`.\n"]
    pub fn set_secret_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {
        DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            name: core::default::Default::default(),
            secret_key: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfosRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfosRef {
        DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfosRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents_ApiKeys {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents_ApiKeys {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys {
        DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgents_ApiKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChildAgents_ApiKeysRef {
        DataAgentPlatformAgents_Agents_ChildAgents_ApiKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ApiKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_Chatbot {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button_background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents_Chatbot {
    #[doc = "Set the field `button_background_color`.\n"]
    pub fn set_button_background_color(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.button_background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo`.\n"]
    pub fn set_logo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_color`.\n"]
    pub fn set_primary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_color`.\n"]
    pub fn set_secondary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_message`.\n"]
    pub fn set_starting_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents_Chatbot {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents_Chatbot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents_Chatbot {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents_Chatbot {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents_Chatbot {
        DataAgentPlatformAgents_Agents_ChildAgents_Chatbot {
            button_background_color: core::default::Default::default(),
            logo: core::default::Default::default(),
            name: core::default::Default::default(),
            primary_color: core::default::Default::default(),
            secondary_color: core::default::Default::default(),
            starting_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ChatbotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgents_ChatbotRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChildAgents_ChatbotRef {
        DataAgentPlatformAgents_Agents_ChildAgents_ChatbotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ChatbotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `button_background_color` after provisioning.\n"]
    pub fn button_background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_background_color", self.base))
    }

    #[doc = "Get a reference to the value of field `logo` after provisioning.\n"]
    pub fn logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_color` after provisioning.\n"]
    pub fn primary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_color", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_color` after provisioning.\n"]
    pub fn secondary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_color", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_message` after provisioning.\n"]
    pub fn starting_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {
    #[doc = "Set the field `chatbot_id`.\n"]
    pub fn set_chatbot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chatbot_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {
        DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers {
            chatbot_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiersRef {
        DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chatbot_id` after provisioning.\n"]
    pub fn chatbot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chatbot_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_Deployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents_Deployment {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visibility = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents_Deployment {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents_Deployment {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents_Deployment {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents_Deployment {
        DataAgentPlatformAgents_Agents_ChildAgents_Deployment {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgents_DeploymentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ChildAgents_DeploymentRef {
        DataAgentPlatformAgents_Agents_ChildAgents_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgents_DeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ChildAgents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_api_key: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_infos: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_Chatbot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_identifiers: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_Deployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ChildAgents {
    #[doc = "Set the field `agent_id`.\n"]
    pub fn set_agent_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_api_key`.\n"]
    pub fn set_anthropic_api_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKey>>,
    ) -> Self {
        self.anthropic_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_infos`.\n"]
    pub fn set_api_key_infos(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfos>>,
    ) -> Self {
        self.api_key_infos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_keys`.\n"]
    pub fn set_api_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeys>>,
    ) -> Self {
        self.api_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot`.\n"]
    pub fn set_chatbot(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_Chatbot>>,
    ) -> Self {
        self.chatbot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot_identifiers`.\n"]
    pub fn set_chatbot_identifiers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiers>>,
    ) -> Self {
        self.chatbot_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents_Deployment>>,
    ) -> Self {
        self.deployment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_uuid`.\n"]
    pub fn set_model_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ChildAgents {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ChildAgents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ChildAgents {}

impl BuildDataAgentPlatformAgents_Agents_ChildAgents {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ChildAgents {
        DataAgentPlatformAgents_Agents_ChildAgents {
            agent_id: core::default::Default::default(),
            anthropic_api_key: core::default::Default::default(),
            api_key_infos: core::default::Default::default(),
            api_keys: core::default::Default::default(),
            chatbot: core::default::Default::default(),
            chatbot_identifiers: core::default::Default::default(),
            deployment: core::default::Default::default(),
            description: core::default::Default::default(),
            instruction: core::default::Default::default(),
            model_uuid: core::default::Default::default(),
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ChildAgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ChildAgentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_ChildAgentsRef {
        DataAgentPlatformAgents_Agents_ChildAgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ChildAgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgents_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.base))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgents_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgents_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgents_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgents_DeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `model_uuid` after provisioning.\n"]
    pub fn model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Deployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Deployment {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visibility = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Deployment {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Deployment {}

impl BuildDataAgentPlatformAgents_Agents_Deployment {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Deployment {
        DataAgentPlatformAgents_Agents_Deployment {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_DeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_DeploymentRef {
        DataAgentPlatformAgents_Agents_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_DeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Functions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faasname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    faasnamespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Functions {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faasname`.\n"]
    pub fn set_faasname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.faasname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faasnamespace`.\n"]
    pub fn set_faasnamespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.faasnamespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_uuid`.\n"]
    pub fn set_guardrail_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guardrail_uuid = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Functions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Functions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Functions {}

impl BuildDataAgentPlatformAgents_Agents_Functions {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Functions {
        DataAgentPlatformAgents_Agents_Functions {
            api_key: core::default::Default::default(),
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            faasname: core::default::Default::default(),
            faasnamespace: core::default::Default::default(),
            guardrail_uuid: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_FunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_FunctionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_FunctionsRef {
        DataAgentPlatformAgents_Agents_FunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_FunctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `faasname` after provisioning.\n"]
    pub fn faasname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.faasname", self.base))
    }

    #[doc = "Get a reference to the value of field `faasnamespace` after provisioning.\n"]
    pub fn faasnamespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.faasnamespace", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_uuid` after provisioning.\n"]
    pub fn guardrail_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
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
pub struct DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_uuids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finished_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    started_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {
    #[doc = "Set the field `completed_datasources`.\n"]
    pub fn set_completed_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.completed_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_uuids`.\n"]
    pub fn set_data_source_uuids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_source_uuids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finished_at`.\n"]
    pub fn set_finished_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finished_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `knowledge_base_uuid`.\n"]
    pub fn set_knowledge_base_uuid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.knowledge_base_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phase`.\n"]
    pub fn set_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `started_at`.\n"]
    pub fn set_started_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.started_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tokens`.\n"]
    pub fn set_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_datasources`.\n"]
    pub fn set_total_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_datasources = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {}

impl BuildDataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {
        DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob {
            completed_datasources: core::default::Default::default(),
            created_at: core::default::Default::default(),
            data_source_uuids: core::default::Default::default(),
            finished_at: core::default::Default::default(),
            knowledge_base_uuid: core::default::Default::default(),
            phase: core::default::Default::default(),
            started_at: core::default::Default::default(),
            tokens: core::default::Default::default(),
            total_datasources: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJobRef {
        DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completed_datasources` after provisioning.\n"]
    pub fn completed_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_datasources", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_uuids` after provisioning.\n"]
    pub fn data_source_uuids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_uuids", self.base))
    }

    #[doc = "Get a reference to the value of field `finished_at` after provisioning.\n"]
    pub fn finished_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finished_at", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\n"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\n"]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc = "Get a reference to the value of field `started_at` after provisioning.\n"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `total_datasources` after provisioning.\n"]
    pub fn total_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_datasources", self.base))
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
pub struct DataAgentPlatformAgents_Agents_KnowledgeBases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    added_to_agent_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_public: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_KnowledgeBases {
    #[doc = "Set the field `added_to_agent_at`.\n"]
    pub fn set_added_to_agent_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.added_to_agent_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_id`.\n"]
    pub fn set_database_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model_uuid`.\n"]
    pub fn set_embedding_model_uuid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.embedding_model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_public`.\n"]
    pub fn set_is_public(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJob>>,
    ) -> Self {
        self.last_indexing_job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_KnowledgeBases {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_KnowledgeBases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_KnowledgeBases {}

impl BuildDataAgentPlatformAgents_Agents_KnowledgeBases {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_KnowledgeBases {
        DataAgentPlatformAgents_Agents_KnowledgeBases {
            added_to_agent_at: core::default::Default::default(),
            created_at: core::default::Default::default(),
            database_id: core::default::Default::default(),
            embedding_model_uuid: core::default::Default::default(),
            is_public: core::default::Default::default(),
            last_indexing_job: core::default::Default::default(),
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
            region: core::default::Default::default(),
            tags: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_id: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_KnowledgeBasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_KnowledgeBasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_KnowledgeBasesRef {
        DataAgentPlatformAgents_Agents_KnowledgeBasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_KnowledgeBasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `added_to_agent_at` after provisioning.\n"]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `database_id` after provisioning.\n"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_model_uuid` after provisioning.\n"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `is_public` after provisioning.\n"]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.base))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgents_Agents_KnowledgeBases_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Model_Agreement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Model_Agreement {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Model_Agreement {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Model_Agreement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Model_Agreement {}

impl BuildDataAgentPlatformAgents_Agents_Model_Agreement {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Model_Agreement {
        DataAgentPlatformAgents_Agents_Model_Agreement {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_Model_AgreementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Model_AgreementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Model_AgreementRef {
        DataAgentPlatformAgents_Agents_Model_AgreementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Model_AgreementRef {
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
pub struct DataAgentPlatformAgents_Agents_Model_Versions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    major: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAgentPlatformAgents_Agents_Model_Versions {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Model_Versions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Model_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Model_Versions {}

impl BuildDataAgentPlatformAgents_Agents_Model_Versions {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Model_Versions {
        DataAgentPlatformAgents_Agents_Model_Versions {
            major: core::default::Default::default(),
            minor: core::default::Default::default(),
            patch: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_Model_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Model_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Model_VersionsRef {
        DataAgentPlatformAgents_Agents_Model_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Model_VersionsRef {
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
pub struct DataAgentPlatformAgents_Agents_Model {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agreement: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Model_Agreement>>,
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
    versions: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Model_Versions>>,
}

impl DataAgentPlatformAgents_Agents_Model {
    #[doc = "Set the field `agreement`.\n"]
    pub fn set_agreement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Model_Agreement>>,
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
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Model_Versions>>,
    ) -> Self {
        self.versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Model {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Model {}

impl BuildDataAgentPlatformAgents_Agents_Model {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Model {
        DataAgentPlatformAgents_Agents_Model {
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
pub struct DataAgentPlatformAgents_Agents_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_ModelRef {
        DataAgentPlatformAgents_Agents_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agreement` after provisioning.\n"]
    pub fn agreement(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Model_AgreementRef> {
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
    pub fn versions(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Model_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_OpenAiApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_OpenAiApiKey {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_OpenAiApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_OpenAiApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_OpenAiApiKey {}

impl BuildDataAgentPlatformAgents_Agents_OpenAiApiKey {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_OpenAiApiKey {
        DataAgentPlatformAgents_Agents_OpenAiApiKey { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_OpenAiApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_OpenAiApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_OpenAiApiKeyRef {
        DataAgentPlatformAgents_Agents_OpenAiApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_OpenAiApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {
        DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            name: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKeyRef {
        DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKeyRef {
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
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_key`.\n"]
    pub fn set_secret_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {
        DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos {
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            deleted_at: core::default::Default::default(),
            name: core::default::Default::default(),
            secret_key: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfosRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfosRef {
        DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfosRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_key` after provisioning.\n"]
    pub fn secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents_ApiKeys {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents_ApiKeys {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys {
        DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgents_ApiKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgents_ApiKeysRef {
        DataAgentPlatformAgents_Agents_ParentAgents_ApiKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ApiKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_Chatbot {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    button_background_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starting_message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents_Chatbot {
    #[doc = "Set the field `button_background_color`.\n"]
    pub fn set_button_background_color(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.button_background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo`.\n"]
    pub fn set_logo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_color`.\n"]
    pub fn set_primary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_color`.\n"]
    pub fn set_secondary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_message`.\n"]
    pub fn set_starting_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents_Chatbot {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents_Chatbot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents_Chatbot {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents_Chatbot {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents_Chatbot {
        DataAgentPlatformAgents_Agents_ParentAgents_Chatbot {
            button_background_color: core::default::Default::default(),
            logo: core::default::Default::default(),
            name: core::default::Default::default(),
            primary_color: core::default::Default::default(),
            secondary_color: core::default::Default::default(),
            starting_message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ChatbotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgents_ChatbotRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgents_ChatbotRef {
        DataAgentPlatformAgents_Agents_ParentAgents_ChatbotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ChatbotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `button_background_color` after provisioning.\n"]
    pub fn button_background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_background_color", self.base))
    }

    #[doc = "Get a reference to the value of field `logo` after provisioning.\n"]
    pub fn logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_color` after provisioning.\n"]
    pub fn primary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_color", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_color` after provisioning.\n"]
    pub fn secondary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_color", self.base))
    }

    #[doc = "Get a reference to the value of field `starting_message` after provisioning.\n"]
    pub fn starting_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {
    #[doc = "Set the field `chatbot_id`.\n"]
    pub fn set_chatbot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chatbot_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {
        DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers {
            chatbot_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiersRef {
        DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chatbot_id` after provisioning.\n"]
    pub fn chatbot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chatbot_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_Deployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents_Deployment {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `visibility`.\n"]
    pub fn set_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visibility = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents_Deployment {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents_Deployment {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents_Deployment {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents_Deployment {
        DataAgentPlatformAgents_Agents_ParentAgents_Deployment {
            created_at: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgents_DeploymentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgents_DeploymentRef {
        DataAgentPlatformAgents_Agents_ParentAgents_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgents_DeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\n"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_ParentAgents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_api_key: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_infos: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_Chatbot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_identifiers: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_Deployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_ParentAgents {
    #[doc = "Set the field `agent_id`.\n"]
    pub fn set_agent_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_api_key`.\n"]
    pub fn set_anthropic_api_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKey>>,
    ) -> Self {
        self.anthropic_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_infos`.\n"]
    pub fn set_api_key_infos(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfos>>,
    ) -> Self {
        self.api_key_infos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_keys`.\n"]
    pub fn set_api_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeys>>,
    ) -> Self {
        self.api_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot`.\n"]
    pub fn set_chatbot(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_Chatbot>>,
    ) -> Self {
        self.chatbot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot_identifiers`.\n"]
    pub fn set_chatbot_identifiers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiers>>,
    ) -> Self {
        self.chatbot_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents_Deployment>>,
    ) -> Self {
        self.deployment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_uuid`.\n"]
    pub fn set_model_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_ParentAgents {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_ParentAgents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_ParentAgents {}

impl BuildDataAgentPlatformAgents_Agents_ParentAgents {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_ParentAgents {
        DataAgentPlatformAgents_Agents_ParentAgents {
            agent_id: core::default::Default::default(),
            anthropic_api_key: core::default::Default::default(),
            api_key_infos: core::default::Default::default(),
            api_keys: core::default::Default::default(),
            chatbot: core::default::Default::default(),
            chatbot_identifiers: core::default::Default::default(),
            deployment: core::default::Default::default(),
            description: core::default::Default::default(),
            instruction: core::default::Default::default(),
            model_uuid: core::default::Default::default(),
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_ParentAgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_ParentAgentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_ParentAgentsRef {
        DataAgentPlatformAgents_Agents_ParentAgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_ParentAgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgents_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.base))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgents_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgents_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgents_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgents_DeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `model_uuid` after provisioning.\n"]
    pub fn model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_uuids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finished_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_base_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    started_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {
    #[doc = "Set the field `completed_datasources`.\n"]
    pub fn set_completed_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.completed_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_uuids`.\n"]
    pub fn set_data_source_uuids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_source_uuids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finished_at`.\n"]
    pub fn set_finished_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.finished_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `knowledge_base_uuid`.\n"]
    pub fn set_knowledge_base_uuid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.knowledge_base_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phase`.\n"]
    pub fn set_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `started_at`.\n"]
    pub fn set_started_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.started_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tokens`.\n"]
    pub fn set_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_datasources`.\n"]
    pub fn set_total_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_datasources = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {}

impl BuildDataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {
        DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob {
            completed_datasources: core::default::Default::default(),
            created_at: core::default::Default::default(),
            data_source_uuids: core::default::Default::default(),
            finished_at: core::default::Default::default(),
            knowledge_base_uuid: core::default::Default::default(),
            phase: core::default::Default::default(),
            started_at: core::default::Default::default(),
            tokens: core::default::Default::default(),
            total_datasources: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJobRef {
        DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completed_datasources` after provisioning.\n"]
    pub fn completed_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_datasources", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_uuids` after provisioning.\n"]
    pub fn data_source_uuids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_uuids", self.base))
    }

    #[doc = "Get a reference to the value of field `finished_at` after provisioning.\n"]
    pub fn finished_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finished_at", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\n"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\n"]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc = "Get a reference to the value of field `started_at` after provisioning.\n"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `total_datasources` after provisioning.\n"]
    pub fn total_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_datasources", self.base))
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
pub struct DataAgentPlatformAgents_Agents_Template_KnowledgeBases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    added_to_agent_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_public: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Template_KnowledgeBases {
    #[doc = "Set the field `added_to_agent_at`.\n"]
    pub fn set_added_to_agent_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.added_to_agent_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_id`.\n"]
    pub fn set_database_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model_uuid`.\n"]
    pub fn set_embedding_model_uuid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.embedding_model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_public`.\n"]
    pub fn set_is_public(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJob>,
                    >,
    ) -> Self {
        self.last_indexing_job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\n"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Template_KnowledgeBases {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Template_KnowledgeBases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Template_KnowledgeBases {}

impl BuildDataAgentPlatformAgents_Agents_Template_KnowledgeBases {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Template_KnowledgeBases {
        DataAgentPlatformAgents_Agents_Template_KnowledgeBases {
            added_to_agent_at: core::default::Default::default(),
            created_at: core::default::Default::default(),
            database_id: core::default::Default::default(),
            embedding_model_uuid: core::default::Default::default(),
            is_public: core::default::Default::default(),
            last_indexing_job: core::default::Default::default(),
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
            region: core::default::Default::default(),
            tags: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_id: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_Template_KnowledgeBasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Template_KnowledgeBasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Template_KnowledgeBasesRef {
        DataAgentPlatformAgents_Agents_Template_KnowledgeBasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Template_KnowledgeBasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `added_to_agent_at` after provisioning.\n"]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `database_id` after provisioning.\n"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.base))
    }

    #[doc = "Get a reference to the value of field `embedding_model_uuid` after provisioning.\n"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `is_public` after provisioning.\n"]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.base))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Template_KnowledgeBases_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Template_Model_Agreement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Template_Model_Agreement {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Template_Model_Agreement {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Template_Model_Agreement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Template_Model_Agreement {}

impl BuildDataAgentPlatformAgents_Agents_Template_Model_Agreement {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Template_Model_Agreement {
        DataAgentPlatformAgents_Agents_Template_Model_Agreement {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_Template_Model_AgreementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Template_Model_AgreementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Template_Model_AgreementRef {
        DataAgentPlatformAgents_Agents_Template_Model_AgreementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Template_Model_AgreementRef {
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
pub struct DataAgentPlatformAgents_Agents_Template_Model_Versions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    major: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAgentPlatformAgents_Agents_Template_Model_Versions {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Template_Model_Versions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Template_Model_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Template_Model_Versions {}

impl BuildDataAgentPlatformAgents_Agents_Template_Model_Versions {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Template_Model_Versions {
        DataAgentPlatformAgents_Agents_Template_Model_Versions {
            major: core::default::Default::default(),
            minor: core::default::Default::default(),
            patch: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_Template_Model_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Template_Model_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Template_Model_VersionsRef {
        DataAgentPlatformAgents_Agents_Template_Model_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Template_Model_VersionsRef {
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
pub struct DataAgentPlatformAgents_Agents_Template_Model {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agreement: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Template_Model_Agreement>>,
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
    versions: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Template_Model_Versions>>,
}

impl DataAgentPlatformAgents_Agents_Template_Model {
    #[doc = "Set the field `agreement`.\n"]
    pub fn set_agreement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Template_Model_Agreement>>,
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
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Template_Model_Versions>>,
    ) -> Self {
        self.versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Template_Model {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Template_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Template_Model {}

impl BuildDataAgentPlatformAgents_Agents_Template_Model {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Template_Model {
        DataAgentPlatformAgents_Agents_Template_Model {
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
pub struct DataAgentPlatformAgents_Agents_Template_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_Template_ModelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgents_Agents_Template_ModelRef {
        DataAgentPlatformAgents_Agents_Template_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_Template_ModelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agreement` after provisioning.\n"]
    pub fn agreement(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Template_Model_AgreementRef> {
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
    pub fn versions(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Template_Model_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Agents_Template {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_bases: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Template_KnowledgeBases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Template_Model>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents_Template {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `knowledge_bases`.\n"]
    pub fn set_knowledge_bases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Template_KnowledgeBases>>,
    ) -> Self {
        self.knowledge_bases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Template_Model>>,
    ) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents_Template {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents_Template>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents_Template {}

impl BuildDataAgentPlatformAgents_Agents_Template {
    pub fn build(self) -> DataAgentPlatformAgents_Agents_Template {
        DataAgentPlatformAgents_Agents_Template {
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            instruction: core::default::Default::default(),
            k: core::default::Default::default(),
            knowledge_bases: core::default::Default::default(),
            max_tokens: core::default::Default::default(),
            model: core::default::Default::default(),
            name: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_Agents_TemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_Agents_TemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_Agents_TemplateRef {
        DataAgentPlatformAgents_Agents_TemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_Agents_TemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `k` after provisioning.\n"]
    pub fn k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_bases` after provisioning.\n"]
    pub fn knowledge_bases(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Template_KnowledgeBasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_bases", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_Template_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
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
pub struct DataAgentPlatformAgents_Agents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_guardrail: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_AgentGuardrail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_api_key: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_AnthropicApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_infos: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ApiKeyInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ApiKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Chatbot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_identifiers: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChatbotIdentifiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    child_agents: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Deployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    functions: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Functions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    if_case: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_bases: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_KnowledgeBases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Model>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_ai_api_key: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_OpenAiApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_agents: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<terrars::ListField<DataAgentPlatformAgents_Agents_Template>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Agents {
    #[doc = "Set the field `agent_guardrail`.\n"]
    pub fn set_agent_guardrail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_AgentGuardrail>>,
    ) -> Self {
        self.agent_guardrail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_id`.\n"]
    pub fn set_agent_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_api_key`.\n"]
    pub fn set_anthropic_api_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_AnthropicApiKey>>,
    ) -> Self {
        self.anthropic_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_infos`.\n"]
    pub fn set_api_key_infos(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ApiKeyInfos>>,
    ) -> Self {
        self.api_key_infos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_keys`.\n"]
    pub fn set_api_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ApiKeys>>,
    ) -> Self {
        self.api_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot`.\n"]
    pub fn set_chatbot(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Chatbot>>,
    ) -> Self {
        self.chatbot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot_identifiers`.\n"]
    pub fn set_chatbot_identifiers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChatbotIdentifiers>>,
    ) -> Self {
        self.chatbot_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `child_agents`.\n"]
    pub fn set_child_agents(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ChildAgents>>,
    ) -> Self {
        self.child_agents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Deployment>>,
    ) -> Self {
        self.deployment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `functions`.\n"]
    pub fn set_functions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Functions>>,
    ) -> Self {
        self.functions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `if_case`.\n"]
    pub fn set_if_case(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.if_case = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `knowledge_bases`.\n"]
    pub fn set_knowledge_bases(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_KnowledgeBases>>,
    ) -> Self {
        self.knowledge_bases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Model>>,
    ) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_uuid`.\n"]
    pub fn set_model_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_ai_api_key`.\n"]
    pub fn set_open_ai_api_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_OpenAiApiKey>>,
    ) -> Self {
        self.open_ai_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent_agents`.\n"]
    pub fn set_parent_agents(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_ParentAgents>>,
    ) -> Self {
        self.parent_agents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieval_method`.\n"]
    pub fn set_retrieval_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retrieval_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_created_at`.\n"]
    pub fn set_route_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_created_by`.\n"]
    pub fn set_route_created_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_created_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_name`.\n"]
    pub fn set_route_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_uuid`.\n"]
    pub fn set_route_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_uuid = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgents_Agents_Template>>,
    ) -> Self {
        self.template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Agents {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Agents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Agents {}

impl BuildDataAgentPlatformAgents_Agents {
    pub fn build(self) -> DataAgentPlatformAgents_Agents {
        DataAgentPlatformAgents_Agents {
            agent_guardrail: core::default::Default::default(),
            agent_id: core::default::Default::default(),
            anthropic_api_key: core::default::Default::default(),
            api_key_infos: core::default::Default::default(),
            api_keys: core::default::Default::default(),
            chatbot: core::default::Default::default(),
            chatbot_identifiers: core::default::Default::default(),
            child_agents: core::default::Default::default(),
            created_at: core::default::Default::default(),
            deployment: core::default::Default::default(),
            description: core::default::Default::default(),
            functions: core::default::Default::default(),
            if_case: core::default::Default::default(),
            instruction: core::default::Default::default(),
            k: core::default::Default::default(),
            knowledge_bases: core::default::Default::default(),
            max_tokens: core::default::Default::default(),
            model: core::default::Default::default(),
            model_uuid: core::default::Default::default(),
            name: core::default::Default::default(),
            open_ai_api_key: core::default::Default::default(),
            parent_agents: core::default::Default::default(),
            project_id: core::default::Default::default(),
            region: core::default::Default::default(),
            retrieval_method: core::default::Default::default(),
            route_created_at: core::default::Default::default(),
            route_created_by: core::default::Default::default(),
            route_name: core::default::Default::default(),
            route_uuid: core::default::Default::default(),
            tags: core::default::Default::default(),
            temperature: core::default::Default::default(),
            template: core::default::Default::default(),
            top_p: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            url: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_AgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_AgentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_AgentsRef {
        DataAgentPlatformAgents_AgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_AgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_guardrail` after provisioning.\n"]
    pub fn agent_guardrail(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_AgentGuardrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_guardrail", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.base))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\n"]
    pub fn child_agents(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ChildAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.child_agents", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_DeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_FunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.functions", self.base))
    }

    #[doc = "Get a reference to the value of field `if_case` after provisioning.\n"]
    pub fn if_case(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.if_case", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `k` after provisioning.\n"]
    pub fn k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_bases` after provisioning.\n"]
    pub fn knowledge_bases(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_KnowledgeBasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_bases", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `model_uuid` after provisioning.\n"]
    pub fn model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `open_ai_api_key` after provisioning.\n"]
    pub fn open_ai_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_OpenAiApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_ai_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `parent_agents` after provisioning.\n"]
    pub fn parent_agents(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_ParentAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parent_agents", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `retrieval_method` after provisioning.\n"]
    pub fn retrieval_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieval_method", self.base))
    }

    #[doc = "Get a reference to the value of field `route_created_at` after provisioning.\n"]
    pub fn route_created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `route_created_by` after provisioning.\n"]
    pub fn route_created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_created_by", self.base))
    }

    #[doc = "Get a reference to the value of field `route_name` after provisioning.\n"]
    pub fn route_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_name", self.base))
    }

    #[doc = "Get a reference to the value of field `route_uuid` after provisioning.\n"]
    pub fn route_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<DataAgentPlatformAgents_Agents_TemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgents_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_by: std::option::Option<terrars::PrimField<std::string::String>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgents_Filter {
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

impl terrars::ToListMappable for DataAgentPlatformAgents_Filter {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataAgentPlatformAgents_Filter {
    pub fn build(self) -> DataAgentPlatformAgents_Filter {
        DataAgentPlatformAgents_Filter {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_FilterRef {
        DataAgentPlatformAgents_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_FilterRef {
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
pub struct DataAgentPlatformAgents_Sort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
}

impl DataAgentPlatformAgents_Sort {
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgents_Sort {
    type O = terrars::BlockAssignable<DataAgentPlatformAgents_Sort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgents_Sort {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformAgents_Sort {
    pub fn build(self) -> DataAgentPlatformAgents_Sort {
        DataAgentPlatformAgents_Sort {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgents_SortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgents_SortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgents_SortRef {
        DataAgentPlatformAgents_SortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgents_SortRef {
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
struct DataAgentPlatformAgentsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgents_Filter>>,
    sort: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgents_Sort>>,
}
