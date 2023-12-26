use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseMysqlConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_hour: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_minute: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binlog_retention_period: Option<PrimField<f64>>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_concat_max_len: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    information_schema_stats_expiry: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_ft_min_token_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_ft_server_stopword_table: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_lock_wait_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_log_buffer_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_online_alter_log_max_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_print_all_deadlocks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    innodb_rollback_on_timeout: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interactive_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_tmp_mem_storage_engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_query_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_allowed_packet: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_heap_table_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    net_read_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    net_write_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slow_query_log: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_buffer_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_require_primary_key: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tmp_table_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait_timeout: Option<PrimField<f64>>,
}

struct DatabaseMysqlConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseMysqlConfigData>,
}

#[derive(Clone)]
pub struct DatabaseMysqlConfig(Rc<DatabaseMysqlConfig_>);

impl DatabaseMysqlConfig {
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

    #[doc= "Set the field `backup_hour`.\n"]
    pub fn set_backup_hour(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_hour = Some(v.into());
        self
    }

    #[doc= "Set the field `backup_minute`.\n"]
    pub fn set_backup_minute(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backup_minute = Some(v.into());
        self
    }

    #[doc= "Set the field `binlog_retention_period`.\n"]
    pub fn set_binlog_retention_period(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().binlog_retention_period = Some(v.into());
        self
    }

