use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    engine: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eviction_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    node_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_network_uuid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    region: PrimField<String>,
    size: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_size_mib: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_restore: Option<Vec<DatabaseClusterBackupRestoreEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_window: Option<Vec<DatabaseClusterMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatabaseClusterTimeoutsEl>,
    dynamic: DatabaseClusterDynamic,
}

struct DatabaseCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseClusterData>,
}

#[derive(Clone)]
pub struct DatabaseCluster(Rc<DatabaseCluster_>);

impl DatabaseCluster {
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

    #[doc= "Set the field `eviction_policy`.\n"]
    pub fn set_eviction_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().eviction_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `private_network_uuid`.\n"]
    pub fn set_private_network_uuid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_network_uuid = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `sql_mode`.\n"]
    pub fn set_sql_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sql_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `storage_size_mib`.\n"]
    pub fn set_storage_size_mib(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_size_mib = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc= "Set the field `backup_restore`.\n"]
    pub fn set_backup_restore(self, v: impl Into<BlockAssignable<DatabaseClusterBackupRestoreEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_restore = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_restore = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(self, v: impl Into<BlockAssignable<DatabaseClusterMaintenanceWindowEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatabaseClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eviction_policy` after provisioning.\n"]
    pub fn eviction_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eviction_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_host` after provisioning.\n"]
    pub fn private_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_network_uuid` after provisioning.\n"]
    pub fn private_network_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_network_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_uri` after provisioning.\n"]
    pub fn private_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sql_mode` after provisioning.\n"]
    pub fn sql_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_size_mib` after provisioning.\n"]
    pub fn storage_size_mib(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_size_mib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_restore` after provisioning.\n"]
    pub fn backup_restore(&self) -> ListRef<DatabaseClusterBackupRestoreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_restore", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<DatabaseClusterMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseClusterTimeoutsElRef {
        DatabaseClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DatabaseCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatabaseCluster { }

impl ToListMappable for DatabaseCluster {
    type O = ListRef<DatabaseClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatabaseCluster_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_database_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseCluster {
    pub tf_id: String,
    #[doc= ""]
    pub engine: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub node_count: PrimField<f64>,
    #[doc= ""]
    pub region: PrimField<String>,
    #[doc= ""]
    pub size: PrimField<String>,
}

impl BuildDatabaseCluster {
    pub fn build(self, stack: &mut Stack) -> DatabaseCluster {
        let out = DatabaseCluster(Rc::new(DatabaseCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatabaseClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                engine: self.engine,
                eviction_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                node_count: self.node_count,
                private_network_uuid: core::default::Default::default(),
                project_id: core::default::Default::default(),
                region: self.region,
                size: self.size,
                sql_mode: core::default::Default::default(),
                storage_size_mib: core::default::Default::default(),
                tags: core::default::Default::default(),
                version: core::default::Default::default(),
                backup_restore: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `eviction_policy` after provisioning.\n"]
    pub fn eviction_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.eviction_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_host` after provisioning.\n"]
    pub fn private_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_network_uuid` after provisioning.\n"]
    pub fn private_network_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_network_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_uri` after provisioning.\n"]
    pub fn private_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sql_mode` after provisioning.\n"]
    pub fn sql_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_size_mib` after provisioning.\n"]
    pub fn storage_size_mib(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_size_mib", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_restore` after provisioning.\n"]
    pub fn backup_restore(&self) -> ListRef<DatabaseClusterBackupRestoreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_restore", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> ListRef<DatabaseClusterMaintenanceWindowElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseClusterTimeoutsElRef {
        DatabaseClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatabaseClusterBackupRestoreEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_created_at: Option<PrimField<String>>,
    database_name: PrimField<String>,
}

impl DatabaseClusterBackupRestoreEl {
    #[doc= "Set the field `backup_created_at`.\n"]
    pub fn set_backup_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backup_created_at = Some(v.into());
        self
    }
}

impl ToListMappable for DatabaseClusterBackupRestoreEl {
    type O = BlockAssignable<DatabaseClusterBackupRestoreEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseClusterBackupRestoreEl {
    #[doc= ""]
    pub database_name: PrimField<String>,
}

impl BuildDatabaseClusterBackupRestoreEl {
    pub fn build(self) -> DatabaseClusterBackupRestoreEl {
        DatabaseClusterBackupRestoreEl {
            backup_created_at: core::default::Default::default(),
            database_name: self.database_name,
        }
    }
}

pub struct DatabaseClusterBackupRestoreElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseClusterBackupRestoreElRef {
    fn new(shared: StackShared, base: String) -> DatabaseClusterBackupRestoreElRef {
        DatabaseClusterBackupRestoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseClusterBackupRestoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backup_created_at` after provisioning.\n"]
    pub fn backup_created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseClusterMaintenanceWindowEl {
    day: PrimField<String>,
    hour: PrimField<String>,
}

impl DatabaseClusterMaintenanceWindowEl { }

impl ToListMappable for DatabaseClusterMaintenanceWindowEl {
    type O = BlockAssignable<DatabaseClusterMaintenanceWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseClusterMaintenanceWindowEl {
    #[doc= ""]
    pub day: PrimField<String>,
    #[doc= ""]
    pub hour: PrimField<String>,
}

impl BuildDatabaseClusterMaintenanceWindowEl {
    pub fn build(self) -> DatabaseClusterMaintenanceWindowEl {
        DatabaseClusterMaintenanceWindowEl {
            day: self.day,
            hour: self.hour,
        }
    }
}

pub struct DatabaseClusterMaintenanceWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseClusterMaintenanceWindowElRef {
    fn new(shared: StackShared, base: String) -> DatabaseClusterMaintenanceWindowElRef {
        DatabaseClusterMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseClusterMaintenanceWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc= "Get a reference to the value of field `hour` after provisioning.\n"]
    pub fn hour(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl DatabaseClusterTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for DatabaseClusterTimeoutsEl {
    type O = BlockAssignable<DatabaseClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseClusterTimeoutsEl {}

impl BuildDatabaseClusterTimeoutsEl {
    pub fn build(self) -> DatabaseClusterTimeoutsEl {
        DatabaseClusterTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct DatabaseClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatabaseClusterTimeoutsElRef {
        DatabaseClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseClusterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseClusterDynamic {
    backup_restore: Option<DynamicBlock<DatabaseClusterBackupRestoreEl>>,
    maintenance_window: Option<DynamicBlock<DatabaseClusterMaintenanceWindowEl>>,
}
