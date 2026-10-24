use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAppData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    app_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppData>,
}

#[derive(Clone)]
pub struct DataApp(std::rc::Rc<DataApp_>);

impl DataApp {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\nA DigitalOcean App Platform Spec"]
    pub fn spec(&self) -> terrars::ListRef<DataAppSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
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
}

impl terrars::Referable for DataApp {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataApp { }

impl terrars::ToListMappable for DataApp {
    type O = terrars::ListRef<DataAppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataApp_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApp {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app_id: terrars::PrimField<std::string::String>,
}

impl BuildDataApp {
    pub fn build(self, stack: &mut terrars::Stack) -> DataApp {
        let out = DataApp(std::rc::Rc::new(DataApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                app_id: self.app_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `spec` after provisioning.\nA DigitalOcean App Platform Spec"]
    pub fn spec(&self) -> terrars::ListRef<DataAppSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataAppSpecElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElAlertEl {
    type O = terrars::BlockAssignable<DataAppSpecElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElAlertEl {}

impl BuildDataAppSpecElAlertEl {
    pub fn build(self) -> DataAppSpecElAlertEl {
        DataAppSpecElAlertEl {
            disabled: core::default::Default::default(),
            rule: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElAlertElRef {
        DataAppSpecElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElAlertElRef {
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
pub struct DataAppSpecElDatabaseEl {
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

impl DataAppSpecElDatabaseEl {
    #[doc = "Set the field `cluster_name`.\n"]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_user`.\n"]
    pub fn set_db_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `production`.\n"]
    pub fn set_production(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.production = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElDatabaseEl {
    type O = terrars::BlockAssignable<DataAppSpecElDatabaseEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElDatabaseEl {}

impl BuildDataAppSpecElDatabaseEl {
    pub fn build(self) -> DataAppSpecElDatabaseEl {
        DataAppSpecElDatabaseEl {
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

pub struct DataAppSpecElDatabaseElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElDatabaseElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElDatabaseElRef {
        DataAppSpecElDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElDatabaseElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `production` after provisioning.\n"]
    pub fn production(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElDomainEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElDomainEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.wildcard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElDomainEl {
    type O = terrars::BlockAssignable<DataAppSpecElDomainEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElDomainEl {}

impl BuildDataAppSpecElDomainEl {
    pub fn build(self) -> DataAppSpecElDomainEl {
        DataAppSpecElDomainEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElDomainElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElDomainElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElDomainElRef {
        DataAppSpecElDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElDomainElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElEnvEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElEnvEl {
    type O = terrars::BlockAssignable<DataAppSpecElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElEnvEl {}

impl BuildDataAppSpecElEnvEl {
    pub fn build(self) -> DataAppSpecElEnvEl {
        DataAppSpecElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElEnvElRef {
        DataAppSpecElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElAlertEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElAlertEl {}

impl BuildDataAppSpecElFunctionElAlertEl {
    pub fn build(self) -> DataAppSpecElFunctionElAlertEl {
        DataAppSpecElFunctionElAlertEl {
            disabled: core::default::Default::default(),
            operator: core::default::Default::default(),
            rule: core::default::Default::default(),
            value: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElAlertElRef {
        DataAppSpecElFunctionElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElAlertElRef {
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
pub struct DataAppSpecElFunctionElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElCorsElAllowOriginsEl {}

impl BuildDataAppSpecElFunctionElCorsElAllowOriginsEl {
    pub fn build(self) -> DataAppSpecElFunctionElCorsElAllowOriginsEl {
        DataAppSpecElFunctionElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElCorsElAllowOriginsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElFunctionElCorsElAllowOriginsElRef {
        DataAppSpecElFunctionElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<DataAppSpecElFunctionElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElCorsEl {
    #[doc = "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElCorsElAllowOriginsEl>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElCorsEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElCorsEl {}

impl BuildDataAppSpecElFunctionElCorsEl {
    pub fn build(self) -> DataAppSpecElFunctionElCorsEl {
        DataAppSpecElFunctionElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElCorsElRef {
        DataAppSpecElFunctionElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<DataAppSpecElFunctionElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElEnvEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElEnvEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElEnvEl {}

impl BuildDataAppSpecElFunctionElEnvEl {
    pub fn build(self) -> DataAppSpecElFunctionElEnvEl {
        DataAppSpecElFunctionElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElEnvElRef {
        DataAppSpecElFunctionElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElGitEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElGitEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElGitEl {}

impl BuildDataAppSpecElFunctionElGitEl {
    pub fn build(self) -> DataAppSpecElFunctionElGitEl {
        DataAppSpecElFunctionElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElGitElRef {
        DataAppSpecElFunctionElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElGithubEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElGithubEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElGithubEl {}

impl BuildDataAppSpecElFunctionElGithubEl {
    pub fn build(self) -> DataAppSpecElFunctionElGithubEl {
        DataAppSpecElFunctionElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElGithubElRef {
        DataAppSpecElFunctionElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElGitlabEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElGitlabEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElGitlabEl {}

impl BuildDataAppSpecElFunctionElGitlabEl {
    pub fn build(self) -> DataAppSpecElFunctionElGitlabEl {
        DataAppSpecElFunctionElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElGitlabElRef {
        DataAppSpecElFunctionElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElLogDestinationElDatadogEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElLogDestinationElDatadogEl {}

impl BuildDataAppSpecElFunctionElLogDestinationElDatadogEl {
    pub fn build(self) -> DataAppSpecElFunctionElLogDestinationElDatadogEl {
        DataAppSpecElFunctionElLogDestinationElDatadogEl {
            api_key: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElLogDestinationElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElFunctionElLogDestinationElDatadogElRef {
        DataAppSpecElFunctionElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElLogDestinationElLogtailEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElLogDestinationElLogtailEl {}

impl BuildDataAppSpecElFunctionElLogDestinationElLogtailEl {
    pub fn build(self) -> DataAppSpecElFunctionElLogDestinationElLogtailEl {
        DataAppSpecElFunctionElLogDestinationElLogtailEl { token: core::default::Default::default() }
    }
}

pub struct DataAppSpecElFunctionElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElLogDestinationElLogtailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElFunctionElLogDestinationElLogtailElRef {
        DataAppSpecElFunctionElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionElLogDestinationElPapertrailEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElLogDestinationElPapertrailEl {}

impl BuildDataAppSpecElFunctionElLogDestinationElPapertrailEl {
    pub fn build(self) -> DataAppSpecElFunctionElLogDestinationElPapertrailEl {
        DataAppSpecElFunctionElLogDestinationElPapertrailEl { endpoint: core::default::Default::default() }
    }
}

pub struct DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
        DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::ListField<DataAppSpecElFunctionElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<terrars::ListField<DataAppSpecElFunctionElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<terrars::ListField<DataAppSpecElFunctionElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElFunctionElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElLogDestinationElDatadogEl>>,
    ) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElLogDestinationElLogtailEl>>,
    ) -> Self {
        self.logtail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElLogDestinationElPapertrailEl>>,
    ) -> Self {
        self.papertrail = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElLogDestinationEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElLogDestinationEl {}

impl BuildDataAppSpecElFunctionElLogDestinationEl {
    pub fn build(self) -> DataAppSpecElFunctionElLogDestinationEl {
        DataAppSpecElFunctionElLogDestinationEl {
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            name: core::default::Default::default(),
            papertrail: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElLogDestinationElRef {
        DataAppSpecElFunctionElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<DataAppSpecElFunctionElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<DataAppSpecElFunctionElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<DataAppSpecElFunctionElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppSpecElFunctionElRoutesEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionElRoutesEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionElRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionElRoutesEl {}

impl BuildDataAppSpecElFunctionElRoutesEl {
    pub fn build(self) -> DataAppSpecElFunctionElRoutesEl {
        DataAppSpecElFunctionElRoutesEl {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElRoutesElRef {
        DataAppSpecElFunctionElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataAppSpecElFunctionElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataAppSpecElFunctionElCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataAppSpecElFunctionElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataAppSpecElFunctionElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataAppSpecElFunctionElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataAppSpecElFunctionElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataAppSpecElFunctionElLogDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<terrars::ListField<DataAppSpecElFunctionElRoutesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElFunctionEl {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElAlertEl>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElCorsEl>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElFunctionElEnvEl>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElGitEl>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElGithubEl>>,
    ) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElGitlabEl>>,
    ) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElLogDestinationEl>>,
    ) -> Self {
        self.log_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routes`.\n"]
    pub fn set_routes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionElRoutesEl>>,
    ) -> Self {
        self.routes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElFunctionEl {
    type O = terrars::BlockAssignable<DataAppSpecElFunctionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElFunctionEl {}

impl BuildDataAppSpecElFunctionEl {
    pub fn build(self) -> DataAppSpecElFunctionEl {
        DataAppSpecElFunctionEl {
            alert: core::default::Default::default(),
            cors: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            name: core::default::Default::default(),
            routes: core::default::Default::default(),
            source_dir: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElFunctionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElFunctionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElFunctionElRef {
        DataAppSpecElFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataAppSpecElFunctionElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataAppSpecElFunctionElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataAppSpecElFunctionElEnvElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataAppSpecElFunctionElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataAppSpecElFunctionElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataAppSpecElFunctionElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataAppSpecElFunctionElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataAppSpecElFunctionElRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElIngressElRuleElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElIngressElRuleElComponentEl {
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

impl terrars::ToListMappable for DataAppSpecElIngressElRuleElComponentEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleElComponentEl {}

impl BuildDataAppSpecElIngressElRuleElComponentEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleElComponentEl {
        DataAppSpecElIngressElRuleElComponentEl {
            name: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
            rewrite: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElIngressElRuleElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElComponentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRuleElComponentElRef {
        DataAppSpecElIngressElRuleElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElComponentElRef {
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
pub struct DataAppSpecElIngressElRuleElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElIngressElRuleElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElIngressElRuleElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleElCorsElAllowOriginsEl {}

impl BuildDataAppSpecElIngressElRuleElCorsElAllowOriginsEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleElCorsElAllowOriginsEl {
        DataAppSpecElIngressElRuleElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElIngressElRuleElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElCorsElAllowOriginsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElIngressElRuleElCorsElAllowOriginsElRef {
        DataAppSpecElIngressElRuleElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElIngressElRuleElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElIngressElRuleElCorsEl {
    #[doc = "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleElCorsElAllowOriginsEl>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElIngressElRuleElCorsEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleElCorsEl {}

impl BuildDataAppSpecElIngressElRuleElCorsEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleElCorsEl {
        DataAppSpecElIngressElRuleElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElIngressElRuleElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRuleElCorsElRef {
        DataAppSpecElIngressElRuleElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElIngressElRuleElMatchElPathEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElIngressElRuleElMatchElPathEl {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElIngressElRuleElMatchElPathEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleElMatchElPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleElMatchElPathEl {}

impl BuildDataAppSpecElIngressElRuleElMatchElPathEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleElMatchElPathEl {
        DataAppSpecElIngressElRuleElMatchElPathEl { prefix: core::default::Default::default() }
    }
}

pub struct DataAppSpecElIngressElRuleElMatchElPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElMatchElPathElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRuleElMatchElPathElRef {
        DataAppSpecElIngressElRuleElMatchElPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElMatchElPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElIngressElRuleElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleElMatchElPathEl>>,
}

impl DataAppSpecElIngressElRuleElMatchEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleElMatchElPathEl>>,
    ) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElIngressElRuleElMatchEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleElMatchEl {}

impl BuildDataAppSpecElIngressElRuleElMatchEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleElMatchEl {
        DataAppSpecElIngressElRuleElMatchEl { path: core::default::Default::default() }
    }
}

pub struct DataAppSpecElIngressElRuleElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRuleElMatchElRef {
        DataAppSpecElIngressElRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElMatchElPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElIngressElRuleElRedirectEl {
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

impl DataAppSpecElIngressElRuleElRedirectEl {
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

impl terrars::ToListMappable for DataAppSpecElIngressElRuleElRedirectEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleElRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleElRedirectEl {}

impl BuildDataAppSpecElIngressElRuleElRedirectEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleElRedirectEl {
        DataAppSpecElIngressElRuleElRedirectEl {
            authority: core::default::Default::default(),
            port: core::default::Default::default(),
            redirect_code: core::default::Default::default(),
            scheme: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElIngressElRuleElRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRuleElRedirectElRef {
        DataAppSpecElIngressElRuleElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElRedirectElRef {
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

#[derive(Serialize)]
pub struct DataAppSpecElIngressElRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleElCorsEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleElRedirectEl>>,
}

impl DataAppSpecElIngressElRuleEl {
    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleElComponentEl>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleElCorsEl>>,
    ) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleElMatchEl>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleElRedirectEl>>,
    ) -> Self {
        self.redirect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElIngressElRuleEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressElRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressElRuleEl {}

impl BuildDataAppSpecElIngressElRuleEl {
    pub fn build(self) -> DataAppSpecElIngressElRuleEl {
        DataAppSpecElIngressElRuleEl {
            component: core::default::Default::default(),
            cors: core::default::Default::default(),
            match_: core::default::Default::default(),
            redirect: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElIngressElRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRuleElRef {
        DataAppSpecElIngressElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElIngressEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::ListField<DataAppSpecElIngressElRuleEl>>,
}

impl DataAppSpecElIngressEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressElRuleEl>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElIngressEl {
    type O = terrars::BlockAssignable<DataAppSpecElIngressEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElIngressEl {}

impl BuildDataAppSpecElIngressEl {
    pub fn build(self) -> DataAppSpecElIngressEl {
        DataAppSpecElIngressEl { rule: core::default::Default::default() }
    }
}

pub struct DataAppSpecElIngressElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElIngressElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElIngressElRef {
        DataAppSpecElIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElIngressElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataAppSpecElIngressElRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElAlertEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElAlertEl {}

impl BuildDataAppSpecElJobElAlertEl {
    pub fn build(self) -> DataAppSpecElJobElAlertEl {
        DataAppSpecElJobElAlertEl {
            disabled: core::default::Default::default(),
            operator: core::default::Default::default(),
            rule: core::default::Default::default(),
            value: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElAlertElRef {
        DataAppSpecElJobElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElAlertElRef {
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
pub struct DataAppSpecElJobElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElEnvEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElEnvEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElEnvEl {}

impl BuildDataAppSpecElJobElEnvEl {
    pub fn build(self) -> DataAppSpecElJobElEnvEl {
        DataAppSpecElJobElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElEnvElRef {
        DataAppSpecElJobElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElGitEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElGitEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElGitEl {}

impl BuildDataAppSpecElJobElGitEl {
    pub fn build(self) -> DataAppSpecElJobElGitEl {
        DataAppSpecElJobElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElGitElRef {
        DataAppSpecElJobElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElGithubEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElGithubEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElGithubEl {}

impl BuildDataAppSpecElJobElGithubEl {
    pub fn build(self) -> DataAppSpecElJobElGithubEl {
        DataAppSpecElJobElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElGithubElRef {
        DataAppSpecElJobElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElGitlabEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElGitlabEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElGitlabEl {}

impl BuildDataAppSpecElJobElGitlabEl {
    pub fn build(self) -> DataAppSpecElJobElGitlabEl {
        DataAppSpecElJobElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElGitlabElRef {
        DataAppSpecElJobElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppSpecElJobElImageElDeployOnPushEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElImageElDeployOnPushEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElImageElDeployOnPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElImageElDeployOnPushEl {}

impl BuildDataAppSpecElJobElImageElDeployOnPushEl {
    pub fn build(self) -> DataAppSpecElJobElImageElDeployOnPushEl {
        DataAppSpecElJobElImageElDeployOnPushEl { enabled: core::default::Default::default() }
    }
}

pub struct DataAppSpecElJobElImageElDeployOnPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElImageElDeployOnPushElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElImageElDeployOnPushElRef {
        DataAppSpecElJobElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElImageElDeployOnPushElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::ListField<DataAppSpecElJobElImageElDeployOnPushEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElImageEl {
    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElImageElDeployOnPushEl>>,
    ) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_type`.\n"]
    pub fn set_registry_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElImageEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElImageEl {}

impl BuildDataAppSpecElJobElImageEl {
    pub fn build(self) -> DataAppSpecElJobElImageEl {
        DataAppSpecElJobElImageEl {
            deploy_on_push: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElImageElRef {
        DataAppSpecElJobElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<DataAppSpecElJobElImageElDeployOnPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_type` after provisioning.\n"]
    pub fn registry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElLogDestinationElDatadogEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElLogDestinationElDatadogEl {}

impl BuildDataAppSpecElJobElLogDestinationElDatadogEl {
    pub fn build(self) -> DataAppSpecElJobElLogDestinationElDatadogEl {
        DataAppSpecElJobElLogDestinationElDatadogEl {
            api_key: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElLogDestinationElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElJobElLogDestinationElDatadogElRef {
        DataAppSpecElJobElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElLogDestinationElLogtailEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElLogDestinationElLogtailEl {}

impl BuildDataAppSpecElJobElLogDestinationElLogtailEl {
    pub fn build(self) -> DataAppSpecElJobElLogDestinationElLogtailEl {
        DataAppSpecElJobElLogDestinationElLogtailEl { token: core::default::Default::default() }
    }
}

pub struct DataAppSpecElJobElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElLogDestinationElLogtailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElJobElLogDestinationElLogtailElRef {
        DataAppSpecElJobElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobElLogDestinationElPapertrailEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElLogDestinationElPapertrailEl {}

impl BuildDataAppSpecElJobElLogDestinationElPapertrailEl {
    pub fn build(self) -> DataAppSpecElJobElLogDestinationElPapertrailEl {
        DataAppSpecElJobElLogDestinationElPapertrailEl { endpoint: core::default::Default::default() }
    }
}

pub struct DataAppSpecElJobElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElJobElLogDestinationElPapertrailElRef {
        DataAppSpecElJobElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::ListField<DataAppSpecElJobElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<terrars::ListField<DataAppSpecElJobElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<terrars::ListField<DataAppSpecElJobElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElJobElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElLogDestinationElDatadogEl>>,
    ) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElLogDestinationElLogtailEl>>,
    ) -> Self {
        self.logtail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElLogDestinationElPapertrailEl>>,
    ) -> Self {
        self.papertrail = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobElLogDestinationEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobElLogDestinationEl {}

impl BuildDataAppSpecElJobElLogDestinationEl {
    pub fn build(self) -> DataAppSpecElJobElLogDestinationEl {
        DataAppSpecElJobElLogDestinationEl {
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            name: core::default::Default::default(),
            papertrail: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElLogDestinationElRef {
        DataAppSpecElJobElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<DataAppSpecElJobElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<DataAppSpecElJobElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<DataAppSpecElJobElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataAppSpecElJobElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataAppSpecElJobElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataAppSpecElJobElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataAppSpecElJobElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataAppSpecElJobElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<DataAppSpecElJobElImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataAppSpecElJobElLogDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElJobEl {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElAlertEl>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElJobElEnvEl>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElGitEl>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElGithubEl>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElGitlabEl>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElImageEl>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size_slug`.\n"]
    pub fn set_instance_size_slug(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_size_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\n"]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobElLogDestinationEl>>,
    ) -> Self {
        self.log_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_command`.\n"]
    pub fn set_run_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElJobEl {
    type O = terrars::BlockAssignable<DataAppSpecElJobEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElJobEl {}

impl BuildDataAppSpecElJobEl {
    pub fn build(self) -> DataAppSpecElJobEl {
        DataAppSpecElJobEl {
            alert: core::default::Default::default(),
            build_command: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            env: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            image: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            kind: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            name: core::default::Default::default(),
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElJobElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElJobElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElJobElRef {
        DataAppSpecElJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataAppSpecElJobElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc = "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataAppSpecElJobElEnvElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataAppSpecElJobElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataAppSpecElJobElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataAppSpecElJobElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<DataAppSpecElJobElImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_size_slug` after provisioning.\n"]
    pub fn instance_size_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataAppSpecElJobElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `run_command` after provisioning.\n"]
    pub fn run_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElAlertEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElAlertEl {}

impl BuildDataAppSpecElServiceElAlertEl {
    pub fn build(self) -> DataAppSpecElServiceElAlertEl {
        DataAppSpecElServiceElAlertEl {
            disabled: core::default::Default::default(),
            operator: core::default::Default::default(),
            rule: core::default::Default::default(),
            value: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElAlertElRef {
        DataAppSpecElServiceElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElAlertElRef {
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
pub struct DataAppSpecElServiceElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElCorsElAllowOriginsEl {}

impl BuildDataAppSpecElServiceElCorsElAllowOriginsEl {
    pub fn build(self) -> DataAppSpecElServiceElCorsElAllowOriginsEl {
        DataAppSpecElServiceElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElCorsElAllowOriginsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElCorsElAllowOriginsElRef {
        DataAppSpecElServiceElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<DataAppSpecElServiceElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElCorsEl {
    #[doc = "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElCorsElAllowOriginsEl>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElCorsEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElCorsEl {}

impl BuildDataAppSpecElServiceElCorsEl {
    pub fn build(self) -> DataAppSpecElServiceElCorsEl {
        DataAppSpecElServiceElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElCorsElRef {
        DataAppSpecElServiceElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<DataAppSpecElServiceElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElEnvEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElEnvEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElEnvEl {}

impl BuildDataAppSpecElServiceElEnvEl {
    pub fn build(self) -> DataAppSpecElServiceElEnvEl {
        DataAppSpecElServiceElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElEnvElRef {
        DataAppSpecElServiceElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElGitEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElGitEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElGitEl {}

impl BuildDataAppSpecElServiceElGitEl {
    pub fn build(self) -> DataAppSpecElServiceElGitEl {
        DataAppSpecElServiceElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElGitElRef {
        DataAppSpecElServiceElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElGithubEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElGithubEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElGithubEl {}

impl BuildDataAppSpecElServiceElGithubEl {
    pub fn build(self) -> DataAppSpecElServiceElGithubEl {
        DataAppSpecElServiceElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElGithubElRef {
        DataAppSpecElServiceElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElGitlabEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElGitlabEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElGitlabEl {}

impl BuildDataAppSpecElServiceElGitlabEl {
    pub fn build(self) -> DataAppSpecElServiceElGitlabEl {
        DataAppSpecElServiceElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElGitlabElRef {
        DataAppSpecElServiceElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElHealthCheckEl {
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

impl DataAppSpecElServiceElHealthCheckEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_path`.\n"]
    pub fn set_http_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_delay_seconds`.\n"]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_threshold`.\n"]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElHealthCheckEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElHealthCheckEl {}

impl BuildDataAppSpecElServiceElHealthCheckEl {
    pub fn build(self) -> DataAppSpecElServiceElHealthCheckEl {
        DataAppSpecElServiceElHealthCheckEl {
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

pub struct DataAppSpecElServiceElHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElHealthCheckElRef {
        DataAppSpecElServiceElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `http_path` after provisioning.\n"]
    pub fn http_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_path", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_delay_seconds` after provisioning.\n"]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `success_threshold` after provisioning.\n"]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppSpecElServiceElImageElDeployOnPushEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElImageElDeployOnPushEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElImageElDeployOnPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElImageElDeployOnPushEl {}

impl BuildDataAppSpecElServiceElImageElDeployOnPushEl {
    pub fn build(self) -> DataAppSpecElServiceElImageElDeployOnPushEl {
        DataAppSpecElServiceElImageElDeployOnPushEl { enabled: core::default::Default::default() }
    }
}

pub struct DataAppSpecElServiceElImageElDeployOnPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElImageElDeployOnPushElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElServiceElImageElDeployOnPushElRef {
        DataAppSpecElServiceElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElImageElDeployOnPushElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::ListField<DataAppSpecElServiceElImageElDeployOnPushEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElImageEl {
    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElImageElDeployOnPushEl>>,
    ) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_type`.\n"]
    pub fn set_registry_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElImageEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElImageEl {}

impl BuildDataAppSpecElServiceElImageEl {
    pub fn build(self) -> DataAppSpecElServiceElImageEl {
        DataAppSpecElServiceElImageEl {
            deploy_on_push: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElImageElRef {
        DataAppSpecElServiceElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<DataAppSpecElServiceElImageElDeployOnPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_type` after provisioning.\n"]
    pub fn registry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElLogDestinationElDatadogEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElLogDestinationElDatadogEl {}

impl BuildDataAppSpecElServiceElLogDestinationElDatadogEl {
    pub fn build(self) -> DataAppSpecElServiceElLogDestinationElDatadogEl {
        DataAppSpecElServiceElLogDestinationElDatadogEl {
            api_key: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElLogDestinationElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElServiceElLogDestinationElDatadogElRef {
        DataAppSpecElServiceElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElLogDestinationElLogtailEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElLogDestinationElLogtailEl {}

impl BuildDataAppSpecElServiceElLogDestinationElLogtailEl {
    pub fn build(self) -> DataAppSpecElServiceElLogDestinationElLogtailEl {
        DataAppSpecElServiceElLogDestinationElLogtailEl { token: core::default::Default::default() }
    }
}

pub struct DataAppSpecElServiceElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElLogDestinationElLogtailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElServiceElLogDestinationElLogtailElRef {
        DataAppSpecElServiceElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceElLogDestinationElPapertrailEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElLogDestinationElPapertrailEl {}

impl BuildDataAppSpecElServiceElLogDestinationElPapertrailEl {
    pub fn build(self) -> DataAppSpecElServiceElLogDestinationElPapertrailEl {
        DataAppSpecElServiceElLogDestinationElPapertrailEl { endpoint: core::default::Default::default() }
    }
}

pub struct DataAppSpecElServiceElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElServiceElLogDestinationElPapertrailElRef {
        DataAppSpecElServiceElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::ListField<DataAppSpecElServiceElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<terrars::ListField<DataAppSpecElServiceElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<terrars::ListField<DataAppSpecElServiceElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElServiceElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElLogDestinationElDatadogEl>>,
    ) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElLogDestinationElLogtailEl>>,
    ) -> Self {
        self.logtail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElLogDestinationElPapertrailEl>>,
    ) -> Self {
        self.papertrail = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElLogDestinationEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElLogDestinationEl {}

impl BuildDataAppSpecElServiceElLogDestinationEl {
    pub fn build(self) -> DataAppSpecElServiceElLogDestinationEl {
        DataAppSpecElServiceElLogDestinationEl {
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            name: core::default::Default::default(),
            papertrail: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElLogDestinationElRef {
        DataAppSpecElServiceElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<DataAppSpecElServiceElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<DataAppSpecElServiceElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<DataAppSpecElServiceElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppSpecElServiceElRoutesEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceElRoutesEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceElRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceElRoutesEl {}

impl BuildDataAppSpecElServiceElRoutesEl {
    pub fn build(self) -> DataAppSpecElServiceElRoutesEl {
        DataAppSpecElServiceElRoutesEl {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElRoutesElRef {
        DataAppSpecElServiceElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataAppSpecElServiceElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataAppSpecElServiceElCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataAppSpecElServiceElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataAppSpecElServiceElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataAppSpecElServiceElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataAppSpecElServiceElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::ListField<DataAppSpecElServiceElHealthCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<DataAppSpecElServiceElImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataAppSpecElServiceElLogDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<terrars::ListField<DataAppSpecElServiceElRoutesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElServiceEl {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElAlertEl>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElCorsEl>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElServiceElEnvEl>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElGitEl>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElGithubEl>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElGitlabEl>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElHealthCheckEl>>,
    ) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_port`.\n"]
    pub fn set_http_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElImageEl>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size_slug`.\n"]
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

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElLogDestinationEl>>,
    ) -> Self {
        self.log_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceElRoutesEl>>) -> Self {
        self.routes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_command`.\n"]
    pub fn set_run_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElServiceEl {
    type O = terrars::BlockAssignable<DataAppSpecElServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElServiceEl {}

impl BuildDataAppSpecElServiceEl {
    pub fn build(self) -> DataAppSpecElServiceEl {
        DataAppSpecElServiceEl {
            alert: core::default::Default::default(),
            build_command: core::default::Default::default(),
            cors: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            env: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            health_check: core::default::Default::default(),
            http_port: core::default::Default::default(),
            image: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            internal_ports: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            name: core::default::Default::default(),
            routes: core::default::Default::default(),
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElServiceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElServiceElRef {
        DataAppSpecElServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataAppSpecElServiceElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataAppSpecElServiceElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataAppSpecElServiceElEnvElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataAppSpecElServiceElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataAppSpecElServiceElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataAppSpecElServiceElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<DataAppSpecElServiceElHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<DataAppSpecElServiceElImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_size_slug` after provisioning.\n"]
    pub fn instance_size_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_ports` after provisioning.\n"]
    pub fn internal_ports(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.internal_ports", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataAppSpecElServiceElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataAppSpecElServiceElRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `run_command` after provisioning.\n"]
    pub fn run_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElCorsElAllowOriginsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElCorsElAllowOriginsEl {}

impl BuildDataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
        DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
        DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElCorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteElCorsEl {
    #[doc = "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElCorsEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElCorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElCorsEl {}

impl BuildDataAppSpecElStaticSiteElCorsEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElCorsEl {
        DataAppSpecElStaticSiteElCorsEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElCorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElCorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElCorsElRef {
        DataAppSpecElStaticSiteElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElCorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElCorsElAllowOriginsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteElEnvEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElEnvEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElEnvEl {}

impl BuildDataAppSpecElStaticSiteElEnvEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElEnvEl {
        DataAppSpecElStaticSiteElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElEnvElRef {
        DataAppSpecElStaticSiteElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteElGitEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElGitEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElGitEl {}

impl BuildDataAppSpecElStaticSiteElGitEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElGitEl {
        DataAppSpecElStaticSiteElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElGitElRef {
        DataAppSpecElStaticSiteElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteElGithubEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElGithubEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElGithubEl {}

impl BuildDataAppSpecElStaticSiteElGithubEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElGithubEl {
        DataAppSpecElStaticSiteElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElGithubElRef {
        DataAppSpecElStaticSiteElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteElGitlabEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElGitlabEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElGitlabEl {}

impl BuildDataAppSpecElStaticSiteElGitlabEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElGitlabEl {
        DataAppSpecElStaticSiteElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElGitlabElRef {
        DataAppSpecElStaticSiteElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElRoutesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppSpecElStaticSiteElRoutesEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preserve_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteElRoutesEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteElRoutesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteElRoutesEl {}

impl BuildDataAppSpecElStaticSiteElRoutesEl {
    pub fn build(self) -> DataAppSpecElStaticSiteElRoutesEl {
        DataAppSpecElStaticSiteElRoutesEl {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElRoutesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElRoutesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElRoutesElRef {
        DataAppSpecElStaticSiteElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElRoutesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catchall_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteElCorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataAppSpecElStaticSiteElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteElRoutesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElStaticSiteEl {
    #[doc = "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catchall_document`.\n"]
    pub fn set_catchall_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.catchall_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteElCorsEl>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElStaticSiteElEnvEl>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_document`.\n"]
    pub fn set_error_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteElGitEl>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteElGithubEl>>,
    ) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteElGitlabEl>>,
    ) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_document`.\n"]
    pub fn set_index_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_dir`.\n"]
    pub fn set_output_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routes`.\n"]
    pub fn set_routes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteElRoutesEl>>,
    ) -> Self {
        self.routes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElStaticSiteEl {
    type O = terrars::BlockAssignable<DataAppSpecElStaticSiteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElStaticSiteEl {}

impl BuildDataAppSpecElStaticSiteEl {
    pub fn build(self) -> DataAppSpecElStaticSiteEl {
        DataAppSpecElStaticSiteEl {
            build_command: core::default::Default::default(),
            catchall_document: core::default::Default::default(),
            cors: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            env: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            error_document: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            index_document: core::default::Default::default(),
            name: core::default::Default::default(),
            output_dir: core::default::Default::default(),
            routes: core::default::Default::default(),
            source_dir: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElStaticSiteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElStaticSiteElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElStaticSiteElRef {
        DataAppSpecElStaticSiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc = "Get a reference to the value of field `catchall_document` after provisioning.\n"]
    pub fn catchall_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catchall_document", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElCorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataAppSpecElStaticSiteElEnvElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_document", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_document", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `output_dir` after provisioning.\n"]
    pub fn output_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElRoutesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElAlertEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElAlertEl {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElAlertEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElAlertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElAlertEl {}

impl BuildDataAppSpecElWorkerElAlertEl {
    pub fn build(self) -> DataAppSpecElWorkerElAlertEl {
        DataAppSpecElWorkerElAlertEl {
            disabled: core::default::Default::default(),
            operator: core::default::Default::default(),
            rule: core::default::Default::default(),
            value: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElAlertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElAlertElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElAlertElRef {
        DataAppSpecElWorkerElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElAlertElRef {
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
pub struct DataAppSpecElWorkerElEnvEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElEnvEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElEnvEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElEnvEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElEnvEl {}

impl BuildDataAppSpecElWorkerElEnvEl {
    pub fn build(self) -> DataAppSpecElWorkerElEnvEl {
        DataAppSpecElWorkerElEnvEl {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElEnvElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElEnvElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElEnvElRef {
        DataAppSpecElWorkerElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElEnvElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElGitEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElGitEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_clone_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElGitEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElGitEl {}

impl BuildDataAppSpecElWorkerElGitEl {
    pub fn build(self) -> DataAppSpecElWorkerElGitEl {
        DataAppSpecElWorkerElGitEl {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElGitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElGitElRef {
        DataAppSpecElWorkerElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElGitElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElGithubEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElGithubEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElGithubEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElGithubEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElGithubEl {}

impl BuildDataAppSpecElWorkerElGithubEl {
    pub fn build(self) -> DataAppSpecElWorkerElGithubEl {
        DataAppSpecElWorkerElGithubEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElGithubElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElGithubElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElGithubElRef {
        DataAppSpecElWorkerElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElGithubElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElGitlabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElGitlabEl {
    #[doc = "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElGitlabEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElGitlabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElGitlabEl {}

impl BuildDataAppSpecElWorkerElGitlabEl {
    pub fn build(self) -> DataAppSpecElWorkerElGitlabEl {
        DataAppSpecElWorkerElGitlabEl {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElGitlabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElGitlabElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElGitlabElRef {
        DataAppSpecElWorkerElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElGitlabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAppSpecElWorkerElImageElDeployOnPushEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElImageElDeployOnPushEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElImageElDeployOnPushEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElImageElDeployOnPushEl {}

impl BuildDataAppSpecElWorkerElImageElDeployOnPushEl {
    pub fn build(self) -> DataAppSpecElWorkerElImageElDeployOnPushEl {
        DataAppSpecElWorkerElImageElDeployOnPushEl { enabled: core::default::Default::default() }
    }
}

pub struct DataAppSpecElWorkerElImageElDeployOnPushElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElImageElDeployOnPushElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElImageElDeployOnPushElRef {
        DataAppSpecElWorkerElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElImageElDeployOnPushElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::ListField<DataAppSpecElWorkerElImageElDeployOnPushEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElImageEl {
    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElImageElDeployOnPushEl>>,
    ) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_type`.\n"]
    pub fn set_registry_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repository = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElImageEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElImageEl {}

impl BuildDataAppSpecElWorkerElImageEl {
    pub fn build(self) -> DataAppSpecElWorkerElImageEl {
        DataAppSpecElWorkerElImageEl {
            deploy_on_push: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElImageElRef {
        DataAppSpecElWorkerElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<DataAppSpecElWorkerElImageElDeployOnPushElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_type` after provisioning.\n"]
    pub fn registry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElLogDestinationElDatadogEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElLogDestinationElDatadogEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElLogDestinationElDatadogEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElLogDestinationElDatadogEl {}

impl BuildDataAppSpecElWorkerElLogDestinationElDatadogEl {
    pub fn build(self) -> DataAppSpecElWorkerElLogDestinationElDatadogEl {
        DataAppSpecElWorkerElLogDestinationElDatadogEl {
            api_key: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElLogDestinationElDatadogElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElLogDestinationElDatadogElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElWorkerElLogDestinationElDatadogElRef {
        DataAppSpecElWorkerElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElDatadogElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElLogDestinationElLogtailEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElLogDestinationElLogtailEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElLogDestinationElLogtailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElLogDestinationElLogtailEl {}

impl BuildDataAppSpecElWorkerElLogDestinationElLogtailEl {
    pub fn build(self) -> DataAppSpecElWorkerElLogDestinationElLogtailEl {
        DataAppSpecElWorkerElLogDestinationElLogtailEl { token: core::default::Default::default() }
    }
}

pub struct DataAppSpecElWorkerElLogDestinationElLogtailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElLogDestinationElLogtailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElWorkerElLogDestinationElLogtailElRef {
        DataAppSpecElWorkerElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElLogtailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerElLogDestinationElPapertrailEl {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElLogDestinationElPapertrailEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElLogDestinationElPapertrailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElLogDestinationElPapertrailEl {}

impl BuildDataAppSpecElWorkerElLogDestinationElPapertrailEl {
    pub fn build(self) -> DataAppSpecElWorkerElLogDestinationElPapertrailEl {
        DataAppSpecElWorkerElLogDestinationElPapertrailEl { endpoint: core::default::Default::default() }
    }
}

pub struct DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
        DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::ListField<DataAppSpecElWorkerElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<terrars::ListField<DataAppSpecElWorkerElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<terrars::ListField<DataAppSpecElWorkerElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElWorkerElLogDestinationEl {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElLogDestinationElDatadogEl>>,
    ) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElLogDestinationElLogtailEl>>,
    ) -> Self {
        self.logtail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElLogDestinationElPapertrailEl>>,
    ) -> Self {
        self.papertrail = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerElLogDestinationEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerElLogDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerElLogDestinationEl {}

impl BuildDataAppSpecElWorkerElLogDestinationEl {
    pub fn build(self) -> DataAppSpecElWorkerElLogDestinationEl {
        DataAppSpecElWorkerElLogDestinationEl {
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            name: core::default::Default::default(),
            papertrail: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElLogDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElLogDestinationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElLogDestinationElRef {
        DataAppSpecElWorkerElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<DataAppSpecElWorkerElLogDestinationElDatadogElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<DataAppSpecElWorkerElLogDestinationElLogtailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<DataAppSpecElWorkerElLogDestinationElPapertrailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataAppSpecElWorkerElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataAppSpecElWorkerElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataAppSpecElWorkerElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataAppSpecElWorkerElGithubEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataAppSpecElWorkerElGitlabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<DataAppSpecElWorkerElImageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataAppSpecElWorkerElLogDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppSpecElWorkerEl {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElAlertEl>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElWorkerElEnvEl>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElGitEl>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElGithubEl>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElGitlabEl>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElImageEl>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size_slug`.\n"]
    pub fn set_instance_size_slug(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_size_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerElLogDestinationEl>>,
    ) -> Self {
        self.log_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_command`.\n"]
    pub fn set_run_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.run_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecElWorkerEl {
    type O = terrars::BlockAssignable<DataAppSpecElWorkerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecElWorkerEl {}

impl BuildDataAppSpecElWorkerEl {
    pub fn build(self) -> DataAppSpecElWorkerEl {
        DataAppSpecElWorkerEl {
            alert: core::default::Default::default(),
            build_command: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            env: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            image: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            name: core::default::Default::default(),
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElWorkerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElWorkerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElWorkerElRef {
        DataAppSpecElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataAppSpecElWorkerElAlertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc = "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataAppSpecElWorkerElEnvElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataAppSpecElWorkerElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataAppSpecElWorkerElGithubElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataAppSpecElWorkerElGitlabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<DataAppSpecElWorkerElImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_size_slug` after provisioning.\n"]
    pub fn instance_size_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataAppSpecElWorkerElLogDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `run_command` after provisioning.\n"]
    pub fn run_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::SetField<DataAppSpecElAlertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::ListField<DataAppSpecElDatabaseEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::ListField<DataAppSpecElDomainEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataAppSpecElEnvEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<terrars::ListField<DataAppSpecElFunctionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<terrars::ListField<DataAppSpecElIngressEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job: std::option::Option<terrars::ListField<DataAppSpecElJobEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::ListField<DataAppSpecElServiceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_site: std::option::Option<terrars::ListField<DataAppSpecElStaticSiteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<terrars::ListField<DataAppSpecElWorkerEl>>,
}

impl DataAppSpecEl {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElAlertEl>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElDatabaseEl>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElDomainEl>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataAppSpecElEnvEl>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features`.\n"]
    pub fn set_features(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.features = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `function`.\n"]
    pub fn set_function(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElFunctionEl>>) -> Self {
        self.function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElIngressEl>>) -> Self {
        self.ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job`.\n"]
    pub fn set_job(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElJobEl>>) -> Self {
        self.job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElServiceEl>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_site`.\n"]
    pub fn set_static_site(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElStaticSiteEl>>) -> Self {
        self.static_site = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(mut self, v: impl std::convert::Into<terrars::ListField<DataAppSpecElWorkerEl>>) -> Self {
        self.worker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppSpecEl {
    type O = terrars::BlockAssignable<DataAppSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppSpecEl {}

impl BuildDataAppSpecEl {
    pub fn build(self) -> DataAppSpecEl {
        DataAppSpecEl {
            alert: core::default::Default::default(),
            database: core::default::Default::default(),
            domain: core::default::Default::default(),
            domains: core::default::Default::default(),
            env: core::default::Default::default(),
            features: core::default::Default::default(),
            function: core::default::Default::default(),
            ingress: core::default::Default::default(),
            job: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            service: core::default::Default::default(),
            static_site: core::default::Default::default(),
            worker: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppSpecElRef {
        DataAppSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::SetRef<DataAppSpecElAlertElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<DataAppSpecElDatabaseElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::ListRef<DataAppSpecElDomainElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataAppSpecElEnvElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.features", self.base))
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::ListRef<DataAppSpecElFunctionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::ListRef<DataAppSpecElIngressElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }

    #[doc = "Get a reference to the value of field `job` after provisioning.\n"]
    pub fn job(&self) -> terrars::ListRef<DataAppSpecElJobElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::ListRef<DataAppSpecElServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `static_site` after provisioning.\n"]
    pub fn static_site(&self) -> terrars::ListRef<DataAppSpecElStaticSiteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_site", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<DataAppSpecElWorkerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}
