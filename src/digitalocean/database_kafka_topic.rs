#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseKafkaTopicData_ {
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
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<DatabaseKafkaTopic_Config>>,
    dynamic: DatabaseKafkaTopicDynamic,
}

struct DatabaseKafkaTopic_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseKafkaTopicData_>,
}

#[derive(Clone)]
pub struct DatabaseKafkaTopic(std::rc::Rc<DatabaseKafkaTopic_>);

impl DatabaseKafkaTopic {
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

    #[doc = "Set the field `partition_count`.\n"]
    pub fn set_partition_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().partition_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_factor`.\n"]
    pub fn set_replication_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replication_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DatabaseKafkaTopic_Config>>) -> Self {
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

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_count` after provisioning.\n"]
    pub fn partition_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_factor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DatabaseKafkaTopic_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseKafkaTopic {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseKafkaTopic { }

impl terrars::ToListMappable for DatabaseKafkaTopic {
    type O = terrars::ListRef<DatabaseKafkaTopicRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseKafkaTopic_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_kafka_topic".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseKafkaTopic {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseKafkaTopic {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseKafkaTopic {
        let out = DatabaseKafkaTopic(std::rc::Rc::new(DatabaseKafkaTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseKafkaTopicData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: self.name,
                partition_count: core::default::Default::default(),
                replication_factor: core::default::Default::default(),
                config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseKafkaTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseKafkaTopicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseKafkaTopicRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_count` after provisioning.\n"]
    pub fn partition_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_factor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DatabaseKafkaTopic_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatabaseKafkaTopicDynamic {
    config: std::option::Option<terrars::DynamicBlock<DatabaseKafkaTopic_Config>>,
}

#[derive(Serialize)]
pub struct DatabaseKafkaTopic_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cleanup_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_retention_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_delete_delay_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flush_messages: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flush_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_interval_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_compaction_lag_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_message_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_down_conversion_enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_format_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_timestamp_difference_max_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_timestamp_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cleanable_dirty_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_compaction_lag_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_insync_replicas: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preallocate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_index_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_jitter_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_ms: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseKafkaTopic_Config {
    #[doc = "Set the field `cleanup_policy`.\n"]
    pub fn set_cleanup_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cleanup_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_retention_ms`.\n"]
    pub fn set_delete_retention_ms(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.delete_retention_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_delete_delay_ms`.\n"]
    pub fn set_file_delete_delay_ms(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_delete_delay_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flush_messages`.\n"]
    pub fn set_flush_messages(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flush_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flush_ms`.\n"]
    pub fn set_flush_ms(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flush_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_interval_bytes`.\n"]
    pub fn set_index_interval_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.index_interval_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_compaction_lag_ms`.\n"]
    pub fn set_max_compaction_lag_ms(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_compaction_lag_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_message_bytes`.\n"]
    pub fn set_max_message_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_message_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_down_conversion_enable`.\n"]
    pub fn set_message_down_conversion_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.message_down_conversion_enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_format_version`.\n"]
    pub fn set_message_format_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_format_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_timestamp_difference_max_ms`.\n"]
    pub fn set_message_timestamp_difference_max_ms(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_timestamp_difference_max_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_timestamp_type`.\n"]
    pub fn set_message_timestamp_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_timestamp_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cleanable_dirty_ratio`.\n"]
    pub fn set_min_cleanable_dirty_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_cleanable_dirty_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_compaction_lag_ms`.\n"]
    pub fn set_min_compaction_lag_ms(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.min_compaction_lag_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_insync_replicas`.\n"]
    pub fn set_min_insync_replicas(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_insync_replicas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preallocate`.\n"]
    pub fn set_preallocate(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preallocate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_bytes`.\n"]
    pub fn set_retention_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_ms`.\n"]
    pub fn set_retention_ms(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_bytes`.\n"]
    pub fn set_segment_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_index_bytes`.\n"]
    pub fn set_segment_index_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.segment_index_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_jitter_ms`.\n"]
    pub fn set_segment_jitter_ms(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment_jitter_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segment_ms`.\n"]
    pub fn set_segment_ms(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.segment_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseKafkaTopic_Config {
    type O = terrars::BlockAssignable<DatabaseKafkaTopic_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseKafkaTopic_Config {}

impl BuildDatabaseKafkaTopic_Config {
    pub fn build(self) -> DatabaseKafkaTopic_Config {
        DatabaseKafkaTopic_Config {
            cleanup_policy: core::default::Default::default(),
            compression_type: core::default::Default::default(),
            delete_retention_ms: core::default::Default::default(),
            file_delete_delay_ms: core::default::Default::default(),
            flush_messages: core::default::Default::default(),
            flush_ms: core::default::Default::default(),
            index_interval_bytes: core::default::Default::default(),
            max_compaction_lag_ms: core::default::Default::default(),
            max_message_bytes: core::default::Default::default(),
            message_down_conversion_enable: core::default::Default::default(),
            message_format_version: core::default::Default::default(),
            message_timestamp_difference_max_ms: core::default::Default::default(),
            message_timestamp_type: core::default::Default::default(),
            min_cleanable_dirty_ratio: core::default::Default::default(),
            min_compaction_lag_ms: core::default::Default::default(),
            min_insync_replicas: core::default::Default::default(),
            preallocate: core::default::Default::default(),
            retention_bytes: core::default::Default::default(),
            retention_ms: core::default::Default::default(),
            segment_bytes: core::default::Default::default(),
            segment_index_bytes: core::default::Default::default(),
            segment_jitter_ms: core::default::Default::default(),
            segment_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseKafkaTopic_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseKafkaTopic_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabaseKafkaTopic_ConfigRef {
        DatabaseKafkaTopic_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseKafkaTopic_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cleanup_policy` after provisioning.\n"]
    pub fn cleanup_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cleanup_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_retention_ms` after provisioning.\n"]
    pub fn delete_retention_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_retention_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `file_delete_delay_ms` after provisioning.\n"]
    pub fn file_delete_delay_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_delete_delay_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `flush_messages` after provisioning.\n"]
    pub fn flush_messages(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flush_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `flush_ms` after provisioning.\n"]
    pub fn flush_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flush_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `index_interval_bytes` after provisioning.\n"]
    pub fn index_interval_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_interval_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `max_compaction_lag_ms` after provisioning.\n"]
    pub fn max_compaction_lag_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_compaction_lag_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `max_message_bytes` after provisioning.\n"]
    pub fn max_message_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_message_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `message_down_conversion_enable` after provisioning.\n"]
    pub fn message_down_conversion_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_down_conversion_enable", self.base))
    }

    #[doc = "Get a reference to the value of field `message_format_version` after provisioning.\n"]
    pub fn message_format_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_format_version", self.base))
    }

    #[doc = "Get a reference to the value of field `message_timestamp_difference_max_ms` after provisioning.\n"]
    pub fn message_timestamp_difference_max_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_timestamp_difference_max_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `message_timestamp_type` after provisioning.\n"]
    pub fn message_timestamp_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_timestamp_type", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cleanable_dirty_ratio` after provisioning.\n"]
    pub fn min_cleanable_dirty_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cleanable_dirty_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `min_compaction_lag_ms` after provisioning.\n"]
    pub fn min_compaction_lag_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_compaction_lag_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `min_insync_replicas` after provisioning.\n"]
    pub fn min_insync_replicas(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_insync_replicas", self.base))
    }

    #[doc = "Get a reference to the value of field `preallocate` after provisioning.\n"]
    pub fn preallocate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preallocate", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_bytes` after provisioning.\n"]
    pub fn retention_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_ms` after provisioning.\n"]
    pub fn retention_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_bytes` after provisioning.\n"]
    pub fn segment_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_index_bytes` after provisioning.\n"]
    pub fn segment_index_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_index_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_jitter_ms` after provisioning.\n"]
    pub fn segment_jitter_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_jitter_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_ms` after provisioning.\n"]
    pub fn segment_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_ms", self.base))
    }
}
