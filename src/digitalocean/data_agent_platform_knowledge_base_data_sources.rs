#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformKnowledgeBaseDataSourcesData_ {
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

struct DataAgentPlatformKnowledgeBaseDataSources_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformKnowledgeBaseDataSourcesData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformKnowledgeBaseDataSources(std::rc::Rc<DataAgentPlatformKnowledgeBaseDataSources_>);

impl DataAgentPlatformKnowledgeBaseDataSources {
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
        "Get a reference to the value of field `datasources` after provisioning.\nList of data sources for the Knowledge Base"]
    pub fn datasources(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformKnowledgeBaseDataSources {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformKnowledgeBaseDataSources { }

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseDataSources {
    type O = terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSourcesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformKnowledgeBaseDataSources_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_knowledge_base_data_sources".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseDataSources {
    pub tf_id: std::string::String,
    #[doc = "UUID of the Knowledge Base"]
    pub knowledge_base_uuid: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformKnowledgeBaseDataSources {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformKnowledgeBaseDataSources {
        let out =
            DataAgentPlatformKnowledgeBaseDataSources(std::rc::Rc::new(DataAgentPlatformKnowledgeBaseDataSources_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataAgentPlatformKnowledgeBaseDataSourcesData_ {
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

pub struct DataAgentPlatformKnowledgeBaseDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseDataSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformKnowledgeBaseDataSourcesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `datasources` after provisioning.\nList of data sources for the Knowledge Base"]
    pub fn datasources(&self) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_file_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_object_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {
    #[doc = "Set the field `original_file_name`.\n"]
    pub fn set_original_file_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.original_file_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_in_bytes`.\n"]
    pub fn set_size_in_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size_in_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stored_object_key`.\n"]
    pub fn set_stored_object_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stored_object_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {}

impl BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource {
            original_file_name: core::default::Default::default(),
            size_in_bytes: core::default::Default::default(),
            stored_object_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSourceRef {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `original_file_name` after provisioning.\n"]
    pub fn original_file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_file_name", self.base))
    }

    #[doc = "Get a reference to the value of field `size_in_bytes` after provisioning.\n"]
    pub fn size_in_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_object_key` after provisioning.\n"]
    pub fn stored_object_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_object_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {
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

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {
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

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {}

impl BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob {
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
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJobRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJobRef {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJobRef {
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
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {
    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `item_path`.\n"]
    pub fn set_item_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.item_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {}

impl BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource {
            bucket_name: core::default::Default::default(),
            item_path: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSourceRef {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `item_path` after provisioning.\n"]
    pub fn item_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_path", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawling_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embed_media: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {
    #[doc = "Set the field `base_url`.\n"]
    pub fn set_base_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crawling_option`.\n"]
    pub fn set_crawling_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawling_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embed_media`.\n"]
    pub fn set_embed_media(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.embed_media = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {}

impl BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource {
            base_url: core::default::Default::default(),
            crawling_option: core::default::Default::default(),
            embed_media: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSourceRef {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_url` after provisioning.\n"]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc = "Get a reference to the value of field `crawling_option` after provisioning.\n"]
    pub fn crawling_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawling_option", self.base))
    }

    #[doc = "Get a reference to the value of field `embed_media` after provisioning.\n"]
    pub fn embed_media(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embed_media", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_Datasources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_upload_data_source: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_indexing_job: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spaces_data_source: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_data_source: std
    ::option
    ::Option<terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource>>,
}

impl DataAgentPlatformKnowledgeBaseDataSources_Datasources {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_upload_data_source`.\n"]
    pub fn set_file_upload_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSource>,
                    >,
    ) -> Self {
        self.file_upload_data_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_indexing_job`.\n"]
    pub fn set_last_indexing_job(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJob>>,
    ) -> Self {
        self.last_indexing_job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spaces_data_source`.\n"]
    pub fn set_spaces_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSource>,
                    >,
    ) -> Self {
        self.spaces_data_source = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `web_crawler_data_source`.\n"]
    pub fn set_web_crawler_data_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSource>,
                    >,
    ) -> Self {
        self.web_crawler_data_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformKnowledgeBaseDataSources_Datasources {
    type O = terrars::BlockAssignable<DataAgentPlatformKnowledgeBaseDataSources_Datasources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources {}

impl BuildDataAgentPlatformKnowledgeBaseDataSources_Datasources {
    pub fn build(self) -> DataAgentPlatformKnowledgeBaseDataSources_Datasources {
        DataAgentPlatformKnowledgeBaseDataSources_Datasources {
            created_at: core::default::Default::default(),
            file_upload_data_source: core::default::Default::default(),
            last_indexing_job: core::default::Default::default(),
            spaces_data_source: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            uuid: core::default::Default::default(),
            web_crawler_data_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef {
        DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformKnowledgeBaseDataSources_DatasourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `file_upload_data_source` after provisioning.\n"]
    pub fn file_upload_data_source(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSources_Datasources_FileUploadDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_upload_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `last_indexing_job` after provisioning.\n"]
    pub fn last_indexing_job(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSources_Datasources_LastIndexingJobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_indexing_job", self.base))
    }

    #[doc = "Get a reference to the value of field `spaces_data_source` after provisioning.\n"]
    pub fn spaces_data_source(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSources_Datasources_SpacesDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spaces_data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `web_crawler_data_source` after provisioning.\n"]
    pub fn web_crawler_data_source(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformKnowledgeBaseDataSources_Datasources_WebCrawlerDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_crawler_data_source", self.base))
    }
}
