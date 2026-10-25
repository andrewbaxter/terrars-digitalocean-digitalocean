#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformIndexingJobDataSourcesData_ {
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
    indexing_job_uuid: terrars::PrimField<std::string::String>,
}

struct DataAgentPlatformIndexingJobDataSources_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformIndexingJobDataSourcesData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformIndexingJobDataSources(std::rc::Rc<DataAgentPlatformIndexingJobDataSources_>);

impl DataAgentPlatformIndexingJobDataSources {
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
        "Get a reference to the value of field `indexed_data_sources` after provisioning.\nList of indexed data sources for the indexing job"]
    pub fn indexed_data_sources(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.indexed_data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indexing_job_uuid` after provisioning.\nUUID of the indexing job"]
    pub fn indexing_job_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexing_job_uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformIndexingJobDataSources {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformIndexingJobDataSources { }

impl terrars::ToListMappable for DataAgentPlatformIndexingJobDataSources {
    type O = terrars::ListRef<DataAgentPlatformIndexingJobDataSourcesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformIndexingJobDataSources_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_indexing_job_data_sources".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformIndexingJobDataSources {
    pub tf_id: std::string::String,
    #[doc = "UUID of the indexing job"]
    pub indexing_job_uuid: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformIndexingJobDataSources {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformIndexingJobDataSources {
        let out = DataAgentPlatformIndexingJobDataSources(std::rc::Rc::new(DataAgentPlatformIndexingJobDataSources_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformIndexingJobDataSourcesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                indexing_job_uuid: self.indexing_job_uuid,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformIndexingJobDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformIndexingJobDataSourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformIndexingJobDataSourcesRef {
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
        "Get a reference to the value of field `indexed_data_sources` after provisioning.\nList of indexed data sources for the indexing job"]
    pub fn indexed_data_sources(
        &self,
    ) -> terrars::ListRef<DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.indexed_data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indexing_job_uuid` after provisioning.\nUUID of the indexing job"]
    pub fn indexing_job_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexing_job_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformIndexingJobDataSources_IndexedDataSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completed_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_msg: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failed_item_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_file_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indexed_item_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    removed_item_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skipped_item_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    started_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_bytes_indexed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_file_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformIndexingJobDataSources_IndexedDataSources {
    #[doc = "Set the field `completed_at`.\n"]
    pub fn set_completed_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.completed_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_uuid`.\n"]
    pub fn set_data_source_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_msg`.\n"]
    pub fn set_error_msg(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_msg = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failed_item_count`.\n"]
    pub fn set_failed_item_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.failed_item_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indexed_file_count`.\n"]
    pub fn set_indexed_file_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.indexed_file_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indexed_item_count`.\n"]
    pub fn set_indexed_item_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.indexed_item_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `removed_item_count`.\n"]
    pub fn set_removed_item_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.removed_item_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skipped_item_count`.\n"]
    pub fn set_skipped_item_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.skipped_item_count = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `total_bytes`.\n"]
    pub fn set_total_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_bytes_indexed`.\n"]
    pub fn set_total_bytes_indexed(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_bytes_indexed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_file_count`.\n"]
    pub fn set_total_file_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_file_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformIndexingJobDataSources_IndexedDataSources {
    type O = terrars::BlockAssignable<DataAgentPlatformIndexingJobDataSources_IndexedDataSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformIndexingJobDataSources_IndexedDataSources {}

impl BuildDataAgentPlatformIndexingJobDataSources_IndexedDataSources {
    pub fn build(self) -> DataAgentPlatformIndexingJobDataSources_IndexedDataSources {
        DataAgentPlatformIndexingJobDataSources_IndexedDataSources {
            completed_at: core::default::Default::default(),
            data_source_uuid: core::default::Default::default(),
            error_details: core::default::Default::default(),
            error_msg: core::default::Default::default(),
            failed_item_count: core::default::Default::default(),
            indexed_file_count: core::default::Default::default(),
            indexed_item_count: core::default::Default::default(),
            removed_item_count: core::default::Default::default(),
            skipped_item_count: core::default::Default::default(),
            started_at: core::default::Default::default(),
            status: core::default::Default::default(),
            total_bytes: core::default::Default::default(),
            total_bytes_indexed: core::default::Default::default(),
            total_file_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef {
        DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformIndexingJobDataSources_IndexedDataSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completed_at` after provisioning.\n"]
    pub fn completed_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completed_at", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_uuid` after provisioning.\n"]
    pub fn data_source_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_details", self.base))
    }

    #[doc = "Get a reference to the value of field `error_msg` after provisioning.\n"]
    pub fn error_msg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_msg", self.base))
    }

    #[doc = "Get a reference to the value of field `failed_item_count` after provisioning.\n"]
    pub fn failed_item_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failed_item_count", self.base))
    }

    #[doc = "Get a reference to the value of field `indexed_file_count` after provisioning.\n"]
    pub fn indexed_file_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_file_count", self.base))
    }

    #[doc = "Get a reference to the value of field `indexed_item_count` after provisioning.\n"]
    pub fn indexed_item_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.indexed_item_count", self.base))
    }

    #[doc = "Get a reference to the value of field `removed_item_count` after provisioning.\n"]
    pub fn removed_item_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.removed_item_count", self.base))
    }

    #[doc = "Get a reference to the value of field `skipped_item_count` after provisioning.\n"]
    pub fn skipped_item_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skipped_item_count", self.base))
    }

    #[doc = "Get a reference to the value of field `started_at` after provisioning.\n"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `total_bytes` after provisioning.\n"]
    pub fn total_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_bytes_indexed` after provisioning.\n"]
    pub fn total_bytes_indexed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_bytes_indexed", self.base))
    }

    #[doc = "Get a reference to the value of field `total_file_count` after provisioning.\n"]
    pub fn total_file_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_file_count", self.base))
    }
}
