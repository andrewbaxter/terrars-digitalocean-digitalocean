#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabasePostgresqlConfigData_ {
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
    autovacuum_analyze_scale_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_analyze_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_freeze_max_age: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_max_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_naptime: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_vacuum_cost_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_vacuum_cost_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_vacuum_scale_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autovacuum_vacuum_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_minute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgwriter_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgwriter_flush_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgwriter_lru_maxpages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgwriter_lru_multiplier: std::option::Option<terrars::PrimField<f64>>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deadlock_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_toast_compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_in_transaction_session_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jit: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_autovacuum_min_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_error_verbosity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_line_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_min_duration_statement: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_files_per_process: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_locks_per_transaction: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_logical_replication_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_workers: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallel_workers_per_gather: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pred_locks_per_transaction: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_prepared_transactions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replication_slots: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_stack_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_standby_archive_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_standby_streaming_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_wal_senders: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_worker_processes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pg_partman_bgw_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pg_partman_bgw_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pg_stat_statements_track: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_buffers_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temp_file_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    track_activity_query_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    track_commit_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    track_functions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    track_io_timing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wal_sender_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wal_writer_delay: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    work_mem: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pgbouncer: std::option::Option<std::vec::Vec<DatabasePostgresqlConfig_Pgbouncer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timescaledb: std::option::Option<std::vec::Vec<DatabasePostgresqlConfig_Timescaledb>>,
    dynamic: DatabasePostgresqlConfigDynamic,
}

struct DatabasePostgresqlConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabasePostgresqlConfigData_>,
}

#[derive(Clone)]
pub struct DatabasePostgresqlConfig(std::rc::Rc<DatabasePostgresqlConfig_>);

impl DatabasePostgresqlConfig {
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

