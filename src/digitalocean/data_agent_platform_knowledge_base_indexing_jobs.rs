#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformKnowledgeBaseIndexingJobsData_ {
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
    knowledge_base_uuid: terrars::PrimField<std::string::String>,
}

struct DataAgentPlatformKnowledgeBaseIndexingJobs_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformKnowledgeBaseIndexingJobsData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformKnowledgeBaseIndexingJobs(std::rc::Rc<DataAgentPlatformKnowledgeBaseIndexingJobs_>);

impl DataAgentPlatformKnowledgeBaseIndexingJobs {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `jobs` after provisioning.\nList of indexing jobs for the Knowledge Base"]
    pub fn jobs(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nPagination metadata"]
    pub fn meta(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformKnowledgeBaseIndexingJobs {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformKnowledgeBaseIndexingJobs { }

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseIndexingJobs {
    type O = terrars::ListRef<DataAgentPlatformKnowledgeBaseIndexingJobsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformKnowledgeBaseIndexingJobs_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_knowledge_base_indexing_jobs".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseIndexingJobs {
    pub tf_id: std::string::String,
    #[doc = "UUID of the Knowledge Base"]
    pub knowledge_base_uuid: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformKnowledgeBaseIndexingJobs {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformKnowledgeBaseIndexingJobs {
        let out =
            DataAgentPlatformKnowledgeBaseIndexingJobs(std::rc::Rc::new(DataAgentPlatformKnowledgeBaseIndexingJobs_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataAgentPlatformKnowledgeBaseIndexingJobsData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    id: core::default::Default::default(),
                    knowledge_base_uuid: self.knowledge_base_uuid,
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformKnowledgeBaseIndexingJobsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseIndexingJobsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformKnowledgeBaseIndexingJobsRef {
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

    #[doc =
        "Get a reference to the value of field `jobs` after provisioning.\nList of indexing jobs for the Knowledge Base"]
    pub fn jobs(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jobs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta` after provisioning.\nPagination metadata"]
    pub fn meta(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meta", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {
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
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_datasources: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_items_failed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_items_indexed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_items_skipped: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {
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

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `total_items_failed`.\n"]
    pub fn set_total_items_failed(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_items_failed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_items_indexed`.\n"]
    pub fn set_total_items_indexed(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_items_indexed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_items_skipped`.\n"]
    pub fn set_total_items_skipped(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_items_skipped = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseIndexingJobs_Jobs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {}

impl BuildDataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {
        DataAgentPlatformKnowledgeBaseIndexingJobs_Jobs {
            completed_datasources: core::default::Default::default(),
            created_at: core::default::Default::default(),
            data_source_uuids: core::default::Default::default(),
            finished_at: core::default::Default::default(),
            knowledge_base_uuid: core::default::Default::default(),
            phase: core::default::Default::default(),
            started_at: core::default::Default::default(),
            status: core::default::Default::default(),
            tokens: core::default::Default::default(),
            total_datasources: core::default::Default::default(),
            total_items_failed: core::default::Default::default(),
            total_items_indexed: core::default::Default::default(),
            total_items_skipped: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef {
        DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseIndexingJobs_JobsRef {
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

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `total_datasources` after provisioning.\n"]
    pub fn total_datasources(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_datasources", self.base))
    }

    #[doc = "Get a reference to the value of field `total_items_failed` after provisioning.\n"]
    pub fn total_items_failed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_items_failed", self.base))
    }

    #[doc = "Get a reference to the value of field `total_items_indexed` after provisioning.\n"]
    pub fn total_items_indexed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_items_indexed", self.base))
    }

    #[doc = "Get a reference to the value of field `total_items_skipped` after provisioning.\n"]
    pub fn total_items_skipped(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_items_skipped", self.base))
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
pub struct DataAgentPlatformKnowledgeBaseIndexingJobs_Meta {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAgentPlatformKnowledgeBaseIndexingJobs_Meta {
    #[doc = "Set the field `page`.\n"]
    pub fn set_page(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pages`.\n"]
    pub fn set_pages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total`.\n"]
    pub fn set_total(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseIndexingJobs_Meta {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseIndexingJobs_Meta>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseIndexingJobs_Meta {}

impl BuildDataAgentPlatformKnowledgeBaseIndexingJobs_Meta {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseIndexingJobs_Meta {
        DataAgentPlatformKnowledgeBaseIndexingJobs_Meta {
            page: core::default::Default::default(),
            pages: core::default::Default::default(),
            total: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef {
        DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseIndexingJobs_MetaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `page` after provisioning.\n"]
    pub fn page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.page", self.base))
    }

    #[doc = "Get a reference to the value of field `pages` after provisioning.\n"]
    pub fn pages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pages", self.base))
    }

    #[doc = "Get a reference to the value of field `total` after provisioning.\n"]
    pub fn total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total", self.base))
    }
}
