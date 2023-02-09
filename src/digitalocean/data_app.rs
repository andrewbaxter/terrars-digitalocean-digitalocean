use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAppData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataApp_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAppData>,
}

#[derive(Clone)]
pub struct DataApp(Rc<DataApp_>);

impl DataApp {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `spec` after provisioning.\nA DigitalOcean App Platform Spec"]
    pub fn spec(&self) -> ListRef<DataAppSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time of when the App was last updated"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource identifier for the app"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }
}

impl Datasource for DataApp {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataApp {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataApp {
    type O = ListRef<DataAppRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataApp_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_app".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApp {
    pub tf_id: String,
    #[doc= ""]
    pub app_id: PrimField<String>,
}

impl BuildDataApp {
    pub fn build(self, stack: &mut Stack) -> DataApp {
        let out = DataApp(Rc::new(DataApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAppData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                app_id: self.app_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `active_deployment_id` after provisioning.\nThe ID the App's currently active deployment"]
    pub fn active_deployment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_deployment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `spec` after provisioning.\nA DigitalOcean App Platform Spec"]
    pub fn spec(&self) -> ListRef<DataAppSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time of when the App was last updated"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nThe uniform resource identifier for the app"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<PrimField<String>>,
}

impl DataAppSpecElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElAlertEl {
    type O = BlockAssignable<DataAppSpecElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElAlertElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElAlertElRef {
        DataAppSpecElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElAlertElRef {
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
pub struct DataAppSpecElDatabaseEl {
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

impl DataAppSpecElDatabaseEl {
    #[doc= "Set the field `cluster_name`.\n"]
    pub fn set_cluster_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_name = Some(v.into());
        self
    }

    #[doc= "Set the field `db_name`.\n"]
    pub fn set_db_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_name = Some(v.into());
        self
    }

    #[doc= "Set the field `db_user`.\n"]
    pub fn set_db_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.db_user = Some(v.into());
        self
    }

    #[doc= "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `production`.\n"]
    pub fn set_production(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.production = Some(v.into());
        self
    }

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElDatabaseEl {
    type O = BlockAssignable<DataAppSpecElDatabaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElDatabaseElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElDatabaseElRef {
        DataAppSpecElDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElDatabaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc= "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc= "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.base))
    }

    #[doc= "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `production` after provisioning.\n"]
    pub fn production(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.production", self.base))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElDomainEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}

impl DataAppSpecElDomainEl {
    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wildcard = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElDomainEl {
    type O = BlockAssignable<DataAppSpecElDomainEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElDomainElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElDomainElRef {
        DataAppSpecElDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElDomainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppSpecElEnvEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElEnvEl {
    type O = BlockAssignable<DataAppSpecElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElEnvElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElEnvElRef {
        DataAppSpecElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElAlertEl {
    type O = BlockAssignable<DataAppSpecElFunctionElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElAlertElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElAlertElRef {
        DataAppSpecElFunctionElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElAlertElRef {
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
pub struct DataAppSpecElFunctionElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElCorsElAllowOriginsEl {
    #[doc= "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElCorsElAllowOriginsEl {
    type O = BlockAssignable<DataAppSpecElFunctionElCorsElAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElCorsElAllowOriginsElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElCorsElAllowOriginsElRef {
        DataAppSpecElFunctionElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElCorsElAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<DataAppSpecElFunctionElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElCorsEl {
    #[doc= "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<DataAppSpecElFunctionElCorsElAllowOriginsEl>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElCorsEl {
    type O = BlockAssignable<DataAppSpecElFunctionElCorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElCorsElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElCorsElRef {
        DataAppSpecElFunctionElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> ListRef<DataAppSpecElFunctionElCorsElAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElEnvEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElEnvEl {
    type O = BlockAssignable<DataAppSpecElFunctionElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElEnvElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElEnvElRef {
        DataAppSpecElFunctionElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElGitEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElGitEl {
    type O = BlockAssignable<DataAppSpecElFunctionElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElGitElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElGitElRef {
        DataAppSpecElFunctionElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElGithubEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElGithubEl {
    type O = BlockAssignable<DataAppSpecElFunctionElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElGithubElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElGithubElRef {
        DataAppSpecElFunctionElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElGitlabEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElGitlabEl {
    type O = BlockAssignable<DataAppSpecElFunctionElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElGitlabElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElGitlabElRef {
        DataAppSpecElFunctionElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElLogDestinationElDatadogEl {
    #[doc= "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElLogDestinationElDatadogEl {
    type O = BlockAssignable<DataAppSpecElFunctionElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElLogDestinationElDatadogElRef {
        DataAppSpecElFunctionElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElLogDestinationElLogtailEl {
    #[doc= "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElLogDestinationElLogtailEl {
    type O = BlockAssignable<DataAppSpecElFunctionElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElLogDestinationElLogtailElRef {
        DataAppSpecElFunctionElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionElLogDestinationElPapertrailEl {
    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElLogDestinationElPapertrailEl {
    type O = BlockAssignable<DataAppSpecElFunctionElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
        DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElLogDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<ListField<DataAppSpecElFunctionElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<ListField<DataAppSpecElFunctionElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<ListField<DataAppSpecElFunctionElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElFunctionElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<ListField<DataAppSpecElFunctionElLogDestinationElDatadogEl>>) -> Self {
        self.datadog = Some(v.into());
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<ListField<DataAppSpecElFunctionElLogDestinationElLogtailEl>>) -> Self {
        self.logtail = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl Into<ListField<DataAppSpecElFunctionElLogDestinationElPapertrailEl>>,
    ) -> Self {
        self.papertrail = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElLogDestinationEl {
    type O = BlockAssignable<DataAppSpecElFunctionElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElLogDestinationElRef {
        DataAppSpecElFunctionElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<DataAppSpecElFunctionElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<DataAppSpecElFunctionElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<DataAppSpecElFunctionElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionElRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<PrimField<bool>>,
}

impl DataAppSpecElFunctionElRoutesEl {
    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_path_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionElRoutesEl {
    type O = BlockAssignable<DataAppSpecElFunctionElRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElRoutesElRef {
        DataAppSpecElFunctionElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<ListField<DataAppSpecElFunctionElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<ListField<DataAppSpecElFunctionElCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataAppSpecElFunctionElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<ListField<DataAppSpecElFunctionElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<ListField<DataAppSpecElFunctionElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<ListField<DataAppSpecElFunctionElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<ListField<DataAppSpecElFunctionElLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<ListField<DataAppSpecElFunctionElRoutesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
}

impl DataAppSpecElFunctionEl {
    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<ListField<DataAppSpecElFunctionElAlertEl>>) -> Self {
        self.alert = Some(v.into());
        self
    }

    #[doc= "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl Into<ListField<DataAppSpecElFunctionElCorsEl>>) -> Self {
        self.cors = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<SetField<DataAppSpecElFunctionElEnvEl>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<ListField<DataAppSpecElFunctionElGitEl>>) -> Self {
        self.git = Some(v.into());
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<ListField<DataAppSpecElFunctionElGithubEl>>) -> Self {
        self.github = Some(v.into());
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<ListField<DataAppSpecElFunctionElGitlabEl>>) -> Self {
        self.gitlab = Some(v.into());
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<ListField<DataAppSpecElFunctionElLogDestinationEl>>) -> Self {
        self.log_destination = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl Into<ListField<DataAppSpecElFunctionElRoutesEl>>) -> Self {
        self.routes = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElFunctionEl {
    type O = BlockAssignable<DataAppSpecElFunctionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElFunctionElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElFunctionElRef {
        DataAppSpecElFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<DataAppSpecElFunctionElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<DataAppSpecElFunctionElCorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataAppSpecElFunctionElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<DataAppSpecElFunctionElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<DataAppSpecElFunctionElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<DataAppSpecElFunctionElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<DataAppSpecElFunctionElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataAppSpecElFunctionElRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<PrimField<String>>,
}

impl DataAppSpecElJobElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElAlertEl {
    type O = BlockAssignable<DataAppSpecElJobElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElAlertElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElAlertElRef {
        DataAppSpecElJobElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElAlertElRef {
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
pub struct DataAppSpecElJobElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppSpecElJobElEnvEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElEnvEl {
    type O = BlockAssignable<DataAppSpecElJobElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElEnvElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElEnvElRef {
        DataAppSpecElJobElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl DataAppSpecElJobElGitEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElGitEl {
    type O = BlockAssignable<DataAppSpecElJobElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElGitElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElGitElRef {
        DataAppSpecElJobElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElJobElGithubEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElGithubEl {
    type O = BlockAssignable<DataAppSpecElJobElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElGithubElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElGithubElRef {
        DataAppSpecElJobElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElJobElGitlabEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElGitlabEl {
    type O = BlockAssignable<DataAppSpecElJobElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElGitlabElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElGitlabElRef {
        DataAppSpecElJobElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataAppSpecElJobElImageElDeployOnPushEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElImageElDeployOnPushEl {
    type O = BlockAssignable<DataAppSpecElJobElImageElDeployOnPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElImageElDeployOnPushElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElImageElDeployOnPushElRef {
        DataAppSpecElJobElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElImageElDeployOnPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<ListField<DataAppSpecElJobElImageElDeployOnPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}

impl DataAppSpecElJobElImageEl {
    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<ListField<DataAppSpecElJobElImageElDeployOnPushEl>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }

    #[doc= "Set the field `registry_type`.\n"]
    pub fn set_registry_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_type = Some(v.into());
        self
    }

    #[doc= "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElImageEl {
    type O = BlockAssignable<DataAppSpecElJobElImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElImageElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElImageElRef {
        DataAppSpecElJobElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> ListRef<DataAppSpecElJobElImageElDeployOnPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_type` after provisioning.\n"]
    pub fn registry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElJobElLogDestinationElDatadogEl {
    #[doc= "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElLogDestinationElDatadogEl {
    type O = BlockAssignable<DataAppSpecElJobElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElLogDestinationElDatadogElRef {
        DataAppSpecElJobElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

impl DataAppSpecElJobElLogDestinationElLogtailEl {
    #[doc= "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElLogDestinationElLogtailEl {
    type O = BlockAssignable<DataAppSpecElJobElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElLogDestinationElLogtailElRef {
        DataAppSpecElJobElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElJobElLogDestinationElPapertrailEl {
    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElLogDestinationElPapertrailEl {
    type O = BlockAssignable<DataAppSpecElJobElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElLogDestinationElPapertrailElRef {
        DataAppSpecElJobElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobElLogDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<ListField<DataAppSpecElJobElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<ListField<DataAppSpecElJobElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<ListField<DataAppSpecElJobElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElJobElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<ListField<DataAppSpecElJobElLogDestinationElDatadogEl>>) -> Self {
        self.datadog = Some(v.into());
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<ListField<DataAppSpecElJobElLogDestinationElLogtailEl>>) -> Self {
        self.logtail = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(mut self, v: impl Into<ListField<DataAppSpecElJobElLogDestinationElPapertrailEl>>) -> Self {
        self.papertrail = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobElLogDestinationEl {
    type O = BlockAssignable<DataAppSpecElJobElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElLogDestinationElRef {
        DataAppSpecElJobElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<DataAppSpecElJobElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<DataAppSpecElJobElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<DataAppSpecElJobElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElJobEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<ListField<DataAppSpecElJobElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataAppSpecElJobElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<ListField<DataAppSpecElJobElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<ListField<DataAppSpecElJobElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<ListField<DataAppSpecElJobElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ListField<DataAppSpecElJobElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<ListField<DataAppSpecElJobElLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
}

impl DataAppSpecElJobEl {
    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<ListField<DataAppSpecElJobElAlertEl>>) -> Self {
        self.alert = Some(v.into());
        self
    }

    #[doc= "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<SetField<DataAppSpecElJobElEnvEl>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<ListField<DataAppSpecElJobElGitEl>>) -> Self {
        self.git = Some(v.into());
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<ListField<DataAppSpecElJobElGithubEl>>) -> Self {
        self.github = Some(v.into());
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<ListField<DataAppSpecElJobElGitlabEl>>) -> Self {
        self.gitlab = Some(v.into());
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<ListField<DataAppSpecElJobElImageEl>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_size_slug`.\n"]
    pub fn set_instance_size_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `kind`.\n"]
    pub fn set_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kind = Some(v.into());
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<ListField<DataAppSpecElJobElLogDestinationEl>>) -> Self {
        self.log_destination = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `run_command`.\n"]
    pub fn set_run_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.run_command = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElJobEl {
    type O = BlockAssignable<DataAppSpecElJobEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElJobElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElJobElRef {
        DataAppSpecElJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<DataAppSpecElJobElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataAppSpecElJobElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<DataAppSpecElJobElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<DataAppSpecElJobElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<DataAppSpecElJobElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<DataAppSpecElJobElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_size_slug` after provisioning.\n"]
    pub fn instance_size_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<DataAppSpecElJobElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `run_command` after provisioning.\n"]
    pub fn run_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElAlertEl {
    type O = BlockAssignable<DataAppSpecElServiceElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElAlertElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElAlertElRef {
        DataAppSpecElServiceElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElAlertElRef {
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
pub struct DataAppSpecElServiceElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElCorsElAllowOriginsEl {
    #[doc= "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElCorsElAllowOriginsEl {
    type O = BlockAssignable<DataAppSpecElServiceElCorsElAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElCorsElAllowOriginsElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElCorsElAllowOriginsElRef {
        DataAppSpecElServiceElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElCorsElAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<DataAppSpecElServiceElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElCorsEl {
    #[doc= "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<DataAppSpecElServiceElCorsElAllowOriginsEl>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElCorsEl {
    type O = BlockAssignable<DataAppSpecElServiceElCorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElCorsElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElCorsElRef {
        DataAppSpecElServiceElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> ListRef<DataAppSpecElServiceElCorsElAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElEnvEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElEnvEl {
    type O = BlockAssignable<DataAppSpecElServiceElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElEnvElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElEnvElRef {
        DataAppSpecElServiceElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElGitEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElGitEl {
    type O = BlockAssignable<DataAppSpecElServiceElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElGitElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElGitElRef {
        DataAppSpecElServiceElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElGithubEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElGithubEl {
    type O = BlockAssignable<DataAppSpecElServiceElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElGithubElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElGithubElRef {
        DataAppSpecElServiceElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElGitlabEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElGitlabEl {
    type O = BlockAssignable<DataAppSpecElServiceElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElGitlabElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElGitlabElRef {
        DataAppSpecElServiceElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElHealthCheckEl {
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

impl DataAppSpecElServiceElHealthCheckEl {
    #[doc= "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `http_path`.\n"]
    pub fn set_http_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_path = Some(v.into());
        self
    }

    #[doc= "Set the field `initial_delay_seconds`.\n"]
    pub fn set_initial_delay_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.initial_delay_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `period_seconds`.\n"]
    pub fn set_period_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.period_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `success_threshold`.\n"]
    pub fn set_success_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.success_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElHealthCheckEl {
    type O = BlockAssignable<DataAppSpecElServiceElHealthCheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
        }
    }
}

pub struct DataAppSpecElServiceElHealthCheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElHealthCheckElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElHealthCheckElRef {
        DataAppSpecElServiceElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `http_path` after provisioning.\n"]
    pub fn http_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_path", self.base))
    }

    #[doc= "Get a reference to the value of field `initial_delay_seconds` after provisioning.\n"]
    pub fn initial_delay_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `period_seconds` after provisioning.\n"]
    pub fn period_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `success_threshold` after provisioning.\n"]
    pub fn success_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataAppSpecElServiceElImageElDeployOnPushEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElImageElDeployOnPushEl {
    type O = BlockAssignable<DataAppSpecElServiceElImageElDeployOnPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElImageElDeployOnPushElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElImageElDeployOnPushElRef {
        DataAppSpecElServiceElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElImageElDeployOnPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<ListField<DataAppSpecElServiceElImageElDeployOnPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElImageEl {
    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<ListField<DataAppSpecElServiceElImageElDeployOnPushEl>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }

    #[doc= "Set the field `registry_type`.\n"]
    pub fn set_registry_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_type = Some(v.into());
        self
    }

    #[doc= "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElImageEl {
    type O = BlockAssignable<DataAppSpecElServiceElImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElImageElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElImageElRef {
        DataAppSpecElServiceElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> ListRef<DataAppSpecElServiceElImageElDeployOnPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_type` after provisioning.\n"]
    pub fn registry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElLogDestinationElDatadogEl {
    #[doc= "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElLogDestinationElDatadogEl {
    type O = BlockAssignable<DataAppSpecElServiceElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElLogDestinationElDatadogElRef {
        DataAppSpecElServiceElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElLogDestinationElLogtailEl {
    #[doc= "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElLogDestinationElLogtailEl {
    type O = BlockAssignable<DataAppSpecElServiceElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElLogDestinationElLogtailElRef {
        DataAppSpecElServiceElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElServiceElLogDestinationElPapertrailEl {
    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElLogDestinationElPapertrailEl {
    type O = BlockAssignable<DataAppSpecElServiceElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElLogDestinationElPapertrailElRef {
        DataAppSpecElServiceElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElLogDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<ListField<DataAppSpecElServiceElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<ListField<DataAppSpecElServiceElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<ListField<DataAppSpecElServiceElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElServiceElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<ListField<DataAppSpecElServiceElLogDestinationElDatadogEl>>) -> Self {
        self.datadog = Some(v.into());
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<ListField<DataAppSpecElServiceElLogDestinationElLogtailEl>>) -> Self {
        self.logtail = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl Into<ListField<DataAppSpecElServiceElLogDestinationElPapertrailEl>>,
    ) -> Self {
        self.papertrail = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElLogDestinationEl {
    type O = BlockAssignable<DataAppSpecElServiceElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElLogDestinationElRef {
        DataAppSpecElServiceElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<DataAppSpecElServiceElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<DataAppSpecElServiceElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<DataAppSpecElServiceElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceElRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<PrimField<bool>>,
}

impl DataAppSpecElServiceElRoutesEl {
    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_path_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceElRoutesEl {
    type O = BlockAssignable<DataAppSpecElServiceElRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElRoutesElRef {
        DataAppSpecElServiceElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<ListField<DataAppSpecElServiceElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<ListField<DataAppSpecElServiceElCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataAppSpecElServiceElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<ListField<DataAppSpecElServiceElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<ListField<DataAppSpecElServiceElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<ListField<DataAppSpecElServiceElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<ListField<DataAppSpecElServiceElHealthCheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ListField<DataAppSpecElServiceElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ports: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<ListField<DataAppSpecElServiceElLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<ListField<DataAppSpecElServiceElRoutesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
}

impl DataAppSpecElServiceEl {
    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<ListField<DataAppSpecElServiceElAlertEl>>) -> Self {
        self.alert = Some(v.into());
        self
    }

    #[doc= "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl Into<ListField<DataAppSpecElServiceElCorsEl>>) -> Self {
        self.cors = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<SetField<DataAppSpecElServiceElEnvEl>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<ListField<DataAppSpecElServiceElGitEl>>) -> Self {
        self.git = Some(v.into());
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<ListField<DataAppSpecElServiceElGithubEl>>) -> Self {
        self.github = Some(v.into());
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<ListField<DataAppSpecElServiceElGitlabEl>>) -> Self {
        self.gitlab = Some(v.into());
        self
    }

    #[doc= "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl Into<ListField<DataAppSpecElServiceElHealthCheckEl>>) -> Self {
        self.health_check = Some(v.into());
        self
    }

    #[doc= "Set the field `http_port`.\n"]
    pub fn set_http_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_port = Some(v.into());
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<ListField<DataAppSpecElServiceElImageEl>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_size_slug`.\n"]
    pub fn set_instance_size_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ports`.\n"]
    pub fn set_internal_ports(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.internal_ports = Some(v.into());
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<ListField<DataAppSpecElServiceElLogDestinationEl>>) -> Self {
        self.log_destination = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl Into<ListField<DataAppSpecElServiceElRoutesEl>>) -> Self {
        self.routes = Some(v.into());
        self
    }

    #[doc= "Set the field `run_command`.\n"]
    pub fn set_run_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.run_command = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElServiceEl {
    type O = BlockAssignable<DataAppSpecElServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElServiceElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElServiceElRef {
        DataAppSpecElServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<DataAppSpecElServiceElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<DataAppSpecElServiceElCorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataAppSpecElServiceElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<DataAppSpecElServiceElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<DataAppSpecElServiceElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<DataAppSpecElServiceElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> ListRef<DataAppSpecElServiceElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc= "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<DataAppSpecElServiceElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_size_slug` after provisioning.\n"]
    pub fn instance_size_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `internal_ports` after provisioning.\n"]
    pub fn internal_ports(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.internal_ports", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<DataAppSpecElServiceElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataAppSpecElServiceElRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc= "Get a reference to the value of field `run_command` after provisioning.\n"]
    pub fn run_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    #[doc= "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElCorsElAllowOriginsEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElCorsElAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
        DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElCorsElAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElCorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<DataAppSpecElStaticSiteElCorsElAllowOriginsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteElCorsEl {
    #[doc= "Set the field `allow_credentials`.\n"]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\n"]
    pub fn set_allow_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\n"]
    pub fn set_allow_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\n"]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteElCorsElAllowOriginsEl>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\n"]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElCorsEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElCorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElCorsElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElCorsElRef {
        DataAppSpecElStaticSiteElCorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElCorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\n"]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\n"]
    pub fn allow_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\n"]
    pub fn allow_methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\n"]
    pub fn allow_origins(&self) -> ListRef<DataAppSpecElStaticSiteElCorsElAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\n"]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteElEnvEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElEnvEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElEnvElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElEnvElRef {
        DataAppSpecElStaticSiteElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteElGitEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElGitEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElGitElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElGitElRef {
        DataAppSpecElStaticSiteElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteElGithubEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElGithubEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElGithubElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElGithubElRef {
        DataAppSpecElStaticSiteElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteElGitlabEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElGitlabEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElGitlabElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElGitlabElRef {
        DataAppSpecElStaticSiteElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteElRoutesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_path_prefix: Option<PrimField<bool>>,
}

impl DataAppSpecElStaticSiteElRoutesEl {
    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_path_prefix`.\n"]
    pub fn set_preserve_path_prefix(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preserve_path_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteElRoutesEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteElRoutesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElRoutesElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElRoutesElRef {
        DataAppSpecElStaticSiteElRoutesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElRoutesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_path_prefix` after provisioning.\n"]
    pub fn preserve_path_prefix(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElStaticSiteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catchall_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors: Option<ListField<DataAppSpecElStaticSiteElCorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataAppSpecElStaticSiteElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<ListField<DataAppSpecElStaticSiteElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<ListField<DataAppSpecElStaticSiteElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<ListField<DataAppSpecElStaticSiteElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_document: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<ListField<DataAppSpecElStaticSiteElRoutesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
}

impl DataAppSpecElStaticSiteEl {
    #[doc= "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `catchall_document`.\n"]
    pub fn set_catchall_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catchall_document = Some(v.into());
        self
    }

    #[doc= "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteElCorsEl>>) -> Self {
        self.cors = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<SetField<DataAppSpecElStaticSiteElEnvEl>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `error_document`.\n"]
    pub fn set_error_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_document = Some(v.into());
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteElGitEl>>) -> Self {
        self.git = Some(v.into());
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteElGithubEl>>) -> Self {
        self.github = Some(v.into());
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteElGitlabEl>>) -> Self {
        self.gitlab = Some(v.into());
        self
    }

    #[doc= "Set the field `index_document`.\n"]
    pub fn set_index_document(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_document = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `output_dir`.\n"]
    pub fn set_output_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `routes`.\n"]
    pub fn set_routes(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteElRoutesEl>>) -> Self {
        self.routes = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElStaticSiteEl {
    type O = BlockAssignable<DataAppSpecElStaticSiteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElStaticSiteElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElStaticSiteElRef {
        DataAppSpecElStaticSiteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElStaticSiteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `catchall_document` after provisioning.\n"]
    pub fn catchall_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catchall_document", self.base))
    }

    #[doc= "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> ListRef<DataAppSpecElStaticSiteElCorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataAppSpecElStaticSiteElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_document", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<DataAppSpecElStaticSiteElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<DataAppSpecElStaticSiteElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<DataAppSpecElStaticSiteElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_document", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `output_dir` after provisioning.\n"]
    pub fn output_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> ListRef<DataAppSpecElStaticSiteElRoutesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElAlertEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElAlertEl {
    #[doc= "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `window`.\n"]
    pub fn set_window(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.window = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElAlertEl {
    type O = BlockAssignable<DataAppSpecElWorkerElAlertEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElAlertElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElAlertElRef {
        DataAppSpecElWorkerElAlertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElAlertElRef {
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
pub struct DataAppSpecElWorkerElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElEnvEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElEnvEl {
    type O = BlockAssignable<DataAppSpecElWorkerElEnvEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElEnvElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElEnvElRef {
        DataAppSpecElWorkerElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo_clone_url: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElGitEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `repo_clone_url`.\n"]
    pub fn set_repo_clone_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo_clone_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElGitEl {
    type O = BlockAssignable<DataAppSpecElWorkerElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElGitElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElGitElRef {
        DataAppSpecElWorkerElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `repo_clone_url` after provisioning.\n"]
    pub fn repo_clone_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo_clone_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElGithubEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElGithubEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElGithubEl {
    type O = BlockAssignable<DataAppSpecElWorkerElGithubEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElGithubElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElGithubElRef {
        DataAppSpecElWorkerElGithubElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElGithubElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElGitlabEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repo: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElGitlabEl {
    #[doc= "Set the field `branch`.\n"]
    pub fn set_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch = Some(v.into());
        self
    }

    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `repo`.\n"]
    pub fn set_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repo = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElGitlabEl {
    type O = BlockAssignable<DataAppSpecElWorkerElGitlabEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElGitlabElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElGitlabElRef {
        DataAppSpecElWorkerElGitlabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElGitlabElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch` after provisioning.\n"]
    pub fn branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `repo` after provisioning.\n"]
    pub fn repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElImageElDeployOnPushEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataAppSpecElWorkerElImageElDeployOnPushEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElImageElDeployOnPushEl {
    type O = BlockAssignable<DataAppSpecElWorkerElImageElDeployOnPushEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElImageElDeployOnPushElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElImageElDeployOnPushElRef {
        DataAppSpecElWorkerElImageElDeployOnPushElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElImageElDeployOnPushElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy_on_push: Option<ListField<DataAppSpecElWorkerElImageElDeployOnPushEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElImageEl {
    #[doc= "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(mut self, v: impl Into<ListField<DataAppSpecElWorkerElImageElDeployOnPushEl>>) -> Self {
        self.deploy_on_push = Some(v.into());
        self
    }

    #[doc= "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry = Some(v.into());
        self
    }

    #[doc= "Set the field `registry_type`.\n"]
    pub fn set_registry_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_type = Some(v.into());
        self
    }

    #[doc= "Set the field `repository`.\n"]
    pub fn set_repository(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElImageEl {
    type O = BlockAssignable<DataAppSpecElWorkerElImageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElImageElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElImageElRef {
        DataAppSpecElWorkerElImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> ListRef<DataAppSpecElWorkerElImageElDeployOnPushElRef> {
        ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc= "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc= "Get a reference to the value of field `registry_type` after provisioning.\n"]
    pub fn registry_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_type", self.base))
    }

    #[doc= "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationElDatadogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElLogDestinationElDatadogEl {
    #[doc= "Set the field `api_key`.\n"]
    pub fn set_api_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElLogDestinationElDatadogEl {
    type O = BlockAssignable<DataAppSpecElWorkerElLogDestinationElDatadogEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElLogDestinationElDatadogElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElLogDestinationElDatadogElRef {
        DataAppSpecElWorkerElLogDestinationElDatadogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElDatadogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationElLogtailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElLogDestinationElLogtailEl {
    #[doc= "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElLogDestinationElLogtailEl {
    type O = BlockAssignable<DataAppSpecElWorkerElLogDestinationElLogtailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElLogDestinationElLogtailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElLogDestinationElLogtailElRef {
        DataAppSpecElWorkerElLogDestinationElLogtailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElLogtailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationElPapertrailEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerElLogDestinationElPapertrailEl {
    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElLogDestinationElPapertrailEl {
    type O = BlockAssignable<DataAppSpecElWorkerElLogDestinationElPapertrailEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
        DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElPapertrailElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerElLogDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    datadog: Option<ListField<DataAppSpecElWorkerElLogDestinationElDatadogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logtail: Option<ListField<DataAppSpecElWorkerElLogDestinationElLogtailEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    papertrail: Option<ListField<DataAppSpecElWorkerElLogDestinationElPapertrailEl>>,
}

impl DataAppSpecElWorkerElLogDestinationEl {
    #[doc= "Set the field `datadog`.\n"]
    pub fn set_datadog(mut self, v: impl Into<ListField<DataAppSpecElWorkerElLogDestinationElDatadogEl>>) -> Self {
        self.datadog = Some(v.into());
        self
    }

    #[doc= "Set the field `logtail`.\n"]
    pub fn set_logtail(mut self, v: impl Into<ListField<DataAppSpecElWorkerElLogDestinationElLogtailEl>>) -> Self {
        self.logtail = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `papertrail`.\n"]
    pub fn set_papertrail(mut self, v: impl Into<ListField<DataAppSpecElWorkerElLogDestinationElPapertrailEl>>) -> Self {
        self.papertrail = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerElLogDestinationEl {
    type O = BlockAssignable<DataAppSpecElWorkerElLogDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElLogDestinationElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElLogDestinationElRef {
        DataAppSpecElWorkerElLogDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElLogDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> ListRef<DataAppSpecElWorkerElLogDestinationElDatadogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc= "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> ListRef<DataAppSpecElWorkerElLogDestinationElLogtailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> ListRef<DataAppSpecElWorkerElLogDestinationElPapertrailElRef> {
        ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecElWorkerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<ListField<DataAppSpecElWorkerElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dockerfile_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataAppSpecElWorkerElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<ListField<DataAppSpecElWorkerElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    github: Option<ListField<DataAppSpecElWorkerElGithubEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gitlab: Option<ListField<DataAppSpecElWorkerElGitlabEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ListField<DataAppSpecElWorkerElImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_size_slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<ListField<DataAppSpecElWorkerElLogDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dir: Option<PrimField<String>>,
}

impl DataAppSpecElWorkerEl {
    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<ListField<DataAppSpecElWorkerElAlertEl>>) -> Self {
        self.alert = Some(v.into());
        self
    }

    #[doc= "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build_command = Some(v.into());
        self
    }

    #[doc= "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dockerfile_path = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<SetField<DataAppSpecElWorkerElEnvEl>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl Into<ListField<DataAppSpecElWorkerElGitEl>>) -> Self {
        self.git = Some(v.into());
        self
    }

    #[doc= "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl Into<ListField<DataAppSpecElWorkerElGithubEl>>) -> Self {
        self.github = Some(v.into());
        self
    }

    #[doc= "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl Into<ListField<DataAppSpecElWorkerElGitlabEl>>) -> Self {
        self.gitlab = Some(v.into());
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<ListField<DataAppSpecElWorkerElImageEl>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_size_slug`.\n"]
    pub fn set_instance_size_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_size_slug = Some(v.into());
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<ListField<DataAppSpecElWorkerElLogDestinationEl>>) -> Self {
        self.log_destination = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `run_command`.\n"]
    pub fn set_run_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.run_command = Some(v.into());
        self
    }

    #[doc= "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dir = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecElWorkerEl {
    type O = BlockAssignable<DataAppSpecElWorkerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElWorkerElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElWorkerElRef {
        DataAppSpecElWorkerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElWorkerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> ListRef<DataAppSpecElWorkerElAlertElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc= "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataAppSpecElWorkerElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<DataAppSpecElWorkerElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> ListRef<DataAppSpecElWorkerElGithubElRef> {
        ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc= "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> ListRef<DataAppSpecElWorkerElGitlabElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> ListRef<DataAppSpecElWorkerElImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_size_slug` after provisioning.\n"]
    pub fn instance_size_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_size_slug", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> ListRef<DataAppSpecElWorkerElLogDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `run_command` after provisioning.\n"]
    pub fn run_command(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.run_command", self.base))
    }

    #[doc= "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<SetField<DataAppSpecElAlertEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<ListField<DataAppSpecElDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<ListField<DataAppSpecElDomainEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domains: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<SetField<DataAppSpecElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<ListField<DataAppSpecElFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    job: Option<ListField<DataAppSpecElJobEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<ListField<DataAppSpecElServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    static_site: Option<ListField<DataAppSpecElStaticSiteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker: Option<ListField<DataAppSpecElWorkerEl>>,
}

impl DataAppSpecEl {
    #[doc= "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl Into<SetField<DataAppSpecElAlertEl>>) -> Self {
        self.alert = Some(v.into());
        self
    }

    #[doc= "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl Into<ListField<DataAppSpecElDatabaseEl>>) -> Self {
        self.database = Some(v.into());
        self
    }

    #[doc= "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<ListField<DataAppSpecElDomainEl>>) -> Self {
        self.domain = Some(v.into());
        self
    }

    #[doc= "Set the field `domains`.\n"]
    pub fn set_domains(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.domains = Some(v.into());
        self
    }

    #[doc= "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl Into<SetField<DataAppSpecElEnvEl>>) -> Self {
        self.env = Some(v.into());
        self
    }

    #[doc= "Set the field `function`.\n"]
    pub fn set_function(mut self, v: impl Into<ListField<DataAppSpecElFunctionEl>>) -> Self {
        self.function = Some(v.into());
        self
    }

    #[doc= "Set the field `job`.\n"]
    pub fn set_job(mut self, v: impl Into<ListField<DataAppSpecElJobEl>>) -> Self {
        self.job = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<ListField<DataAppSpecElServiceEl>>) -> Self {
        self.service = Some(v.into());
        self
    }

    #[doc= "Set the field `static_site`.\n"]
    pub fn set_static_site(mut self, v: impl Into<ListField<DataAppSpecElStaticSiteEl>>) -> Self {
        self.static_site = Some(v.into());
        self
    }

    #[doc= "Set the field `worker`.\n"]
    pub fn set_worker(mut self, v: impl Into<ListField<DataAppSpecElWorkerEl>>) -> Self {
        self.worker = Some(v.into());
        self
    }
}

impl ToListMappable for DataAppSpecEl {
    type O = BlockAssignable<DataAppSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            function: core::default::Default::default(),
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAppSpecElRef {
    fn new(shared: StackShared, base: String) -> DataAppSpecElRef {
        DataAppSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> SetRef<DataAppSpecElAlertElRef> {
        SetRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<DataAppSpecElDatabaseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> ListRef<DataAppSpecElDomainElRef> {
        ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc= "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc= "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> SetRef<DataAppSpecElEnvElRef> {
        SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc= "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> ListRef<DataAppSpecElFunctionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc= "Get a reference to the value of field `job` after provisioning.\n"]
    pub fn job(&self) -> ListRef<DataAppSpecElJobElRef> {
        ListRef::new(self.shared().clone(), format!("{}.job", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> ListRef<DataAppSpecElServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc= "Get a reference to the value of field `static_site` after provisioning.\n"]
    pub fn static_site(&self) -> ListRef<DataAppSpecElStaticSiteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.static_site", self.base))
    }

    #[doc= "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> ListRef<DataAppSpecElWorkerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}
