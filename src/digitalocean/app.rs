use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AppSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppTimeoutsEl>,
    dynamic: AppDynamic,
}

struct App_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppData>,
}

#[derive(Clone)]
pub struct App(Rc<App_>);

impl App {
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

    #[doc= "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<AppSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date and time of when the App was created"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_ingress` after provisioning.\nThe default URL to access the App"]
    pub fn default_ingress(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ingress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `live_url` after provisioning.\n"]
    pub fn live_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.live_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time of when the App was last updated"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource identifier for the app"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppTimeoutsElRef {
        AppTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for App {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for App { }

impl ToListMappable for App {
    type O = ListRef<AppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for App_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApp {
    pub tf_id: String,
}

impl BuildApp {
    pub fn build(self, stack: &mut Stack) -> App {
        let out = App(Rc::new(App_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nThe date and time of when the App was created"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_ingress` after provisioning.\nThe default URL to access the App"]
    pub fn default_ingress(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ingress", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `live_url` after provisioning.\n"]
    pub fn live_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.live_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time of when the App was last updated"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource identifier for the app"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppTimeoutsElRef {
        AppTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppSpecElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    rule: PrimField<String>,
}

impl AppSpecElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElAlertEl {
    type O = BlockAssignable<AppSpecElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElAlertEl {
    #[doc= ""]
    pub rule: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElAlertElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElAlertElRef {
        AppSpecElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElAlertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    production: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl AppSpecElDatabaseEl {
    #[doc= "Set the field `cluster_name`.\nThe name of the underlying DigitalOcean DBaaS cluster. This is required for production databases. For dev databases, if cluster_name is not set, a new cluster will be provisioned."]
    pub fn set_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_name = Some(v.into());
        self
    }

    #[doc= "Set the field `db_name`.\nThe name of the MySQL or PostgreSQL database to configure."]
    pub fn set_db_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_name = Some(v.into());
        self
    }

    #[doc= "Set the field `db_user`.\nThe name of the MySQL or PostgreSQL user to configure."]
    pub fn set_db_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_user = Some(v.into());
        self
    }

    #[doc= "Set the field `engine`.\nThe database engine to use."]
    pub fn set_engine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the component"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `production`.\nWhether this is a production or dev database."]
    pub fn set_production(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.production = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\nThe version of the database engine."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElDatabaseEl {
    type O = BlockAssignable<AppSpecElDatabaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElDatabaseElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElDatabaseElRef {
        AppSpecElDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElDatabaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the underlying DigitalOcean DBaaS cluster. This is required for production databases. For dev databases, if cluster_name is not set, a new cluster will be provisioned."]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc= "Get a reference to the value of field `db_name` after provisioning.\nThe name of the MySQL or PostgreSQL database to configure."]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc= "Get a reference to the value of field `db_user` after provisioning.\nThe name of the MySQL or PostgreSQL user to configure."]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\nThe database engine to use."]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `production` after provisioning.\nWhether this is a production or dev database."]
    pub fn production(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.production", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nThe version of the database engine."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElDomainEl {
    name: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}

impl AppSpecElDomainEl {
    #[doc= "Set the field `type_`.\nThe type of the domain."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `wildcard`.\nIndicates whether the domain includes all sub-domains, in addition to the given domain."]
    pub fn set_wildcard(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wildcard = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nIf the domain uses DigitalOcean DNS and you would like App Platform to automatically manage it for you, set this to the name of the domain on your account."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElDomainEl {
    type O = BlockAssignable<AppSpecElDomainEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElDomainEl {
    #[doc= "The hostname for the domain."]
    pub name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElDomainElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElDomainElRef {
        AppSpecElDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElDomainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe hostname for the domain."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the domain."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `wildcard` after provisioning.\nIndicates whether the domain includes all sub-domains, in addition to the given domain."]
    pub fn wildcard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nIf the domain uses DigitalOcean DNS and you would like App Platform to automatically manage it for you, set this to the name of the domain on your account."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppSpecElEnvEl {
    #[doc= "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElEnvEl {
    type O = BlockAssignable<AppSpecElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElEnvElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElEnvElRef {
        AppSpecElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    operator: PrimField<String>,
    rule: PrimField<String>,
    value: PrimField<f64>,
    window: PrimField<String>,
}

impl AppSpecElFunctionElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElAlertEl {
    type O = BlockAssignable<AppSpecElFunctionElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElAlertEl {
    #[doc= ""]
    pub operator: PrimField<String>,
    #[doc= ""]
    pub rule: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<f64>,
    #[doc= ""]
    pub window: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElAlertElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElAlertElRef {
        AppSpecElFunctionElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElAlertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl AppSpecElFunctionElCorsElAllowOriginsEl {
    #[doc= "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElCorsElAllowOriginsEl {
    type O = BlockAssignable<AppSpecElFunctionElCorsElAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElCorsElAllowOriginsElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElCorsElAllowOriginsElRef {
        AppSpecElFunctionElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElCorsElAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElFunctionElCorsElDynamic {
    allow_origins: Option<DynamicBlock<AppSpecElFunctionElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<Vec<AppSpecElFunctionElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElFunctionElCorsElDynamic,
}

impl AppSpecElFunctionElCorsEl {
    #[doc= "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElCorsElAllowOriginsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow_origins = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElFunctionElCorsEl {
    type O = BlockAssignable<AppSpecElFunctionElCorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElCorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElCorsElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElCorsElRef {
        AppSpecElFunctionElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> ListRef<AppSpecElFunctionElCorsElAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppSpecElFunctionElEnvEl {
    #[doc= "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElEnvEl {
    type O = BlockAssignable<AppSpecElFunctionElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElEnvElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElEnvElRef {
        AppSpecElFunctionElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl AppSpecElFunctionElGitEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElGitEl {
    type O = BlockAssignable<AppSpecElFunctionElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElGitElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElGitElRef {
        AppSpecElFunctionElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElFunctionElGithubEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElGithubEl {
    type O = BlockAssignable<AppSpecElFunctionElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElGithubElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElGithubElRef {
        AppSpecElFunctionElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElFunctionElGitlabEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElGitlabEl {
    type O = BlockAssignable<AppSpecElFunctionElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElGitlabElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElGitlabElRef {
        AppSpecElFunctionElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationElDatadogEl {
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl AppSpecElFunctionElLogDestinationElDatadogEl {
    #[doc= "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElLogDestinationElDatadogEl {
    type O = BlockAssignable<AppSpecElFunctionElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationElDatadogEl {
    #[doc= "Datadog API key."]
    pub api_key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElLogDestinationElDatadogElRef {
        AppSpecElFunctionElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationElLogtailEl {
    token: PrimField<String>,
}

impl AppSpecElFunctionElLogDestinationElLogtailEl { }

impl ToListMappable for AppSpecElFunctionElLogDestinationElLogtailEl {
    type O = BlockAssignable<AppSpecElFunctionElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationElLogtailEl {
    #[doc= "Logtail token."]
    pub token: PrimField<String>,
}

impl BuildAppSpecElFunctionElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationElLogtailEl {
        AppSpecElFunctionElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElFunctionElLogDestinationElLogtailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElLogDestinationElLogtailElRef {
        AppSpecElFunctionElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationElPapertrailEl {
    endpoint: PrimField<String>,
}

impl AppSpecElFunctionElLogDestinationElPapertrailEl { }

impl ToListMappable for AppSpecElFunctionElLogDestinationElPapertrailEl {
    type O = BlockAssignable<AppSpecElFunctionElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationElPapertrailEl {
    #[doc= "Papertrail syslog endpoint."]
    pub endpoint: PrimField<String>,
}

impl BuildAppSpecElFunctionElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationElPapertrailEl {
        AppSpecElFunctionElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElFunctionElLogDestinationElPapertrailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElLogDestinationElPapertrailElRef {
        AppSpecElFunctionElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElFunctionElLogDestinationElDynamic {
    datadog: Option<DynamicBlock<AppSpecElFunctionElLogDestinationElDatadogEl>>,
    logtail: Option<DynamicBlock<AppSpecElFunctionElLogDestinationElLogtailEl>>,
    papertrail: Option<DynamicBlock<AppSpecElFunctionElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElLogDestinationEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<Vec<AppSpecElFunctionElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<Vec<AppSpecElFunctionElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<Vec<AppSpecElFunctionElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElFunctionElLogDestinationElDynamic,
}

impl AppSpecElFunctionElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElLogDestinationElDatadogEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElLogDestinationElLogtailEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logtail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl Into<BlockAssignable<AppSpecElFunctionElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.papertrail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElFunctionElLogDestinationEl {
    type O = BlockAssignable<AppSpecElFunctionElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionElLogDestinationEl {
    #[doc= "Name of the log destination"]
    pub name: PrimField<String>,
}

impl BuildAppSpecElFunctionElLogDestinationEl {
    pub fn build(self) -> AppSpecElFunctionElLogDestinationEl {
        AppSpecElFunctionElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElLogDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElLogDestinationElRef {
        AppSpecElFunctionElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<AppSpecElFunctionElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<AppSpecElFunctionElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<AppSpecElFunctionElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElFunctionElRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<PrimField<bool>>,
}

impl AppSpecElFunctionElRoutesEl {
    #[doc= "Set the field `path`.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_prefix`.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn set_preserve_path_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_path_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElFunctionElRoutesEl {
    type O = BlockAssignable<AppSpecElFunctionElRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElRoutesElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElRoutesElRef {
        AppSpecElFunctionElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn preserve_path_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElFunctionElDynamic {
    alert: Option<DynamicBlock<AppSpecElFunctionElAlertEl>>,
    cors: Option<DynamicBlock<AppSpecElFunctionElCorsEl>>,
    env: Option<DynamicBlock<AppSpecElFunctionElEnvEl>>,
    git: Option<DynamicBlock<AppSpecElFunctionElGitEl>>,
    github: Option<DynamicBlock<AppSpecElFunctionElGithubEl>>,
    gitlab: Option<DynamicBlock<AppSpecElFunctionElGitlabEl>>,
    log_destination: Option<DynamicBlock<AppSpecElFunctionElLogDestinationEl>>,
    routes: Option<DynamicBlock<AppSpecElFunctionElRoutesEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElFunctionEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Vec<AppSpecElFunctionElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<Vec<AppSpecElFunctionElCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<AppSpecElFunctionElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<AppSpecElFunctionElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AppSpecElFunctionElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<Vec<AppSpecElFunctionElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<Vec<AppSpecElFunctionElLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<Vec<AppSpecElFunctionElRoutesEl>>,
    dynamic: AppSpecElFunctionElDynamic,
}

impl AppSpecElFunctionEl {
    #[doc= "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElAlertEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alert = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElCorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElEnvEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElGitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElGitlabEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gitlab = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElLogDestinationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionElRoutesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.routes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.routes = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElFunctionEl {
    type O = BlockAssignable<AppSpecElFunctionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElFunctionEl {
    #[doc= "The name of the component"]
    pub name: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElFunctionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElFunctionElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElFunctionElRef {
        AppSpecElFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<AppSpecElFunctionElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<AppSpecElFunctionElCorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<AppSpecElFunctionElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AppSpecElFunctionElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<AppSpecElFunctionElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<AppSpecElFunctionElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<AppSpecElFunctionElRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    operator: PrimField<String>,
    rule: PrimField<String>,
    value: PrimField<f64>,
    window: PrimField<String>,
}

impl AppSpecElJobElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElAlertEl {
    type O = BlockAssignable<AppSpecElJobElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElAlertEl {
    #[doc= ""]
    pub operator: PrimField<String>,
    #[doc= ""]
    pub rule: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<f64>,
    #[doc= ""]
    pub window: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElAlertElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElAlertElRef {
        AppSpecElJobElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElAlertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppSpecElJobElEnvEl {
    #[doc= "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElEnvEl {
    type O = BlockAssignable<AppSpecElJobElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElEnvElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElEnvElRef {
        AppSpecElJobElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl AppSpecElJobElGitEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElGitEl {
    type O = BlockAssignable<AppSpecElJobElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElGitElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElGitElRef {
        AppSpecElJobElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElJobElGithubEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElGithubEl {
    type O = BlockAssignable<AppSpecElJobElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElGithubElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElGithubElRef {
        AppSpecElJobElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElJobElGitlabEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElGitlabEl {
    type O = BlockAssignable<AppSpecElJobElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElGitlabElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElGitlabElRef {
        AppSpecElJobElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl AppSpecElJobElImageElDeployOnPushEl {
    #[doc= "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElImageElDeployOnPushEl {
    type O = BlockAssignable<AppSpecElJobElImageElDeployOnPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElImageElDeployOnPushElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElImageElDeployOnPushElRef {
        AppSpecElJobElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElImageElDeployOnPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElJobElImageElDynamic {
    deploy_on_push: Option<DynamicBlock<AppSpecElJobElImageElDeployOnPushEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElJobElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
    registry_type: PrimField<String>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<Vec<AppSpecElJobElImageElDeployOnPushEl>>,
    dynamic: AppSpecElJobElImageElDynamic,
}

impl AppSpecElJobElImageEl {
    #[doc= "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<BlockAssignable<AppSpecElJobElImageElDeployOnPushEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deploy_on_push = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_on_push = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElJobElImageEl {
    type O = BlockAssignable<AppSpecElJobElImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElImageEl {
    #[doc= "The registry type."]
    pub registry_type: PrimField<String>,
    #[doc= "The repository name."]
    pub repository: PrimField<String>,
}

impl BuildAppSpecElJobElImageEl {
    pub fn build(self) -> AppSpecElJobElImageEl {
        AppSpecElJobElImageEl {
            registry: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElJobElImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElImageElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElImageElRef {
        AppSpecElJobElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_type` after provisioning.\nThe registry type."]
    pub fn registry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> ListRef<AppSpecElJobElImageElDeployOnPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationElDatadogEl {
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl AppSpecElJobElLogDestinationElDatadogEl {
    #[doc= "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElJobElLogDestinationElDatadogEl {
    type O = BlockAssignable<AppSpecElJobElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationElDatadogEl {
    #[doc= "Datadog API key."]
    pub api_key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElLogDestinationElDatadogElRef {
        AppSpecElJobElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationElLogtailEl {
    token: PrimField<String>,
}

impl AppSpecElJobElLogDestinationElLogtailEl { }

impl ToListMappable for AppSpecElJobElLogDestinationElLogtailEl {
    type O = BlockAssignable<AppSpecElJobElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationElLogtailEl {
    #[doc= "Logtail token."]
    pub token: PrimField<String>,
}

impl BuildAppSpecElJobElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationElLogtailEl {
        AppSpecElJobElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElJobElLogDestinationElLogtailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElLogDestinationElLogtailElRef {
        AppSpecElJobElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationElPapertrailEl {
    endpoint: PrimField<String>,
}

impl AppSpecElJobElLogDestinationElPapertrailEl { }

impl ToListMappable for AppSpecElJobElLogDestinationElPapertrailEl {
    type O = BlockAssignable<AppSpecElJobElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationElPapertrailEl {
    #[doc= "Papertrail syslog endpoint."]
    pub endpoint: PrimField<String>,
}

impl BuildAppSpecElJobElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationElPapertrailEl {
        AppSpecElJobElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElJobElLogDestinationElPapertrailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElLogDestinationElPapertrailElRef {
        AppSpecElJobElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElJobElLogDestinationElDynamic {
    datadog: Option<DynamicBlock<AppSpecElJobElLogDestinationElDatadogEl>>,
    logtail: Option<DynamicBlock<AppSpecElJobElLogDestinationElLogtailEl>>,
    papertrail: Option<DynamicBlock<AppSpecElJobElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElJobElLogDestinationEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<Vec<AppSpecElJobElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<Vec<AppSpecElJobElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<Vec<AppSpecElJobElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElJobElLogDestinationElDynamic,
}

impl AppSpecElJobElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<BlockAssignable<AppSpecElJobElLogDestinationElDatadogEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<BlockAssignable<AppSpecElJobElLogDestinationElLogtailEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logtail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(mut self, v: impl Into<BlockAssignable<AppSpecElJobElLogDestinationElPapertrailEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.papertrail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElJobElLogDestinationEl {
    type O = BlockAssignable<AppSpecElJobElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobElLogDestinationEl {
    #[doc= "Name of the log destination"]
    pub name: PrimField<String>,
}

impl BuildAppSpecElJobElLogDestinationEl {
    pub fn build(self) -> AppSpecElJobElLogDestinationEl {
        AppSpecElJobElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElJobElLogDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElLogDestinationElRef {
        AppSpecElJobElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<AppSpecElJobElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<AppSpecElJobElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<AppSpecElJobElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElJobElDynamic {
    alert: Option<DynamicBlock<AppSpecElJobElAlertEl>>,
    env: Option<DynamicBlock<AppSpecElJobElEnvEl>>,
    git: Option<DynamicBlock<AppSpecElJobElGitEl>>,
    github: Option<DynamicBlock<AppSpecElJobElGithubEl>>,
    gitlab: Option<DynamicBlock<AppSpecElJobElGitlabEl>>,
    image: Option<DynamicBlock<AppSpecElJobElImageEl>>,
    log_destination: Option<DynamicBlock<AppSpecElJobElLogDestinationEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElJobEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Vec<AppSpecElJobElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<AppSpecElJobElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<AppSpecElJobElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AppSpecElJobElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<Vec<AppSpecElJobElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<AppSpecElJobElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<Vec<AppSpecElJobElLogDestinationEl>>,
    dynamic: AppSpecElJobElDynamic,
}

impl AppSpecElJobEl {
    #[doc= "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\nThe amount of instances that this component should be scaled to."]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_size_slug`.\nThe instance size to use for this component."]
    pub fn set_instance_size_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `kind`.\nThe type of job and when it will be run during the deployment process."]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }

    #[doc= "Set the field `run_command`.\nAn optional run command to override the component's default."]
    pub fn set_run_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.run_command = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<BlockAssignable<AppSpecElJobElAlertEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alert = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<BlockAssignable<AppSpecElJobElEnvEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<BlockAssignable<AppSpecElJobElGitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AppSpecElJobElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<BlockAssignable<AppSpecElJobElGitlabEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gitlab = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<BlockAssignable<AppSpecElJobElImageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<BlockAssignable<AppSpecElJobElLogDestinationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElJobEl {
    type O = BlockAssignable<AppSpecElJobEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElJobEl {
    #[doc= "The name of the component"]
    pub name: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElJobElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElJobElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElJobElRef {
        AppSpecElJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\nThe amount of instances that this component should be scaled to."]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_size_slug` after provisioning.\nThe instance size to use for this component."]
    pub fn instance_size_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nThe type of job and when it will be run during the deployment process."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `run_command` after provisioning.\nAn optional run command to override the component's default."]
    pub fn run_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<AppSpecElJobElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<AppSpecElJobElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AppSpecElJobElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<AppSpecElJobElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<AppSpecElJobElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<AppSpecElJobElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    operator: PrimField<String>,
    rule: PrimField<String>,
    value: PrimField<f64>,
    window: PrimField<String>,
}

impl AppSpecElServiceElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElAlertEl {
    type O = BlockAssignable<AppSpecElServiceElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElAlertEl {
    #[doc= ""]
    pub operator: PrimField<String>,
    #[doc= ""]
    pub rule: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<f64>,
    #[doc= ""]
    pub window: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElAlertElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElAlertElRef {
        AppSpecElServiceElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElAlertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl AppSpecElServiceElCorsElAllowOriginsEl {
    #[doc= "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElCorsElAllowOriginsEl {
    type O = BlockAssignable<AppSpecElServiceElCorsElAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElCorsElAllowOriginsElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElCorsElAllowOriginsElRef {
        AppSpecElServiceElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElCorsElAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElCorsElDynamic {
    allow_origins: Option<DynamicBlock<AppSpecElServiceElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceElCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<Vec<AppSpecElServiceElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElServiceElCorsElDynamic,
}

impl AppSpecElServiceElCorsEl {
    #[doc= "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElCorsElAllowOriginsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow_origins = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElServiceElCorsEl {
    type O = BlockAssignable<AppSpecElServiceElCorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElServiceElCorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElCorsElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElCorsElRef {
        AppSpecElServiceElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> ListRef<AppSpecElServiceElCorsElAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppSpecElServiceElEnvEl {
    #[doc= "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElEnvEl {
    type O = BlockAssignable<AppSpecElServiceElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElEnvElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElEnvElRef {
        AppSpecElServiceElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl AppSpecElServiceElGitEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElGitEl {
    type O = BlockAssignable<AppSpecElServiceElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElGitElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElGitElRef {
        AppSpecElServiceElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElServiceElGithubEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElGithubEl {
    type O = BlockAssignable<AppSpecElServiceElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElGithubElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElGithubElRef {
        AppSpecElServiceElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElServiceElGitlabEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElGitlabEl {
    type O = BlockAssignable<AppSpecElServiceElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElGitlabElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElGitlabElRef {
        AppSpecElServiceElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_delay_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
}

impl AppSpecElServiceElHealthCheckEl {
    #[doc= "Set the field `failure_threshold`.\nThe number of failed health checks before considered unhealthy."]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `http_path`.\nThe route path used for the HTTP health check ping."]
    pub fn set_http_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_path = Some(v.into());
        self
    }

    #[doc= "Set the field `initial_delay_seconds`.\nThe number of seconds to wait before beginning health checks."]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `period_seconds`.\nThe number of seconds to wait between health checks."]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `success_threshold`.\nThe number of successful health checks before considered healthy."]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout_seconds`.\nThe number of seconds after which the check times out."]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElHealthCheckEl {
    type O = BlockAssignable<AppSpecElServiceElHealthCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct AppSpecElServiceElHealthCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElHealthCheckElRef {
        AppSpecElServiceElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `failure_threshold` after provisioning.\nThe number of failed health checks before considered unhealthy."]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `http_path` after provisioning.\nThe route path used for the HTTP health check ping."]
    pub fn http_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_path", self.base))
    }

    #[doc= "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nThe number of seconds to wait before beginning health checks."]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `period_seconds` after provisioning.\nThe number of seconds to wait between health checks."]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `success_threshold` after provisioning.\nThe number of successful health checks before considered healthy."]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout_seconds` after provisioning.\nThe number of seconds after which the check times out."]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl AppSpecElServiceElImageElDeployOnPushEl {
    #[doc= "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElImageElDeployOnPushEl {
    type O = BlockAssignable<AppSpecElServiceElImageElDeployOnPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElImageElDeployOnPushElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElImageElDeployOnPushElRef {
        AppSpecElServiceElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElImageElDeployOnPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElImageElDynamic {
    deploy_on_push: Option<DynamicBlock<AppSpecElServiceElImageElDeployOnPushEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
    registry_type: PrimField<String>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<Vec<AppSpecElServiceElImageElDeployOnPushEl>>,
    dynamic: AppSpecElServiceElImageElDynamic,
}

impl AppSpecElServiceElImageEl {
    #[doc= "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElImageElDeployOnPushEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deploy_on_push = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_on_push = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElServiceElImageEl {
    type O = BlockAssignable<AppSpecElServiceElImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElImageEl {
    #[doc= "The registry type."]
    pub registry_type: PrimField<String>,
    #[doc= "The repository name."]
    pub repository: PrimField<String>,
}

impl BuildAppSpecElServiceElImageEl {
    pub fn build(self) -> AppSpecElServiceElImageEl {
        AppSpecElServiceElImageEl {
            registry: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElServiceElImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElImageElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElImageElRef {
        AppSpecElServiceElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_type` after provisioning.\nThe registry type."]
    pub fn registry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> ListRef<AppSpecElServiceElImageElDeployOnPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationElDatadogEl {
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl AppSpecElServiceElLogDestinationElDatadogEl {
    #[doc= "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElLogDestinationElDatadogEl {
    type O = BlockAssignable<AppSpecElServiceElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationElDatadogEl {
    #[doc= "Datadog API key."]
    pub api_key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElLogDestinationElDatadogElRef {
        AppSpecElServiceElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationElLogtailEl {
    token: PrimField<String>,
}

impl AppSpecElServiceElLogDestinationElLogtailEl { }

impl ToListMappable for AppSpecElServiceElLogDestinationElLogtailEl {
    type O = BlockAssignable<AppSpecElServiceElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationElLogtailEl {
    #[doc= "Logtail token."]
    pub token: PrimField<String>,
}

impl BuildAppSpecElServiceElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationElLogtailEl {
        AppSpecElServiceElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElServiceElLogDestinationElLogtailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElLogDestinationElLogtailElRef {
        AppSpecElServiceElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationElPapertrailEl {
    endpoint: PrimField<String>,
}

impl AppSpecElServiceElLogDestinationElPapertrailEl { }

impl ToListMappable for AppSpecElServiceElLogDestinationElPapertrailEl {
    type O = BlockAssignable<AppSpecElServiceElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationElPapertrailEl {
    #[doc= "Papertrail syslog endpoint."]
    pub endpoint: PrimField<String>,
}

impl BuildAppSpecElServiceElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationElPapertrailEl {
        AppSpecElServiceElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElServiceElLogDestinationElPapertrailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElLogDestinationElPapertrailElRef {
        AppSpecElServiceElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElLogDestinationElDynamic {
    datadog: Option<DynamicBlock<AppSpecElServiceElLogDestinationElDatadogEl>>,
    logtail: Option<DynamicBlock<AppSpecElServiceElLogDestinationElLogtailEl>>,
    papertrail: Option<DynamicBlock<AppSpecElServiceElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceElLogDestinationEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<Vec<AppSpecElServiceElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<Vec<AppSpecElServiceElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<Vec<AppSpecElServiceElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElServiceElLogDestinationElDynamic,
}

impl AppSpecElServiceElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElLogDestinationElDatadogEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElLogDestinationElLogtailEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logtail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl Into<BlockAssignable<AppSpecElServiceElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.papertrail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElServiceElLogDestinationEl {
    type O = BlockAssignable<AppSpecElServiceElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceElLogDestinationEl {
    #[doc= "Name of the log destination"]
    pub name: PrimField<String>,
}

impl BuildAppSpecElServiceElLogDestinationEl {
    pub fn build(self) -> AppSpecElServiceElLogDestinationEl {
        AppSpecElServiceElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElServiceElLogDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElLogDestinationElRef {
        AppSpecElServiceElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<AppSpecElServiceElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<AppSpecElServiceElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<AppSpecElServiceElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElServiceElRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<PrimField<bool>>,
}

impl AppSpecElServiceElRoutesEl {
    #[doc= "Set the field `path`.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_prefix`.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn set_preserve_path_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_path_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElServiceElRoutesEl {
    type O = BlockAssignable<AppSpecElServiceElRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElRoutesElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElRoutesElRef {
        AppSpecElServiceElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn preserve_path_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElServiceElDynamic {
    alert: Option<DynamicBlock<AppSpecElServiceElAlertEl>>,
    cors: Option<DynamicBlock<AppSpecElServiceElCorsEl>>,
    env: Option<DynamicBlock<AppSpecElServiceElEnvEl>>,
    git: Option<DynamicBlock<AppSpecElServiceElGitEl>>,
    github: Option<DynamicBlock<AppSpecElServiceElGithubEl>>,
    gitlab: Option<DynamicBlock<AppSpecElServiceElGitlabEl>>,
    health_check: Option<DynamicBlock<AppSpecElServiceElHealthCheckEl>>,
    image: Option<DynamicBlock<AppSpecElServiceElImageEl>>,
    log_destination: Option<DynamicBlock<AppSpecElServiceElLogDestinationEl>>,
    routes: Option<DynamicBlock<AppSpecElServiceElRoutesEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ports: Option<SetField<PrimField<f64>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Vec<AppSpecElServiceElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<Vec<AppSpecElServiceElCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<AppSpecElServiceElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<AppSpecElServiceElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AppSpecElServiceElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<Vec<AppSpecElServiceElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<Vec<AppSpecElServiceElHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<AppSpecElServiceElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<Vec<AppSpecElServiceElLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<Vec<AppSpecElServiceElRoutesEl>>,
    dynamic: AppSpecElServiceElDynamic,
}

impl AppSpecElServiceEl {
    #[doc= "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `http_port`.\nThe internal port on which this service's run command will listen."]
    pub fn set_http_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_port = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\nThe amount of instances that this component should be scaled to."]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_size_slug`.\nThe instance size to use for this component."]
    pub fn set_instance_size_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ports`.\n"]
    pub fn set_internal_ports(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.internal_ports = Some(v.into());
        self
    }

    #[doc= "Set the field `run_command`.\nAn optional run command to override the component's default."]
    pub fn set_run_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.run_command = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElAlertEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alert = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElCorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElEnvEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElGitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElGitlabEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gitlab = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElHealthCheckEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_check = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElImageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElLogDestinationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl Into<BlockAssignable<AppSpecElServiceElRoutesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.routes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.routes = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElServiceEl {
    type O = BlockAssignable<AppSpecElServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElServiceEl {
    #[doc= "The name of the component"]
    pub name: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElServiceElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElServiceElRef {
        AppSpecElServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `http_port` after provisioning.\nThe internal port on which this service's run command will listen."]
    pub fn http_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\nThe amount of instances that this component should be scaled to."]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_size_slug` after provisioning.\nThe instance size to use for this component."]
    pub fn instance_size_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `internal_ports` after provisioning.\n"]
    pub fn internal_ports(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.internal_ports", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `run_command` after provisioning.\nAn optional run command to override the component's default."]
    pub fn run_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<AppSpecElServiceElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<AppSpecElServiceElCorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<AppSpecElServiceElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AppSpecElServiceElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<AppSpecElServiceElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<AppSpecElServiceElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<AppSpecElServiceElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<AppSpecElServiceElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<AppSpecElServiceElRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl AppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[doc= "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\nPrefix-based match. "]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\nRE2 style regex-based match."]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElCorsElAllowOriginsEl {
    type O = BlockAssignable<AppSpecElStaticSiteElCorsElAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElCorsElAllowOriginsElRef {
        AppSpecElStaticSiteElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match. "]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElStaticSiteElCorsElDynamic {
    allow_origins: Option<DynamicBlock<AppSpecElStaticSiteElCorsElAllowOriginsEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<Vec<AppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    dynamic: AppSpecElStaticSiteElCorsElDynamic,
}

impl AppSpecElStaticSiteElCorsEl {
    #[doc= "Set the field `allow_credentials`.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl Into<BlockAssignable<AppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.allow_origins = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_origins = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElCorsEl {
    type O = BlockAssignable<AppSpecElStaticSiteElCorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElCorsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElCorsElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElCorsElRef {
        AppSpecElStaticSiteElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nWhether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is `include`. This configures the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nThe set of allowed HTTP request headers. This configures the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nThe set of allowed HTTP methods. This configures the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nThe set of HTTP response headers that browsers are allowed to access. This configures the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nAn optional duration specifying how long browsers can cache the results of a preflight request. This configures the Access-Control-Max-Age header. Example: `5h30m`."]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> ListRef<AppSpecElStaticSiteElCorsElAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppSpecElStaticSiteElEnvEl {
    #[doc= "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElEnvEl {
    type O = BlockAssignable<AppSpecElStaticSiteElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElEnvElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElEnvElRef {
        AppSpecElStaticSiteElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl AppSpecElStaticSiteElGitEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElGitEl {
    type O = BlockAssignable<AppSpecElStaticSiteElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElGitElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElGitElRef {
        AppSpecElStaticSiteElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElStaticSiteElGithubEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElGithubEl {
    type O = BlockAssignable<AppSpecElStaticSiteElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElGithubElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElGithubElRef {
        AppSpecElStaticSiteElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElStaticSiteElGitlabEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElGitlabEl {
    type O = BlockAssignable<AppSpecElStaticSiteElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElGitlabElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElGitlabElRef {
        AppSpecElStaticSiteElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteElRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<PrimField<bool>>,
}

impl AppSpecElStaticSiteElRoutesEl {
    #[doc= "Set the field `path`.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_prefix`.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn set_preserve_path_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_path_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteElRoutesEl {
    type O = BlockAssignable<AppSpecElStaticSiteElRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElRoutesElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElRoutesElRef {
        AppSpecElStaticSiteElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nPath specifies an route by HTTP path prefix. Paths must start with / and must be unique within the app."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n An optional flag to preserve the path that is forwarded to the backend service."]
    pub fn preserve_path_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElStaticSiteElDynamic {
    cors: Option<DynamicBlock<AppSpecElStaticSiteElCorsEl>>,
    env: Option<DynamicBlock<AppSpecElStaticSiteElEnvEl>>,
    git: Option<DynamicBlock<AppSpecElStaticSiteElGitEl>>,
    github: Option<DynamicBlock<AppSpecElStaticSiteElGithubEl>>,
    gitlab: Option<DynamicBlock<AppSpecElStaticSiteElGitlabEl>>,
    routes: Option<DynamicBlock<AppSpecElStaticSiteElRoutesEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElStaticSiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catchall_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_document: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<Vec<AppSpecElStaticSiteElCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<AppSpecElStaticSiteElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<AppSpecElStaticSiteElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AppSpecElStaticSiteElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<Vec<AppSpecElStaticSiteElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<Vec<AppSpecElStaticSiteElRoutesEl>>,
    dynamic: AppSpecElStaticSiteElDynamic,
}

impl AppSpecElStaticSiteEl {
    #[doc= "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `catchall_document`.\nThe name of the document to use as the fallback for any requests to documents that are not found when serving this static site."]
    pub fn set_catchall_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catchall_document = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `error_document`.\nThe name of the error document to use when serving this static site."]
    pub fn set_error_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_document = Some(v.into());
        self
    }

    #[doc= "Set the field `index_document`.\nThe name of the index document to use when serving this static site."]
    pub fn set_index_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_document = Some(v.into());
        self
    }

    #[doc= "Set the field `output_dir`.\nAn optional path to where the built assets will be located, relative to the build context. If not set, App Platform will automatically scan for these directory names: `_static`, `dist`, `public`."]
    pub fn set_output_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteElCorsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteElEnvEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteElGitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteElGitlabEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gitlab = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteElRoutesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.routes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.routes = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElStaticSiteEl {
    type O = BlockAssignable<AppSpecElStaticSiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElStaticSiteEl {
    #[doc= "The name of the component"]
    pub name: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElStaticSiteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElStaticSiteElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElStaticSiteElRef {
        AppSpecElStaticSiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElStaticSiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `catchall_document` after provisioning.\nThe name of the document to use as the fallback for any requests to documents that are not found when serving this static site."]
    pub fn catchall_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catchall_document", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `error_document` after provisioning.\nThe name of the error document to use when serving this static site."]
    pub fn error_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_document", self.base))
    }

    #[doc= "Get a reference to the value of field `index_document` after provisioning.\nThe name of the index document to use when serving this static site."]
    pub fn index_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_document", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `output_dir` after provisioning.\nAn optional path to where the built assets will be located, relative to the build context. If not set, App Platform will automatically scan for these directory names: `_static`, `dist`, `public`."]
    pub fn output_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<AppSpecElStaticSiteElCorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<AppSpecElStaticSiteElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AppSpecElStaticSiteElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<AppSpecElStaticSiteElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<AppSpecElStaticSiteElRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    operator: PrimField<String>,
    rule: PrimField<String>,
    value: PrimField<f64>,
    window: PrimField<String>,
}

impl AppSpecElWorkerElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElAlertEl {
    type O = BlockAssignable<AppSpecElWorkerElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElAlertEl {
    #[doc= ""]
    pub operator: PrimField<String>,
    #[doc= ""]
    pub rule: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<f64>,
    #[doc= ""]
    pub window: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElAlertElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElAlertElRef {
        AppSpecElWorkerElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElAlertElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppSpecElWorkerElEnvEl {
    #[doc= "Set the field `key`.\nThe name of the environment variable."]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\nThe visibility scope of the environment variable."]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of the environment variable."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\nThe value of the environment variable."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElEnvEl {
    type O = BlockAssignable<AppSpecElWorkerElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElEnvElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElEnvElRef {
        AppSpecElWorkerElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nThe name of the environment variable."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\nThe visibility scope of the environment variable."]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the environment variable."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the environment variable."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl AppSpecElWorkerElGitEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\nThe clone URL of the repo."]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElGitEl {
    type O = BlockAssignable<AppSpecElWorkerElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElGitElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElGitElRef {
        AppSpecElWorkerElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\nThe clone URL of the repo."]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElWorkerElGithubEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElGithubEl {
    type O = BlockAssignable<AppSpecElWorkerElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElGithubElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElGithubElRef {
        AppSpecElWorkerElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl AppSpecElWorkerElGitlabEl {
    #[doc= "Set the field `branch`.\nThe name of the branch to use."]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\nWhether to automatically deploy new commits made to the repo"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\nThe name of the repo in the format `owner/repo`."]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElGitlabEl {
    type O = BlockAssignable<AppSpecElWorkerElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElGitlabElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElGitlabElRef {
        AppSpecElWorkerElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\nThe name of the branch to use."]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\nWhether to automatically deploy new commits made to the repo"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\nThe name of the repo in the format `owner/repo`."]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl AppSpecElWorkerElImageElDeployOnPushEl {
    #[doc= "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElImageElDeployOnPushEl {
    type O = BlockAssignable<AppSpecElWorkerElImageElDeployOnPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElImageElDeployOnPushElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElImageElDeployOnPushElRef {
        AppSpecElWorkerElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElImageElDeployOnPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElWorkerElImageElDynamic {
    deploy_on_push: Option<DynamicBlock<AppSpecElWorkerElImageElDeployOnPushEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
    registry_type: PrimField<String>,
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<Vec<AppSpecElWorkerElImageElDeployOnPushEl>>,
    dynamic: AppSpecElWorkerElImageElDynamic,
}

impl AppSpecElWorkerElImageEl {
    #[doc= "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElImageElDeployOnPushEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.deploy_on_push = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy_on_push = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElWorkerElImageEl {
    type O = BlockAssignable<AppSpecElWorkerElImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElImageEl {
    #[doc= "The registry type."]
    pub registry_type: PrimField<String>,
    #[doc= "The repository name."]
    pub repository: PrimField<String>,
}

impl BuildAppSpecElWorkerElImageEl {
    pub fn build(self) -> AppSpecElWorkerElImageEl {
        AppSpecElWorkerElImageEl {
            registry: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElImageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElImageElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElImageElRef {
        AppSpecElWorkerElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_type` after provisioning.\nThe registry type."]
    pub fn registry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\nThe repository name."]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> ListRef<AppSpecElWorkerElImageElDeployOnPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationElDatadogEl {
    api_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl AppSpecElWorkerElLogDestinationElDatadogEl {
    #[doc= "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for AppSpecElWorkerElLogDestinationElDatadogEl {
    type O = BlockAssignable<AppSpecElWorkerElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationElDatadogEl {
    #[doc= "Datadog API key."]
    pub api_key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElLogDestinationElDatadogElRef {
        AppSpecElWorkerElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\nDatadog API key."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nDatadog HTTP log intake endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationElLogtailEl {
    token: PrimField<String>,
}

impl AppSpecElWorkerElLogDestinationElLogtailEl { }

impl ToListMappable for AppSpecElWorkerElLogDestinationElLogtailEl {
    type O = BlockAssignable<AppSpecElWorkerElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationElLogtailEl {
    #[doc= "Logtail token."]
    pub token: PrimField<String>,
}

impl BuildAppSpecElWorkerElLogDestinationElLogtailEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationElLogtailEl {
        AppSpecElWorkerElLogDestinationElLogtailEl { token: self.token }
    }
}

pub struct AppSpecElWorkerElLogDestinationElLogtailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElLogDestinationElLogtailElRef {
        AppSpecElWorkerElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationElPapertrailEl {
    endpoint: PrimField<String>,
}

impl AppSpecElWorkerElLogDestinationElPapertrailEl { }

impl ToListMappable for AppSpecElWorkerElLogDestinationElPapertrailEl {
    type O = BlockAssignable<AppSpecElWorkerElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationElPapertrailEl {
    #[doc= "Papertrail syslog endpoint."]
    pub endpoint: PrimField<String>,
}

impl BuildAppSpecElWorkerElLogDestinationElPapertrailEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationElPapertrailEl {
        AppSpecElWorkerElLogDestinationElPapertrailEl { endpoint: self.endpoint }
    }
}

pub struct AppSpecElWorkerElLogDestinationElPapertrailElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElLogDestinationElPapertrailElRef {
        AppSpecElWorkerElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElWorkerElLogDestinationElDynamic {
    datadog: Option<DynamicBlock<AppSpecElWorkerElLogDestinationElDatadogEl>>,
    logtail: Option<DynamicBlock<AppSpecElWorkerElLogDestinationElLogtailEl>>,
    papertrail: Option<DynamicBlock<AppSpecElWorkerElLogDestinationElPapertrailEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElWorkerElLogDestinationEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<Vec<AppSpecElWorkerElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<Vec<AppSpecElWorkerElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<Vec<AppSpecElWorkerElLogDestinationElPapertrailEl>>,
    dynamic: AppSpecElWorkerElLogDestinationElDynamic,
}

impl AppSpecElWorkerElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElLogDestinationElDatadogEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.datadog = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.datadog = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElLogDestinationElLogtailEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.logtail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.logtail = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl Into<BlockAssignable<AppSpecElWorkerElLogDestinationElPapertrailEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.papertrail = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.papertrail = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElWorkerElLogDestinationEl {
    type O = BlockAssignable<AppSpecElWorkerElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerElLogDestinationEl {
    #[doc= "Name of the log destination"]
    pub name: PrimField<String>,
}

impl BuildAppSpecElWorkerElLogDestinationEl {
    pub fn build(self) -> AppSpecElWorkerElLogDestinationEl {
        AppSpecElWorkerElLogDestinationEl {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElLogDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElLogDestinationElRef {
        AppSpecElWorkerElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<AppSpecElWorkerElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<AppSpecElWorkerElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<AppSpecElWorkerElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElWorkerElDynamic {
    alert: Option<DynamicBlock<AppSpecElWorkerElAlertEl>>,
    env: Option<DynamicBlock<AppSpecElWorkerElEnvEl>>,
    git: Option<DynamicBlock<AppSpecElWorkerElGitEl>>,
    github: Option<DynamicBlock<AppSpecElWorkerElGithubEl>>,
    gitlab: Option<DynamicBlock<AppSpecElWorkerElGitlabEl>>,
    image: Option<DynamicBlock<AppSpecElWorkerElImageEl>>,
    log_destination: Option<DynamicBlock<AppSpecElWorkerElLogDestinationEl>>,
}

#[derive(Serialize)]
pub struct AppSpecElWorkerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size_slug: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Vec<AppSpecElWorkerElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<AppSpecElWorkerElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<AppSpecElWorkerElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<Vec<AppSpecElWorkerElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<Vec<AppSpecElWorkerElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Vec<AppSpecElWorkerElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<Vec<AppSpecElWorkerElLogDestinationEl>>,
    dynamic: AppSpecElWorkerElDynamic,
}

impl AppSpecElWorkerEl {
    #[doc= "Set the field `build_command`.\nAn optional build command to run while building this component from source."]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\nAn environment slug describing the type of this app."]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\nThe amount of instances that this component should be scaled to."]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_size_slug`.\nThe instance size to use for this component."]
    pub fn set_instance_size_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `run_command`.\nAn optional run command to override the component's default."]
    pub fn set_run_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.run_command = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElAlertEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alert = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElEnvEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElGitEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElGithubEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.github = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.github = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElGitlabEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gitlab = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gitlab = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElImageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerElLogDestinationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecElWorkerEl {
    type O = BlockAssignable<AppSpecElWorkerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecElWorkerEl {
    #[doc= "The name of the component"]
    pub name: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElWorkerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElWorkerElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElWorkerElRef {
        AppSpecElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElWorkerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\nAn optional build command to run while building this component from source."]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\nThe path to a Dockerfile relative to the root of the repo. If set, overrides usage of buildpacks."]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\nAn environment slug describing the type of this app."]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\nThe amount of instances that this component should be scaled to."]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_size_slug` after provisioning.\nThe instance size to use for this component."]
    pub fn instance_size_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the component"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `run_command` after provisioning.\nAn optional run command to override the component's default."]
    pub fn run_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\nAn optional path to the working directory to use for the build."]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<AppSpecElWorkerElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<AppSpecElWorkerElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<AppSpecElWorkerElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<AppSpecElWorkerElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<AppSpecElWorkerElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<AppSpecElWorkerElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppSpecElDynamic {
    alert: Option<DynamicBlock<AppSpecElAlertEl>>,
    database: Option<DynamicBlock<AppSpecElDatabaseEl>>,
    domain: Option<DynamicBlock<AppSpecElDomainEl>>,
    env: Option<DynamicBlock<AppSpecElEnvEl>>,
    function: Option<DynamicBlock<AppSpecElFunctionEl>>,
    job: Option<DynamicBlock<AppSpecElJobEl>>,
    service: Option<DynamicBlock<AppSpecElServiceEl>>,
    static_site: Option<DynamicBlock<AppSpecElStaticSiteEl>>,
    worker: Option<DynamicBlock<AppSpecElWorkerEl>>,
}

#[derive(Serialize)]
pub struct AppSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Vec<AppSpecElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<AppSpecElDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<Vec<AppSpecElDomainEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<AppSpecElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<Vec<AppSpecElFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job: Option<Vec<AppSpecElJobEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<Vec<AppSpecElServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_site: Option<Vec<AppSpecElStaticSiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker: Option<Vec<AppSpecElWorkerEl>>,
    dynamic: AppSpecElDynamic,
}

impl AppSpecEl {
    #[doc= "Set the field `domains`.\n"]
    pub fn set_domains(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.domains = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe slug for the DigitalOcean data center region hosting the app"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<BlockAssignable<AppSpecElAlertEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alert = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alert = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl Into<BlockAssignable<AppSpecElDatabaseEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<BlockAssignable<AppSpecElDomainEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.domain = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.domain = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<BlockAssignable<AppSpecElEnvEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `function`.\n"]
    pub fn set_function(mut self, v: impl Into<BlockAssignable<AppSpecElFunctionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.function = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.function = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `job`.\n"]
    pub fn set_job(mut self, v: impl Into<BlockAssignable<AppSpecElJobEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.job = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.job = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<BlockAssignable<AppSpecElServiceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `static_site`.\n"]
    pub fn set_static_site(mut self, v: impl Into<BlockAssignable<AppSpecElStaticSiteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.static_site = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.static_site = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `worker`.\n"]
    pub fn set_worker(mut self, v: impl Into<BlockAssignable<AppSpecElWorkerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppSpecEl {
    type O = BlockAssignable<AppSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppSpecEl {
    #[doc= "The name of the app. Must be unique across all apps in the same account."]
    pub name: PrimField<String>,
}

impl BuildAppSpecEl {
    pub fn build(self) -> AppSpecEl {
        AppSpecEl {
            domains: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            alert: core::default::Default::default(),
            database: core::default::Default::default(),
            domain: core::default::Default::default(),
            env: core::default::Default::default(),
            function: core::default::Default::default(),
            job: core::default::Default::default(),
            service: core::default::Default::default(),
            static_site: core::default::Default::default(),
            worker: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppSpecElRef {
    fn new(shared: StackShared, base: String) -> AppSpecElRef {
        AppSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the app. Must be unique across all apps in the same account."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe slug for the DigitalOcean data center region hosting the app"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<AppSpecElDatabaseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> ListRef<AppSpecElDomainElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc= "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> ListRef<AppSpecElFunctionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc= "Get a reference to the value of field `job` after provisioning.\n"]
    pub fn job(&self) -> ListRef<AppSpecElJobElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> ListRef<AppSpecElServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc= "Get a reference to the value of field `static_site` after provisioning.\n"]
    pub fn static_site(&self) -> ListRef<AppSpecElStaticSiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.static_site", self.base))
    }

    #[doc= "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> ListRef<AppSpecElWorkerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct AppTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl AppTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for AppTimeoutsEl {
    type O = BlockAssignable<AppTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AppTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppTimeoutsElRef {
        AppTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppDynamic {
    spec: Option<DynamicBlock<AppSpecEl>>,
}
