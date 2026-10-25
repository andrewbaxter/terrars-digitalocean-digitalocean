#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformKnowledgeBaseData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    added_to_agent_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embedding_model_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
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
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std::option::Option<std::vec::Vec<DataAgentPlatformKnowledgeBase_LastIndexingJob>>,
    dynamic: DataAgentPlatformKnowledgeBaseDynamic,
}

struct DataAgentPlatformKnowledgeBase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformKnowledgeBaseData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformKnowledgeBase(std::rc::Rc<DataAgentPlatformKnowledgeBase_>);

impl DataAgentPlatformKnowledgeBase {
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

    #[doc = "Set the field `added_to_agent_at`.\nTimestamp when the Knowledge Base was added to the Agent"]
    pub fn set_added_to_agent_at(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().added_to_agent_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_id`.\nDatabase ID of the Knowledge Base"]
    pub fn set_database_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embedding_model_uuid`.\nEmbedding model UUID for the Knowledge Base"]
    pub fn set_embedding_model_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().embedding_model_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_public`.\nIndicates if the Knowledge Base is public"]
    pub fn set_is_public(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_public = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the Knowledge Base"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nProject ID of the Knowledge Base"]
    pub fn set_project_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion of the Knowledge Base"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nList of tags"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\nUser ID of the Knowledge Base"]
    pub fn set_user_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uuid`.\nUUID of the Knowledge Base"]
    pub fn set_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAgentPlatformKnowledgeBase_LastIndexingJob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().last_indexing_job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.last_indexing_job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `added_to_agent_at` after provisioning.\nTimestamp when the Knowledge Base was added to the Agent"]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Knowledge Base"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nDatabase ID of the Knowledge Base"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `embedding_model_uuid` after provisioning.\nEmbedding model UUID for the Knowledge Base"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_public` after provisioning.\nIndicates if the Knowledge Base is public"]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Knowledge Base"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID of the Knowledge Base"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the Knowledge Base"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nList of tags"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Knowledge Base was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nUser ID of the Knowledge Base"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBase_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformKnowledgeBase {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformKnowledgeBase { }

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBase {
    type O = terrars::ListRef<DataAgentPlatformKnowledgeBaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformKnowledgeBase_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_knowledge_base".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformKnowledgeBase {
    pub tf_id: std::string::String,
}

impl BuildDataAgentPlatformKnowledgeBase {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformKnowledgeBase {
        let out = DataAgentPlatformKnowledgeBase(std::rc::Rc::new(DataAgentPlatformKnowledgeBase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformKnowledgeBaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                added_to_agent_at: core::default::Default::default(),
                database_id: core::default::Default::default(),
                embedding_model_uuid: core::default::Default::default(),
                id: core::default::Default::default(),
                is_public: core::default::Default::default(),
                name: core::default::Default::default(),
                project_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                user_id: core::default::Default::default(),
                uuid: core::default::Default::default(),
                last_indexing_job: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformKnowledgeBaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformKnowledgeBaseRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `added_to_agent_at` after provisioning.\nTimestamp when the Knowledge Base was added to the Agent"]
    pub fn added_to_agent_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.added_to_agent_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nCreated At timestamp for the Knowledge Base"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nDatabase ID of the Knowledge Base"]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `embedding_model_uuid` after provisioning.\nEmbedding model UUID for the Knowledge Base"]
    pub fn embedding_model_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embedding_model_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_public` after provisioning.\nIndicates if the Knowledge Base is public"]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Knowledge Base"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID of the Knowledge Base"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the Knowledge Base"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nList of tags"]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the Knowledge Base was updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\nUser ID of the Knowledge Base"]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBase_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformKnowledgeBase_LastIndexingJob {
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

impl DataAgentPlatformKnowledgeBase_LastIndexingJob {
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

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBase_LastIndexingJob {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBase_LastIndexingJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBase_LastIndexingJob {}

impl BuildDataAgentPlatformKnowledgeBase_LastIndexingJob {
    pub fn build(self) -> DataAgentPlatformKnowledgeBase_LastIndexingJob {
        DataAgentPlatformKnowledgeBase_LastIndexingJob {
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
pub struct DataAgentPlatformKnowledgeBase_LastIndexingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBase_LastIndexingJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBase_LastIndexingJobRef {
        DataAgentPlatformKnowledgeBase_LastIndexingJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBase_LastIndexingJobRef {
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
struct DataAgentPlatformKnowledgeBaseDynamic {
    last_indexing_job: std::option::Option<terrars::DynamicBlock<DataAgentPlatformKnowledgeBase_LastIndexingJob>>,
}
