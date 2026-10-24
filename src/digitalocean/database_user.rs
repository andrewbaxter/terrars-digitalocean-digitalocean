use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseUserData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_auth_plugin: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<DatabaseUserSettingsEl>>,
    dynamic: DatabaseUserDynamic,
}

struct DatabaseUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseUserData>,
}

#[derive(Clone)]
pub struct DatabaseUser(std::rc::Rc<DatabaseUser_>);

impl DatabaseUser {
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

    #[doc = "Set the field `mysql_auth_plugin`.\n"]
    pub fn set_mysql_auth_plugin(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mysql_auth_plugin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(self, v: impl std::convert::Into<terrars::BlockAssignable<DatabaseUserSettingsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.settings = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DatabaseUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseUser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseUser { }

impl terrars::ToListMappable for DatabaseUser {
    type O = terrars::ListRef<DatabaseUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseUser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseUser {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseUser {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseUser {
        let out = DatabaseUser(std::rc::Rc::new(DatabaseUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseUserData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                mysql_auth_plugin: core::default::Default::default(),
                name: self.name,
                settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseUserRef {
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

    #[doc = "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DatabaseUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatabaseUserSettingsElAclEl {
    permission: terrars::PrimField<std::string::String>,
    topic: terrars::PrimField<std::string::String>,
}

impl DatabaseUserSettingsElAclEl { }

impl terrars::ToListMappable for DatabaseUserSettingsElAclEl {
    type O = terrars::BlockAssignable<DatabaseUserSettingsElAclEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseUserSettingsElAclEl {
    #[doc = ""]
    pub permission: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseUserSettingsElAclElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabaseUserSettingsElAclElRef {
        DatabaseUserSettingsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseUserSettingsElAclElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseUserSettingsElDynamic {
    acl: std::option::Option<terrars::DynamicBlock<DatabaseUserSettingsElAclEl>>,
}

#[derive(Serialize)]
pub struct DatabaseUserSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<std::vec::Vec<DatabaseUserSettingsElAclEl>>,
    dynamic: DatabaseUserSettingsElDynamic,
}

impl DatabaseUserSettingsEl {
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(mut self, v: impl std::convert::Into<terrars::BlockAssignable<DatabaseUserSettingsElAclEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.acl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.acl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseUserSettingsEl {
    type O = terrars::BlockAssignable<DatabaseUserSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatabaseUserSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseUserSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabaseUserSettingsElRef {
        DatabaseUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseUserSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::ListRef<DatabaseUserSettingsElAclElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseUserDynamic {
    settings: std::option::Option<terrars::DynamicBlock<DatabaseUserSettingsEl>>,
}