    #[doc = "Set the field `autovacuum_analyze_scale_factor`.\n"]
    pub fn set_autovacuum_analyze_scale_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_analyze_scale_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_analyze_threshold`.\n"]
    pub fn set_autovacuum_analyze_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_analyze_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_freeze_max_age`.\n"]
    pub fn set_autovacuum_freeze_max_age(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_freeze_max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_max_workers`.\n"]
    pub fn set_autovacuum_max_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_max_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_naptime`.\n"]
    pub fn set_autovacuum_naptime(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_naptime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_vacuum_cost_delay`.\n"]
    pub fn set_autovacuum_vacuum_cost_delay(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_vacuum_cost_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_vacuum_cost_limit`.\n"]
    pub fn set_autovacuum_vacuum_cost_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_vacuum_cost_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_vacuum_scale_factor`.\n"]
    pub fn set_autovacuum_vacuum_scale_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_vacuum_scale_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autovacuum_vacuum_threshold`.\n"]
    pub fn set_autovacuum_vacuum_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().autovacuum_vacuum_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_hour`.\n"]
    pub fn set_backup_hour(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_hour = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_minute`.\n"]
    pub fn set_backup_minute(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgwriter_delay`.\n"]
    pub fn set_bgwriter_delay(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().bgwriter_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgwriter_flush_after`.\n"]
    pub fn set_bgwriter_flush_after(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().bgwriter_flush_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgwriter_lru_maxpages`.\n"]
    pub fn set_bgwriter_lru_maxpages(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().bgwriter_lru_maxpages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgwriter_lru_multiplier`.\n"]
    pub fn set_bgwriter_lru_multiplier(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().bgwriter_lru_multiplier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deadlock_timeout`.\n"]
    pub fn set_deadlock_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deadlock_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_toast_compression`.\n"]
    pub fn set_default_toast_compression(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_toast_compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_in_transaction_session_timeout`.\n"]
    pub fn set_idle_in_transaction_session_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().idle_in_transaction_session_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jit`.\n"]
    pub fn set_jit(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().jit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_autovacuum_min_duration`.\n"]
    pub fn set_log_autovacuum_min_duration(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_autovacuum_min_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_error_verbosity`.\n"]
    pub fn set_log_error_verbosity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_error_verbosity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_line_prefix`.\n"]
    pub fn set_log_line_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_line_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_min_duration_statement`.\n"]
    pub fn set_log_min_duration_statement(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_min_duration_statement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_files_per_process`.\n"]
    pub fn set_max_files_per_process(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_files_per_process = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_locks_per_transaction`.\n"]
    pub fn set_max_locks_per_transaction(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_locks_per_transaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_logical_replication_workers`.\n"]
    pub fn set_max_logical_replication_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_logical_replication_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_workers`.\n"]
    pub fn set_max_parallel_workers(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_parallel_workers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_workers_per_gather`.\n"]
    pub fn set_max_parallel_workers_per_gather(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_parallel_workers_per_gather = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_pred_locks_per_transaction`.\n"]
    pub fn set_max_pred_locks_per_transaction(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_pred_locks_per_transaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_prepared_transactions`.\n"]
    pub fn set_max_prepared_transactions(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_prepared_transactions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_replication_slots`.\n"]
    pub fn set_max_replication_slots(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_replication_slots = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_stack_depth`.\n"]
    pub fn set_max_stack_depth(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_stack_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_standby_archive_delay`.\n"]
    pub fn set_max_standby_archive_delay(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_standby_archive_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_standby_streaming_delay`.\n"]
    pub fn set_max_standby_streaming_delay(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_standby_streaming_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_wal_senders`.\n"]
    pub fn set_max_wal_senders(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_wal_senders = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_worker_processes`.\n"]
    pub fn set_max_worker_processes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_worker_processes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pg_partman_bgw_interval`.\n"]
    pub fn set_pg_partman_bgw_interval(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().pg_partman_bgw_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pg_partman_bgw_role`.\n"]
    pub fn set_pg_partman_bgw_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pg_partman_bgw_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pg_stat_statements_track`.\n"]
    pub fn set_pg_stat_statements_track(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().pg_stat_statements_track = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shared_buffers_percentage`.\n"]
    pub fn set_shared_buffers_percentage(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().shared_buffers_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temp_file_limit`.\n"]
    pub fn set_temp_file_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().temp_file_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timezone`.\n"]
    pub fn set_timezone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `track_activity_query_size`.\n"]
    pub fn set_track_activity_query_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().track_activity_query_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `track_commit_timestamp`.\n"]
    pub fn set_track_commit_timestamp(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().track_commit_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `track_functions`.\n"]
    pub fn set_track_functions(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().track_functions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `track_io_timing`.\n"]
    pub fn set_track_io_timing(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().track_io_timing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wal_sender_timeout`.\n"]
    pub fn set_wal_sender_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wal_sender_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wal_writer_delay`.\n"]
    pub fn set_wal_writer_delay(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wal_writer_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `work_mem`.\n"]
    pub fn set_work_mem(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().work_mem = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pgbouncer`.\n"]
    pub fn set_pgbouncer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabasePostgresqlConfig_Pgbouncer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pgbouncer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pgbouncer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timescaledb`.\n"]
    pub fn set_timescaledb(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabasePostgresqlConfig_Timescaledb>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timescaledb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timescaledb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `autovacuum_analyze_scale_factor` after provisioning.\n"]
    pub fn autovacuum_analyze_scale_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autovacuum_analyze_scale_factor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `autovacuum_analyze_threshold` after provisioning.\n"]
    pub fn autovacuum_analyze_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_analyze_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_freeze_max_age` after provisioning.\n"]
    pub fn autovacuum_freeze_max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_freeze_max_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_max_workers` after provisioning.\n"]
    pub fn autovacuum_max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_max_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_naptime` after provisioning.\n"]
    pub fn autovacuum_naptime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_naptime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_cost_delay` after provisioning.\n"]
    pub fn autovacuum_vacuum_cost_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_vacuum_cost_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_cost_limit` after provisioning.\n"]
    pub fn autovacuum_vacuum_cost_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_vacuum_cost_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_scale_factor` after provisioning.\n"]
    pub fn autovacuum_vacuum_scale_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autovacuum_vacuum_scale_factor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_threshold` after provisioning.\n"]
    pub fn autovacuum_vacuum_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_vacuum_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_hour` after provisioning.\n"]
    pub fn backup_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_hour", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_minute` after provisioning.\n"]
    pub fn backup_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_minute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_delay` after provisioning.\n"]
    pub fn bgwriter_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_flush_after` after provisioning.\n"]
    pub fn bgwriter_flush_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_flush_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_lru_maxpages` after provisioning.\n"]
    pub fn bgwriter_lru_maxpages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_lru_maxpages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_lru_multiplier` after provisioning.\n"]
    pub fn bgwriter_lru_multiplier(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_lru_multiplier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deadlock_timeout` after provisioning.\n"]
    pub fn deadlock_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deadlock_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_toast_compression` after provisioning.\n"]
    pub fn default_toast_compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_toast_compression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_in_transaction_session_timeout` after provisioning.\n"]
    pub fn idle_in_transaction_session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_in_transaction_session_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `jit` after provisioning.\n"]
    pub fn jit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_autovacuum_min_duration` after provisioning.\n"]
    pub fn log_autovacuum_min_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_autovacuum_min_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_error_verbosity` after provisioning.\n"]
    pub fn log_error_verbosity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_error_verbosity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_line_prefix` after provisioning.\n"]
    pub fn log_line_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_line_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_min_duration_statement` after provisioning.\n"]
    pub fn log_min_duration_statement(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_min_duration_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_files_per_process` after provisioning.\n"]
    pub fn max_files_per_process(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_files_per_process", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_locks_per_transaction` after provisioning.\n"]
    pub fn max_locks_per_transaction(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_locks_per_transaction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_logical_replication_workers` after provisioning.\n"]
    pub fn max_logical_replication_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_logical_replication_workers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_parallel_workers` after provisioning.\n"]
    pub fn max_parallel_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_parallel_workers_per_gather` after provisioning.\n"]
    pub fn max_parallel_workers_per_gather(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_workers_per_gather", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_pred_locks_per_transaction` after provisioning.\n"]
    pub fn max_pred_locks_per_transaction(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pred_locks_per_transaction", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_prepared_transactions` after provisioning.\n"]
    pub fn max_prepared_transactions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_prepared_transactions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_replication_slots` after provisioning.\n"]
    pub fn max_replication_slots(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replication_slots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_stack_depth` after provisioning.\n"]
    pub fn max_stack_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_stack_depth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_standby_archive_delay` after provisioning.\n"]
    pub fn max_standby_archive_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_standby_archive_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_standby_streaming_delay` after provisioning.\n"]
    pub fn max_standby_streaming_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_standby_streaming_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_wal_senders` after provisioning.\n"]
    pub fn max_wal_senders(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_wal_senders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_worker_processes` after provisioning.\n"]
    pub fn max_worker_processes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_worker_processes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pg_partman_bgw_interval` after provisioning.\n"]
    pub fn pg_partman_bgw_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pg_partman_bgw_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pg_partman_bgw_role` after provisioning.\n"]
    pub fn pg_partman_bgw_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pg_partman_bgw_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pg_stat_statements_track` after provisioning.\n"]
    pub fn pg_stat_statements_track(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pg_stat_statements_track", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_buffers_percentage` after provisioning.\n"]
    pub fn shared_buffers_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_buffers_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `temp_file_limit` after provisioning.\n"]
    pub fn temp_file_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temp_file_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_activity_query_size` after provisioning.\n"]
    pub fn track_activity_query_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_activity_query_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_commit_timestamp` after provisioning.\n"]
    pub fn track_commit_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_commit_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_functions` after provisioning.\n"]
    pub fn track_functions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_io_timing` after provisioning.\n"]
    pub fn track_io_timing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_io_timing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wal_sender_timeout` after provisioning.\n"]
    pub fn wal_sender_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wal_sender_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wal_writer_delay` after provisioning.\n"]
    pub fn wal_writer_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wal_writer_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `work_mem` after provisioning.\n"]
    pub fn work_mem(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.work_mem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pgbouncer` after provisioning.\n"]
    pub fn pgbouncer(&self) -> terrars::ListRef<DatabasePostgresqlConfig_PgbouncerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pgbouncer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timescaledb` after provisioning.\n"]
    pub fn timescaledb(&self) -> terrars::ListRef<DatabasePostgresqlConfig_TimescaledbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timescaledb", self.extract_ref()))
    }
}

impl terrars::Referable for DatabasePostgresqlConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabasePostgresqlConfig { }

impl terrars::ToListMappable for DatabasePostgresqlConfig {
    type O = terrars::ListRef<DatabasePostgresqlConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabasePostgresqlConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_postgresql_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabasePostgresqlConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabasePostgresqlConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabasePostgresqlConfig {
        let out = DatabasePostgresqlConfig(std::rc::Rc::new(DatabasePostgresqlConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabasePostgresqlConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                autovacuum_analyze_scale_factor: core::default::Default::default(),
                autovacuum_analyze_threshold: core::default::Default::default(),
                autovacuum_freeze_max_age: core::default::Default::default(),
                autovacuum_max_workers: core::default::Default::default(),
                autovacuum_naptime: core::default::Default::default(),
                autovacuum_vacuum_cost_delay: core::default::Default::default(),
                autovacuum_vacuum_cost_limit: core::default::Default::default(),
                autovacuum_vacuum_scale_factor: core::default::Default::default(),
                autovacuum_vacuum_threshold: core::default::Default::default(),
                backup_hour: core::default::Default::default(),
                backup_minute: core::default::Default::default(),
                bgwriter_delay: core::default::Default::default(),
                bgwriter_flush_after: core::default::Default::default(),
                bgwriter_lru_maxpages: core::default::Default::default(),
                bgwriter_lru_multiplier: core::default::Default::default(),
                cluster_id: self.cluster_id,
                deadlock_timeout: core::default::Default::default(),
                default_toast_compression: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_in_transaction_session_timeout: core::default::Default::default(),
                jit: core::default::Default::default(),
                log_autovacuum_min_duration: core::default::Default::default(),
                log_error_verbosity: core::default::Default::default(),
                log_line_prefix: core::default::Default::default(),
                log_min_duration_statement: core::default::Default::default(),
                max_files_per_process: core::default::Default::default(),
                max_locks_per_transaction: core::default::Default::default(),
                max_logical_replication_workers: core::default::Default::default(),
                max_parallel_workers: core::default::Default::default(),
                max_parallel_workers_per_gather: core::default::Default::default(),
                max_pred_locks_per_transaction: core::default::Default::default(),
                max_prepared_transactions: core::default::Default::default(),
                max_replication_slots: core::default::Default::default(),
                max_stack_depth: core::default::Default::default(),
                max_standby_archive_delay: core::default::Default::default(),
                max_standby_streaming_delay: core::default::Default::default(),
                max_wal_senders: core::default::Default::default(),
                max_worker_processes: core::default::Default::default(),
                pg_partman_bgw_interval: core::default::Default::default(),
                pg_partman_bgw_role: core::default::Default::default(),
                pg_stat_statements_track: core::default::Default::default(),
                shared_buffers_percentage: core::default::Default::default(),
                temp_file_limit: core::default::Default::default(),
                timezone: core::default::Default::default(),
                track_activity_query_size: core::default::Default::default(),
                track_commit_timestamp: core::default::Default::default(),
                track_functions: core::default::Default::default(),
                track_io_timing: core::default::Default::default(),
                wal_sender_timeout: core::default::Default::default(),
                wal_writer_delay: core::default::Default::default(),
                work_mem: core::default::Default::default(),
                pgbouncer: core::default::Default::default(),
                timescaledb: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabasePostgresqlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabasePostgresqlConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabasePostgresqlConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autovacuum_analyze_scale_factor` after provisioning.\n"]
    pub fn autovacuum_analyze_scale_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autovacuum_analyze_scale_factor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `autovacuum_analyze_threshold` after provisioning.\n"]
    pub fn autovacuum_analyze_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_analyze_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_freeze_max_age` after provisioning.\n"]
    pub fn autovacuum_freeze_max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_freeze_max_age", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_max_workers` after provisioning.\n"]
    pub fn autovacuum_max_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_max_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_naptime` after provisioning.\n"]
    pub fn autovacuum_naptime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_naptime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_cost_delay` after provisioning.\n"]
    pub fn autovacuum_vacuum_cost_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_vacuum_cost_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_cost_limit` after provisioning.\n"]
    pub fn autovacuum_vacuum_cost_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_vacuum_cost_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_scale_factor` after provisioning.\n"]
    pub fn autovacuum_vacuum_scale_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.autovacuum_vacuum_scale_factor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `autovacuum_vacuum_threshold` after provisioning.\n"]
    pub fn autovacuum_vacuum_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autovacuum_vacuum_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_hour` after provisioning.\n"]
    pub fn backup_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_hour", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_minute` after provisioning.\n"]
    pub fn backup_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_minute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_delay` after provisioning.\n"]
    pub fn bgwriter_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_flush_after` after provisioning.\n"]
    pub fn bgwriter_flush_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_flush_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_lru_maxpages` after provisioning.\n"]
    pub fn bgwriter_lru_maxpages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_lru_maxpages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgwriter_lru_multiplier` after provisioning.\n"]
    pub fn bgwriter_lru_multiplier(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgwriter_lru_multiplier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deadlock_timeout` after provisioning.\n"]
    pub fn deadlock_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deadlock_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_toast_compression` after provisioning.\n"]
    pub fn default_toast_compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_toast_compression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_in_transaction_session_timeout` after provisioning.\n"]
    pub fn idle_in_transaction_session_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_in_transaction_session_timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `jit` after provisioning.\n"]
    pub fn jit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_autovacuum_min_duration` after provisioning.\n"]
    pub fn log_autovacuum_min_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_autovacuum_min_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_error_verbosity` after provisioning.\n"]
    pub fn log_error_verbosity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_error_verbosity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_line_prefix` after provisioning.\n"]
    pub fn log_line_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_line_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_min_duration_statement` after provisioning.\n"]
    pub fn log_min_duration_statement(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_min_duration_statement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_files_per_process` after provisioning.\n"]
    pub fn max_files_per_process(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_files_per_process", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_locks_per_transaction` after provisioning.\n"]
    pub fn max_locks_per_transaction(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_locks_per_transaction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_logical_replication_workers` after provisioning.\n"]
    pub fn max_logical_replication_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_logical_replication_workers", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_parallel_workers` after provisioning.\n"]
    pub fn max_parallel_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_workers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_parallel_workers_per_gather` after provisioning.\n"]
    pub fn max_parallel_workers_per_gather(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_parallel_workers_per_gather", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_pred_locks_per_transaction` after provisioning.\n"]
    pub fn max_pred_locks_per_transaction(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_pred_locks_per_transaction", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_prepared_transactions` after provisioning.\n"]
    pub fn max_prepared_transactions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_prepared_transactions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_replication_slots` after provisioning.\n"]
    pub fn max_replication_slots(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replication_slots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_stack_depth` after provisioning.\n"]
    pub fn max_stack_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_stack_depth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_standby_archive_delay` after provisioning.\n"]
    pub fn max_standby_archive_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_standby_archive_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_standby_streaming_delay` after provisioning.\n"]
    pub fn max_standby_streaming_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_standby_streaming_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_wal_senders` after provisioning.\n"]
    pub fn max_wal_senders(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_wal_senders", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_worker_processes` after provisioning.\n"]
    pub fn max_worker_processes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_worker_processes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pg_partman_bgw_interval` after provisioning.\n"]
    pub fn pg_partman_bgw_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pg_partman_bgw_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pg_partman_bgw_role` after provisioning.\n"]
    pub fn pg_partman_bgw_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pg_partman_bgw_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pg_stat_statements_track` after provisioning.\n"]
    pub fn pg_stat_statements_track(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pg_stat_statements_track", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shared_buffers_percentage` after provisioning.\n"]
    pub fn shared_buffers_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_buffers_percentage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `temp_file_limit` after provisioning.\n"]
    pub fn temp_file_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temp_file_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_activity_query_size` after provisioning.\n"]
    pub fn track_activity_query_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_activity_query_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_commit_timestamp` after provisioning.\n"]
    pub fn track_commit_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_commit_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_functions` after provisioning.\n"]
    pub fn track_functions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `track_io_timing` after provisioning.\n"]
    pub fn track_io_timing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_io_timing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wal_sender_timeout` after provisioning.\n"]
    pub fn wal_sender_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wal_sender_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wal_writer_delay` after provisioning.\n"]
    pub fn wal_writer_delay(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wal_writer_delay", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `work_mem` after provisioning.\n"]
    pub fn work_mem(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.work_mem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pgbouncer` after provisioning.\n"]
    pub fn pgbouncer(&self) -> terrars::ListRef<DatabasePostgresqlConfig_PgbouncerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pgbouncer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timescaledb` after provisioning.\n"]
    pub fn timescaledb(&self) -> terrars::ListRef<DatabasePostgresqlConfig_TimescaledbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timescaledb", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatabasePostgresqlConfig_Pgbouncer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodb_idle_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodb_max_db_connections: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodb_pool_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodb_pool_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_startup_parameters: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_pool_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_idle_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_lifetime: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_reset_query_always: std::option::Option<terrars::PrimField<bool>>,
}

impl DatabasePostgresqlConfig_Pgbouncer {
    #[doc = "Set the field `autodb_idle_timeout`.\n"]
    pub fn set_autodb_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.autodb_idle_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autodb_max_db_connections`.\n"]
    pub fn set_autodb_max_db_connections(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.autodb_max_db_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autodb_pool_mode`.\n"]
    pub fn set_autodb_pool_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.autodb_pool_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autodb_pool_size`.\n"]
    pub fn set_autodb_pool_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.autodb_pool_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_startup_parameters`.\n"]
    pub fn set_ignore_startup_parameters(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ignore_startup_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_pool_size`.\n"]
    pub fn set_min_pool_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_pool_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_idle_timeout`.\n"]
    pub fn set_server_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.server_idle_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_lifetime`.\n"]
    pub fn set_server_lifetime(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.server_lifetime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_reset_query_always`.\n"]
    pub fn set_server_reset_query_always(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.server_reset_query_always = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabasePostgresqlConfig_Pgbouncer {
    type O = terrars::BlockAssignable<DatabasePostgresqlConfig_Pgbouncer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabasePostgresqlConfig_Pgbouncer {}

impl BuildDatabasePostgresqlConfig_Pgbouncer {
    pub fn build(self) -> DatabasePostgresqlConfig_Pgbouncer {
        DatabasePostgresqlConfig_Pgbouncer {
            autodb_idle_timeout: core::default::Default::default(),
            autodb_max_db_connections: core::default::Default::default(),
            autodb_pool_mode: core::default::Default::default(),
            autodb_pool_size: core::default::Default::default(),
            ignore_startup_parameters: core::default::Default::default(),
            min_pool_size: core::default::Default::default(),
            server_idle_timeout: core::default::Default::default(),
            server_lifetime: core::default::Default::default(),
            server_reset_query_always: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabasePostgresqlConfig_PgbouncerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabasePostgresqlConfig_PgbouncerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabasePostgresqlConfig_PgbouncerRef {
        DatabasePostgresqlConfig_PgbouncerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabasePostgresqlConfig_PgbouncerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autodb_idle_timeout` after provisioning.\n"]
    pub fn autodb_idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodb_idle_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `autodb_max_db_connections` after provisioning.\n"]
    pub fn autodb_max_db_connections(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodb_max_db_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `autodb_pool_mode` after provisioning.\n"]
    pub fn autodb_pool_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodb_pool_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `autodb_pool_size` after provisioning.\n"]
    pub fn autodb_pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodb_pool_size", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_startup_parameters` after provisioning.\n"]
    pub fn ignore_startup_parameters(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ignore_startup_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `min_pool_size` after provisioning.\n"]
    pub fn min_pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_pool_size", self.base))
    }

    #[doc = "Get a reference to the value of field `server_idle_timeout` after provisioning.\n"]
    pub fn server_idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_idle_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `server_lifetime` after provisioning.\n"]
    pub fn server_lifetime(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_lifetime", self.base))
    }

    #[doc = "Get a reference to the value of field `server_reset_query_always` after provisioning.\n"]
    pub fn server_reset_query_always(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_reset_query_always", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabasePostgresqlConfig_Timescaledb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_background_workers: std::option::Option<terrars::PrimField<f64>>,
}

impl DatabasePostgresqlConfig_Timescaledb {
    #[doc = "Set the field `max_background_workers`.\n"]
    pub fn set_max_background_workers(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_background_workers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabasePostgresqlConfig_Timescaledb {
    type O = terrars::BlockAssignable<DatabasePostgresqlConfig_Timescaledb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabasePostgresqlConfig_Timescaledb {}

impl BuildDatabasePostgresqlConfig_Timescaledb {
    pub fn build(self) -> DatabasePostgresqlConfig_Timescaledb {
        DatabasePostgresqlConfig_Timescaledb { max_background_workers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatabasePostgresqlConfig_TimescaledbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabasePostgresqlConfig_TimescaledbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabasePostgresqlConfig_TimescaledbRef {
        DatabasePostgresqlConfig_TimescaledbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabasePostgresqlConfig_TimescaledbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_background_workers` after provisioning.\n"]
    pub fn max_background_workers(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_background_workers", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabasePostgresqlConfigDynamic {
    pgbouncer: std::option::Option<terrars::DynamicBlock<DatabasePostgresqlConfig_Pgbouncer>>,
    timescaledb: std::option::Option<terrars::DynamicBlock<DatabasePostgresqlConfig_Timescaledb>>,
}
