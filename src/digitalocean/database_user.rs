use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseUserData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    mysql_auth_plugin: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<Vec<DatabaseUserSettingsEl>>,
    dynamic: DatabaseUserDynamic,
}

struct DatabaseUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseUserData>,
}

#[derive(Clone)]
pub struct DatabaseUser(Rc<DatabaseUser_>);

impl DatabaseUser {
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

    #[doc= "Set the field `mysql_auth_plugin`.\n"]
    pub fn set_mysql_auth_plugin(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mysql_auth_plugin = Some(v.into());
        self
    }

    #[doc= "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl Into<BlockAssignable<DatabaseUserSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = Some(d);
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

    #[doc= "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<DatabaseUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl Referable for DatabaseUser {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatabaseUser { }

impl ToListMappable for DatabaseUser {
    type O = ListRef<DatabaseUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatabaseUser_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_database_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseUser {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDatabaseUser {
    pub fn build(self, stack: &mut Stack) -> DatabaseUser {
        let out = DatabaseUser(Rc::new(DatabaseUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatabaseUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                mysql_auth_plugin: core::default::Default::default(),
                name: self.name,
                settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseUserRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseUserRef {
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

    #[doc= "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<DatabaseUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatabaseUserSettingsElAclEl {
    permission: PrimField<String>,
    topic: PrimField<String>,
}

impl DatabaseUserSettingsElAclEl { }

impl ToListMappable for DatabaseUserSettingsElAclEl {
    type O = BlockAssignable<DatabaseUserSettingsElAclEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseUserSettingsElAclEl {
    #[doc= ""]
    pub permission: PrimField<String>,
    #[doc= ""]
    pub topic: PrimField<String>,
}

impl BuildDatabaseUserSettingsElAclEl {
    pub fn build(self) -> DatabaseUserSettingsElAclEl {
        DatabaseUserSettingsElAclEl {
            permission: self.permission,
            topic: self.topic,
        }
    }
}

pub struct DatabaseUserSettingsElAclElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseUserSettingsElAclElRef {
    fn new(shared: StackShared, base: String) -> DatabaseUserSettingsElAclElRef {
        DatabaseUserSettingsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseUserSettingsElAclElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseUserSettingsElDynamic {
    acl: Option<DynamicBlock<DatabaseUserSettingsElAclEl>>,
}

#[derive(Serialize)]
pub struct DatabaseUserSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<Vec<DatabaseUserSettingsElAclEl>>,
    dynamic: DatabaseUserSettingsElDynamic,
}

impl DatabaseUserSettingsEl {
    #[doc= "Set the field `acl`.\n"]
    pub fn set_acl(mut self, v: impl Into<BlockAssignable<DatabaseUserSettingsElAclEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.acl = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.acl = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DatabaseUserSettingsEl {
    type O = BlockAssignable<DatabaseUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseUserSettingsEl {}

impl BuildDatabaseUserSettingsEl {
    pub fn build(self) -> DatabaseUserSettingsEl {
        DatabaseUserSettingsEl {
            acl: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DatabaseUserSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> DatabaseUserSettingsElRef {
        DatabaseUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> ListRef<DatabaseUserSettingsElAclElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseUserDynamic {
    settings: Option<DynamicBlock<DatabaseUserSettingsEl>>,
}
