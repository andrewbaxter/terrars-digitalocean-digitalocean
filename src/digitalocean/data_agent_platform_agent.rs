#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformAgentData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    if_case: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_method: std::option::Option<terrars::PrimField<std::string::String>>,
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
    top_p: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_guardrail: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_AgentGuardrail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_api_key: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_AnthropicApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_infos: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_ApiKeyInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_ApiKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Chatbot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_identifiers: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_ChatbotIdentifiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Deployment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    functions: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Functions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_bases: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_KnowledgeBases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Model>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_ai_api_key: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_OpenAiApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Template>>,
    dynamic: DataAgentPlatformAgentDynamic,
}

struct DataAgentPlatformAgent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformAgentData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformAgent(std::rc::Rc<DataAgentPlatformAgent_>);

impl DataAgentPlatformAgent {
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

    #[doc = "Set the field `description`.\nDescription for the Agent"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `if_case`.\nIf case condition"]
    pub fn set_if_case(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().if_case = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `k`.\nK value"]
    pub fn set_k(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().k = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\nMaximum tokens allowed"]
    pub fn set_max_tokens(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieval_method`.\nRetrieval method used"]
    pub fn set_retrieval_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().retrieval_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_created_by`.\nUser who created the route"]
    pub fn set_route_created_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().route_created_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_name`.\nRoute name"]
    pub fn set_route_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().route_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_uuid`.\nRoute UUID"]
    pub fn set_route_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().route_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nList of Tags"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\nAgent temperature setting"]
    pub fn set_temperature(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\nTop P sampling parameter"]
    pub fn set_top_p(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().top_p = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nURL for the Agent"]
    pub fn set_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\nUser ID linked with the Agent"]
    pub fn set_user_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_guardrail`.\n"]
    pub fn set_agent_guardrail(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_AgentGuardrail>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().agent_guardrail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.agent_guardrail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `anthropic_api_key`.\n"]
    pub fn set_anthropic_api_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_AnthropicApiKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().anthropic_api_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.anthropic_api_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `api_key_infos`.\n"]
    pub fn set_api_key_infos(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_ApiKeyInfos>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_key_infos = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_key_infos = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `api_keys`.\n"]
    pub fn set_api_keys(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_ApiKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `chatbot`.\n"]
    pub fn set_chatbot(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Chatbot>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().chatbot = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.chatbot = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `chatbot_identifiers`.\n"]
    pub fn set_chatbot_identifiers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_ChatbotIdentifiers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().chatbot_identifiers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.chatbot_identifiers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Deployment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `functions`.\n"]
    pub fn set_functions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Functions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().functions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.functions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `knowledge_bases`.\n"]
    pub fn set_knowledge_bases(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_KnowledgeBases>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_bases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.knowledge_bases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(self, v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Model>>) -> Self {
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

    #[doc = "Set the field `open_ai_api_key`.\n"]
    pub fn set_open_ai_api_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_OpenAiApiKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_ai_api_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_ai_api_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template`.\n"]
    pub fn set_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Template>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\nID of the Agent to retrieve"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\nList of child agents"]
    pub fn child_agents(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.child_agents", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the Agent was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the Agent"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `if_case` after provisioning.\nIf case condition"]
    pub fn if_case(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.if_case", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\nInstruction for the Agent"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `k` after provisioning.\nK value"]
    pub fn k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\nMaximum tokens allowed"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_uuid` after provisioning.\nModel UUID of the Agent"]
    pub fn model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Agent"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_agents` after provisioning.\nList of parent agents"]
    pub fn parent_agents(&self) -> terrars::ListRef<DataAgentPlatformAgent_ParentAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parent_agents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID of the Agent"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the Agent is deployed"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retrieval_method` after provisioning.\nRetrieval method used"]
    pub fn retrieval_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieval_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `route_created_at` after provisioning.\nTimestamp when the route was created"]
    pub fn route_created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_created_by` after provisioning.\nUser who created the route"]
    pub fn route_created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_name` after provisioning.\nRoute name"]
    pub fn route_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_uuid` after provisioning.\nRoute UUID"]
    pub fn route_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nList of Tags"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nAgent temperature setting"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\nTop P sampling parameter"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Agent was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL for the Agent"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nUser ID linked with the Agent"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_guardrail` after provisioning.\n"]
    pub fn agent_guardrail(&self) -> terrars::ListRef<DataAgentPlatformAgent_AgentGuardrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_guardrail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgent_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgent_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgent_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgent_DeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(&self) -> terrars::ListRef<DataAgentPlatformAgent_FunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_bases` after provisioning.\n"]
    pub fn knowledge_bases(&self) -> terrars::ListRef<DataAgentPlatformAgent_KnowledgeBasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_bases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<DataAgentPlatformAgent_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_ai_api_key` after provisioning.\n"]
    pub fn open_ai_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgent_OpenAiApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_ai_api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<DataAgentPlatformAgent_TemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformAgent {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformAgent { }

impl terrars::ToListMappable for DataAgentPlatformAgent {
    type O = terrars::ListRef<DataAgentPlatformAgentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformAgent_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_agent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformAgent {
    pub tf_id: std::string::String,
    #[doc = "ID of the Agent to retrieve"]
    pub agent_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformAgent {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformAgent {
        let out = DataAgentPlatformAgent(std::rc::Rc::new(DataAgentPlatformAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformAgentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                agent_id: self.agent_id,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                if_case: core::default::Default::default(),
                k: core::default::Default::default(),
                max_tokens: core::default::Default::default(),
                retrieval_method: core::default::Default::default(),
                route_created_by: core::default::Default::default(),
                route_name: core::default::Default::default(),
                route_uuid: core::default::Default::default(),
                tags: core::default::Default::default(),
                temperature: core::default::Default::default(),
                top_p: core::default::Default::default(),
                url: core::default::Default::default(),
                user_id: core::default::Default::default(),
                agent_guardrail: core::default::Default::default(),
                anthropic_api_key: core::default::Default::default(),
                api_key_infos: core::default::Default::default(),
                api_keys: core::default::Default::default(),
                chatbot: core::default::Default::default(),
                chatbot_identifiers: core::default::Default::default(),
                deployment: core::default::Default::default(),
                functions: core::default::Default::default(),
                knowledge_bases: core::default::Default::default(),
                model: core::default::Default::default(),
                open_ai_api_key: core::default::Default::default(),
                template: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformAgentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\nID of the Agent to retrieve"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\nList of child agents"]
    pub fn child_agents(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.child_agents", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the Agent was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription for the Agent"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `if_case` after provisioning.\nIf case condition"]
    pub fn if_case(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.if_case", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\nInstruction for the Agent"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `k` after provisioning.\nK value"]
    pub fn k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\nMaximum tokens allowed"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_uuid` after provisioning.\nModel UUID of the Agent"]
    pub fn model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Agent"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_agents` after provisioning.\nList of parent agents"]
    pub fn parent_agents(&self) -> terrars::ListRef<DataAgentPlatformAgent_ParentAgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parent_agents", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID of the Agent"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the Agent is deployed"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retrieval_method` after provisioning.\nRetrieval method used"]
    pub fn retrieval_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieval_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `route_created_at` after provisioning.\nTimestamp when the route was created"]
    pub fn route_created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_created_by` after provisioning.\nUser who created the route"]
    pub fn route_created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_name` after provisioning.\nRoute name"]
    pub fn route_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_uuid` after provisioning.\nRoute UUID"]
    pub fn route_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nList of Tags"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nAgent temperature setting"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\nTop P sampling parameter"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Agent was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL for the Agent"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nUser ID linked with the Agent"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `agent_guardrail` after provisioning.\n"]
    pub fn agent_guardrail(&self) -> terrars::ListRef<DataAgentPlatformAgent_AgentGuardrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_guardrail", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgent_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgent_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgent_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgent_DeploymentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(&self) -> terrars::ListRef<DataAgentPlatformAgent_FunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_bases` after provisioning.\n"]
    pub fn knowledge_bases(&self) -> terrars::ListRef<DataAgentPlatformAgent_KnowledgeBasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_bases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<DataAgentPlatformAgent_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_ai_api_key` after provisioning.\n"]
    pub fn open_ai_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgent_OpenAiApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_ai_api_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<DataAgentPlatformAgent_TemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataAgentPlatformAgentDynamic {
    agent_guardrail: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_AgentGuardrail>>,
    anthropic_api_key: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_AnthropicApiKey>>,
    api_key_infos: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_ApiKeyInfos>>,
    api_keys: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_ApiKeys>>,
    chatbot: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Chatbot>>,
    chatbot_identifiers: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_ChatbotIdentifiers>>,
    deployment: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Deployment>>,
    functions: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Functions>>,
    knowledge_bases: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_KnowledgeBases>>,
    model: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Model>>,
    open_ai_api_key: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_OpenAiApiKey>>,
    template: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Template>>,
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_ChildAgents_AnthropicApiKey {
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

impl DataAgentPlatformAgent_ChildAgents_AnthropicApiKey {
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

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents_AnthropicApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents_AnthropicApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents_AnthropicApiKey {}

impl BuildDataAgentPlatformAgent_ChildAgents_AnthropicApiKey {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents_AnthropicApiKey {
        DataAgentPlatformAgent_ChildAgents_AnthropicApiKey {
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
pub struct DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef {
        DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef {
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
pub struct DataAgentPlatformAgent_ChildAgents_ApiKeyInfos {
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

impl DataAgentPlatformAgent_ChildAgents_ApiKeyInfos {
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

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents_ApiKeyInfos {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents_ApiKeyInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents_ApiKeyInfos {}

impl BuildDataAgentPlatformAgent_ChildAgents_ApiKeyInfos {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents_ApiKeyInfos {
        DataAgentPlatformAgent_ChildAgents_ApiKeyInfos {
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
pub struct DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef {
        DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef {
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
pub struct DataAgentPlatformAgent_ChildAgents_ApiKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_ChildAgents_ApiKeys {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents_ApiKeys {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents_ApiKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents_ApiKeys {}

impl BuildDataAgentPlatformAgent_ChildAgents_ApiKeys {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents_ApiKeys {
        DataAgentPlatformAgent_ChildAgents_ApiKeys { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_ChildAgents_ApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgents_ApiKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ChildAgents_ApiKeysRef {
        DataAgentPlatformAgent_ChildAgents_ApiKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgents_ApiKeysRef {
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
pub struct DataAgentPlatformAgent_ChildAgents_Chatbot {
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

impl DataAgentPlatformAgent_ChildAgents_Chatbot {
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

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents_Chatbot {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents_Chatbot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents_Chatbot {}

impl BuildDataAgentPlatformAgent_ChildAgents_Chatbot {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents_Chatbot {
        DataAgentPlatformAgent_ChildAgents_Chatbot {
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
pub struct DataAgentPlatformAgent_ChildAgents_ChatbotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgents_ChatbotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ChildAgents_ChatbotRef {
        DataAgentPlatformAgent_ChildAgents_ChatbotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgents_ChatbotRef {
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
pub struct DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers {
    #[doc = "Set the field `chatbot_id`.\n"]
    pub fn set_chatbot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chatbot_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers {}

impl BuildDataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers {
        DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers { chatbot_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef {
        DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef {
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
pub struct DataAgentPlatformAgent_ChildAgents_Deployment {
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

impl DataAgentPlatformAgent_ChildAgents_Deployment {
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

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents_Deployment {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents_Deployment {}

impl BuildDataAgentPlatformAgent_ChildAgents_Deployment {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents_Deployment {
        DataAgentPlatformAgent_ChildAgents_Deployment {
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
pub struct DataAgentPlatformAgent_ChildAgents_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgents_DeploymentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgent_ChildAgents_DeploymentRef {
        DataAgentPlatformAgent_ChildAgents_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgents_DeploymentRef {
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
pub struct DataAgentPlatformAgent_ChildAgents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_api_key: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_AnthropicApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_infos: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeyInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Chatbot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_identifiers: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Deployment>>,
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

impl DataAgentPlatformAgent_ChildAgents {
    #[doc = "Set the field `agent_id`.\n"]
    pub fn set_agent_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_api_key`.\n"]
    pub fn set_anthropic_api_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_AnthropicApiKey>>,
    ) -> Self {
        self.anthropic_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_infos`.\n"]
    pub fn set_api_key_infos(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeyInfos>>,
    ) -> Self {
        self.api_key_infos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_keys`.\n"]
    pub fn set_api_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeys>>,
    ) -> Self {
        self.api_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot`.\n"]
    pub fn set_chatbot(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Chatbot>>,
    ) -> Self {
        self.chatbot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot_identifiers`.\n"]
    pub fn set_chatbot_identifiers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers>>,
    ) -> Self {
        self.chatbot_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Deployment>>,
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

impl terrars::ToListMappable for DataAgentPlatformAgent_ChildAgents {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChildAgents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChildAgents {}

impl BuildDataAgentPlatformAgent_ChildAgents {
    pub fn build(self) -> DataAgentPlatformAgent_ChildAgents {
        DataAgentPlatformAgent_ChildAgents {
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
pub struct DataAgentPlatformAgent_ChildAgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChildAgentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ChildAgentsRef {
        DataAgentPlatformAgent_ChildAgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChildAgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.base))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_DeploymentRef> {
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
pub struct DataAgentPlatformAgent_ParentAgents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anthropic_api_key: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_AnthropicApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_infos: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeyInfos>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Chatbot>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_identifiers: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Deployment>>,
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

impl DataAgentPlatformAgent_ParentAgents {
    #[doc = "Set the field `agent_id`.\n"]
    pub fn set_agent_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anthropic_api_key`.\n"]
    pub fn set_anthropic_api_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_AnthropicApiKey>>,
    ) -> Self {
        self.anthropic_api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_infos`.\n"]
    pub fn set_api_key_infos(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeyInfos>>,
    ) -> Self {
        self.api_key_infos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_keys`.\n"]
    pub fn set_api_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ApiKeys>>,
    ) -> Self {
        self.api_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot`.\n"]
    pub fn set_chatbot(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Chatbot>>,
    ) -> Self {
        self.chatbot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chatbot_identifiers`.\n"]
    pub fn set_chatbot_identifiers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiers>>,
    ) -> Self {
        self.chatbot_identifiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformAgent_ChildAgents_Deployment>>,
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

impl terrars::ToListMappable for DataAgentPlatformAgent_ParentAgents {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ParentAgents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ParentAgents {}

impl BuildDataAgentPlatformAgent_ParentAgents {
    pub fn build(self) -> DataAgentPlatformAgent_ParentAgents {
        DataAgentPlatformAgent_ParentAgents {
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
pub struct DataAgentPlatformAgent_ParentAgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ParentAgentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ParentAgentsRef {
        DataAgentPlatformAgent_ParentAgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ParentAgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_id` after provisioning.\n"]
    pub fn agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_id", self.base))
    }

    #[doc = "Get a reference to the value of field `anthropic_api_key` after provisioning.\n"]
    pub fn anthropic_api_key(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_AnthropicApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anthropic_api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_infos` after provisioning.\n"]
    pub fn api_key_infos(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ApiKeyInfosRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_infos", self.base))
    }

    #[doc = "Get a reference to the value of field `api_keys` after provisioning.\n"]
    pub fn api_keys(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ApiKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot` after provisioning.\n"]
    pub fn chatbot(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ChatbotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot", self.base))
    }

    #[doc = "Get a reference to the value of field `chatbot_identifiers` after provisioning.\n"]
    pub fn chatbot_identifiers(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_ChatbotIdentifiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chatbot_identifiers", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<DataAgentPlatformAgent_ChildAgents_DeploymentRef> {
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
pub struct DataAgentPlatformAgent_AgentGuardrail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrail_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_AgentGuardrail {
    #[doc = "Set the field `agent_uuid`.\nAgent UUID for the Guardrail"]
    pub fn set_agent_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_response`.\nDefault response for the Guardrail"]
    pub fn set_default_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the Guardrail"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_uuid`.\nGuardrail UUID"]
    pub fn set_guardrail_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guardrail_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_default`.\nIndicates if the Guardrail is default"]
    pub fn set_is_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of Guardrail"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\nPriority of the Guardrail"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nType of the Guardrail"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nGuardrail UUID"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_AgentGuardrail {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_AgentGuardrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_AgentGuardrail {}

impl BuildDataAgentPlatformAgent_AgentGuardrail {
    pub fn build(self) -> DataAgentPlatformAgent_AgentGuardrail {
        DataAgentPlatformAgent_AgentGuardrail {
            agent_uuid: core::default::Default::default(),
            default_response: core::default::Default::default(),
            description: core::default::Default::default(),
            guardrail_uuid: core::default::Default::default(),
            is_default: core::default::Default::default(),
            name: core::default::Default::default(),
            priority: core::default::Default::default(),
            type_: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_AgentGuardrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_AgentGuardrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_AgentGuardrailRef {
        DataAgentPlatformAgent_AgentGuardrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_AgentGuardrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_uuid` after provisioning.\nAgent UUID for the Guardrail"]
    pub fn agent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_uuid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Guardrail"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_response` after provisioning.\nDefault response for the Guardrail"]
    pub fn default_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_response", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the Guardrail"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrail_uuid` after provisioning.\nGuardrail UUID"]
    pub fn guardrail_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_uuid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_attached` after provisioning.\nIndicates if the Guardrail is attached"]
    pub fn is_attached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_attached", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_default` after provisioning.\nIndicates if the Guardrail is default"]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of Guardrail"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority of the Guardrail"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the Guardrail"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nUpdated At timestamp for the Guardrail"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nGuardrail UUID"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_AnthropicApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_AnthropicApiKey {
    #[doc = "Set the field `created_by`.\nCreated By user ID for the API Key"]
    pub fn set_created_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the API Key"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nAPI Key value"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_AnthropicApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_AnthropicApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_AnthropicApiKey {}

impl BuildDataAgentPlatformAgent_AnthropicApiKey {
    pub fn build(self) -> DataAgentPlatformAgent_AnthropicApiKey {
        DataAgentPlatformAgent_AnthropicApiKey {
            created_by: core::default::Default::default(),
            name: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_AnthropicApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_AnthropicApiKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_AnthropicApiKeyRef {
        DataAgentPlatformAgent_AnthropicApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_AnthropicApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the API Key was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_by` after provisioning.\nCreated By user ID for the API Key"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nDeleted At timestamp for the API Key"]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the API Key"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nUpdated At timestamp for the API Key"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nAPI Key value"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_ApiKeyInfos {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_ApiKeyInfos {
    #[doc = "Set the field `created_by`.\nCreated By user ID for the API Key"]
    pub fn set_created_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the API Key"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_key`.\nUpdated At timestamp for the API Key"]
    pub fn set_secret_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nAPI Key value"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_ApiKeyInfos {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ApiKeyInfos>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ApiKeyInfos {}

impl BuildDataAgentPlatformAgent_ApiKeyInfos {
    pub fn build(self) -> DataAgentPlatformAgent_ApiKeyInfos {
        DataAgentPlatformAgent_ApiKeyInfos {
            created_by: core::default::Default::default(),
            name: core::default::Default::default(),
            secret_key: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_ApiKeyInfosRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ApiKeyInfosRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ApiKeyInfosRef {
        DataAgentPlatformAgent_ApiKeyInfosRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ApiKeyInfosRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nAPI Key value"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_by` after provisioning.\nCreated By user ID for the API Key"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deleted_at` after provisioning.\nDeleted At timestamp for the API Key"]
    pub fn deleted_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the API Key"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_key` after provisioning.\nUpdated At timestamp for the API Key"]
    pub fn secret_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_key", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nAPI Key value"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_ApiKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_ApiKeys {
    #[doc = "Set the field `api_key`.\nAPI Key value"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_ApiKeys {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ApiKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ApiKeys {}

impl BuildDataAgentPlatformAgent_ApiKeys {
    pub fn build(self) -> DataAgentPlatformAgent_ApiKeys {
        DataAgentPlatformAgent_ApiKeys { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_ApiKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ApiKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ApiKeysRef {
        DataAgentPlatformAgent_ApiKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ApiKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nAPI Key value"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_Chatbot {
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

impl DataAgentPlatformAgent_Chatbot {
    #[doc = "Set the field `button_background_color`.\nBackground color for the chatbot button"]
    pub fn set_button_background_color(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.button_background_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logo`.\nLogo for the chatbot"]
    pub fn set_logo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the chatbot"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_color`.\nPrimary color for the chatbot"]
    pub fn set_primary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_color`.\nSecondary color for the chatbot"]
    pub fn set_secondary_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secondary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starting_message`.\nStarting message for the chatbot"]
    pub fn set_starting_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.starting_message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_Chatbot {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Chatbot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Chatbot {}

impl BuildDataAgentPlatformAgent_Chatbot {
    pub fn build(self) -> DataAgentPlatformAgent_Chatbot {
        DataAgentPlatformAgent_Chatbot {
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
pub struct DataAgentPlatformAgent_ChatbotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChatbotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ChatbotRef {
        DataAgentPlatformAgent_ChatbotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChatbotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `button_background_color` after provisioning.\nBackground color for the chatbot button"]
    pub fn button_background_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.button_background_color", self.base))
    }

    #[doc = "Get a reference to the value of field `logo` after provisioning.\nLogo for the chatbot"]
    pub fn logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logo", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the chatbot"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_color` after provisioning.\nPrimary color for the chatbot"]
    pub fn primary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secondary_color` after provisioning.\nSecondary color for the chatbot"]
    pub fn secondary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_color", self.base))
    }

    #[doc =
        "Get a reference to the value of field `starting_message` after provisioning.\nStarting message for the chatbot"]
    pub fn starting_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_ChatbotIdentifiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chatbot_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_ChatbotIdentifiers {
    #[doc = "Set the field `chatbot_id`.\nChatbot ID"]
    pub fn set_chatbot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.chatbot_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_ChatbotIdentifiers {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_ChatbotIdentifiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_ChatbotIdentifiers {}

impl BuildDataAgentPlatformAgent_ChatbotIdentifiers {
    pub fn build(self) -> DataAgentPlatformAgent_ChatbotIdentifiers {
        DataAgentPlatformAgent_ChatbotIdentifiers { chatbot_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_ChatbotIdentifiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ChatbotIdentifiersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ChatbotIdentifiersRef {
        DataAgentPlatformAgent_ChatbotIdentifiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ChatbotIdentifiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chatbot_id` after provisioning.\nChatbot ID"]
    pub fn chatbot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chatbot_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_Deployment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_Deployment {
    #[doc = "Set the field `name`.\nName of the API Key"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\nStatus of the Deployment"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nUrl of the Deployment"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nAPI Key value"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visibility`.\nVisibility of the Deployment"]
    pub fn set_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.visibility = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_Deployment {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Deployment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Deployment {}

impl BuildDataAgentPlatformAgent_Deployment {
    pub fn build(self) -> DataAgentPlatformAgent_Deployment {
        DataAgentPlatformAgent_Deployment {
            name: core::default::Default::default(),
            status: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
            visibility: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_DeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_DeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_DeploymentRef {
        DataAgentPlatformAgent_DeploymentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_DeploymentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nAPI Key value"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the API Key"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the Deployment"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nUpdated At timestamp for the Agent"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nUrl of the Deployment"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nAPI Key value"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `visibility` after provisioning.\nVisibility of the Deployment"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_Functions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
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
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_Functions {
    #[doc = "Set the field `api_key`.\nAPI Key value"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the Function"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faasname`.\nName of function"]
    pub fn set_faasname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.faasname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `faasnamespace`.\nNamespace of function"]
    pub fn set_faasnamespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.faasnamespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrail_uuid`.\nGuardrail UUID for the Function"]
    pub fn set_guardrail_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.guardrail_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of function"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nUrl of the Deployment"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nAPI Key value"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_Functions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Functions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Functions {}

impl BuildDataAgentPlatformAgent_Functions {
    pub fn build(self) -> DataAgentPlatformAgent_Functions {
        DataAgentPlatformAgent_Functions {
            api_key: core::default::Default::default(),
            description: core::default::Default::default(),
            faasname: core::default::Default::default(),
            faasnamespace: core::default::Default::default(),
            guardrail_uuid: core::default::Default::default(),
            name: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_FunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_FunctionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_FunctionsRef {
        DataAgentPlatformAgent_FunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_FunctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nAPI Key value"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Function"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the Function"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `faasname` after provisioning.\nName of function"]
    pub fn faasname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.faasname", self.base))
    }

    #[doc = "Get a reference to the value of field `faasnamespace` after provisioning.\nNamespace of function"]
    pub fn faasnamespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.faasnamespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `guardrail_uuid` after provisioning.\nGuardrail UUID for the Function"]
    pub fn guardrail_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of function"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nUpdated At timestamp for the Agent"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nUrl of the Deployment"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nAPI Key value"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_uuids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {
    #[doc = "Set the field `completed_datasources`.\nNumber of completed datasources in the last indexing job"]
    pub fn set_completed_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.completed_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_uuids`.\nDatasource UUIDs for the last indexing job"]
    pub fn set_data_source_uuids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_source_uuids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phase`.\nPhase of the last indexing job"]
    pub fn set_phase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tokens`.\nNumber of tokens processed in the last indexing job"]
    pub fn set_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_datasources`.\nTotal number of datasources in the last indexing job"]
    pub fn set_total_datasources(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_datasources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nUUID  of the last indexing job"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {}

impl BuildDataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {
    pub fn build(self) -> DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {
        DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob {
            completed_datasources: core::default::Default::default(),
            data_source_uuids: core::default::Default::default(),
            phase: core::default::Default::default(),
            tokens: core::default::Default::default(),
            total_datasources: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_KnowledgeBases_LastIndexingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_KnowledgeBases_LastIndexingJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformAgent_KnowledgeBases_LastIndexingJobRef {
        DataAgentPlatformAgent_KnowledgeBases_LastIndexingJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_KnowledgeBases_LastIndexingJobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `completed_datasources` after provisioning.\nNumber of completed datasources in the last indexing job"]
    pub fn completed_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_datasources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the last indexing job"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_source_uuids` after provisioning.\nDatasource UUIDs for the last indexing job"]
    pub fn data_source_uuids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_uuids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `finished_at` after provisioning.\nTimestamp when the last indexing job finished"]
    pub fn finished_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finished_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID  of the Knowledge Base for the last indexing job"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\nPhase of the last indexing job"]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `started_at` after provisioning.\nTimestamp when the last indexing job started"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tokens` after provisioning.\nNumber of tokens processed in the last indexing job"]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_datasources` after provisioning.\nTotal number of datasources in the last indexing job"]
    pub fn total_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_datasources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the last indexing job updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID  of the last indexing job"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAgentPlatformAgent_KnowledgeBasesDynamic {
    last_indexing_job: std
    ::option
    ::Option<terrars::DynamicBlock<DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob>>,
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_KnowledgeBases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_public: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob>>,
    dynamic: DataAgentPlatformAgent_KnowledgeBasesDynamic,
}

impl DataAgentPlatformAgent_KnowledgeBases {
    #[doc = "Set the field `database_id`.\nDatabase ID of the Knowledge Base"]
    pub fn set_database_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model_uuid`.\nEmbedding model UUID for the Knowledge Base"]
    pub fn set_embedding_model_uuid(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.embedding_model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_public`.\nIndicates if the Knowledge Base is public"]
    pub fn set_is_public(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the Knowledge Base"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nProject ID of the Knowledge Base"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion of the Knowledge Base"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nList of tags"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\nUser ID of the Knowledge Base"]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_KnowledgeBases_LastIndexingJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_indexing_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_indexing_job = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_KnowledgeBases {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_KnowledgeBases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_KnowledgeBases {}

impl BuildDataAgentPlatformAgent_KnowledgeBases {
    pub fn build(self) -> DataAgentPlatformAgent_KnowledgeBases {
        DataAgentPlatformAgent_KnowledgeBases {
            database_id: core::default::Default::default(),
            embedding_model_uuid: core::default::Default::default(),
            is_public: core::default::Default::default(),
            name: core::default::Default::default(),
            project_id: core::default::Default::default(),
            region: core::default::Default::default(),
            tags: core::default::Default::default(),
            user_id: core::default::Default::default(),
            last_indexing_job: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_KnowledgeBasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_KnowledgeBasesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_KnowledgeBasesRef {
        DataAgentPlatformAgent_KnowledgeBasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_KnowledgeBasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `added_to_agent_at` after provisioning.\nTimestamp when the Knowledge Base was added to the Agent"]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Knowledge Base"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nDatabase ID of the Knowledge Base"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `embedding_model_uuid` after provisioning.\nEmbedding model UUID for the Knowledge Base"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_public` after provisioning.\nIndicates if the Knowledge Base is public"]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Knowledge Base"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID of the Knowledge Base"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the Knowledge Base"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nList of tags"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Knowledge Base was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nUser ID of the Knowledge Base"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(&self) -> terrars::ListRef<DataAgentPlatformAgent_KnowledgeBases_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_Model_Agreement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_Model_Agreement {
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

impl terrars::ToListMappable for DataAgentPlatformAgent_Model_Agreement {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Model_Agreement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Model_Agreement {}

impl BuildDataAgentPlatformAgent_Model_Agreement {
    pub fn build(self) -> DataAgentPlatformAgent_Model_Agreement {
        DataAgentPlatformAgent_Model_Agreement {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            url: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_Model_AgreementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_Model_AgreementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_Model_AgreementRef {
        DataAgentPlatformAgent_Model_AgreementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_Model_AgreementRef {
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
pub struct DataAgentPlatformAgent_Model_Versions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    major: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAgentPlatformAgent_Model_Versions {
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

impl terrars::ToListMappable for DataAgentPlatformAgent_Model_Versions {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Model_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Model_Versions {}

impl BuildDataAgentPlatformAgent_Model_Versions {
    pub fn build(self) -> DataAgentPlatformAgent_Model_Versions {
        DataAgentPlatformAgent_Model_Versions {
            major: core::default::Default::default(),
            minor: core::default::Default::default(),
            patch: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_Model_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_Model_VersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_Model_VersionsRef {
        DataAgentPlatformAgent_Model_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_Model_VersionsRef {
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
struct DataAgentPlatformAgent_ModelDynamic {
    agreement: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Model_Agreement>>,
    versions: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Model_Versions>>,
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_Model {
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
    agreement: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Model_Agreement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Model_Versions>>,
    dynamic: DataAgentPlatformAgent_ModelDynamic,
}

impl DataAgentPlatformAgent_Model {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Model_Agreement>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Model_Versions>>,
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

impl terrars::ToListMappable for DataAgentPlatformAgent_Model {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Model>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Model {}

impl BuildDataAgentPlatformAgent_Model {
    pub fn build(self) -> DataAgentPlatformAgent_Model {
        DataAgentPlatformAgent_Model {
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
pub struct DataAgentPlatformAgent_ModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_ModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_ModelRef {
        DataAgentPlatformAgent_ModelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_ModelRef {
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
    pub fn agreement(&self) -> terrars::ListRef<DataAgentPlatformAgent_Model_AgreementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agreement", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<DataAgentPlatformAgent_Model_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_OpenAiApiKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformAgent_OpenAiApiKey {
    #[doc = "Set the field `api_key`.\nOpenAI API Key"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_OpenAiApiKey {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_OpenAiApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_OpenAiApiKey {}

impl BuildDataAgentPlatformAgent_OpenAiApiKey {
    pub fn build(self) -> DataAgentPlatformAgent_OpenAiApiKey {
        DataAgentPlatformAgent_OpenAiApiKey { api_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_OpenAiApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_OpenAiApiKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_OpenAiApiKeyRef {
        DataAgentPlatformAgent_OpenAiApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_OpenAiApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nOpenAI API Key"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAgentPlatformAgent_TemplateDynamic {
    knowledge_bases: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_KnowledgeBases>>,
    model: std::option::Option<terrars::DynamicBlock<DataAgentPlatformAgent_Model>>,
}

#[derive(Serialize)]
pub struct DataAgentPlatformAgent_Template {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    k: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knowledge_bases: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_KnowledgeBases>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<std::vec::Vec<DataAgentPlatformAgent_Model>>,
    dynamic: DataAgentPlatformAgent_TemplateDynamic,
}

impl DataAgentPlatformAgent_Template {
    #[doc = "Set the field `description`.\nDescription of the Agent Template"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\nInstruction for the Agent"]
    pub fn set_instruction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `k`.\nK value for the Agent Template"]
    pub fn set_k(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.k = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_tokens`.\nMaximum tokens allowed"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the Agent Template"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\nAgent temperature setting"]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\nTop P sampling parameter"]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nuuid of the Agent Template"]
    pub fn set_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `knowledge_bases`.\n"]
    pub fn set_knowledge_bases(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_KnowledgeBases>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knowledge_bases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knowledge_bases = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model`.\n"]
    pub fn set_model(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformAgent_Model>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformAgent_Template {
    type O = terrars::BlockAssignable<DataAgentPlatformAgent_Template>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformAgent_Template {}

impl BuildDataAgentPlatformAgent_Template {
    pub fn build(self) -> DataAgentPlatformAgent_Template {
        DataAgentPlatformAgent_Template {
            description: core::default::Default::default(),
            instruction: core::default::Default::default(),
            k: core::default::Default::default(),
            max_tokens: core::default::Default::default(),
            name: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
            uuid: core::default::Default::default(),
            knowledge_bases: core::default::Default::default(),
            model: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformAgent_TemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformAgent_TemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformAgent_TemplateRef {
        DataAgentPlatformAgent_TemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformAgent_TemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Knowledge Base"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the Agent Template"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\nInstruction for the Agent"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `k` after provisioning.\nK value for the Agent Template"]
    pub fn k(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.k", self.base))
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\nMaximum tokens allowed"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Agent Template"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nAgent temperature setting"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\nTop P sampling parameter"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nUpdated At timestamp for the Agent Template"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nuuid of the Agent Template"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `knowledge_bases` after provisioning.\n"]
    pub fn knowledge_bases(&self) -> terrars::ListRef<DataAgentPlatformAgent_KnowledgeBasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knowledge_bases", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::ListRef<DataAgentPlatformAgent_ModelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model", self.base))
    }
}
