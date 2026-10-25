#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseOpensearchConfigData_ {
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
    action_auto_create_index_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_destructive_requires_name: std::option::Option<terrars::PrimField<bool>>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_max_shards_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_routing_allocation_node_concurrent_recoveries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_security_audit: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_max_content_length_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_max_header_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_max_initial_line_length_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_fielddata_cache_size_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_memory_index_buffer_size_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_memory_max_index_buffer_size_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_memory_min_index_buffer_size_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_queries_cache_size_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_query_bool_max_clause_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_recovery_max_concurrent_file_chunks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    indices_recovery_max_mb_per_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ism_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ism_history_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ism_history_max_age_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ism_history_max_docs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ism_history_rollover_check_period_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ism_history_rollover_retention_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_main_response_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugins_alerting_filter_by_backend_roles_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reindex_remote_whitelist: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_max_compilations_rate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_max_buckets: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_analyze_queue_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_analyze_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_force_merge_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_get_queue_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_get_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_search_queue_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_search_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_search_throttled_queue_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_search_throttled_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_write_queue_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thread_pool_write_size: std::option::Option<terrars::PrimField<f64>>,
}

struct DatabaseOpensearchConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseOpensearchConfigData_>,
}

#[derive(Clone)]
pub struct DatabaseOpensearchConfig(std::rc::Rc<DatabaseOpensearchConfig_>);

impl DatabaseOpensearchConfig {
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

