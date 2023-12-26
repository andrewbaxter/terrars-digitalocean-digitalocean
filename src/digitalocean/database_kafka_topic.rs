use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseKafkaTopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<DatabaseKafkaTopicConfigEl>>,
    dynamic: DatabaseKafkaTopicDynamic,
}

struct DatabaseKafkaTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseKafkaTopicData>,
}

#[derive(Clone)]
pub struct DatabaseKafkaTopic(Rc<DatabaseKafkaTopic_>);

impl DatabaseKafkaTopic {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `partition_count`.\n"]
    pub fn set_partition_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().partition_count = Some(v.into());
        self
    }

    #[doc= "Set the field `replication_factor`.\n"]
    pub fn set_replication_factor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().replication_factor = Some(v.into());
        self
    }

    #[doc= "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<DatabaseKafkaTopicConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `partition_count` after provisioning.\n"]
    pub fn partition_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_factor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DatabaseKafkaTopicConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

impl Referable for DatabaseKafkaTopic {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatabaseKafkaTopic { }

impl ToListMappable for DatabaseKafkaTopic {
    type O = ListRef<DatabaseKafkaTopicRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatabaseKafkaTopic_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_database_kafka_topic".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseKafkaTopic {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDatabaseKafkaTopic {
    pub fn build(self, stack: &mut Stack) -> DatabaseKafkaTopic {
        let out = DatabaseKafkaTopic(Rc::new(DatabaseKafkaTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatabaseKafkaTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: self.name,
                partition_count: core::default::Default::default(),
                replication_factor: core::default::Default::default(),
                config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseKafkaTopicRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseKafkaTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseKafkaTopicRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `partition_count` after provisioning.\n"]
    pub fn partition_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.partition_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replication_factor` after provisioning.\n"]
    pub fn replication_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.replication_factor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<DatabaseKafkaTopicConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatabaseKafkaTopicConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cleanup_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_retention_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_delete_delay_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flush_messages: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flush_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_interval_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_compaction_lag_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_message_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_down_conversion_enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_format_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_timestamp_difference_max_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_timestamp_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cleanable_dirty_ratio: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_compaction_lag_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_insync_replicas: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preallocate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_index_bytes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_jitter_ms: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    segment_ms: Option<PrimField<String>>,
}

impl DatabaseKafkaTopicConfigEl {
    #[doc= "Set the field `cleanup_policy`.\n"]
    pub fn set_cleanup_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cleanup_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `compression_type`.\n"]
    pub fn set_compression_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression_type = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_retention_ms`.\n"]
    pub fn set_delete_retention_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_retention_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `file_delete_delay_ms`.\n"]
    pub fn set_file_delete_delay_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_delete_delay_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `flush_messages`.\n"]
    pub fn set_flush_messages(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flush_messages = Some(v.into());
        self
    }

    #[doc= "Set the field `flush_ms`.\n"]
    pub fn set_flush_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flush_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `index_interval_bytes`.\n"]
    pub fn set_index_interval_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_interval_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `max_compaction_lag_ms`.\n"]
    pub fn set_max_compaction_lag_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_compaction_lag_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `max_message_bytes`.\n"]
    pub fn set_max_message_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_message_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `message_down_conversion_enable`.\n"]
    pub fn set_message_down_conversion_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.message_down_conversion_enable = Some(v.into());
        self
    }

    #[doc= "Set the field `message_format_version`.\n"]
    pub fn set_message_format_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_format_version = Some(v.into());
        self
    }

    #[doc= "Set the field `message_timestamp_difference_max_ms`.\n"]
    pub fn set_message_timestamp_difference_max_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_timestamp_difference_max_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `message_timestamp_type`.\n"]
    pub fn set_message_timestamp_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message_timestamp_type = Some(v.into());
        self
    }

    #[doc= "Set the field `min_cleanable_dirty_ratio`.\n"]
    pub fn set_min_cleanable_dirty_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_cleanable_dirty_ratio = Some(v.into());
        self
    }

    #[doc= "Set the field `min_compaction_lag_ms`.\n"]
    pub fn set_min_compaction_lag_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_compaction_lag_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `min_insync_replicas`.\n"]
    pub fn set_min_insync_replicas(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_insync_replicas = Some(v.into());
        self
    }