    #[doc= "Set the field `connect_timeout`.\n"]
    pub fn set_connect_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().connect_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `default_time_zone`.\n"]
    pub fn set_default_time_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_time_zone = Some(v.into());
        self
    }

    #[doc= "Set the field `group_concat_max_len`.\n"]
    pub fn set_group_concat_max_len(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().group_concat_max_len = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `information_schema_stats_expiry`.\n"]
    pub fn set_information_schema_stats_expiry(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().information_schema_stats_expiry = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_ft_min_token_size`.\n"]
    pub fn set_innodb_ft_min_token_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().innodb_ft_min_token_size = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_ft_server_stopword_table`.\n"]
    pub fn set_innodb_ft_server_stopword_table(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().innodb_ft_server_stopword_table = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_lock_wait_timeout`.\n"]
    pub fn set_innodb_lock_wait_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().innodb_lock_wait_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_log_buffer_size`.\n"]
    pub fn set_innodb_log_buffer_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().innodb_log_buffer_size = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_online_alter_log_max_size`.\n"]
    pub fn set_innodb_online_alter_log_max_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().innodb_online_alter_log_max_size = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_print_all_deadlocks`.\n"]
    pub fn set_innodb_print_all_deadlocks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().innodb_print_all_deadlocks = Some(v.into());
        self
    }

    #[doc= "Set the field `innodb_rollback_on_timeout`.\n"]
    pub fn set_innodb_rollback_on_timeout(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().innodb_rollback_on_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `interactive_timeout`.\n"]
    pub fn set_interactive_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interactive_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_tmp_mem_storage_engine`.\n"]
    pub fn set_internal_tmp_mem_storage_engine(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().internal_tmp_mem_storage_engine = Some(v.into());
        self
    }

    #[doc= "Set the field `long_query_time`.\n"]
    pub fn set_long_query_time(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().long_query_time = Some(v.into());
        self
    }

    #[doc= "Set the field `max_allowed_packet`.\n"]
    pub fn set_max_allowed_packet(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_allowed_packet = Some(v.into());
        self
    }

    #[doc= "Set the field `max_heap_table_size`.\n"]
    pub fn set_max_heap_table_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_heap_table_size = Some(v.into());
        self
    }

    #[doc= "Set the field `net_read_timeout`.\n"]
    pub fn set_net_read_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().net_read_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `net_write_timeout`.\n"]
    pub fn set_net_write_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().net_write_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `slow_query_log`.\n"]
    pub fn set_slow_query_log(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().slow_query_log = Some(v.into());
        self
    }

    #[doc= "Set the field `sort_buffer_size`.\n"]
    pub fn set_sort_buffer_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().sort_buffer_size = Some(v.into());
        self
    }

    #[doc= "Set the field `sql_mode`.\n"]
    pub fn set_sql_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sql_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `sql_require_primary_key`.\n"]
    pub fn set_sql_require_primary_key(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().sql_require_primary_key = Some(v.into());
        self
    }

    #[doc= "Set the field `tmp_table_size`.\n"]
    pub fn set_tmp_table_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tmp_table_size = Some(v.into());
        self
    }

    #[doc= "Set the field `wait_timeout`.\n"]
    pub fn set_wait_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().wait_timeout = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `backup_hour` after provisioning.\n"]
    pub fn backup_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_hour", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_minute` after provisioning.\n"]
    pub fn backup_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_minute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `binlog_retention_period` after provisioning.\n"]
    pub fn binlog_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.binlog_retention_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connect_timeout` after provisioning.\n"]
    pub fn connect_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_time_zone` after provisioning.\n"]
    pub fn default_time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_time_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_concat_max_len` after provisioning.\n"]
    pub fn group_concat_max_len(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_concat_max_len", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `information_schema_stats_expiry` after provisioning.\n"]
    pub fn information_schema_stats_expiry(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.information_schema_stats_expiry", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_ft_min_token_size` after provisioning.\n"]
    pub fn innodb_ft_min_token_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_ft_min_token_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_ft_server_stopword_table` after provisioning.\n"]
    pub fn innodb_ft_server_stopword_table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_ft_server_stopword_table", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_lock_wait_timeout` after provisioning.\n"]
    pub fn innodb_lock_wait_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_lock_wait_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_log_buffer_size` after provisioning.\n"]
    pub fn innodb_log_buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_log_buffer_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_online_alter_log_max_size` after provisioning.\n"]
    pub fn innodb_online_alter_log_max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_online_alter_log_max_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_print_all_deadlocks` after provisioning.\n"]
    pub fn innodb_print_all_deadlocks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_print_all_deadlocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_rollback_on_timeout` after provisioning.\n"]
    pub fn innodb_rollback_on_timeout(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_rollback_on_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interactive_timeout` after provisioning.\n"]
    pub fn interactive_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interactive_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal_tmp_mem_storage_engine` after provisioning.\n"]
    pub fn internal_tmp_mem_storage_engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_tmp_mem_storage_engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `long_query_time` after provisioning.\n"]
    pub fn long_query_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_query_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_allowed_packet` after provisioning.\n"]
    pub fn max_allowed_packet(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_allowed_packet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_heap_table_size` after provisioning.\n"]
    pub fn max_heap_table_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_heap_table_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `net_read_timeout` after provisioning.\n"]
    pub fn net_read_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.net_read_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `net_write_timeout` after provisioning.\n"]
    pub fn net_write_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.net_write_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slow_query_log` after provisioning.\n"]
    pub fn slow_query_log(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.slow_query_log", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort_buffer_size` after provisioning.\n"]
    pub fn sort_buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_buffer_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sql_mode` after provisioning.\n"]
    pub fn sql_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sql_require_primary_key` after provisioning.\n"]
    pub fn sql_require_primary_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_require_primary_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tmp_table_size` after provisioning.\n"]
    pub fn tmp_table_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tmp_table_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_timeout` after provisioning.\n"]
    pub fn wait_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }
}

impl Referable for DatabaseMysqlConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatabaseMysqlConfig { }

impl ToListMappable for DatabaseMysqlConfig {
    type O = ListRef<DatabaseMysqlConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatabaseMysqlConfig_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_database_mysql_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseMysqlConfig {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
}

impl BuildDatabaseMysqlConfig {
    pub fn build(self, stack: &mut Stack) -> DatabaseMysqlConfig {
        let out = DatabaseMysqlConfig(Rc::new(DatabaseMysqlConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatabaseMysqlConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backup_hour: core::default::Default::default(),
                backup_minute: core::default::Default::default(),
                binlog_retention_period: core::default::Default::default(),
                cluster_id: self.cluster_id,
                connect_timeout: core::default::Default::default(),
                default_time_zone: core::default::Default::default(),
                group_concat_max_len: core::default::Default::default(),
                id: core::default::Default::default(),
                information_schema_stats_expiry: core::default::Default::default(),
                innodb_ft_min_token_size: core::default::Default::default(),
                innodb_ft_server_stopword_table: core::default::Default::default(),
                innodb_lock_wait_timeout: core::default::Default::default(),
                innodb_log_buffer_size: core::default::Default::default(),
                innodb_online_alter_log_max_size: core::default::Default::default(),
                innodb_print_all_deadlocks: core::default::Default::default(),
                innodb_rollback_on_timeout: core::default::Default::default(),
                interactive_timeout: core::default::Default::default(),
                internal_tmp_mem_storage_engine: core::default::Default::default(),
                long_query_time: core::default::Default::default(),
                max_allowed_packet: core::default::Default::default(),
                max_heap_table_size: core::default::Default::default(),
                net_read_timeout: core::default::Default::default(),
                net_write_timeout: core::default::Default::default(),
                slow_query_log: core::default::Default::default(),
                sort_buffer_size: core::default::Default::default(),
                sql_mode: core::default::Default::default(),
                sql_require_primary_key: core::default::Default::default(),
                tmp_table_size: core::default::Default::default(),
                wait_timeout: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseMysqlConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseMysqlConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseMysqlConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backup_hour` after provisioning.\n"]
    pub fn backup_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_hour", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_minute` after provisioning.\n"]
    pub fn backup_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_minute", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `binlog_retention_period` after provisioning.\n"]
    pub fn binlog_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.binlog_retention_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connect_timeout` after provisioning.\n"]
    pub fn connect_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_time_zone` after provisioning.\n"]
    pub fn default_time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_time_zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_concat_max_len` after provisioning.\n"]
    pub fn group_concat_max_len(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_concat_max_len", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `information_schema_stats_expiry` after provisioning.\n"]
    pub fn information_schema_stats_expiry(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.information_schema_stats_expiry", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_ft_min_token_size` after provisioning.\n"]
    pub fn innodb_ft_min_token_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_ft_min_token_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_ft_server_stopword_table` after provisioning.\n"]
    pub fn innodb_ft_server_stopword_table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_ft_server_stopword_table", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_lock_wait_timeout` after provisioning.\n"]
    pub fn innodb_lock_wait_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_lock_wait_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_log_buffer_size` after provisioning.\n"]
    pub fn innodb_log_buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_log_buffer_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_online_alter_log_max_size` after provisioning.\n"]
    pub fn innodb_online_alter_log_max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_online_alter_log_max_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_print_all_deadlocks` after provisioning.\n"]
    pub fn innodb_print_all_deadlocks(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_print_all_deadlocks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `innodb_rollback_on_timeout` after provisioning.\n"]
    pub fn innodb_rollback_on_timeout(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.innodb_rollback_on_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interactive_timeout` after provisioning.\n"]
    pub fn interactive_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interactive_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `internal_tmp_mem_storage_engine` after provisioning.\n"]
    pub fn internal_tmp_mem_storage_engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_tmp_mem_storage_engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `long_query_time` after provisioning.\n"]
    pub fn long_query_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.long_query_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_allowed_packet` after provisioning.\n"]
    pub fn max_allowed_packet(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_allowed_packet", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_heap_table_size` after provisioning.\n"]
    pub fn max_heap_table_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_heap_table_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `net_read_timeout` after provisioning.\n"]
    pub fn net_read_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.net_read_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `net_write_timeout` after provisioning.\n"]
    pub fn net_write_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.net_write_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slow_query_log` after provisioning.\n"]
    pub fn slow_query_log(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.slow_query_log", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort_buffer_size` after provisioning.\n"]
    pub fn sort_buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_buffer_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sql_mode` after provisioning.\n"]
    pub fn sql_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sql_require_primary_key` after provisioning.\n"]
    pub fn sql_require_primary_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_require_primary_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tmp_table_size` after provisioning.\n"]
    pub fn tmp_table_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tmp_table_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `wait_timeout` after provisioning.\n"]
    pub fn wait_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.wait_timeout", self.extract_ref()))
    }
}