    #[doc = "Set the field `action_auto_create_index_enabled`.\n"]
    pub fn set_action_auto_create_index_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().action_auto_create_index_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action_destructive_requires_name`.\n"]
    pub fn set_action_destructive_requires_name(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().action_destructive_requires_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_max_shards_per_node`.\n"]
    pub fn set_cluster_max_shards_per_node(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cluster_max_shards_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_routing_allocation_node_concurrent_recoveries`.\n"]
    pub fn set_cluster_routing_allocation_node_concurrent_recoveries(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().cluster_routing_allocation_node_concurrent_recoveries =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_security_audit`.\n"]
    pub fn set_enable_security_audit(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_security_audit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_max_content_length_bytes`.\n"]
    pub fn set_http_max_content_length_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_max_content_length_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_max_header_size_bytes`.\n"]
    pub fn set_http_max_header_size_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_max_header_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_max_initial_line_length_bytes`.\n"]
    pub fn set_http_max_initial_line_length_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_max_initial_line_length_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_fielddata_cache_size_percentage`.\n"]
    pub fn set_indices_fielddata_cache_size_percentage(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().indices_fielddata_cache_size_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_memory_index_buffer_size_percentage`.\n"]
    pub fn set_indices_memory_index_buffer_size_percentage(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().indices_memory_index_buffer_size_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_memory_max_index_buffer_size_mb`.\n"]
    pub fn set_indices_memory_max_index_buffer_size_mb(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().indices_memory_max_index_buffer_size_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_memory_min_index_buffer_size_mb`.\n"]
    pub fn set_indices_memory_min_index_buffer_size_mb(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().indices_memory_min_index_buffer_size_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_queries_cache_size_percentage`.\n"]
    pub fn set_indices_queries_cache_size_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().indices_queries_cache_size_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_query_bool_max_clause_count`.\n"]
    pub fn set_indices_query_bool_max_clause_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().indices_query_bool_max_clause_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_recovery_max_concurrent_file_chunks`.\n"]
    pub fn set_indices_recovery_max_concurrent_file_chunks(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().indices_recovery_max_concurrent_file_chunks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `indices_recovery_max_mb_per_sec`.\n"]
    pub fn set_indices_recovery_max_mb_per_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().indices_recovery_max_mb_per_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ism_enabled`.\n"]
    pub fn set_ism_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ism_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ism_history_enabled`.\n"]
    pub fn set_ism_history_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ism_history_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ism_history_max_age_hours`.\n"]
    pub fn set_ism_history_max_age_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ism_history_max_age_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ism_history_max_docs`.\n"]
    pub fn set_ism_history_max_docs(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ism_history_max_docs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ism_history_rollover_check_period_hours`.\n"]
    pub fn set_ism_history_rollover_check_period_hours(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().ism_history_rollover_check_period_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ism_history_rollover_retention_period_days`.\n"]
    pub fn set_ism_history_rollover_retention_period_days(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().ism_history_rollover_retention_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_main_response_version`.\n"]
    pub fn set_override_main_response_version(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().override_main_response_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plugins_alerting_filter_by_backend_roles_enabled`.\n"]
    pub fn set_plugins_alerting_filter_by_backend_roles_enabled(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().plugins_alerting_filter_by_backend_roles_enabled =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reindex_remote_whitelist`.\n"]
    pub fn set_reindex_remote_whitelist(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().reindex_remote_whitelist = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script_max_compilations_rate`.\n"]
    pub fn set_script_max_compilations_rate(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().script_max_compilations_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search_max_buckets`.\n"]
    pub fn set_search_max_buckets(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().search_max_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_analyze_queue_size`.\n"]
    pub fn set_thread_pool_analyze_queue_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_analyze_queue_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_analyze_size`.\n"]
    pub fn set_thread_pool_analyze_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_analyze_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_force_merge_size`.\n"]
    pub fn set_thread_pool_force_merge_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_force_merge_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_get_queue_size`.\n"]
    pub fn set_thread_pool_get_queue_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_get_queue_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_get_size`.\n"]
    pub fn set_thread_pool_get_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_get_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_search_queue_size`.\n"]
    pub fn set_thread_pool_search_queue_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_search_queue_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_search_size`.\n"]
    pub fn set_thread_pool_search_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_search_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_search_throttled_queue_size`.\n"]
    pub fn set_thread_pool_search_throttled_queue_size(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().thread_pool_search_throttled_queue_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_search_throttled_size`.\n"]
    pub fn set_thread_pool_search_throttled_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_search_throttled_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_write_queue_size`.\n"]
    pub fn set_thread_pool_write_queue_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_write_queue_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thread_pool_write_size`.\n"]
    pub fn set_thread_pool_write_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().thread_pool_write_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `action_auto_create_index_enabled` after provisioning.\n"]
    pub fn action_auto_create_index_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_auto_create_index_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_destructive_requires_name` after provisioning.\n"]
    pub fn action_destructive_requires_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_destructive_requires_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_max_shards_per_node` after provisioning.\n"]
    pub fn cluster_max_shards_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_max_shards_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_routing_allocation_node_concurrent_recoveries` after provisioning.\n"]
    pub fn cluster_routing_allocation_node_concurrent_recoveries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_routing_allocation_node_concurrent_recoveries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_security_audit` after provisioning.\n"]
    pub fn enable_security_audit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_security_audit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_max_content_length_bytes` after provisioning.\n"]
    pub fn http_max_content_length_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_max_content_length_bytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_max_header_size_bytes` after provisioning.\n"]
    pub fn http_max_header_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_max_header_size_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_max_initial_line_length_bytes` after provisioning.\n"]
    pub fn http_max_initial_line_length_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_max_initial_line_length_bytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indices_fielddata_cache_size_percentage` after provisioning.\n"]
    pub fn indices_fielddata_cache_size_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_fielddata_cache_size_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_memory_index_buffer_size_percentage` after provisioning.\n"]
    pub fn indices_memory_index_buffer_size_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_memory_index_buffer_size_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_memory_max_index_buffer_size_mb` after provisioning.\n"]
    pub fn indices_memory_max_index_buffer_size_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_memory_max_index_buffer_size_mb", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_memory_min_index_buffer_size_mb` after provisioning.\n"]
    pub fn indices_memory_min_index_buffer_size_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_memory_min_index_buffer_size_mb", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_queries_cache_size_percentage` after provisioning.\n"]
    pub fn indices_queries_cache_size_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_queries_cache_size_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_query_bool_max_clause_count` after provisioning.\n"]
    pub fn indices_query_bool_max_clause_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_query_bool_max_clause_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_recovery_max_concurrent_file_chunks` after provisioning.\n"]
    pub fn indices_recovery_max_concurrent_file_chunks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_recovery_max_concurrent_file_chunks", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_recovery_max_mb_per_sec` after provisioning.\n"]
    pub fn indices_recovery_max_mb_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_recovery_max_mb_per_sec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ism_enabled` after provisioning.\n"]
    pub fn ism_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_enabled` after provisioning.\n"]
    pub fn ism_history_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_history_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_max_age_hours` after provisioning.\n"]
    pub fn ism_history_max_age_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_history_max_age_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_max_docs` after provisioning.\n"]
    pub fn ism_history_max_docs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_history_max_docs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_rollover_check_period_hours` after provisioning.\n"]
    pub fn ism_history_rollover_check_period_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ism_history_rollover_check_period_hours", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ism_history_rollover_retention_period_days` after provisioning.\n"]
    pub fn ism_history_rollover_retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ism_history_rollover_retention_period_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `override_main_response_version` after provisioning.\n"]
    pub fn override_main_response_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_main_response_version", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `plugins_alerting_filter_by_backend_roles_enabled` after provisioning.\n"]
    pub fn plugins_alerting_filter_by_backend_roles_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugins_alerting_filter_by_backend_roles_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reindex_remote_whitelist` after provisioning.\n"]
    pub fn reindex_remote_whitelist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.reindex_remote_whitelist", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_max_compilations_rate` after provisioning.\n"]
    pub fn script_max_compilations_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_max_compilations_rate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_max_buckets` after provisioning.\n"]
    pub fn search_max_buckets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_max_buckets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_analyze_queue_size` after provisioning.\n"]
    pub fn thread_pool_analyze_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_analyze_queue_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_analyze_size` after provisioning.\n"]
    pub fn thread_pool_analyze_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_analyze_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_force_merge_size` after provisioning.\n"]
    pub fn thread_pool_force_merge_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_force_merge_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_get_queue_size` after provisioning.\n"]
    pub fn thread_pool_get_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_get_queue_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_get_size` after provisioning.\n"]
    pub fn thread_pool_get_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_get_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_queue_size` after provisioning.\n"]
    pub fn thread_pool_search_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_search_queue_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_size` after provisioning.\n"]
    pub fn thread_pool_search_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_search_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_throttled_queue_size` after provisioning.\n"]
    pub fn thread_pool_search_throttled_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_search_throttled_queue_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_throttled_size` after provisioning.\n"]
    pub fn thread_pool_search_throttled_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_search_throttled_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_write_queue_size` after provisioning.\n"]
    pub fn thread_pool_write_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_write_queue_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_write_size` after provisioning.\n"]
    pub fn thread_pool_write_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_write_size", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseOpensearchConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseOpensearchConfig { }

impl terrars::ToListMappable for DatabaseOpensearchConfig {
    type O = terrars::ListRef<DatabaseOpensearchConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseOpensearchConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_opensearch_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseOpensearchConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseOpensearchConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseOpensearchConfig {
        let out = DatabaseOpensearchConfig(std::rc::Rc::new(DatabaseOpensearchConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseOpensearchConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action_auto_create_index_enabled: core::default::Default::default(),
                action_destructive_requires_name: core::default::Default::default(),
                cluster_id: self.cluster_id,
                cluster_max_shards_per_node: core::default::Default::default(),
                cluster_routing_allocation_node_concurrent_recoveries: core::default::Default::default(),
                enable_security_audit: core::default::Default::default(),
                http_max_content_length_bytes: core::default::Default::default(),
                http_max_header_size_bytes: core::default::Default::default(),
                http_max_initial_line_length_bytes: core::default::Default::default(),
                id: core::default::Default::default(),
                indices_fielddata_cache_size_percentage: core::default::Default::default(),
                indices_memory_index_buffer_size_percentage: core::default::Default::default(),
                indices_memory_max_index_buffer_size_mb: core::default::Default::default(),
                indices_memory_min_index_buffer_size_mb: core::default::Default::default(),
                indices_queries_cache_size_percentage: core::default::Default::default(),
                indices_query_bool_max_clause_count: core::default::Default::default(),
                indices_recovery_max_concurrent_file_chunks: core::default::Default::default(),
                indices_recovery_max_mb_per_sec: core::default::Default::default(),
                ism_enabled: core::default::Default::default(),
                ism_history_enabled: core::default::Default::default(),
                ism_history_max_age_hours: core::default::Default::default(),
                ism_history_max_docs: core::default::Default::default(),
                ism_history_rollover_check_period_hours: core::default::Default::default(),
                ism_history_rollover_retention_period_days: core::default::Default::default(),
                override_main_response_version: core::default::Default::default(),
                plugins_alerting_filter_by_backend_roles_enabled: core::default::Default::default(),
                reindex_remote_whitelist: core::default::Default::default(),
                script_max_compilations_rate: core::default::Default::default(),
                search_max_buckets: core::default::Default::default(),
                thread_pool_analyze_queue_size: core::default::Default::default(),
                thread_pool_analyze_size: core::default::Default::default(),
                thread_pool_force_merge_size: core::default::Default::default(),
                thread_pool_get_queue_size: core::default::Default::default(),
                thread_pool_get_size: core::default::Default::default(),
                thread_pool_search_queue_size: core::default::Default::default(),
                thread_pool_search_size: core::default::Default::default(),
                thread_pool_search_throttled_queue_size: core::default::Default::default(),
                thread_pool_search_throttled_size: core::default::Default::default(),
                thread_pool_write_queue_size: core::default::Default::default(),
                thread_pool_write_size: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseOpensearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseOpensearchConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseOpensearchConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_auto_create_index_enabled` after provisioning.\n"]
    pub fn action_auto_create_index_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_auto_create_index_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `action_destructive_requires_name` after provisioning.\n"]
    pub fn action_destructive_requires_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_destructive_requires_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_max_shards_per_node` after provisioning.\n"]
    pub fn cluster_max_shards_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_max_shards_per_node", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_routing_allocation_node_concurrent_recoveries` after provisioning.\n"]
    pub fn cluster_routing_allocation_node_concurrent_recoveries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_routing_allocation_node_concurrent_recoveries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `enable_security_audit` after provisioning.\n"]
    pub fn enable_security_audit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_security_audit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_max_content_length_bytes` after provisioning.\n"]
    pub fn http_max_content_length_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_max_content_length_bytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `http_max_header_size_bytes` after provisioning.\n"]
    pub fn http_max_header_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_max_header_size_bytes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_max_initial_line_length_bytes` after provisioning.\n"]
    pub fn http_max_initial_line_length_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.http_max_initial_line_length_bytes", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `indices_fielddata_cache_size_percentage` after provisioning.\n"]
    pub fn indices_fielddata_cache_size_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_fielddata_cache_size_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_memory_index_buffer_size_percentage` after provisioning.\n"]
    pub fn indices_memory_index_buffer_size_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_memory_index_buffer_size_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_memory_max_index_buffer_size_mb` after provisioning.\n"]
    pub fn indices_memory_max_index_buffer_size_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_memory_max_index_buffer_size_mb", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_memory_min_index_buffer_size_mb` after provisioning.\n"]
    pub fn indices_memory_min_index_buffer_size_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_memory_min_index_buffer_size_mb", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_queries_cache_size_percentage` after provisioning.\n"]
    pub fn indices_queries_cache_size_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_queries_cache_size_percentage", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_query_bool_max_clause_count` after provisioning.\n"]
    pub fn indices_query_bool_max_clause_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_query_bool_max_clause_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_recovery_max_concurrent_file_chunks` after provisioning.\n"]
    pub fn indices_recovery_max_concurrent_file_chunks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_recovery_max_concurrent_file_chunks", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `indices_recovery_max_mb_per_sec` after provisioning.\n"]
    pub fn indices_recovery_max_mb_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.indices_recovery_max_mb_per_sec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ism_enabled` after provisioning.\n"]
    pub fn ism_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_enabled` after provisioning.\n"]
    pub fn ism_history_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_history_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_max_age_hours` after provisioning.\n"]
    pub fn ism_history_max_age_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_history_max_age_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_max_docs` after provisioning.\n"]
    pub fn ism_history_max_docs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ism_history_max_docs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ism_history_rollover_check_period_hours` after provisioning.\n"]
    pub fn ism_history_rollover_check_period_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ism_history_rollover_check_period_hours", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ism_history_rollover_retention_period_days` after provisioning.\n"]
    pub fn ism_history_rollover_retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ism_history_rollover_retention_period_days", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `override_main_response_version` after provisioning.\n"]
    pub fn override_main_response_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_main_response_version", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `plugins_alerting_filter_by_backend_roles_enabled` after provisioning.\n"]
    pub fn plugins_alerting_filter_by_backend_roles_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.plugins_alerting_filter_by_backend_roles_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `reindex_remote_whitelist` after provisioning.\n"]
    pub fn reindex_remote_whitelist(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.reindex_remote_whitelist", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_max_compilations_rate` after provisioning.\n"]
    pub fn script_max_compilations_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_max_compilations_rate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `search_max_buckets` after provisioning.\n"]
    pub fn search_max_buckets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_max_buckets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_analyze_queue_size` after provisioning.\n"]
    pub fn thread_pool_analyze_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_analyze_queue_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_analyze_size` after provisioning.\n"]
    pub fn thread_pool_analyze_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_analyze_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_force_merge_size` after provisioning.\n"]
    pub fn thread_pool_force_merge_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_force_merge_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_get_queue_size` after provisioning.\n"]
    pub fn thread_pool_get_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_get_queue_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_get_size` after provisioning.\n"]
    pub fn thread_pool_get_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_get_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_queue_size` after provisioning.\n"]
    pub fn thread_pool_search_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_search_queue_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_size` after provisioning.\n"]
    pub fn thread_pool_search_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_search_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_throttled_queue_size` after provisioning.\n"]
    pub fn thread_pool_search_throttled_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_search_throttled_queue_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_search_throttled_size` after provisioning.\n"]
    pub fn thread_pool_search_throttled_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.thread_pool_search_throttled_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `thread_pool_write_queue_size` after provisioning.\n"]
    pub fn thread_pool_write_queue_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_write_queue_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `thread_pool_write_size` after provisioning.\n"]
    pub fn thread_pool_write_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thread_pool_write_size", self.extract_ref()))
    }
}
