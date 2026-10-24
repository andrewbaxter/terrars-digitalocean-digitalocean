use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AppData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppTimeoutsEl>,
    dynamic: AppDynamic,
}

struct App_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppData>,
}

#[derive(Clone)]
pub struct App(std::rc::Rc<App_>);

impl App {
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

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time of when the App was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_ingress` after provisioning.\nThe default URL to access the App"]
    pub fn default_ingress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ingress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `live_url` after provisioning.\n"]
    pub fn live_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time of when the App was last updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource identifier for the app"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppTimeoutsElRef {
        AppTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for App {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for App { }

impl terrars::ToListMappable for App {
    type O = terrars::ListRef<AppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for App_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApp {
    pub tf_id: std::string::String,
}

impl BuildApp {
    pub fn build(self, stack: &mut terrars::Stack) -> App {
        let out = App(std::rc::Rc::new(App_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time of when the App was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_ingress` after provisioning.\nThe default URL to access the App"]
    pub fn default_ingress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ingress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `live_url` after provisioning.\n"]
    pub fn live_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time of when the App was last updated"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource identifier for the app"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppTimeoutsElRef {
        AppTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppSpecElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    rule: terrars::PrimField<std::string::String>,
}

impl AppSpecElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElAlertEl {
    type O = terrars::BlockAssignable<AppSpecElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElAlertEl {
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElAlertEl {
    pub fn build(self) -> AppSpecElAlertEl {
        AppSpecElAlertEl {
            disabled: core::default::Default::default(),
            rule: self.rule,
        }
    }
}

pub struct AppSpecElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElAlertElRef {
        AppSpecElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElAlertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElDatabaseEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElDatabaseEl {
    #[doc =
        "Set the field `cluster_name`.\nThe name of the underlying DigitalOcean DBaaS cluster. This is required for production databases. For dev databases, if cluster_name is not set, a new cluster will be provisioned."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_name`.\nThe name of the MySQL or PostgreSQL database to configure."]
    pub fn set_db_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_user`.\nThe name of the MySQL or PostgreSQL user to configure."]
    pub fn set_db_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\nThe database engine to use."]
    pub fn set_engine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the component"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `production`.\nWhether this is a production or dev database."]
    pub fn set_production(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.production = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nThe version of the database engine."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElDatabaseEl {
    type O = terrars::BlockAssignable<AppSpecElDatabaseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElDatabaseEl {}

impl BuildAppSpecElDatabaseEl {
    pub fn build(self) -> AppSpecElDatabaseEl {
        AppSpecElDatabaseEl {
            cluster_name: core::default::Default::default(),
            db_name: core::default::Default::default(),
            db_user: core::default::Default::default(),
            engine: core::default::Default::default(),
            name: core::default::Default::default(),
            production: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElDatabaseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElDatabaseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElDatabaseElRef {
        AppSpecElDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElDatabaseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the underlying DigitalOcean DBaaS cluster. This is required for production databases. For dev databases, if cluster_name is not set, a new cluster will be provisioned."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_name` after provisioning.\nThe name of the MySQL or PostgreSQL database to configure."]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_user` after provisioning.\nThe name of the MySQL or PostgreSQL user to configure."]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\nThe database engine to use."]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `production` after provisioning.\nWhether this is a production or dev database."]
    pub fn production(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the database engine."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElDomainEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElDomainEl {
    #[doc = "Set the field `type_`.\nThe type of the domain."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wildcard`.\nIndicates whether the domain includes all sub-domains, in addition to the given domain."]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nIf the domain uses DigitalOcean DNS and you would like App Platform to automatically manage it for you, set this to the name of the domain on your account."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElDomainEl {
    type O = terrars::BlockAssignable<AppSpecElDomainEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElDomainEl {
    #[doc = "The hostname for the domain."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElDomainEl {
    pub fn build(self) -> AppSpecElDomainEl {
        AppSpecElDomainEl {
            name: self.name,
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElDomainElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElDomainElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElDomainElRef {
        AppSpecElDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElDomainElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe hostname for the domain."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the domain."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wildcard` after provisioning.\nIndicates whether the domain includes all sub-domains, in addition to the given domain."]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nIf the domain uses DigitalOcean DNS and you would like App Platform to automatically manage it for you, set this to the name of the domain on your account."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElEnvEl {
    #[doc = "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElEnvEl {
    type O = terrars::BlockAssignable<AppSpecElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElEnvEl {}

impl BuildAppSpecElEnvEl {
    pub fn build(self) -> AppSpecElEnvEl {
        AppSpecElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElEnvElRef {
        AppSpecElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
}

impl AppSpecElFunctionElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElAlertEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElAlertEl {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElFunctionElAlertEl {
    pub fn build(self) -> AppSpecElFunctionElAlertEl {
        AppSpecElFunctionElAlertEl {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
        }
    }
}

pub struct AppSpecElFunctionElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElAlertElRef {
        AppSpecElFunctionElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElAlertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElFunctionElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElCorsElAllowOriginsEl {}

impl BuildAppSpecElFunctionElCorsElAllowOriginsEl {
    pub fn build(self) -> AppSpecElFunctionElCorsElAllowOriginsEl {
        AppSpecElFunctionElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElCorsElAllowOriginsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElCorsElAllowOriginsElRef {
        AppSpecElFunctionElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElFunctionElCorsElDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<std::vec::Vec<AppSpecElFunctionElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElFunctionElCorsElDynamic,
}

impl AppSpecElFunctionElCorsEl {
    #[doc =
        "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElCorsElAllowOriginsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow_origins = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElCorsEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElCorsEl {}

impl BuildAppSpecElFunctionElCorsEl {
    pub fn build(self) -> AppSpecElFunctionElCorsEl {
        AppSpecElFunctionElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElCorsElRef {
        AppSpecElFunctionElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<AppSpecElFunctionElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElFunctionElEnvEl {
    #[doc = "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElEnvEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElEnvEl {}

impl BuildAppSpecElFunctionElEnvEl {
    pub fn build(self) -> AppSpecElFunctionElEnvEl {
        AppSpecElFunctionElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElEnvElRef {
        AppSpecElFunctionElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElFunctionElGitEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElGitEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElGitEl {}

impl BuildAppSpecElFunctionElGitEl {
    pub fn build(self) -> AppSpecElFunctionElGitEl {
        AppSpecElFunctionElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElGitElRef {
        AppSpecElFunctionElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElFunctionElGithubEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElGithubEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElGithubEl {}

impl BuildAppSpecElFunctionElGithubEl {
    pub fn build(self) -> AppSpecElFunctionElGithubEl {
        AppSpecElFunctionElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElGithubElRef {
        AppSpecElFunctionElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElFunctionElGitlabEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElGitlabEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElGitlabEl {}

impl BuildAppSpecElFunctionElGitlabEl {
    pub fn build(self) -> AppSpecElFunctionElGitlabEl {
        AppSpecElFunctionElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElGitlabElRef {
        AppSpecElFunctionElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationElDatadogEl {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElFunctionElLogDestinationElDatadogEl {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationElDatadogEl {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElFunctionElLogDestinationElDatadogEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationElDatadogEl {
        AppSpecElFunctionElLogDestinationElDatadogEl {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElLogDestinationElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElFunctionElLogDestinationElDatadogElRef {
        AppSpecElFunctionElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationElLogtailEl {
    token: terrars::PrimField<std::string::String>,
}

impl AppSpecElFunctionElLogDestinationElLogtailEl { }

impl terrars::ToListMappable for AppSpecElFunctionElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationElLogtailEl {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElFunctionElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationElLogtailEl {
        AppSpecElFunctionElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElFunctionElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElLogDestinationElLogtailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElFunctionElLogDestinationElLogtailElRef {
        AppSpecElFunctionElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationElPapertrailEl {
    endpoint: terrars::PrimField<std::string::String>,
}

impl AppSpecElFunctionElLogDestinationElPapertrailEl { }

impl terrars::ToListMappable for AppSpecElFunctionElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationElPapertrailEl {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElFunctionElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationElPapertrailEl {
        AppSpecElFunctionElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElFunctionElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElFunctionElLogDestinationElPapertrailElRef {
        AppSpecElFunctionElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElFunctionElLogDestinationElDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElLogDestinationElDatadogEl>>,
    logtail: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElLogDestinationElLogtailEl>>,
    papertrail: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<AppSpecElFunctionElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<AppSpecElFunctionElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<AppSpecElFunctionElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElFunctionElLogDestinationElDynamic,
}

impl AppSpecElFunctionElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElLogDestinationElDatadogEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElLogDestinationElLogtailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logtail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.papertrail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElLogDestinationEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationEl {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElFunctionElLogDestinationEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationEl {
        AppSpecElFunctionElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElLogDestinationElRef {
        AppSpecElFunctionElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<AppSpecElFunctionElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<AppSpecElFunctionElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<AppSpecElFunctionElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl AppSpecElFunctionElRoutesEl {
    #[doc =
        "Set the field `path`.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preserve_path_prefix`.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionElRoutesEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionElRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElRoutesEl {}

impl BuildAppSpecElFunctionElRoutesEl {
    pub fn build(self) -> AppSpecElFunctionElRoutesEl {
        AppSpecElFunctionElRoutesEl {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElRoutesElRef {
        AppSpecElFunctionElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElFunctionElDynamic {
    alert: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElAlertEl>>,
    cors: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElCorsEl>>,
    env: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElEnvEl>>,
    git: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElGitEl>>,
    github: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElGithubEl>>,
    gitlab: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElGitlabEl>>,
    log_destination: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElLogDestinationEl>>,
    routes: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionElRoutesEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElFunctionEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<AppSpecElFunctionElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<AppSpecElFunctionElCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<AppSpecElFunctionElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<AppSpecElFunctionElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AppSpecElFunctionElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<AppSpecElFunctionElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<AppSpecElFunctionElLogDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<std::vec::Vec<AppSpecElFunctionElRoutesEl>>,
    dynamic: AppSpecElFunctionElDynamic,
}

impl AppSpecElFunctionEl {
    #[doc = "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElAlertEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElCorsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElEnvEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElGitEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElGitlabEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gitlab = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routes`.\n"]
    pub fn set_routes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionElRoutesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElFunctionEl {
    type O = terrars::BlockAssignable<AppSpecElFunctionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionEl {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElFunctionEl {
    pub fn build(self) -> AppSpecElFunctionEl {
        AppSpecElFunctionEl {
            name: self.name,
            source_dir: core::default::Default::default(),
            alert: core::default::Default::default(),
            cors: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            routes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElFunctionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElFunctionElRef {
        AppSpecElFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<AppSpecElFunctionElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<AppSpecElFunctionElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<AppSpecElFunctionElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AppSpecElFunctionElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<AppSpecElFunctionElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<AppSpecElFunctionElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<AppSpecElFunctionElRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElIngressElRuleElComponentEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rewrite`.\n"]
    pub fn set_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleElComponentEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleElComponentEl {}

impl BuildAppSpecElIngressElRuleElComponentEl {
    pub fn build(self) -> AppSpecElIngressElRuleElComponentEl {
        AppSpecElIngressElRuleElComponentEl {
            name: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
            rewrite: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRuleElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElComponentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRuleElComponentElRef {
        AppSpecElIngressElRuleElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElComponentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElIngressElRuleElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleElCorsElAllowOriginsEl {}

impl BuildAppSpecElIngressElRuleElCorsElAllowOriginsEl {
    pub fn build(self) -> AppSpecElIngressElRuleElCorsElAllowOriginsEl {
        AppSpecElIngressElRuleElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRuleElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElCorsElAllowOriginsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElIngressElRuleElCorsElAllowOriginsElRef {
        AppSpecElIngressElRuleElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElIngressElRuleElCorsElDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElIngressElRuleElCorsElDynamic,
}

impl AppSpecElIngressElRuleElCorsEl {
    #[doc =
        "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleElCorsElAllowOriginsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow_origins = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleElCorsEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleElCorsEl {}

impl BuildAppSpecElIngressElRuleElCorsEl {
    pub fn build(self) -> AppSpecElIngressElRuleElCorsEl {
        AppSpecElIngressElRuleElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRuleElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRuleElCorsElRef {
        AppSpecElIngressElRuleElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<AppSpecElIngressElRuleElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleElMatchElPathEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElIngressElRuleElMatchElPathEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleElMatchElPathEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleElMatchElPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleElMatchElPathEl {}

impl BuildAppSpecElIngressElRuleElMatchElPathEl {
    pub fn build(self) -> AppSpecElIngressElRuleElMatchElPathEl {
        AppSpecElIngressElRuleElMatchElPathEl { prefix: core::default::Default::default() }
    }
}

pub struct AppSpecElIngressElRuleElMatchElPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElMatchElPathElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRuleElMatchElPathElRef {
        AppSpecElIngressElRuleElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElMatchElPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElIngressElRuleElMatchElDynamic {
    path: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleElMatchElPathEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleElMatchElPathEl>>,
    dynamic: AppSpecElIngressElRuleElMatchElDynamic,
}

impl AppSpecElIngressElRuleElMatchEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleElMatchElPathEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleElMatchEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleElMatchEl {}

impl BuildAppSpecElIngressElRuleElMatchEl {
    pub fn build(self) -> AppSpecElIngressElRuleElMatchEl {
        AppSpecElIngressElRuleElMatchEl {
            path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRuleElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRuleElMatchElRef {
        AppSpecElIngressElRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<AppSpecElIngressElRuleElMatchElPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleElRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElIngressElRuleElRedirectEl {
    #[doc = "Set the field `authority`.\n"]
    pub fn set_authority(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.authority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_code`.\n"]
    pub fn set_redirect_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.redirect_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleElRedirectEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleElRedirectEl {}

impl BuildAppSpecElIngressElRuleElRedirectEl {
    pub fn build(self) -> AppSpecElIngressElRuleElRedirectEl {
        AppSpecElIngressElRuleElRedirectEl {
            authority: core::default::Default::default(),
            port: core::default::Default::default(),
            redirect_code: core::default::Default::default(),
            scheme: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRuleElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRuleElRedirectElRef {
        AppSpecElIngressElRuleElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authority", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_code` after provisioning.\n"]
    pub fn redirect_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_code", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElIngressElRuleElDynamic {
    component: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleElComponentEl>>,
    cors: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleElCorsEl>>,
    match_: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleElMatchEl>>,
    redirect: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleElRedirectEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElIngressElRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleElCorsEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleElRedirectEl>>,
    dynamic: AppSpecElIngressElRuleElDynamic,
}

impl AppSpecElIngressElRuleEl {
    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleElComponentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.component = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.component = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleElCorsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleElMatchEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleElRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressElRuleEl {
    type O = terrars::BlockAssignable<AppSpecElIngressElRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressElRuleEl {}

impl BuildAppSpecElIngressElRuleEl {
    pub fn build(self) -> AppSpecElIngressElRuleEl {
        AppSpecElIngressElRuleEl {
            component: core::default::Default::default(),
            cors: core::default::Default::default(),
            match_: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRuleElRef {
        AppSpecElIngressElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<AppSpecElIngressElRuleElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<AppSpecElIngressElRuleElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<AppSpecElIngressElRuleElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<AppSpecElIngressElRuleElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElIngressElDynamic {
    rule: std::option::Option<terrars::DynamicBlock<AppSpecElIngressElRuleEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElIngressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<AppSpecElIngressElRuleEl>>,
    dynamic: AppSpecElIngressElDynamic,
}

impl AppSpecElIngressEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressElRuleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElIngressEl {
    type O = terrars::BlockAssignable<AppSpecElIngressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElIngressEl {}

impl BuildAppSpecElIngressEl {
    pub fn build(self) -> AppSpecElIngressEl {
        AppSpecElIngressEl {
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElIngressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElIngressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElIngressElRef {
        AppSpecElIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElIngressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<AppSpecElIngressElRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
}

impl AppSpecElJobElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElAlertEl {
    type O = terrars::BlockAssignable<AppSpecElJobElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElAlertEl {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobElAlertEl {
    pub fn build(self) -> AppSpecElJobElAlertEl {
        AppSpecElJobElAlertEl {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
        }
    }
}

pub struct AppSpecElJobElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElAlertElRef {
        AppSpecElJobElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElAlertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElJobElEnvEl {
    #[doc = "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElEnvEl {
    type O = terrars::BlockAssignable<AppSpecElJobElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElEnvEl {}

impl BuildAppSpecElJobElEnvEl {
    pub fn build(self) -> AppSpecElJobElEnvEl {
        AppSpecElJobElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElEnvElRef {
        AppSpecElJobElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElJobElGitEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElGitEl {
    type O = terrars::BlockAssignable<AppSpecElJobElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElGitEl {}

impl BuildAppSpecElJobElGitEl {
    pub fn build(self) -> AppSpecElJobElGitEl {
        AppSpecElJobElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElGitElRef {
        AppSpecElJobElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElJobElGithubEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElGithubEl {
    type O = terrars::BlockAssignable<AppSpecElJobElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElGithubEl {}

impl BuildAppSpecElJobElGithubEl {
    pub fn build(self) -> AppSpecElJobElGithubEl {
        AppSpecElJobElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElGithubElRef {
        AppSpecElJobElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElJobElGitlabEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElGitlabEl {
    type O = terrars::BlockAssignable<AppSpecElJobElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElGitlabEl {}

impl BuildAppSpecElJobElGitlabEl {
    pub fn build(self) -> AppSpecElJobElGitlabEl {
        AppSpecElJobElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElGitlabElRef {
        AppSpecElJobElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AppSpecElJobElImageElDeployOnPushEl {
    #[doc = "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElImageElDeployOnPushEl {
    type O = terrars::BlockAssignable<AppSpecElJobElImageElDeployOnPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElImageElDeployOnPushEl {}

impl BuildAppSpecElJobElImageElDeployOnPushEl {
    pub fn build(self) -> AppSpecElJobElImageElDeployOnPushEl {
        AppSpecElJobElImageElDeployOnPushEl { enabled: core::default::Default::default() }
    }
}

pub struct AppSpecElJobElImageElDeployOnPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElImageElDeployOnPushElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElImageElDeployOnPushElRef {
        AppSpecElJobElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElImageElDeployOnPushElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElJobElImageElDynamic {
    deploy_on_push: std::option::Option<terrars::DynamicBlock<AppSpecElJobElImageElDeployOnPushEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElJobElImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_type: terrars::PrimField<std::string::String>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<std::vec::Vec<AppSpecElJobElImageElDeployOnPushEl>>,
    dynamic: AppSpecElJobElImageElDynamic,
}

impl AppSpecElJobElImageEl {
    #[doc = "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElImageElDeployOnPushEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deploy_on_push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_on_push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElImageEl {
    type O = terrars::BlockAssignable<AppSpecElJobElImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElImageEl {
    #[doc = "The registry type."]
    pub registry_type: terrars::PrimField<std::string::String>,
    #[doc = "The repository name."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobElImageEl {
    pub fn build(self) -> AppSpecElJobElImageEl {
        AppSpecElJobElImageEl {
            registry: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElImageElRef {
        AppSpecElJobElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_type` after provisioning.\nThe registry type."]
    pub fn registry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe repository name."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<AppSpecElJobElImageElDeployOnPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationElDatadogEl {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElJobElLogDestinationElDatadogEl {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<AppSpecElJobElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationElDatadogEl {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobElLogDestinationElDatadogEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationElDatadogEl {
        AppSpecElJobElLogDestinationElDatadogEl {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElLogDestinationElDatadogElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElLogDestinationElDatadogElRef {
        AppSpecElJobElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationElLogtailEl {
    token: terrars::PrimField<std::string::String>,
}

impl AppSpecElJobElLogDestinationElLogtailEl { }

impl terrars::ToListMappable for AppSpecElJobElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<AppSpecElJobElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationElLogtailEl {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationElLogtailEl {
        AppSpecElJobElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElJobElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElLogDestinationElLogtailElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElLogDestinationElLogtailElRef {
        AppSpecElJobElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationElPapertrailEl {
    endpoint: terrars::PrimField<std::string::String>,
}

impl AppSpecElJobElLogDestinationElPapertrailEl { }

impl terrars::ToListMappable for AppSpecElJobElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<AppSpecElJobElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationElPapertrailEl {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationElPapertrailEl {
        AppSpecElJobElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElJobElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElLogDestinationElPapertrailElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElLogDestinationElPapertrailElRef {
        AppSpecElJobElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElJobElLogDestinationElDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<AppSpecElJobElLogDestinationElDatadogEl>>,
    logtail: std::option::Option<terrars::DynamicBlock<AppSpecElJobElLogDestinationElLogtailEl>>,
    papertrail: std::option::Option<terrars::DynamicBlock<AppSpecElJobElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<AppSpecElJobElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<AppSpecElJobElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<AppSpecElJobElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElJobElLogDestinationElDynamic,
}

impl AppSpecElJobElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElLogDestinationElDatadogEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElLogDestinationElLogtailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logtail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.papertrail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobElLogDestinationEl {
    type O = terrars::BlockAssignable<AppSpecElJobElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationEl {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobElLogDestinationEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationEl {
        AppSpecElJobElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElLogDestinationElRef {
        AppSpecElJobElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<AppSpecElJobElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<AppSpecElJobElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<AppSpecElJobElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElJobElDynamic {
    alert: std::option::Option<terrars::DynamicBlock<AppSpecElJobElAlertEl>>,
    env: std::option::Option<terrars::DynamicBlock<AppSpecElJobElEnvEl>>,
    git: std::option::Option<terrars::DynamicBlock<AppSpecElJobElGitEl>>,
    github: std::option::Option<terrars::DynamicBlock<AppSpecElJobElGithubEl>>,
    gitlab: std::option::Option<terrars::DynamicBlock<AppSpecElJobElGitlabEl>>,
    image: std::option::Option<terrars::DynamicBlock<AppSpecElJobElImageEl>>,
    log_destination: std::option::Option<terrars::DynamicBlock<AppSpecElJobElLogDestinationEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<AppSpecElJobElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<AppSpecElJobElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<AppSpecElJobElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AppSpecElJobElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<AppSpecElJobElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<AppSpecElJobElImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<AppSpecElJobElLogDestinationEl>>,
    dynamic: AppSpecElJobElDynamic,
}

impl AppSpecElJobEl {
    #[doc =
        "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\nThe amount of instances that this component should be scaled to."]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size_slug`.\nThe instance size to use for this component."]
    pub fn set_instance_size_slug(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_size_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\nThe type of job and when it will be run during the deployment process."]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_command`.\nAn optional run command to override the component's default."]
    pub fn set_run_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElAlertEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElEnvEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElGitEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElGithubEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElGitlabEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gitlab = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElImageEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobElLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElJobEl {
    type O = terrars::BlockAssignable<AppSpecElJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobEl {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElJobEl {
    pub fn build(self) -> AppSpecElJobEl {
        AppSpecElJobEl {
            build_command: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            kind: core::default::Default::default(),
            name: self.name,
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            alert: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            image: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElJobElRef {
        AppSpecElJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_count` after provisioning.\nThe amount of instances that this component should be scaled to."]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_size_slug` after provisioning.\nThe instance size to use for this component."]
    pub fn instance_size_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nThe type of job and when it will be run during the deployment process."]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_command` after provisioning.\nAn optional run command to override the component's default."]
    pub fn run_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<AppSpecElJobElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<AppSpecElJobElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AppSpecElJobElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<AppSpecElJobElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<AppSpecElJobElImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<AppSpecElJobElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
}

impl AppSpecElServiceElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElAlertEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElAlertEl {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceElAlertEl {
    pub fn build(self) -> AppSpecElServiceElAlertEl {
        AppSpecElServiceElAlertEl {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
        }
    }
}

pub struct AppSpecElServiceElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElAlertElRef {
        AppSpecElServiceElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElAlertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElServiceElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElCorsElAllowOriginsEl {}

impl BuildAppSpecElServiceElCorsElAllowOriginsEl {
    pub fn build(self) -> AppSpecElServiceElCorsElAllowOriginsEl {
        AppSpecElServiceElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElCorsElAllowOriginsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElCorsElAllowOriginsElRef {
        AppSpecElServiceElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElCorsElDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<std::vec::Vec<AppSpecElServiceElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElServiceElCorsElDynamic,
}

impl AppSpecElServiceElCorsEl {
    #[doc =
        "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElCorsElAllowOriginsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow_origins = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElCorsEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElCorsEl {}

impl BuildAppSpecElServiceElCorsEl {
    pub fn build(self) -> AppSpecElServiceElCorsEl {
        AppSpecElServiceElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElCorsElRef {
        AppSpecElServiceElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<AppSpecElServiceElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElServiceElEnvEl {
    #[doc = "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElEnvEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElEnvEl {}

impl BuildAppSpecElServiceElEnvEl {
    pub fn build(self) -> AppSpecElServiceElEnvEl {
        AppSpecElServiceElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElEnvElRef {
        AppSpecElServiceElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElServiceElGitEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElGitEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElGitEl {}

impl BuildAppSpecElServiceElGitEl {
    pub fn build(self) -> AppSpecElServiceElGitEl {
        AppSpecElServiceElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElGitElRef {
        AppSpecElServiceElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElServiceElGithubEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElGithubEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElGithubEl {}

impl BuildAppSpecElServiceElGithubEl {
    pub fn build(self) -> AppSpecElServiceElGithubEl {
        AppSpecElServiceElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElGithubElRef {
        AppSpecElServiceElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElServiceElGitlabEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElGitlabEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElGitlabEl {}

impl BuildAppSpecElServiceElGitlabEl {
    pub fn build(self) -> AppSpecElServiceElGitlabEl {
        AppSpecElServiceElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElGitlabElRef {
        AppSpecElServiceElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl AppSpecElServiceElHealthCheckEl {
    #[doc = "Set the field `failure_threshold`.\nThe number of failed health checks before considered unhealthy."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_path`.\nThe route path used for the HTTP health check ping."]
    pub fn set_http_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_delay_seconds`.\nThe number of seconds to wait before beginning health checks."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_seconds`.\nThe number of seconds to wait between health checks."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port on which the health check will be performed. If not set, the health check will be performed on the component's http_port."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_threshold`.\nThe number of successful health checks before considered healthy."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\nThe number of seconds after which the check times out."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElHealthCheckEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElHealthCheckEl {}

impl BuildAppSpecElServiceElHealthCheckEl {
    pub fn build(self) -> AppSpecElServiceElHealthCheckEl {
        AppSpecElServiceElHealthCheckEl {
            failure_threshold: core::default::Default::default(),
            http_path: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            port: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElHealthCheckElRef {
        AppSpecElServiceElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nThe number of failed health checks before considered unhealthy."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_path` after provisioning.\nThe route path used for the HTTP health check ping."]
    pub fn http_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nThe number of seconds to wait before beginning health checks."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nThe number of seconds to wait between health checks."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port on which the health check will be performed. If not set, the health check will be performed on the component's http_port."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nThe number of successful health checks before considered healthy."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nThe number of seconds after which the check times out."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AppSpecElServiceElImageElDeployOnPushEl {
    #[doc = "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElImageElDeployOnPushEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElImageElDeployOnPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElImageElDeployOnPushEl {}

impl BuildAppSpecElServiceElImageElDeployOnPushEl {
    pub fn build(self) -> AppSpecElServiceElImageElDeployOnPushEl {
        AppSpecElServiceElImageElDeployOnPushEl { enabled: core::default::Default::default() }
    }
}

pub struct AppSpecElServiceElImageElDeployOnPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElImageElDeployOnPushElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElImageElDeployOnPushElRef {
        AppSpecElServiceElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElImageElDeployOnPushElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElImageElDynamic {
    deploy_on_push: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElImageElDeployOnPushEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceElImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_type: terrars::PrimField<std::string::String>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<std::vec::Vec<AppSpecElServiceElImageElDeployOnPushEl>>,
    dynamic: AppSpecElServiceElImageElDynamic,
}

impl AppSpecElServiceElImageEl {
    #[doc = "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElImageElDeployOnPushEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deploy_on_push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_on_push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElImageEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElImageEl {
    #[doc = "The registry type."]
    pub registry_type: terrars::PrimField<std::string::String>,
    #[doc = "The repository name."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceElImageEl {
    pub fn build(self) -> AppSpecElServiceElImageEl {
        AppSpecElServiceElImageEl {
            registry: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElImageElRef {
        AppSpecElServiceElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_type` after provisioning.\nThe registry type."]
    pub fn registry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe repository name."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<AppSpecElServiceElImageElDeployOnPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationElDatadogEl {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElServiceElLogDestinationElDatadogEl {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationElDatadogEl {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceElLogDestinationElDatadogEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationElDatadogEl {
        AppSpecElServiceElLogDestinationElDatadogEl {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElLogDestinationElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElServiceElLogDestinationElDatadogElRef {
        AppSpecElServiceElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationElLogtailEl {
    token: terrars::PrimField<std::string::String>,
}

impl AppSpecElServiceElLogDestinationElLogtailEl { }

impl terrars::ToListMappable for AppSpecElServiceElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationElLogtailEl {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationElLogtailEl {
        AppSpecElServiceElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElServiceElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElLogDestinationElLogtailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElServiceElLogDestinationElLogtailElRef {
        AppSpecElServiceElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationElPapertrailEl {
    endpoint: terrars::PrimField<std::string::String>,
}

impl AppSpecElServiceElLogDestinationElPapertrailEl { }

impl terrars::ToListMappable for AppSpecElServiceElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationElPapertrailEl {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationElPapertrailEl {
        AppSpecElServiceElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElServiceElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElServiceElLogDestinationElPapertrailElRef {
        AppSpecElServiceElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElLogDestinationElDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElLogDestinationElDatadogEl>>,
    logtail: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElLogDestinationElLogtailEl>>,
    papertrail: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<AppSpecElServiceElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<AppSpecElServiceElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<AppSpecElServiceElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElServiceElLogDestinationElDynamic,
}

impl AppSpecElServiceElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElLogDestinationElDatadogEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElLogDestinationElLogtailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logtail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.papertrail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElLogDestinationEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationEl {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceElLogDestinationEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationEl {
        AppSpecElServiceElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElLogDestinationElRef {
        AppSpecElServiceElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<AppSpecElServiceElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<AppSpecElServiceElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<AppSpecElServiceElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl AppSpecElServiceElRoutesEl {
    #[doc =
        "Set the field `path`.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preserve_path_prefix`.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceElRoutesEl {
    type O = terrars::BlockAssignable<AppSpecElServiceElRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElRoutesEl {}

impl BuildAppSpecElServiceElRoutesEl {
    pub fn build(self) -> AppSpecElServiceElRoutesEl {
        AppSpecElServiceElRoutesEl {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElRoutesElRef {
        AppSpecElServiceElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElDynamic {
    alert: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElAlertEl>>,
    cors: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElCorsEl>>,
    env: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElEnvEl>>,
    git: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElGitEl>>,
    github: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElGithubEl>>,
    gitlab: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElGitlabEl>>,
    health_check: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElHealthCheckEl>>,
    image: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElImageEl>>,
    log_destination: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElLogDestinationEl>>,
    routes: std::option::Option<terrars::DynamicBlock<AppSpecElServiceElRoutesEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<AppSpecElServiceElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<AppSpecElServiceElCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<AppSpecElServiceElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<AppSpecElServiceElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AppSpecElServiceElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<AppSpecElServiceElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<AppSpecElServiceElHealthCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<AppSpecElServiceElImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<AppSpecElServiceElLogDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<std::vec::Vec<AppSpecElServiceElRoutesEl>>,
    dynamic: AppSpecElServiceElDynamic,
}

impl AppSpecElServiceEl {
    #[doc =
        "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_port`.\nThe internal port on which this service's run command will listen."]
    pub fn set_http_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\nThe amount of instances that this component should be scaled to."]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size_slug`.\nThe instance size to use for this component."]
    pub fn set_instance_size_slug(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_size_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_ports`.\n"]
    pub fn set_internal_ports(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.internal_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_command`.\nAn optional run command to override the component's default."]
    pub fn set_run_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElAlertEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElCorsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElEnvEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElGitEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElGitlabEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gitlab = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElHealthCheckEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElImageEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routes`.\n"]
    pub fn set_routes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceElRoutesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElServiceEl {
    type O = terrars::BlockAssignable<AppSpecElServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceEl {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElServiceEl {
    pub fn build(self) -> AppSpecElServiceEl {
        AppSpecElServiceEl {
            build_command: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            http_port: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            internal_ports: core::default::Default::default(),
            name: self.name,
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            alert: core::default::Default::default(),
            cors: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            health_check: core::default::Default::default(),
            image: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            routes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElServiceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElServiceElRef {
        AppSpecElServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_port` after provisioning.\nThe internal port on which this service's run command will listen."]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_count` after provisioning.\nThe amount of instances that this component should be scaled to."]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_size_slug` after provisioning.\nThe instance size to use for this component."]
    pub fn instance_size_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_ports` after provisioning.\n"]
    pub fn internal_ports(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.internal_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_command` after provisioning.\nAn optional run command to override the component's default."]
    pub fn run_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<AppSpecElServiceElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<AppSpecElServiceElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<AppSpecElServiceElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AppSpecElServiceElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<AppSpecElServiceElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<AppSpecElServiceElHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<AppSpecElServiceElImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<AppSpecElServiceElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<AppSpecElServiceElRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElCorsElAllowOriginsEl {}

impl BuildAppSpecElStaticSiteElCorsElAllowOriginsEl {
    pub fn build(self) -> AppSpecElStaticSiteElCorsElAllowOriginsEl {
        AppSpecElStaticSiteElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElCorsElAllowOriginsElRef {
        AppSpecElStaticSiteElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElStaticSiteElCorsElDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElStaticSiteElCorsElDynamic,
}

impl AppSpecElStaticSiteElCorsEl {
    #[doc =
        "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow_origins = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElCorsEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElCorsEl {}

impl BuildAppSpecElStaticSiteElCorsEl {
    pub fn build(self) -> AppSpecElStaticSiteElCorsEl {
        AppSpecElStaticSiteElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElCorsElRef {
        AppSpecElStaticSiteElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<AppSpecElStaticSiteElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElStaticSiteElEnvEl {
    #[doc = "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElEnvEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElEnvEl {}

impl BuildAppSpecElStaticSiteElEnvEl {
    pub fn build(self) -> AppSpecElStaticSiteElEnvEl {
        AppSpecElStaticSiteElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElEnvElRef {
        AppSpecElStaticSiteElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElStaticSiteElGitEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElGitEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElGitEl {}

impl BuildAppSpecElStaticSiteElGitEl {
    pub fn build(self) -> AppSpecElStaticSiteElGitEl {
        AppSpecElStaticSiteElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElGitElRef {
        AppSpecElStaticSiteElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElStaticSiteElGithubEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElGithubEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElGithubEl {}

impl BuildAppSpecElStaticSiteElGithubEl {
    pub fn build(self) -> AppSpecElStaticSiteElGithubEl {
        AppSpecElStaticSiteElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElGithubElRef {
        AppSpecElStaticSiteElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElStaticSiteElGitlabEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElGitlabEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElGitlabEl {}

impl BuildAppSpecElStaticSiteElGitlabEl {
    pub fn build(self) -> AppSpecElStaticSiteElGitlabEl {
        AppSpecElStaticSiteElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElGitlabElRef {
        AppSpecElStaticSiteElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl AppSpecElStaticSiteElRoutesEl {
    #[doc =
        "Set the field `path`.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preserve_path_prefix`.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteElRoutesEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteElRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteElRoutesEl {}

impl BuildAppSpecElStaticSiteElRoutesEl {
    pub fn build(self) -> AppSpecElStaticSiteElRoutesEl {
        AppSpecElStaticSiteElRoutesEl {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElRoutesElRef {
        AppSpecElStaticSiteElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElStaticSiteElDynamic {
    cors: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElCorsEl>>,
    env: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElEnvEl>>,
    git: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElGitEl>>,
    github: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElGithubEl>>,
    gitlab: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElGitlabEl>>,
    routes: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteElRoutesEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catchall_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_document: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<std::vec::Vec<AppSpecElStaticSiteElRoutesEl>>,
    dynamic: AppSpecElStaticSiteElDynamic,
}

impl AppSpecElStaticSiteEl {
    #[doc =
        "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `catchall_document`.\nThe name of the document to use as the fallback for any requests to documents that are not found when serving this static site."]
    pub fn set_catchall_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catchall_document = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_document`.\nThe name of the error document to use when serving this static site."]
    pub fn set_error_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_document`.\nThe name of the index document to use when serving this static site."]
    pub fn set_index_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_document = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_dir`.\nAn optional path to where the built assets will be located, relative to the build context. If not set, App Platform will automatically scan for these directory names: `_static`, `dist`, `public`."]
    pub fn set_output_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElCorsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElEnvEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElGitEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElGitlabEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gitlab = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routes`.\n"]
    pub fn set_routes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteElRoutesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.routes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.routes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElStaticSiteEl {
    type O = terrars::BlockAssignable<AppSpecElStaticSiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteEl {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElStaticSiteEl {
    pub fn build(self) -> AppSpecElStaticSiteEl {
        AppSpecElStaticSiteEl {
            build_command: core::default::Default::default(),
            catchall_document: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            error_document: core::default::Default::default(),
            index_document: core::default::Default::default(),
            name: self.name,
            output_dir: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            cors: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            routes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElStaticSiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElStaticSiteElRef {
        AppSpecElStaticSiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `catchall_document` after provisioning.\nThe name of the document to use as the fallback for any requests to documents that are not found when serving this static site."]
    pub fn catchall_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catchall_document", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `error_document` after provisioning.\nThe name of the error document to use when serving this static site."]
    pub fn error_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_document", self.base))
    }

    #[doc =
        "Get a reference to the value of field `index_document` after provisioning.\nThe name of the index document to use when serving this static site."]
    pub fn index_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_document", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_dir` after provisioning.\nAn optional path to where the built assets will be located, relative to the build context. If not set, App Platform will automatically scan for these directory names: `_static`, `dist`, `public`."]
    pub fn output_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<AppSpecElStaticSiteElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<AppSpecElStaticSiteElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AppSpecElStaticSiteElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<AppSpecElStaticSiteElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<AppSpecElStaticSiteElRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
}

impl AppSpecElWorkerElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElAlertEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElAlertEl {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerElAlertEl {
    pub fn build(self) -> AppSpecElWorkerElAlertEl {
        AppSpecElWorkerElAlertEl {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
        }
    }
}

pub struct AppSpecElWorkerElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElAlertElRef {
        AppSpecElWorkerElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElAlertElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElWorkerElEnvEl {
    #[doc = "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElEnvEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElEnvEl {}

impl BuildAppSpecElWorkerElEnvEl {
    pub fn build(self) -> AppSpecElWorkerElEnvEl {
        AppSpecElWorkerElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElEnvElRef {
        AppSpecElWorkerElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElWorkerElGitEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElGitEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElGitEl {}

impl BuildAppSpecElWorkerElGitEl {
    pub fn build(self) -> AppSpecElWorkerElGitEl {
        AppSpecElWorkerElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElGitElRef {
        AppSpecElWorkerElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElWorkerElGithubEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElGithubEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElGithubEl {}

impl BuildAppSpecElWorkerElGithubEl {
    pub fn build(self) -> AppSpecElWorkerElGithubEl {
        AppSpecElWorkerElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElGithubElRef {
        AppSpecElWorkerElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElWorkerElGitlabEl {
    #[doc = "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElGitlabEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElGitlabEl {}

impl BuildAppSpecElWorkerElGitlabEl {
    pub fn build(self) -> AppSpecElWorkerElGitlabEl {
        AppSpecElWorkerElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElGitlabElRef {
        AppSpecElWorkerElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl AppSpecElWorkerElImageElDeployOnPushEl {
    #[doc = "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElImageElDeployOnPushEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElImageElDeployOnPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElImageElDeployOnPushEl {}

impl BuildAppSpecElWorkerElImageElDeployOnPushEl {
    pub fn build(self) -> AppSpecElWorkerElImageElDeployOnPushEl {
        AppSpecElWorkerElImageElDeployOnPushEl { enabled: core::default::Default::default() }
    }
}

pub struct AppSpecElWorkerElImageElDeployOnPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElImageElDeployOnPushElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElImageElDeployOnPushElRef {
        AppSpecElWorkerElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElImageElDeployOnPushElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElWorkerElImageElDynamic {
    deploy_on_push: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElImageElDeployOnPushEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_type: terrars::PrimField<std::string::String>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<std::vec::Vec<AppSpecElWorkerElImageElDeployOnPushEl>>,
    dynamic: AppSpecElWorkerElImageElDynamic,
}

impl AppSpecElWorkerElImageEl {
    #[doc = "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElImageElDeployOnPushEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deploy_on_push = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_on_push = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElImageEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElImageEl {
    #[doc = "The registry type."]
    pub registry_type: terrars::PrimField<std::string::String>,
    #[doc = "The repository name."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerElImageEl {
    pub fn build(self) -> AppSpecElWorkerElImageEl {
        AppSpecElWorkerElImageEl {
            registry: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElImageElRef {
        AppSpecElWorkerElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_type` after provisioning.\nThe registry type."]
    pub fn registry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe repository name."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<AppSpecElWorkerElImageElDeployOnPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationElDatadogEl {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppSpecElWorkerElLogDestinationElDatadogEl {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationElDatadogEl {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerElLogDestinationElDatadogEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationElDatadogEl {
        AppSpecElWorkerElLogDestinationElDatadogEl {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElLogDestinationElDatadogElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElLogDestinationElDatadogElRef {
        AppSpecElWorkerElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationElLogtailEl {
    token: terrars::PrimField<std::string::String>,
}

impl AppSpecElWorkerElLogDestinationElLogtailEl { }

impl terrars::ToListMappable for AppSpecElWorkerElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationElLogtailEl {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationElLogtailEl {
        AppSpecElWorkerElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElWorkerElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElLogDestinationElLogtailElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElLogDestinationElLogtailElRef {
        AppSpecElWorkerElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationElPapertrailEl {
    endpoint: terrars::PrimField<std::string::String>,
}

impl AppSpecElWorkerElLogDestinationElPapertrailEl { }

impl terrars::ToListMappable for AppSpecElWorkerElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationElPapertrailEl {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationElPapertrailEl {
        AppSpecElWorkerElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElWorkerElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppSpecElWorkerElLogDestinationElPapertrailElRef {
        AppSpecElWorkerElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElWorkerElLogDestinationElDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElLogDestinationElDatadogEl>>,
    logtail: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElLogDestinationElLogtailEl>>,
    papertrail: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<AppSpecElWorkerElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<AppSpecElWorkerElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<AppSpecElWorkerElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElWorkerElLogDestinationElDynamic,
}

impl AppSpecElWorkerElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElLogDestinationElDatadogEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datadog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElLogDestinationElLogtailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logtail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.papertrail = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerElLogDestinationEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationEl {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerElLogDestinationEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationEl {
        AppSpecElWorkerElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElLogDestinationElRef {
        AppSpecElWorkerElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<AppSpecElWorkerElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<AppSpecElWorkerElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<AppSpecElWorkerElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElWorkerElDynamic {
    alert: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElAlertEl>>,
    env: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElEnvEl>>,
    git: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElGitEl>>,
    github: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElGithubEl>>,
    gitlab: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElGitlabEl>>,
    image: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElImageEl>>,
    log_destination: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerElLogDestinationEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElWorkerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<AppSpecElWorkerElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<AppSpecElWorkerElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<AppSpecElWorkerElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<AppSpecElWorkerElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<AppSpecElWorkerElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<AppSpecElWorkerElImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<AppSpecElWorkerElLogDestinationEl>>,
    dynamic: AppSpecElWorkerElDynamic,
}

impl AppSpecElWorkerEl {
    #[doc =
        "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\nThe amount of instances that this component should be scaled to."]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size_slug`.\nThe instance size to use for this component."]
    pub fn set_instance_size_slug(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_size_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_command`.\nAn optional run command to override the component's default."]
    pub fn set_run_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElAlertEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElEnvEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElGitEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElGithubEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.github = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.github = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElGitlabEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gitlab = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElImageEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerElLogDestinationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecElWorkerEl {
    type O = terrars::BlockAssignable<AppSpecElWorkerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerEl {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecElWorkerEl {
    pub fn build(self) -> AppSpecElWorkerEl {
        AppSpecElWorkerEl {
            build_command: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            name: self.name,
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            alert: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            image: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElWorkerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElWorkerElRef {
        AppSpecElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_count` after provisioning.\nThe amount of instances that this component should be scaled to."]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_size_slug` after provisioning.\nThe instance size to use for this component."]
    pub fn instance_size_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_command` after provisioning.\nAn optional run command to override the component's default."]
    pub fn run_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<AppSpecElWorkerElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<AppSpecElWorkerElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<AppSpecElWorkerElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<AppSpecElWorkerElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<AppSpecElWorkerElImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<AppSpecElWorkerElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElDynamic {
    alert: std::option::Option<terrars::DynamicBlock<AppSpecElAlertEl>>,
    database: std::option::Option<terrars::DynamicBlock<AppSpecElDatabaseEl>>,
    domain: std::option::Option<terrars::DynamicBlock<AppSpecElDomainEl>>,
    env: std::option::Option<terrars::DynamicBlock<AppSpecElEnvEl>>,
    function: std::option::Option<terrars::DynamicBlock<AppSpecElFunctionEl>>,
    ingress: std::option::Option<terrars::DynamicBlock<AppSpecElIngressEl>>,
    job: std::option::Option<terrars::DynamicBlock<AppSpecElJobEl>>,
    service: std::option::Option<terrars::DynamicBlock<AppSpecElServiceEl>>,
    static_site: std::option::Option<terrars::DynamicBlock<AppSpecElStaticSiteEl>>,
    worker: std::option::Option<terrars::DynamicBlock<AppSpecElWorkerEl>>,
}

#[derive(Serialize)]
pub struct AppSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<AppSpecElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<AppSpecElDatabaseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<std::vec::Vec<AppSpecElDomainEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<AppSpecElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<std::vec::Vec<AppSpecElFunctionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<std::vec::Vec<AppSpecElIngressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job: std::option::Option<std::vec::Vec<AppSpecElJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<std::vec::Vec<AppSpecElServiceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_site: std::option::Option<std::vec::Vec<AppSpecElStaticSiteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<std::vec::Vec<AppSpecElWorkerEl>>,
    dynamic: AppSpecElDynamic,
}

impl AppSpecEl {
    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features`.\nList of features which is applied to the app"]
    pub fn set_features(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe slug for the DigitalOcean data center region hosting the app"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElAlertEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElDatabaseEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElDomainEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElEnvEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `function`.\n"]
    pub fn set_function(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElFunctionEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElIngressEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job`.\n"]
    pub fn set_job(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElJobEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElServiceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `static_site`.\n"]
    pub fn set_static_site(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElStaticSiteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_site = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_site = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(mut self, v: impl std::convert::Into<terrars::BlockAssignable<AppSpecElWorkerEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppSpecEl {
    type O = terrars::BlockAssignable<AppSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecEl {
    #[doc = "The name of the app. Must be unique across all apps in the same account."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppSpecEl {
    pub fn build(self) -> AppSpecEl {
        AppSpecEl {
            domains: core::default::Default::default(),
            features: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            alert: core::default::Default::default(),
            database: core::default::Default::default(),
            domain: core::default::Default::default(),
            env: core::default::Default::default(),
            function: core::default::Default::default(),
            ingress: core::default::Default::default(),
            job: core::default::Default::default(),
            service: core::default::Default::default(),
            static_site: core::default::Default::default(),
            worker: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppSpecElRef {
        AppSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `features` after provisioning.\nList of features which is applied to the app"]
    pub fn features(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.features", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the app. Must be unique across all apps in the same account."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe slug for the DigitalOcean data center region hosting the app"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<AppSpecElDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::ListRef<AppSpecElDomainElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::ListRef<AppSpecElFunctionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::ListRef<AppSpecElIngressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }

    #[doc = "Get a reference to the value of field `job` after provisioning.\n"]
    pub fn job(&self) -> terrars::ListRef<AppSpecElJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::ListRef<AppSpecElServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `static_site` after provisioning.\n"]
    pub fn static_site(&self) -> terrars::ListRef<AppSpecElStaticSiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_site", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<AppSpecElWorkerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct AppTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppTimeoutsEl {
    type O = terrars::BlockAssignable<AppTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppTimeoutsEl {}

impl BuildAppTimeoutsEl {
    pub fn build(self) -> AppTimeoutsEl {
        AppTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct AppTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppTimeoutsElRef {
        AppTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppSpecEl>>,
}
