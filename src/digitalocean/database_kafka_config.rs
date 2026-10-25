#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseKafkaConfigData_ {
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
    auto_create_topics_enable: std::option::Option<terrars::PrimField<bool>>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_initial_rebalance_delay_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_max_session_timeout_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_min_session_timeout_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_cleaner_delete_retention_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_cleaner_min_compaction_lag_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_flush_interval_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_index_interval_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_message_downconversion_enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_message_timestamp_difference_max_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_preallocate: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_retention_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_retention_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_retention_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_roll_jitter_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_segment_delete_delay_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_max_bytes: std::option::Option<terrars::PrimField<f64>>,
}

struct DatabaseKafkaConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseKafkaConfigData_>,
}

#[derive(Clone)]
pub struct DatabaseKafkaConfig(std::rc::Rc<DatabaseKafkaConfig_>);

impl DatabaseKafkaConfig {
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

    #[doc = "Set the field `auto_create_topics_enable`.\n"]
    pub fn set_auto_create_topics_enable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_create_topics_enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_initial_rebalance_delay_ms`.\n"]
    pub fn set_group_initial_rebalance_delay_ms(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_initial_rebalance_delay_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_max_session_timeout_ms`.\n"]
    pub fn set_group_max_session_timeout_ms(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_max_session_timeout_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_min_session_timeout_ms`.\n"]
    pub fn set_group_min_session_timeout_ms(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_min_session_timeout_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_cleaner_delete_retention_ms`.\n"]
    pub fn set_log_cleaner_delete_retention_ms(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_cleaner_delete_retention_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_cleaner_min_compaction_lag_ms`.\n"]
    pub fn set_log_cleaner_min_compaction_lag_ms(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().log_cleaner_min_compaction_lag_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_flush_interval_ms`.\n"]
    pub fn set_log_flush_interval_ms(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_flush_interval_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_index_interval_bytes`.\n"]
    pub fn set_log_index_interval_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_index_interval_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_message_downconversion_enable`.\n"]
    pub fn set_log_message_downconversion_enable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_message_downconversion_enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_message_timestamp_difference_max_ms`.\n"]
    pub fn set_log_message_timestamp_difference_max_ms(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().log_message_timestamp_difference_max_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_preallocate`.\n"]
    pub fn set_log_preallocate(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_preallocate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_retention_bytes`.\n"]
    pub fn set_log_retention_bytes(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_retention_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_retention_hours`.\n"]
    pub fn set_log_retention_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_retention_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_retention_ms`.\n"]
    pub fn set_log_retention_ms(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_retention_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_roll_jitter_ms`.\n"]
    pub fn set_log_roll_jitter_ms(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_roll_jitter_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_segment_delete_delay_ms`.\n"]
    pub fn set_log_segment_delete_delay_ms(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_segment_delete_delay_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_max_bytes`.\n"]
    pub fn set_message_max_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().message_max_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `auto_create_topics_enable` after provisioning.\n"]
    pub fn auto_create_topics_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_create_topics_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_initial_rebalance_delay_ms` after provisioning.\n"]
    pub fn group_initial_rebalance_delay_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_initial_rebalance_delay_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `group_max_session_timeout_ms` after provisioning.\n"]
    pub fn group_max_session_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_max_session_timeout_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_min_session_timeout_ms` after provisioning.\n"]
    pub fn group_min_session_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_min_session_timeout_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_cleaner_delete_retention_ms` after provisioning.\n"]
    pub fn log_cleaner_delete_retention_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_cleaner_delete_retention_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_cleaner_min_compaction_lag_ms` after provisioning.\n"]
    pub fn log_cleaner_min_compaction_lag_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_cleaner_min_compaction_lag_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_flush_interval_ms` after provisioning.\n"]
    pub fn log_flush_interval_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_flush_interval_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_index_interval_bytes` after provisioning.\n"]
    pub fn log_index_interval_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_index_interval_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_message_downconversion_enable` after provisioning.\n"]
    pub fn log_message_downconversion_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_message_downconversion_enable", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_message_timestamp_difference_max_ms` after provisioning.\n"]
    pub fn log_message_timestamp_difference_max_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_message_timestamp_difference_max_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_preallocate` after provisioning.\n"]
    pub fn log_preallocate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_preallocate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_retention_bytes` after provisioning.\n"]
    pub fn log_retention_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_retention_hours` after provisioning.\n"]
    pub fn log_retention_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_retention_ms` after provisioning.\n"]
    pub fn log_retention_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_roll_jitter_ms` after provisioning.\n"]
    pub fn log_roll_jitter_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_roll_jitter_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_segment_delete_delay_ms` after provisioning.\n"]
    pub fn log_segment_delete_delay_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_segment_delete_delay_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_max_bytes` after provisioning.\n"]
    pub fn message_max_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_max_bytes", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseKafkaConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseKafkaConfig { }

impl terrars::ToListMappable for DatabaseKafkaConfig {
    type O = terrars::ListRef<DatabaseKafkaConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseKafkaConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_kafka_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseKafkaConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseKafkaConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseKafkaConfig {
        let out = DatabaseKafkaConfig(std::rc::Rc::new(DatabaseKafkaConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseKafkaConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_create_topics_enable: core::default::Default::default(),
                cluster_id: self.cluster_id,
                group_initial_rebalance_delay_ms: core::default::Default::default(),
                group_max_session_timeout_ms: core::default::Default::default(),
                group_min_session_timeout_ms: core::default::Default::default(),
                id: core::default::Default::default(),
                log_cleaner_delete_retention_ms: core::default::Default::default(),
                log_cleaner_min_compaction_lag_ms: core::default::Default::default(),
                log_flush_interval_ms: core::default::Default::default(),
                log_index_interval_bytes: core::default::Default::default(),
                log_message_downconversion_enable: core::default::Default::default(),
                log_message_timestamp_difference_max_ms: core::default::Default::default(),
                log_preallocate: core::default::Default::default(),
                log_retention_bytes: core::default::Default::default(),
                log_retention_hours: core::default::Default::default(),
                log_retention_ms: core::default::Default::default(),
                log_roll_jitter_ms: core::default::Default::default(),
                log_segment_delete_delay_ms: core::default::Default::default(),
                message_max_bytes: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseKafkaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseKafkaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseKafkaConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_create_topics_enable` after provisioning.\n"]
    pub fn auto_create_topics_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_create_topics_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_initial_rebalance_delay_ms` after provisioning.\n"]
    pub fn group_initial_rebalance_delay_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_initial_rebalance_delay_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `group_max_session_timeout_ms` after provisioning.\n"]
    pub fn group_max_session_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_max_session_timeout_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_min_session_timeout_ms` after provisioning.\n"]
    pub fn group_min_session_timeout_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_min_session_timeout_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_cleaner_delete_retention_ms` after provisioning.\n"]
    pub fn log_cleaner_delete_retention_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_cleaner_delete_retention_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_cleaner_min_compaction_lag_ms` after provisioning.\n"]
    pub fn log_cleaner_min_compaction_lag_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_cleaner_min_compaction_lag_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_flush_interval_ms` after provisioning.\n"]
    pub fn log_flush_interval_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_flush_interval_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_index_interval_bytes` after provisioning.\n"]
    pub fn log_index_interval_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_index_interval_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_message_downconversion_enable` after provisioning.\n"]
    pub fn log_message_downconversion_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_message_downconversion_enable", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_message_timestamp_difference_max_ms` after provisioning.\n"]
    pub fn log_message_timestamp_difference_max_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_message_timestamp_difference_max_ms", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_preallocate` after provisioning.\n"]
    pub fn log_preallocate(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_preallocate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_retention_bytes` after provisioning.\n"]
    pub fn log_retention_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_retention_hours` after provisioning.\n"]
    pub fn log_retention_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_retention_ms` after provisioning.\n"]
    pub fn log_retention_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_roll_jitter_ms` after provisioning.\n"]
    pub fn log_roll_jitter_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_roll_jitter_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_segment_delete_delay_ms` after provisioning.\n"]
    pub fn log_segment_delete_delay_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_segment_delete_delay_ms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_max_bytes` after provisioning.\n"]
    pub fn message_max_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_max_bytes", self.extract_ref()))
    }
}
