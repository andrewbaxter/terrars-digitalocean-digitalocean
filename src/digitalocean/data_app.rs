#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAppData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_ips: std::option::Option<std::vec::Vec<DataApp_DedicatedIps>>,
    dynamic: DataAppDynamic,
}

struct DataApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppData_>,
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

    #[doc = "Set the field `dedicated_ips`.\n"]
    pub fn set_dedicated_ips(self, v: impl std::convert::Into<terrars::BlockAssignable<DataApp_DedicatedIps>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_ips = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_ips = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `live_domain` after provisioning.\n"]
    pub fn live_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `live_url` after provisioning.\n"]
    pub fn live_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\nA DigitalOcean App Platform Spec"]
    pub fn spec(&self) -> terrars::ListRef<DataApp_SpecRef> {
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

    #[doc = "Get a reference to the value of field `dedicated_ips` after provisioning.\n"]
    pub fn dedicated_ips(&self) -> terrars::ListRef<DataApp_DedicatedIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_ips", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataAppData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                app_id: self.app_id,
                id: core::default::Default::default(),
                dedicated_ips: core::default::Default::default(),
                dynamic: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `live_domain` after provisioning.\n"]
    pub fn live_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `live_url` after provisioning.\n"]
    pub fn live_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\nA DigitalOcean App Platform Spec"]
    pub fn spec(&self) -> terrars::ListRef<DataApp_SpecRef> {
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

    #[doc = "Get a reference to the value of field `dedicated_ips` after provisioning.\n"]
    pub fn dedicated_ips(&self) -> terrars::ListRef<DataApp_DedicatedIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_ips", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataAppDynamic {
    dedicated_ips: std::option::Option<terrars::DynamicBlock<DataApp_DedicatedIps>>,
}

#[derive(Serialize)]
pub struct DataApp_Spec_Alert_Destinations_SlackWebhooks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Alert_Destinations_SlackWebhooks {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Alert_Destinations_SlackWebhooks {
    type O = terrars::BlockAssignable<DataApp_Spec_Alert_Destinations_SlackWebhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Alert_Destinations_SlackWebhooks {}

impl BuildDataApp_Spec_Alert_Destinations_SlackWebhooks {
    pub fn build(self) -> DataApp_Spec_Alert_Destinations_SlackWebhooks {
        DataApp_Spec_Alert_Destinations_SlackWebhooks {
            channel: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Alert_Destinations_SlackWebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Alert_Destinations_SlackWebhooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApp_Spec_Alert_Destinations_SlackWebhooksRef {
        DataApp_Spec_Alert_Destinations_SlackWebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Alert_Destinations_SlackWebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Alert_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_webhooks: std::option::Option<terrars::ListField<DataApp_Spec_Alert_Destinations_SlackWebhooks>>,
}

impl DataApp_Spec_Alert_Destinations {
    #[doc = "Set the field `emails`.\n"]
    pub fn set_emails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slack_webhooks`.\n"]
    pub fn set_slack_webhooks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Alert_Destinations_SlackWebhooks>>,
    ) -> Self {
        self.slack_webhooks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Alert_Destinations {
    type O = terrars::BlockAssignable<DataApp_Spec_Alert_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Alert_Destinations {}

impl BuildDataApp_Spec_Alert_Destinations {
    pub fn build(self) -> DataApp_Spec_Alert_Destinations {
        DataApp_Spec_Alert_Destinations {
            emails: core::default::Default::default(),
            slack_webhooks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Alert_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Alert_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Alert_DestinationsRef {
        DataApp_Spec_Alert_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Alert_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_webhooks` after provisioning.\n"]
    pub fn slack_webhooks(&self) -> terrars::ListRef<DataApp_Spec_Alert_Destinations_SlackWebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_webhooks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::SetField<DataApp_Spec_Alert_Destinations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Alert {
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Alert_Destinations>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataApp_Spec_Alert {
    type O = terrars::BlockAssignable<DataApp_Spec_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Alert {}

impl BuildDataApp_Spec_Alert {
    pub fn build(self) -> DataApp_Spec_Alert {
        DataApp_Spec_Alert {
            destinations: core::default::Default::default(),
            disabled: core::default::Default::default(),
            rule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_AlertRef {
        DataApp_Spec_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_AlertRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::SetRef<DataApp_Spec_Alert_DestinationsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destinations", self.base))
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
pub struct DataApp_Spec_Database {
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

impl DataApp_Spec_Database {
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

impl terrars::ToListMappable for DataApp_Spec_Database {
    type O = terrars::BlockAssignable<DataApp_Spec_Database>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Database {}

impl BuildDataApp_Spec_Database {
    pub fn build(self) -> DataApp_Spec_Database {
        DataApp_Spec_Database {
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

#[allow(dead_code)]
pub struct DataApp_Spec_DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_DatabaseRef {
        DataApp_Spec_DatabaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_DatabaseRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Domain {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Domain {
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

impl terrars::ToListMappable for DataApp_Spec_Domain {
    type O = terrars::BlockAssignable<DataApp_Spec_Domain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Domain {}

impl BuildDataApp_Spec_Domain {
    pub fn build(self) -> DataApp_Spec_Domain {
        DataApp_Spec_Domain {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_DomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_DomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_DomainRef {
        DataApp_Spec_DomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_DomainRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Egress {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Egress {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Egress {
    type O = terrars::BlockAssignable<DataApp_Spec_Egress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Egress {}

impl BuildDataApp_Spec_Egress {
    pub fn build(self) -> DataApp_Spec_Egress {
        DataApp_Spec_Egress { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_EgressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_EgressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_EgressRef {
        DataApp_Spec_EgressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_EgressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Env {
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

impl terrars::ToListMappable for DataApp_Spec_Env {
    type O = terrars::BlockAssignable<DataApp_Spec_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Env {}

impl BuildDataApp_Spec_Env {
    pub fn build(self) -> DataApp_Spec_Env {
        DataApp_Spec_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_EnvRef {
        DataApp_Spec_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_EnvRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Job_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<terrars::SetField<DataApp_Spec_Alert_Destinations>>,
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

impl DataApp_Spec_Job_Alert {
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Alert_Destinations>>,
    ) -> Self {
        self.destinations = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataApp_Spec_Job_Alert {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Alert {}

impl BuildDataApp_Spec_Job_Alert {
    pub fn build(self) -> DataApp_Spec_Job_Alert {
        DataApp_Spec_Job_Alert {
            destinations: core::default::Default::default(),
            disabled: core::default::Default::default(),
            operator: core::default::Default::default(),
            rule: core::default::Default::default(),
            value: core::default::Default::default(),
            window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_AlertRef {
        DataApp_Spec_Job_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_AlertRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::SetRef<DataApp_Spec_Alert_DestinationsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destinations", self.base))
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
pub struct DataApp_Spec_Job_Bitbucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_Bitbucket {
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

impl terrars::ToListMappable for DataApp_Spec_Job_Bitbucket {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Bitbucket {}

impl BuildDataApp_Spec_Job_Bitbucket {
    pub fn build(self) -> DataApp_Spec_Job_Bitbucket {
        DataApp_Spec_Job_Bitbucket {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_BitbucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_BitbucketRef {
        DataApp_Spec_Job_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_BitbucketRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Service_Cors_AllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Service_Cors_AllowOrigins {
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

impl terrars::ToListMappable for DataApp_Spec_Service_Cors_AllowOrigins {
    type O = terrars::BlockAssignable<DataApp_Spec_Service_Cors_AllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Service_Cors_AllowOrigins {}

impl BuildDataApp_Spec_Service_Cors_AllowOrigins {
    pub fn build(self) -> DataApp_Spec_Service_Cors_AllowOrigins {
        DataApp_Spec_Service_Cors_AllowOrigins {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Service_Cors_AllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Service_Cors_AllowOriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Service_Cors_AllowOriginsRef {
        DataApp_Spec_Service_Cors_AllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Service_Cors_AllowOriginsRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Service_Cors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<DataApp_Spec_Service_Cors_AllowOrigins>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Service_Cors {
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
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Cors_AllowOrigins>>,
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

impl terrars::ToListMappable for DataApp_Spec_Service_Cors {
    type O = terrars::BlockAssignable<DataApp_Spec_Service_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Service_Cors {}

impl BuildDataApp_Spec_Service_Cors {
    pub fn build(self) -> DataApp_Spec_Service_Cors {
        DataApp_Spec_Service_Cors {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Service_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Service_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Service_CorsRef {
        DataApp_Spec_Service_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Service_CorsRef {
    #[allow(dead_code)]
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
    pub fn allow_origins(&self) -> terrars::ListRef<DataApp_Spec_Service_Cors_AllowOriginsRef> {
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
pub struct DataApp_Spec_Job_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_Git {
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

impl terrars::ToListMappable for DataApp_Spec_Job_Git {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Git {}

impl BuildDataApp_Spec_Job_Git {
    pub fn build(self) -> DataApp_Spec_Job_Git {
        DataApp_Spec_Job_Git {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_GitRef {
        DataApp_Spec_Job_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_GitRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Job_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_Github {
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

impl terrars::ToListMappable for DataApp_Spec_Job_Github {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Github {}

impl BuildDataApp_Spec_Job_Github {
    pub fn build(self) -> DataApp_Spec_Job_Github {
        DataApp_Spec_Job_Github {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_GithubRef {
        DataApp_Spec_Job_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_GithubRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Job_Gitlab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_Gitlab {
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

impl terrars::ToListMappable for DataApp_Spec_Job_Gitlab {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Gitlab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Gitlab {}

impl BuildDataApp_Spec_Job_Gitlab {
    pub fn build(self) -> DataApp_Spec_Job_Gitlab {
        DataApp_Spec_Job_Gitlab {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_GitlabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_GitlabRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_GitlabRef {
        DataApp_Spec_Job_GitlabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_GitlabRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Job_LogDestination_Datadog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_LogDestination_Datadog {
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

impl terrars::ToListMappable for DataApp_Spec_Job_LogDestination_Datadog {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_LogDestination_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_LogDestination_Datadog {}

impl BuildDataApp_Spec_Job_LogDestination_Datadog {
    pub fn build(self) -> DataApp_Spec_Job_LogDestination_Datadog {
        DataApp_Spec_Job_LogDestination_Datadog {
            api_key: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_LogDestination_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_LogDestination_DatadogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_LogDestination_DatadogRef {
        DataApp_Spec_Job_LogDestination_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_LogDestination_DatadogRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Job_LogDestination_Logtail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_LogDestination_Logtail {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_LogDestination_Logtail {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_LogDestination_Logtail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_LogDestination_Logtail {}

impl BuildDataApp_Spec_Job_LogDestination_Logtail {
    pub fn build(self) -> DataApp_Spec_Job_LogDestination_Logtail {
        DataApp_Spec_Job_LogDestination_Logtail { token: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_LogDestination_LogtailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_LogDestination_LogtailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_LogDestination_LogtailRef {
        DataApp_Spec_Job_LogDestination_LogtailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_LogDestination_LogtailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {}

impl BuildDataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    pub fn build(self) -> DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
        DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
            password: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
        DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job_LogDestination_OpenSearch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_LogDestination_OpenSearch {
    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuth>>,
    ) -> Self {
        self.basic_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_name`.\n"]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\n"]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_LogDestination_OpenSearch {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_LogDestination_OpenSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_LogDestination_OpenSearch {}

impl BuildDataApp_Spec_Job_LogDestination_OpenSearch {
    pub fn build(self) -> DataApp_Spec_Job_LogDestination_OpenSearch {
        DataApp_Spec_Job_LogDestination_OpenSearch {
            basic_auth: core::default::Default::default(),
            cluster_name: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            index_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_LogDestination_OpenSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_LogDestination_OpenSearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_LogDestination_OpenSearchRef {
        DataApp_Spec_Job_LogDestination_OpenSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_LogDestination_OpenSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestination_OpenSearch_BasicAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job_LogDestination_Papertrail {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_LogDestination_Papertrail {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_LogDestination_Papertrail {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_LogDestination_Papertrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_LogDestination_Papertrail {}

impl BuildDataApp_Spec_Job_LogDestination_Papertrail {
    pub fn build(self) -> DataApp_Spec_Job_LogDestination_Papertrail {
        DataApp_Spec_Job_LogDestination_Papertrail { endpoint: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_LogDestination_PapertrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_LogDestination_PapertrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_LogDestination_PapertrailRef {
        DataApp_Spec_Job_LogDestination_PapertrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_LogDestination_PapertrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job_LogDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination_Logtail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_search: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination_OpenSearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination_Papertrail>>,
}

impl DataApp_Spec_Job_LogDestination {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination_Datadog>>,
    ) -> Self {
        self.datadog = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logtail`.\n"]
    pub fn set_logtail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination_Logtail>>,
    ) -> Self {
        self.logtail = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_search`.\n"]
    pub fn set_open_search(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination_OpenSearch>>,
    ) -> Self {
        self.open_search = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination_Papertrail>>,
    ) -> Self {
        self.papertrail = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_LogDestination {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_LogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_LogDestination {}

impl BuildDataApp_Spec_Job_LogDestination {
    pub fn build(self) -> DataApp_Spec_Job_LogDestination {
        DataApp_Spec_Job_LogDestination {
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            name: core::default::Default::default(),
            open_search: core::default::Default::default(),
            papertrail: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_LogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_LogDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_LogDestinationRef {
        DataApp_Spec_Job_LogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_LogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestination_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestination_LogtailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `open_search` after provisioning.\n"]
    pub fn open_search(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestination_OpenSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_search", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestination_PapertrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Service_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl DataApp_Spec_Service_Routes {
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

impl terrars::ToListMappable for DataApp_Spec_Service_Routes {
    type O = terrars::BlockAssignable<DataApp_Spec_Service_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Service_Routes {}

impl BuildDataApp_Spec_Service_Routes {
    pub fn build(self) -> DataApp_Spec_Service_Routes {
        DataApp_Spec_Service_Routes {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Service_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Service_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Service_RoutesRef {
        DataApp_Spec_Service_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Service_RoutesRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Function {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataApp_Spec_Job_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<terrars::ListField<DataApp_Spec_Job_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataApp_Spec_Service_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataApp_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataApp_Spec_Job_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataApp_Spec_Job_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataApp_Spec_Job_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<terrars::ListField<DataApp_Spec_Service_Routes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Function {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Alert>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Bitbucket>>) -> Self {
        self.bitbucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Cors>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Env>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Git>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Github>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Gitlab>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
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
    pub fn set_routes(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Routes>>) -> Self {
        self.routes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Function {
    type O = terrars::BlockAssignable<DataApp_Spec_Function>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Function {}

impl BuildDataApp_Spec_Function {
    pub fn build(self) -> DataApp_Spec_Function {
        DataApp_Spec_Function {
            alert: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct DataApp_Spec_FunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_FunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_FunctionRef {
        DataApp_Spec_FunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_FunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataApp_Spec_Job_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<DataApp_Spec_Job_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataApp_Spec_Service_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataApp_Spec_EnvRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataApp_Spec_Job_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataApp_Spec_Job_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataApp_Spec_Job_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataApp_Spec_Service_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Ingress_Rule_Component {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Ingress_Rule_Component {
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

impl terrars::ToListMappable for DataApp_Spec_Ingress_Rule_Component {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_Rule_Component>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_Rule_Component {}

impl BuildDataApp_Spec_Ingress_Rule_Component {
    pub fn build(self) -> DataApp_Spec_Ingress_Rule_Component {
        DataApp_Spec_Ingress_Rule_Component {
            name: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
            rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_Rule_ComponentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_Rule_ComponentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_Rule_ComponentRef {
        DataApp_Spec_Ingress_Rule_ComponentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_Rule_ComponentRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Ingress_Rule_Match_Authority {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Ingress_Rule_Match_Authority {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Ingress_Rule_Match_Authority {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_Rule_Match_Authority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_Rule_Match_Authority {}

impl BuildDataApp_Spec_Ingress_Rule_Match_Authority {
    pub fn build(self) -> DataApp_Spec_Ingress_Rule_Match_Authority {
        DataApp_Spec_Ingress_Rule_Match_Authority { exact: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_Rule_Match_AuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_Rule_Match_AuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_Rule_Match_AuthorityRef {
        DataApp_Spec_Ingress_Rule_Match_AuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_Rule_Match_AuthorityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Ingress_Rule_Match_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Ingress_Rule_Match_Path {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Ingress_Rule_Match_Path {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_Rule_Match_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_Rule_Match_Path {}

impl BuildDataApp_Spec_Ingress_Rule_Match_Path {
    pub fn build(self) -> DataApp_Spec_Ingress_Rule_Match_Path {
        DataApp_Spec_Ingress_Rule_Match_Path { prefix: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_Rule_Match_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_Rule_Match_PathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_Rule_Match_PathRef {
        DataApp_Spec_Ingress_Rule_Match_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_Rule_Match_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Ingress_Rule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authority: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_Rule_Match_Authority>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_Rule_Match_Path>>,
}

impl DataApp_Spec_Ingress_Rule_Match {
    #[doc = "Set the field `authority`.\n"]
    pub fn set_authority(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_Rule_Match_Authority>>,
    ) -> Self {
        self.authority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_Rule_Match_Path>>,
    ) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Ingress_Rule_Match {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_Rule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_Rule_Match {}

impl BuildDataApp_Spec_Ingress_Rule_Match {
    pub fn build(self) -> DataApp_Spec_Ingress_Rule_Match {
        DataApp_Spec_Ingress_Rule_Match {
            authority: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_Rule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_Rule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_Rule_MatchRef {
        DataApp_Spec_Ingress_Rule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_Rule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> terrars::ListRef<DataApp_Spec_Ingress_Rule_Match_AuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authority", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<DataApp_Spec_Ingress_Rule_Match_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Ingress_Rule_Redirect {
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

impl DataApp_Spec_Ingress_Rule_Redirect {
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

impl terrars::ToListMappable for DataApp_Spec_Ingress_Rule_Redirect {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_Rule_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_Rule_Redirect {}

impl BuildDataApp_Spec_Ingress_Rule_Redirect {
    pub fn build(self) -> DataApp_Spec_Ingress_Rule_Redirect {
        DataApp_Spec_Ingress_Rule_Redirect {
            authority: core::default::Default::default(),
            port: core::default::Default::default(),
            redirect_code: core::default::Default::default(),
            scheme: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_Rule_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_Rule_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_Rule_RedirectRef {
        DataApp_Spec_Ingress_Rule_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_Rule_RedirectRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Ingress_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_Rule_Component>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataApp_Spec_Service_Cors>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_Rule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_Rule_Redirect>>,
}

impl DataApp_Spec_Ingress_Rule {
    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_Rule_Component>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Cors>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_Rule_Match>>) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_Rule_Redirect>>,
    ) -> Self {
        self.redirect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Ingress_Rule {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_Rule {}

impl BuildDataApp_Spec_Ingress_Rule {
    pub fn build(self) -> DataApp_Spec_Ingress_Rule {
        DataApp_Spec_Ingress_Rule {
            component: core::default::Default::default(),
            cors: core::default::Default::default(),
            match_: core::default::Default::default(),
            redirect: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_RuleRef {
        DataApp_Spec_Ingress_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataApp_Spec_Ingress_Rule_ComponentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataApp_Spec_Service_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataApp_Spec_Ingress_Rule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<DataApp_Spec_Ingress_Rule_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Ingress_SecureHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Ingress_SecureHeader {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Ingress_SecureHeader {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress_SecureHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress_SecureHeader {}

impl BuildDataApp_Spec_Ingress_SecureHeader {
    pub fn build(self) -> DataApp_Spec_Ingress_SecureHeader {
        DataApp_Spec_Ingress_SecureHeader {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Ingress_SecureHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Ingress_SecureHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Ingress_SecureHeaderRef {
        DataApp_Spec_Ingress_SecureHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Ingress_SecureHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Ingress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure_header: std::option::Option<terrars::ListField<DataApp_Spec_Ingress_SecureHeader>>,
}

impl DataApp_Spec_Ingress {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_Rule>>) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secure_header`.\n"]
    pub fn set_secure_header(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress_SecureHeader>>,
    ) -> Self {
        self.secure_header = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Ingress {
    type O = terrars::BlockAssignable<DataApp_Spec_Ingress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Ingress {}

impl BuildDataApp_Spec_Ingress {
    pub fn build(self) -> DataApp_Spec_Ingress {
        DataApp_Spec_Ingress {
            rule: core::default::Default::default(),
            secure_header: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_IngressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_IngressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_IngressRef {
        DataApp_Spec_IngressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_IngressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataApp_Spec_Ingress_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `secure_header` after provisioning.\n"]
    pub fn secure_header(&self) -> terrars::ListRef<DataApp_Spec_Ingress_SecureHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secure_header", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job_Image_DeployOnPush {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataApp_Spec_Job_Image_DeployOnPush {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_Image_DeployOnPush {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Image_DeployOnPush>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Image_DeployOnPush {}

impl BuildDataApp_Spec_Job_Image_DeployOnPush {
    pub fn build(self) -> DataApp_Spec_Job_Image_DeployOnPush {
        DataApp_Spec_Job_Image_DeployOnPush { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_Image_DeployOnPushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_Image_DeployOnPushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_Image_DeployOnPushRef {
        DataApp_Spec_Job_Image_DeployOnPushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_Image_DeployOnPushRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job_Image {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::ListField<DataApp_Spec_Job_Image_DeployOnPush>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Job_Image {
    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Image_DeployOnPush>>,
    ) -> Self {
        self.deploy_on_push = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digest`.\n"]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\n"]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_credentials`.\n"]
    pub fn set_registry_credentials(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.registry_credentials = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataApp_Spec_Job_Image {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Image {}

impl BuildDataApp_Spec_Job_Image {
    pub fn build(self) -> DataApp_Spec_Job_Image {
        DataApp_Spec_Job_Image {
            deploy_on_push: core::default::Default::default(),
            digest: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_credentials: core::default::Default::default(),
            registry_type: core::default::Default::default(),
            repository: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_ImageRef {
        DataApp_Spec_Job_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<DataApp_Spec_Job_Image_DeployOnPushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }

    #[doc = "Get a reference to the value of field `digest` after provisioning.\n"]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc = "Get a reference to the value of field `registry` after provisioning.\n"]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_credentials` after provisioning.\n"]
    pub fn registry_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_credentials", self.base))
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
pub struct DataApp_Spec_Job_Termination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataApp_Spec_Job_Termination {
    #[doc = "Set the field `grace_period_seconds`.\n"]
    pub fn set_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grace_period_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job_Termination {
    type O = terrars::BlockAssignable<DataApp_Spec_Job_Termination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job_Termination {}

impl BuildDataApp_Spec_Job_Termination {
    pub fn build(self) -> DataApp_Spec_Job_Termination {
        DataApp_Spec_Job_Termination { grace_period_seconds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Job_TerminationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Job_TerminationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Job_TerminationRef {
        DataApp_Spec_Job_TerminationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Job_TerminationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grace_period_seconds` after provisioning.\n"]
    pub fn grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_period_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Job {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataApp_Spec_Job_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<terrars::ListField<DataApp_Spec_Job_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataApp_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataApp_Spec_Job_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataApp_Spec_Job_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataApp_Spec_Job_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<DataApp_Spec_Job_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination: std::option::Option<terrars::ListField<DataApp_Spec_Job_Termination>>,
}

impl DataApp_Spec_Job {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Alert>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Bitbucket>>) -> Self {
        self.bitbucket = std::option::Option::Some(v.into());
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
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Env>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Git>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Github>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Gitlab>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Image>>) -> Self {
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
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
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

    #[doc = "Set the field `termination`.\n"]
    pub fn set_termination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Termination>>,
    ) -> Self {
        self.termination = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Job {
    type O = terrars::BlockAssignable<DataApp_Spec_Job>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Job {}

impl BuildDataApp_Spec_Job {
    pub fn build(self) -> DataApp_Spec_Job {
        DataApp_Spec_Job {
            alert: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
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
            termination: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_JobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_JobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_JobRef {
        DataApp_Spec_JobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_JobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataApp_Spec_Job_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<DataApp_Spec_Job_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
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
    pub fn env(&self) -> terrars::SetRef<DataApp_Spec_EnvRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataApp_Spec_Job_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataApp_Spec_Job_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataApp_Spec_Job_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<DataApp_Spec_Job_ImageRef> {
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
    pub fn log_destination(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestinationRef> {
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

    #[doc = "Get a reference to the value of field `termination` after provisioning.\n"]
    pub fn termination(&self) -> terrars::ListRef<DataApp_Spec_Job_TerminationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Maintenance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offline_page_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Maintenance {
    #[doc = "Set the field `archive`.\n"]
    pub fn set_archive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.archive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offline_page_url`.\n"]
    pub fn set_offline_page_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.offline_page_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Maintenance {
    type O = terrars::BlockAssignable<DataApp_Spec_Maintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Maintenance {}

impl BuildDataApp_Spec_Maintenance {
    pub fn build(self) -> DataApp_Spec_Maintenance {
        DataApp_Spec_Maintenance {
            archive: core::default::Default::default(),
            enabled: core::default::Default::default(),
            offline_page_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_MaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_MaintenanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_MaintenanceRef {
        DataApp_Spec_MaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_MaintenanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive` after provisioning.\n"]
    pub fn archive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `offline_page_url` after provisioning.\n"]
    pub fn offline_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offline_page_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Worker_Autoscaling_Metrics_Cpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataApp_Spec_Worker_Autoscaling_Metrics_Cpu {
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Worker_Autoscaling_Metrics_Cpu {
    type O = terrars::BlockAssignable<DataApp_Spec_Worker_Autoscaling_Metrics_Cpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Worker_Autoscaling_Metrics_Cpu {}

impl BuildDataApp_Spec_Worker_Autoscaling_Metrics_Cpu {
    pub fn build(self) -> DataApp_Spec_Worker_Autoscaling_Metrics_Cpu {
        DataApp_Spec_Worker_Autoscaling_Metrics_Cpu { percent: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Worker_Autoscaling_Metrics_CpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Worker_Autoscaling_Metrics_CpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataApp_Spec_Worker_Autoscaling_Metrics_CpuRef {
        DataApp_Spec_Worker_Autoscaling_Metrics_CpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Worker_Autoscaling_Metrics_CpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Worker_Autoscaling_Metrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::ListField<DataApp_Spec_Worker_Autoscaling_Metrics_Cpu>>,
}

impl DataApp_Spec_Worker_Autoscaling_Metrics {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker_Autoscaling_Metrics_Cpu>>,
    ) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Worker_Autoscaling_Metrics {
    type O = terrars::BlockAssignable<DataApp_Spec_Worker_Autoscaling_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Worker_Autoscaling_Metrics {}

impl BuildDataApp_Spec_Worker_Autoscaling_Metrics {
    pub fn build(self) -> DataApp_Spec_Worker_Autoscaling_Metrics {
        DataApp_Spec_Worker_Autoscaling_Metrics { cpu: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Worker_Autoscaling_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Worker_Autoscaling_MetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Worker_Autoscaling_MetricsRef {
        DataApp_Spec_Worker_Autoscaling_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Worker_Autoscaling_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::ListRef<DataApp_Spec_Worker_Autoscaling_Metrics_CpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Worker_Autoscaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<terrars::ListField<DataApp_Spec_Worker_Autoscaling_Metrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instance_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataApp_Spec_Worker_Autoscaling {
    #[doc = "Set the field `max_instance_count`.\n"]
    pub fn set_max_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker_Autoscaling_Metrics>>,
    ) -> Self {
        self.metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instance_count`.\n"]
    pub fn set_min_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instance_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Worker_Autoscaling {
    type O = terrars::BlockAssignable<DataApp_Spec_Worker_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Worker_Autoscaling {}

impl BuildDataApp_Spec_Worker_Autoscaling {
    pub fn build(self) -> DataApp_Spec_Worker_Autoscaling {
        DataApp_Spec_Worker_Autoscaling {
            max_instance_count: core::default::Default::default(),
            metrics: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Worker_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Worker_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Worker_AutoscalingRef {
        DataApp_Spec_Worker_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Worker_AutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_instance_count` after provisioning.\n"]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<DataApp_Spec_Worker_Autoscaling_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instance_count` after provisioning.\n"]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Service_HealthCheck {
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

impl DataApp_Spec_Service_HealthCheck {
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

impl terrars::ToListMappable for DataApp_Spec_Service_HealthCheck {
    type O = terrars::BlockAssignable<DataApp_Spec_Service_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Service_HealthCheck {}

impl BuildDataApp_Spec_Service_HealthCheck {
    pub fn build(self) -> DataApp_Spec_Service_HealthCheck {
        DataApp_Spec_Service_HealthCheck {
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

#[allow(dead_code)]
pub struct DataApp_Spec_Service_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Service_HealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Service_HealthCheckRef {
        DataApp_Spec_Service_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Service_HealthCheckRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Worker_LivenessHealthCheck {
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

impl DataApp_Spec_Worker_LivenessHealthCheck {
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

impl terrars::ToListMappable for DataApp_Spec_Worker_LivenessHealthCheck {
    type O = terrars::BlockAssignable<DataApp_Spec_Worker_LivenessHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Worker_LivenessHealthCheck {}

impl BuildDataApp_Spec_Worker_LivenessHealthCheck {
    pub fn build(self) -> DataApp_Spec_Worker_LivenessHealthCheck {
        DataApp_Spec_Worker_LivenessHealthCheck {
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

#[allow(dead_code)]
pub struct DataApp_Spec_Worker_LivenessHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Worker_LivenessHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Worker_LivenessHealthCheckRef {
        DataApp_Spec_Worker_LivenessHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Worker_LivenessHealthCheckRef {
    #[allow(dead_code)]
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
pub struct DataApp_Spec_Service_Termination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drain_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataApp_Spec_Service_Termination {
    #[doc = "Set the field `drain_seconds`.\n"]
    pub fn set_drain_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.drain_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grace_period_seconds`.\n"]
    pub fn set_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grace_period_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Service_Termination {
    type O = terrars::BlockAssignable<DataApp_Spec_Service_Termination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Service_Termination {}

impl BuildDataApp_Spec_Service_Termination {
    pub fn build(self) -> DataApp_Spec_Service_Termination {
        DataApp_Spec_Service_Termination {
            drain_seconds: core::default::Default::default(),
            grace_period_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_Service_TerminationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_Service_TerminationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_Service_TerminationRef {
        DataApp_Spec_Service_TerminationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_Service_TerminationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `drain_seconds` after provisioning.\n"]
    pub fn drain_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grace_period_seconds` after provisioning.\n"]
    pub fn grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_period_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Service {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataApp_Spec_Job_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<terrars::ListField<DataApp_Spec_Worker_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<terrars::ListField<DataApp_Spec_Job_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataApp_Spec_Service_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataApp_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataApp_Spec_Job_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataApp_Spec_Job_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataApp_Spec_Job_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::ListField<DataApp_Spec_Service_HealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<DataApp_Spec_Job_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ports: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_health_check: std::option::Option<terrars::ListField<DataApp_Spec_Worker_LivenessHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<terrars::ListField<DataApp_Spec_Service_Routes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination: std::option::Option<terrars::ListField<DataApp_Spec_Service_Termination>>,
}

impl DataApp_Spec_Service {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Alert>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker_Autoscaling>>,
    ) -> Self {
        self.autoscaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Bitbucket>>) -> Self {
        self.bitbucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_command`.\n"]
    pub fn set_build_command(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Cors>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Env>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Git>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Github>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Gitlab>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_HealthCheck>>,
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
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Image>>) -> Self {
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

    #[doc = "Set the field `liveness_health_check`.\n"]
    pub fn set_liveness_health_check(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker_LivenessHealthCheck>>,
    ) -> Self {
        self.liveness_health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
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
    pub fn set_routes(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Routes>>) -> Self {
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

    #[doc = "Set the field `termination`.\n"]
    pub fn set_termination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Termination>>,
    ) -> Self {
        self.termination = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Service {
    type O = terrars::BlockAssignable<DataApp_Spec_Service>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Service {}

impl BuildDataApp_Spec_Service {
    pub fn build(self) -> DataApp_Spec_Service {
        DataApp_Spec_Service {
            alert: core::default::Default::default(),
            autoscaling: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
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
            liveness_health_check: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            name: core::default::Default::default(),
            routes: core::default::Default::default(),
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            termination: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_ServiceRef {
        DataApp_Spec_ServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_ServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataApp_Spec_Job_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<DataApp_Spec_Worker_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<DataApp_Spec_Job_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `build_command` after provisioning.\n"]
    pub fn build_command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_command", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<DataApp_Spec_Service_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataApp_Spec_EnvRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataApp_Spec_Job_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataApp_Spec_Job_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataApp_Spec_Job_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<DataApp_Spec_Service_HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `http_port` after provisioning.\n"]
    pub fn http_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_port", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<DataApp_Spec_Job_ImageRef> {
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

    #[doc = "Get a reference to the value of field `liveness_health_check` after provisioning.\n"]
    pub fn liveness_health_check(&self) -> terrars::ListRef<DataApp_Spec_Worker_LivenessHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<DataApp_Spec_Service_RoutesRef> {
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

    #[doc = "Get a reference to the value of field `termination` after provisioning.\n"]
    pub fn termination(&self) -> terrars::ListRef<DataApp_Spec_Service_TerminationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_StaticSite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<terrars::ListField<DataApp_Spec_Job_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catchall_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<terrars::ListField<DataApp_Spec_Service_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataApp_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataApp_Spec_Job_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataApp_Spec_Job_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataApp_Spec_Job_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<terrars::ListField<DataApp_Spec_Service_Routes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_StaticSite {
    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Bitbucket>>) -> Self {
        self.bitbucket = std::option::Option::Some(v.into());
        self
    }

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
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Cors>>) -> Self {
        self.cors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dockerfile_path`.\n"]
    pub fn set_dockerfile_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dockerfile_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Env>>) -> Self {
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
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Git>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Github>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Gitlab>>) -> Self {
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
    pub fn set_routes(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service_Routes>>) -> Self {
        self.routes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dir`.\n"]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_StaticSite {
    type O = terrars::BlockAssignable<DataApp_Spec_StaticSite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_StaticSite {}

impl BuildDataApp_Spec_StaticSite {
    pub fn build(self) -> DataApp_Spec_StaticSite {
        DataApp_Spec_StaticSite {
            bitbucket: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct DataApp_Spec_StaticSiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_StaticSiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_StaticSiteRef {
        DataApp_Spec_StaticSiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_StaticSiteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<DataApp_Spec_Job_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
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
    pub fn cors(&self) -> terrars::ListRef<DataApp_Spec_Service_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `dockerfile_path` after provisioning.\n"]
    pub fn dockerfile_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dockerfile_path", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataApp_Spec_EnvRef> {
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
    pub fn git(&self) -> terrars::ListRef<DataApp_Spec_Job_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataApp_Spec_Job_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataApp_Spec_Job_GitlabRef> {
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
    pub fn routes(&self) -> terrars::ListRef<DataApp_Spec_Service_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dir` after provisioning.\n"]
    pub fn source_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Vpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_Spec_Vpc {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Vpc {
    type O = terrars::BlockAssignable<DataApp_Spec_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Vpc {}

impl BuildDataApp_Spec_Vpc {
    pub fn build(self) -> DataApp_Spec_Vpc {
        DataApp_Spec_Vpc { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_VpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_VpcRef {
        DataApp_Spec_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec_Worker {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataApp_Spec_Job_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<terrars::ListField<DataApp_Spec_Worker_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<terrars::ListField<DataApp_Spec_Job_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dockerfile_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataApp_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<terrars::ListField<DataApp_Spec_Job_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<terrars::ListField<DataApp_Spec_Job_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<terrars::ListField<DataApp_Spec_Job_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<DataApp_Spec_Job_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_health_check: std::option::Option<terrars::ListField<DataApp_Spec_Worker_LivenessHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_command: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination: std::option::Option<terrars::ListField<DataApp_Spec_Job_Termination>>,
}

impl DataApp_Spec_Worker {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Alert>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker_Autoscaling>>,
    ) -> Self {
        self.autoscaling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Bitbucket>>) -> Self {
        self.bitbucket = std::option::Option::Some(v.into());
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
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Env>>) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_slug`.\n"]
    pub fn set_environment_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Git>>) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `github`.\n"]
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Github>>) -> Self {
        self.github = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gitlab`.\n"]
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Gitlab>>) -> Self {
        self.gitlab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Image>>) -> Self {
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

    #[doc = "Set the field `liveness_health_check`.\n"]
    pub fn set_liveness_health_check(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker_LivenessHealthCheck>>,
    ) -> Self {
        self.liveness_health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_LogDestination>>,
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

    #[doc = "Set the field `termination`.\n"]
    pub fn set_termination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job_Termination>>,
    ) -> Self {
        self.termination = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec_Worker {
    type O = terrars::BlockAssignable<DataApp_Spec_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec_Worker {}

impl BuildDataApp_Spec_Worker {
    pub fn build(self) -> DataApp_Spec_Worker {
        DataApp_Spec_Worker {
            alert: core::default::Default::default(),
            autoscaling: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
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
            liveness_health_check: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            name: core::default::Default::default(),
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            termination: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_Spec_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_Spec_WorkerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_Spec_WorkerRef {
        DataApp_Spec_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_Spec_WorkerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataApp_Spec_Job_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<DataApp_Spec_Worker_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<DataApp_Spec_Job_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
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
    pub fn env(&self) -> terrars::SetRef<DataApp_Spec_EnvRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_slug` after provisioning.\n"]
    pub fn environment_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataApp_Spec_Job_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<DataApp_Spec_Job_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<DataApp_Spec_Job_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<DataApp_Spec_Job_ImageRef> {
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

    #[doc = "Get a reference to the value of field `liveness_health_check` after provisioning.\n"]
    pub fn liveness_health_check(&self) -> terrars::ListRef<DataApp_Spec_Worker_LivenessHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<DataApp_Spec_Job_LogDestinationRef> {
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

    #[doc = "Get a reference to the value of field `termination` after provisioning.\n"]
    pub fn termination(&self) -> terrars::ListRef<DataApp_Spec_Job_TerminationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<terrars::ListField<DataApp_Spec_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::ListField<DataApp_Spec_Database>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_edge_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_email_obfuscation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::ListField<DataApp_Spec_Domain>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::ListField<DataApp_Spec_Egress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_threat_control_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::SetField<DataApp_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<terrars::ListField<DataApp_Spec_Function>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<terrars::ListField<DataApp_Spec_Ingress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job: std::option::Option<terrars::ListField<DataApp_Spec_Job>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance: std::option::Option<terrars::ListField<DataApp_Spec_Maintenance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::ListField<DataApp_Spec_Service>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_site: std::option::Option<terrars::ListField<DataApp_Spec_StaticSite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<DataApp_Spec_Vpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<terrars::ListField<DataApp_Spec_Worker>>,
}

impl DataApp_Spec {
    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Alert>>) -> Self {
        self.alert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\n"]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Database>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_edge_cache`.\n"]
    pub fn set_disable_edge_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_edge_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_email_obfuscation`.\n"]
    pub fn set_disable_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_email_obfuscation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Domain>>) -> Self {
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

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Egress>>) -> Self {
        self.egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enhanced_threat_control_enabled`.\n"]
    pub fn set_enhanced_threat_control_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enhanced_threat_control_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::SetField<DataApp_Spec_Env>>) -> Self {
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
    pub fn set_function(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Function>>) -> Self {
        self.function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Ingress>>) -> Self {
        self.ingress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job`.\n"]
    pub fn set_job(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Job>>) -> Self {
        self.job = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance`.\n"]
    pub fn set_maintenance(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Maintenance>>) -> Self {
        self.maintenance = std::option::Option::Some(v.into());
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
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Service>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_site`.\n"]
    pub fn set_static_site(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_StaticSite>>) -> Self {
        self.static_site = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Vpc>>) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(mut self, v: impl std::convert::Into<terrars::ListField<DataApp_Spec_Worker>>) -> Self {
        self.worker = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_Spec {
    type O = terrars::BlockAssignable<DataApp_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_Spec {}

impl BuildDataApp_Spec {
    pub fn build(self) -> DataApp_Spec {
        DataApp_Spec {
            alert: core::default::Default::default(),
            database: core::default::Default::default(),
            disable_edge_cache: core::default::Default::default(),
            disable_email_obfuscation: core::default::Default::default(),
            domain: core::default::Default::default(),
            domains: core::default::Default::default(),
            egress: core::default::Default::default(),
            enhanced_threat_control_enabled: core::default::Default::default(),
            env: core::default::Default::default(),
            features: core::default::Default::default(),
            function: core::default::Default::default(),
            ingress: core::default::Default::default(),
            job: core::default::Default::default(),
            maintenance: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            service: core::default::Default::default(),
            static_site: core::default::Default::default(),
            vpc: core::default::Default::default(),
            worker: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_SpecRef {
        DataApp_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<DataApp_Spec_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<DataApp_Spec_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_edge_cache` after provisioning.\n"]
    pub fn disable_edge_cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_edge_cache", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_email_obfuscation` after provisioning.\n"]
    pub fn disable_email_obfuscation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_email_obfuscation", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::ListRef<DataApp_Spec_DomainRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::ListRef<DataApp_Spec_EgressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `enhanced_threat_control_enabled` after provisioning.\n"]
    pub fn enhanced_threat_control_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_threat_control_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::SetRef<DataApp_Spec_EnvRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.features", self.base))
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::ListRef<DataApp_Spec_FunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::ListRef<DataApp_Spec_IngressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }

    #[doc = "Get a reference to the value of field `job` after provisioning.\n"]
    pub fn job(&self) -> terrars::ListRef<DataApp_Spec_JobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> terrars::ListRef<DataApp_Spec_MaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance", self.base))
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
    pub fn service(&self) -> terrars::ListRef<DataApp_Spec_ServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `static_site` after provisioning.\n"]
    pub fn static_site(&self) -> terrars::ListRef<DataApp_Spec_StaticSiteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_site", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<DataApp_Spec_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<DataApp_Spec_WorkerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct DataApp_DedicatedIps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataApp_DedicatedIps {
    #[doc = "Set the field `id`.\nThe ID of the dedicated egress IP."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip`.\nThe IP address of the dedicated egress IP."]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nThe status of the dedicated egress IP: 'UNKNOWN', 'ASSIGNING', 'ASSIGNED', or 'REMOVED'"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataApp_DedicatedIps {
    type O = terrars::BlockAssignable<DataApp_DedicatedIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApp_DedicatedIps {}

impl BuildDataApp_DedicatedIps {
    pub fn build(self) -> DataApp_DedicatedIps {
        DataApp_DedicatedIps {
            id: core::default::Default::default(),
            ip: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataApp_DedicatedIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataApp_DedicatedIpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataApp_DedicatedIpsRef {
        DataApp_DedicatedIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApp_DedicatedIpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the dedicated egress IP."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nThe IP address of the dedicated egress IP."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the dedicated egress IP: 'UNKNOWN', 'ASSIGNING', 'ASSIGNED', or 'REMOVED'"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