    #[doc= "Set the field `preallocate`.\n"]
    pub fn set_preallocate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preallocate = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_bytes`.\n"]
    pub fn set_retention_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `retention_ms`.\n"]
    pub fn set_retention_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retention_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `segment_bytes`.\n"]
    pub fn set_segment_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `segment_index_bytes`.\n"]
    pub fn set_segment_index_bytes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_index_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `segment_jitter_ms`.\n"]
    pub fn set_segment_jitter_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_jitter_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `segment_ms`.\n"]
    pub fn set_segment_ms(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.segment_ms = Some(v.into());
        self
    }
}

impl ToListMappable for DatabaseKafkaTopicConfigEl {
    type O = BlockAssignable<DatabaseKafkaTopicConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseKafkaTopicConfigEl {}

impl BuildDatabaseKafkaTopicConfigEl {
    pub fn build(self) -> DatabaseKafkaTopicConfigEl {
        DatabaseKafkaTopicConfigEl {
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

pub struct DatabaseKafkaTopicConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseKafkaTopicConfigElRef {
    fn new(shared: StackShared, base: String) -> DatabaseKafkaTopicConfigElRef {
        DatabaseKafkaTopicConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseKafkaTopicConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cleanup_policy` after provisioning.\n"]
    pub fn cleanup_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cleanup_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.base))
    }

    #[doc= "Get a reference to the value of field `delete_retention_ms` after provisioning.\n"]
    pub fn delete_retention_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_retention_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `file_delete_delay_ms` after provisioning.\n"]
    pub fn file_delete_delay_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_delete_delay_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `flush_messages` after provisioning.\n"]
    pub fn flush_messages(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flush_messages", self.base))
    }

    #[doc= "Get a reference to the value of field `flush_ms` after provisioning.\n"]
    pub fn flush_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flush_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `index_interval_bytes` after provisioning.\n"]
    pub fn index_interval_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_interval_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `max_compaction_lag_ms` after provisioning.\n"]
    pub fn max_compaction_lag_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_compaction_lag_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `max_message_bytes` after provisioning.\n"]
    pub fn max_message_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_message_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `message_down_conversion_enable` after provisioning.\n"]
    pub fn message_down_conversion_enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_down_conversion_enable", self.base))
    }

    #[doc= "Get a reference to the value of field `message_format_version` after provisioning.\n"]
    pub fn message_format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_format_version", self.base))
    }

    #[doc= "Get a reference to the value of field `message_timestamp_difference_max_ms` after provisioning.\n"]
    pub fn message_timestamp_difference_max_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_timestamp_difference_max_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `message_timestamp_type` after provisioning.\n"]
    pub fn message_timestamp_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_timestamp_type", self.base))
    }

    #[doc= "Get a reference to the value of field `min_cleanable_dirty_ratio` after provisioning.\n"]
    pub fn min_cleanable_dirty_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cleanable_dirty_ratio", self.base))
    }

    #[doc= "Get a reference to the value of field `min_compaction_lag_ms` after provisioning.\n"]
    pub fn min_compaction_lag_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_compaction_lag_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `min_insync_replicas` after provisioning.\n"]
    pub fn min_insync_replicas(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_insync_replicas", self.base))
    }

    #[doc= "Get a reference to the value of field `preallocate` after provisioning.\n"]
    pub fn preallocate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preallocate", self.base))
    }

    #[doc= "Get a reference to the value of field `retention_bytes` after provisioning.\n"]
    pub fn retention_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `retention_ms` after provisioning.\n"]
    pub fn retention_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `segment_bytes` after provisioning.\n"]
    pub fn segment_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `segment_index_bytes` after provisioning.\n"]
    pub fn segment_index_bytes(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_index_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `segment_jitter_ms` after provisioning.\n"]
    pub fn segment_jitter_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_jitter_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `segment_ms` after provisioning.\n"]
    pub fn segment_ms(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.segment_ms", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseKafkaTopicDynamic {
    config: Option<DynamicBlock<DatabaseKafkaTopicConfigEl>>,
}
