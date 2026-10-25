#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AppData_ {
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
    deployment_per_page: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_ips: std::option::Option<std::vec::Vec<App_DedicatedIps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<App_Spec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<App_Timeouts>,
    dynamic: AppDynamic,
}

struct App_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppData_>,
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

    #[doc =
        "Set the field `deployment_per_page`.\nNumber of deployments to request per page when polling for deployments"]
    pub fn set_deployment_per_page(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().deployment_per_page = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_ips`.\n"]
    pub fn set_dedicated_ips(self, v: impl std::convert::Into<terrars::BlockAssignable<App_DedicatedIps>>) -> Self {
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

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<App_Timeouts>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `deployment_per_page` after provisioning.\nNumber of deployments to request per page when polling for deployments"]
    pub fn deployment_per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_per_page", self.extract_ref()))
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
    pub fn dedicated_ips(&self) -> terrars::ListRef<App_DedicatedIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<App_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> App_TimeoutsRef {
        App_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(AppData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deployment_per_page: core::default::Default::default(),
                id: core::default::Default::default(),
                project_id: core::default::Default::default(),
                dedicated_ips: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `deployment_per_page` after provisioning.\nNumber of deployments to request per page when polling for deployments"]
    pub fn deployment_per_page(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_per_page", self.extract_ref()))
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
    pub fn dedicated_ips(&self) -> terrars::ListRef<App_DedicatedIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<App_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> App_TimeoutsRef {
        App_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct App_DedicatedIps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_DedicatedIps {
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

impl terrars::ToListMappable for App_DedicatedIps {
    type O = terrars::BlockAssignable<App_DedicatedIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_DedicatedIps {}

impl BuildApp_DedicatedIps {
    pub fn build(self) -> App_DedicatedIps {
        App_DedicatedIps {
            id: core::default::Default::default(),
            ip: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_DedicatedIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_DedicatedIpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_DedicatedIpsRef {
        App_DedicatedIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_DedicatedIpsRef {
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

#[derive(Serialize)]
pub struct App_Spec_Alert_Destinations_SlackWebhooks {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl App_Spec_Alert_Destinations_SlackWebhooks { }

impl terrars::ToListMappable for App_Spec_Alert_Destinations_SlackWebhooks {
    type O = terrars::BlockAssignable<App_Spec_Alert_Destinations_SlackWebhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Alert_Destinations_SlackWebhooks {
    #[doc = "The Slack channel to send notifications to."]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The Slack webhook URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Alert_Destinations_SlackWebhooks {
    pub fn build(self) -> App_Spec_Alert_Destinations_SlackWebhooks {
        App_Spec_Alert_Destinations_SlackWebhooks {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Alert_Destinations_SlackWebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Alert_Destinations_SlackWebhooksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Alert_Destinations_SlackWebhooksRef {
        App_Spec_Alert_Destinations_SlackWebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Alert_Destinations_SlackWebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send notifications to."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Slack webhook URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Alert_DestinationsDynamic {
    slack_webhooks: std::option::Option<terrars::DynamicBlock<App_Spec_Alert_Destinations_SlackWebhooks>>,
}

#[derive(Serialize)]
pub struct App_Spec_Alert_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_webhooks: std::option::Option<std::vec::Vec<App_Spec_Alert_Destinations_SlackWebhooks>>,
    dynamic: App_Spec_Alert_DestinationsDynamic,
}

impl App_Spec_Alert_Destinations {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Alert_Destinations_SlackWebhooks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack_webhooks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_webhooks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Alert_Destinations {
    type O = terrars::BlockAssignable<App_Spec_Alert_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Alert_Destinations {}

impl BuildApp_Spec_Alert_Destinations {
    pub fn build(self) -> App_Spec_Alert_Destinations {
        App_Spec_Alert_Destinations {
            emails: core::default::Default::default(),
            slack_webhooks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Alert_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Alert_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Alert_DestinationsRef {
        App_Spec_Alert_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Alert_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_webhooks` after provisioning.\n"]
    pub fn slack_webhooks(&self) -> terrars::ListRef<App_Spec_Alert_Destinations_SlackWebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_webhooks", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_AlertDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<App_Spec_Alert_Destinations>>,
}

#[derive(Serialize)]
pub struct App_Spec_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    rule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<App_Spec_Alert_Destinations>>,
    dynamic: App_Spec_AlertDynamic,
}

impl App_Spec_Alert {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Alert_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Alert {
    type O = terrars::BlockAssignable<App_Spec_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Alert {
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Alert {
    pub fn build(self) -> App_Spec_Alert {
        App_Spec_Alert {
            disabled: core::default::Default::default(),
            rule: self.rule,
            destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_AlertRef {
        App_Spec_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_AlertRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Database {
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

impl App_Spec_Database {
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

impl terrars::ToListMappable for App_Spec_Database {
    type O = terrars::BlockAssignable<App_Spec_Database>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Database {}

impl BuildApp_Spec_Database {
    pub fn build(self) -> App_Spec_Database {
        App_Spec_Database {
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
pub struct App_Spec_DatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_DatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_DatabaseRef {
        App_Spec_DatabaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_DatabaseRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Domain {
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wildcard: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Domain {
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

impl terrars::ToListMappable for App_Spec_Domain {
    type O = terrars::BlockAssignable<App_Spec_Domain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Domain {
    #[doc = "The hostname for the domain."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Domain {
    pub fn build(self) -> App_Spec_Domain {
        App_Spec_Domain {
            name: self.name,
            type_: core::default::Default::default(),
            wildcard: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_DomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_DomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_DomainRef {
        App_Spec_DomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_DomainRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Egress {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Egress {
    #[doc = "Set the field `type_`.\nThe app egress type."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Egress {
    type O = terrars::BlockAssignable<App_Spec_Egress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Egress {}

impl BuildApp_Spec_Egress {
    pub fn build(self) -> App_Spec_Egress {
        App_Spec_Egress { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_EgressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_EgressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_EgressRef {
        App_Spec_EgressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_EgressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe app egress type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Env {
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

impl terrars::ToListMappable for App_Spec_Env {
    type O = terrars::BlockAssignable<App_Spec_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Env {}

impl BuildApp_Spec_Env {
    pub fn build(self) -> App_Spec_Env {
        App_Spec_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_EnvRef {
        App_Spec_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_EnvRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_Alert_Destinations_SlackWebhooks {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl App_Spec_Function_Alert_Destinations_SlackWebhooks { }

impl terrars::ToListMappable for App_Spec_Function_Alert_Destinations_SlackWebhooks {
    type O = terrars::BlockAssignable<App_Spec_Function_Alert_Destinations_SlackWebhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Alert_Destinations_SlackWebhooks {
    #[doc = "The Slack channel to send notifications to."]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The Slack webhook URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function_Alert_Destinations_SlackWebhooks {
    pub fn build(self) -> App_Spec_Function_Alert_Destinations_SlackWebhooks {
        App_Spec_Function_Alert_Destinations_SlackWebhooks {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_Alert_Destinations_SlackWebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_Alert_Destinations_SlackWebhooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Function_Alert_Destinations_SlackWebhooksRef {
        App_Spec_Function_Alert_Destinations_SlackWebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_Alert_Destinations_SlackWebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send notifications to."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Slack webhook URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Function_Alert_DestinationsDynamic {
    slack_webhooks: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Alert_Destinations_SlackWebhooks>>,
}

#[derive(Serialize)]
pub struct App_Spec_Function_Alert_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_webhooks: std::option::Option<std::vec::Vec<App_Spec_Function_Alert_Destinations_SlackWebhooks>>,
    dynamic: App_Spec_Function_Alert_DestinationsDynamic,
}

impl App_Spec_Function_Alert_Destinations {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Alert_Destinations_SlackWebhooks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack_webhooks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_webhooks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Function_Alert_Destinations {
    type O = terrars::BlockAssignable<App_Spec_Function_Alert_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Alert_Destinations {}

impl BuildApp_Spec_Function_Alert_Destinations {
    pub fn build(self) -> App_Spec_Function_Alert_Destinations {
        App_Spec_Function_Alert_Destinations {
            emails: core::default::Default::default(),
            slack_webhooks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_Alert_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_Alert_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_Alert_DestinationsRef {
        App_Spec_Function_Alert_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_Alert_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_webhooks` after provisioning.\n"]
    pub fn slack_webhooks(&self) -> terrars::ListRef<App_Spec_Function_Alert_Destinations_SlackWebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_webhooks", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Function_AlertDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Alert_Destinations>>,
}

#[derive(Serialize)]
pub struct App_Spec_Function_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<App_Spec_Function_Alert_Destinations>>,
    dynamic: App_Spec_Function_AlertDynamic,
}

impl App_Spec_Function_Alert {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Alert_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Function_Alert {
    type O = terrars::BlockAssignable<App_Spec_Function_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Alert {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function_Alert {
    pub fn build(self) -> App_Spec_Function_Alert {
        App_Spec_Function_Alert {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
            destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_AlertRef {
        App_Spec_Function_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_AlertRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_Bitbucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_Bitbucket {
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

impl terrars::ToListMappable for App_Spec_Function_Bitbucket {
    type O = terrars::BlockAssignable<App_Spec_Function_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Bitbucket {}

impl BuildApp_Spec_Function_Bitbucket {
    pub fn build(self) -> App_Spec_Function_Bitbucket {
        App_Spec_Function_Bitbucket {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_BitbucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_BitbucketRef {
        App_Spec_Function_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_BitbucketRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_Cors_AllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_Cors_AllowOrigins {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match."]
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

impl terrars::ToListMappable for App_Spec_Function_Cors_AllowOrigins {
    type O = terrars::BlockAssignable<App_Spec_Function_Cors_AllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Cors_AllowOrigins {}

impl BuildApp_Spec_Function_Cors_AllowOrigins {
    pub fn build(self) -> App_Spec_Function_Cors_AllowOrigins {
        App_Spec_Function_Cors_AllowOrigins {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_Cors_AllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_Cors_AllowOriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_Cors_AllowOriginsRef {
        App_Spec_Function_Cors_AllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_Cors_AllowOriginsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Function_CorsDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Cors_AllowOrigins>>,
}

#[derive(Serialize)]
pub struct App_Spec_Function_Cors {
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
    allow_origins: std::option::Option<std::vec::Vec<App_Spec_Function_Cors_AllowOrigins>>,
    dynamic: App_Spec_Function_CorsDynamic,
}

impl App_Spec_Function_Cors {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Cors_AllowOrigins>>,
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

impl terrars::ToListMappable for App_Spec_Function_Cors {
    type O = terrars::BlockAssignable<App_Spec_Function_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Cors {}

impl BuildApp_Spec_Function_Cors {
    pub fn build(self) -> App_Spec_Function_Cors {
        App_Spec_Function_Cors {
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

#[allow(dead_code)]
pub struct App_Spec_Function_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_CorsRef {
        App_Spec_Function_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_CorsRef {
    #[allow(dead_code)]
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
    pub fn allow_origins(&self) -> terrars::ListRef<App_Spec_Function_Cors_AllowOriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Function_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_Env {
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

impl terrars::ToListMappable for App_Spec_Function_Env {
    type O = terrars::BlockAssignable<App_Spec_Function_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Env {}

impl BuildApp_Spec_Function_Env {
    pub fn build(self) -> App_Spec_Function_Env {
        App_Spec_Function_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_EnvRef {
        App_Spec_Function_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_EnvRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_Git {
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

impl terrars::ToListMappable for App_Spec_Function_Git {
    type O = terrars::BlockAssignable<App_Spec_Function_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Git {}

impl BuildApp_Spec_Function_Git {
    pub fn build(self) -> App_Spec_Function_Git {
        App_Spec_Function_Git {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_GitRef {
        App_Spec_Function_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_GitRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_Github {
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

impl terrars::ToListMappable for App_Spec_Function_Github {
    type O = terrars::BlockAssignable<App_Spec_Function_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Github {}

impl BuildApp_Spec_Function_Github {
    pub fn build(self) -> App_Spec_Function_Github {
        App_Spec_Function_Github {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_GithubRef {
        App_Spec_Function_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_GithubRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_Gitlab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_Gitlab {
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

impl terrars::ToListMappable for App_Spec_Function_Gitlab {
    type O = terrars::BlockAssignable<App_Spec_Function_Gitlab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Gitlab {}

impl BuildApp_Spec_Function_Gitlab {
    pub fn build(self) -> App_Spec_Function_Gitlab {
        App_Spec_Function_Gitlab {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_GitlabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_GitlabRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_GitlabRef {
        App_Spec_Function_GitlabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_GitlabRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_LogDestination_Datadog {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_LogDestination_Datadog {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Function_LogDestination_Datadog {
    type O = terrars::BlockAssignable<App_Spec_Function_LogDestination_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_LogDestination_Datadog {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function_LogDestination_Datadog {
    pub fn build(self) -> App_Spec_Function_LogDestination_Datadog {
        App_Spec_Function_LogDestination_Datadog {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_LogDestination_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_LogDestination_DatadogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_LogDestination_DatadogRef {
        App_Spec_Function_LogDestination_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_LogDestination_DatadogRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Function_LogDestination_Logtail {
    token: terrars::PrimField<std::string::String>,
}

impl App_Spec_Function_LogDestination_Logtail { }

impl terrars::ToListMappable for App_Spec_Function_LogDestination_Logtail {
    type O = terrars::BlockAssignable<App_Spec_Function_LogDestination_Logtail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_LogDestination_Logtail {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function_LogDestination_Logtail {
    pub fn build(self) -> App_Spec_Function_LogDestination_Logtail {
        App_Spec_Function_LogDestination_Logtail { token: self.token }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_LogDestination_LogtailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_LogDestination_LogtailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_LogDestination_LogtailRef {
        App_Spec_Function_LogDestination_LogtailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_LogDestination_LogtailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Function_LogDestination_OpenSearch_BasicAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Function_LogDestination_OpenSearch_BasicAuth {
    #[doc = "Set the field `password`.\nPassword for basic authentication."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nuser for basic authentication."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Function_LogDestination_OpenSearch_BasicAuth {
    type O = terrars::BlockAssignable<App_Spec_Function_LogDestination_OpenSearch_BasicAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_LogDestination_OpenSearch_BasicAuth {}

impl BuildApp_Spec_Function_LogDestination_OpenSearch_BasicAuth {
    pub fn build(self) -> App_Spec_Function_LogDestination_OpenSearch_BasicAuth {
        App_Spec_Function_LogDestination_OpenSearch_BasicAuth {
            password: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_LogDestination_OpenSearch_BasicAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_LogDestination_OpenSearch_BasicAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Function_LogDestination_OpenSearch_BasicAuthRef {
        App_Spec_Function_LogDestination_OpenSearch_BasicAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_LogDestination_OpenSearch_BasicAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for basic authentication."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nuser for basic authentication."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Function_LogDestination_OpenSearchDynamic {
    basic_auth: std::option::Option<terrars::DynamicBlock<App_Spec_Function_LogDestination_OpenSearch_BasicAuth>>,
}

#[derive(Serialize)]
pub struct App_Spec_Function_LogDestination_OpenSearch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth: std::option::Option<std::vec::Vec<App_Spec_Function_LogDestination_OpenSearch_BasicAuth>>,
    dynamic: App_Spec_Function_LogDestination_OpenSearchDynamic,
}

impl App_Spec_Function_LogDestination_OpenSearch {
    #[doc = "Set the field `cluster_name`.\nOpenSearch cluster name."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nOpenSearch endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\nOpenSearch index name."]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_LogDestination_OpenSearch_BasicAuth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Function_LogDestination_OpenSearch {
    type O = terrars::BlockAssignable<App_Spec_Function_LogDestination_OpenSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_LogDestination_OpenSearch {}

impl BuildApp_Spec_Function_LogDestination_OpenSearch {
    pub fn build(self) -> App_Spec_Function_LogDestination_OpenSearch {
        App_Spec_Function_LogDestination_OpenSearch {
            cluster_name: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            index_name: core::default::Default::default(),
            basic_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_LogDestination_OpenSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_LogDestination_OpenSearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Function_LogDestination_OpenSearchRef {
        App_Spec_Function_LogDestination_OpenSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_LogDestination_OpenSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nOpenSearch cluster name."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nOpenSearch endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\nOpenSearch index name."]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::ListRef<App_Spec_Function_LogDestination_OpenSearch_BasicAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Function_LogDestination_Papertrail {
    endpoint: terrars::PrimField<std::string::String>,
}

impl App_Spec_Function_LogDestination_Papertrail { }

impl terrars::ToListMappable for App_Spec_Function_LogDestination_Papertrail {
    type O = terrars::BlockAssignable<App_Spec_Function_LogDestination_Papertrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_LogDestination_Papertrail {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function_LogDestination_Papertrail {
    pub fn build(self) -> App_Spec_Function_LogDestination_Papertrail {
        App_Spec_Function_LogDestination_Papertrail { endpoint: self.endpoint }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_LogDestination_PapertrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_LogDestination_PapertrailRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Function_LogDestination_PapertrailRef {
        App_Spec_Function_LogDestination_PapertrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_LogDestination_PapertrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Function_LogDestinationDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<App_Spec_Function_LogDestination_Datadog>>,
    logtail: std::option::Option<terrars::DynamicBlock<App_Spec_Function_LogDestination_Logtail>>,
    open_search: std::option::Option<terrars::DynamicBlock<App_Spec_Function_LogDestination_OpenSearch>>,
    papertrail: std::option::Option<terrars::DynamicBlock<App_Spec_Function_LogDestination_Papertrail>>,
}

#[derive(Serialize)]
pub struct App_Spec_Function_LogDestination {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<App_Spec_Function_LogDestination_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<App_Spec_Function_LogDestination_Logtail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_search: std::option::Option<std::vec::Vec<App_Spec_Function_LogDestination_OpenSearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<App_Spec_Function_LogDestination_Papertrail>>,
    dynamic: App_Spec_Function_LogDestinationDynamic,
}

impl App_Spec_Function_LogDestination {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_LogDestination_Datadog>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_LogDestination_Logtail>>,
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

    #[doc = "Set the field `open_search`.\n"]
    pub fn set_open_search(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_LogDestination_OpenSearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_search = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_LogDestination_Papertrail>>,
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

impl terrars::ToListMappable for App_Spec_Function_LogDestination {
    type O = terrars::BlockAssignable<App_Spec_Function_LogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_LogDestination {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function_LogDestination {
    pub fn build(self) -> App_Spec_Function_LogDestination {
        App_Spec_Function_LogDestination {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            open_search: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_LogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_LogDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_LogDestinationRef {
        App_Spec_Function_LogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_LogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<App_Spec_Function_LogDestination_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<App_Spec_Function_LogDestination_LogtailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `open_search` after provisioning.\n"]
    pub fn open_search(&self) -> terrars::ListRef<App_Spec_Function_LogDestination_OpenSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_search", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<App_Spec_Function_LogDestination_PapertrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Function_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl App_Spec_Function_Routes {
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

impl terrars::ToListMappable for App_Spec_Function_Routes {
    type O = terrars::BlockAssignable<App_Spec_Function_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function_Routes {}

impl BuildApp_Spec_Function_Routes {
    pub fn build(self) -> App_Spec_Function_Routes {
        App_Spec_Function_Routes {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Function_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Function_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Function_RoutesRef {
        App_Spec_Function_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Function_RoutesRef {
    #[allow(dead_code)]
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
struct App_Spec_FunctionDynamic {
    alert: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Alert>>,
    bitbucket: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Bitbucket>>,
    cors: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Cors>>,
    env: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Env>>,
    git: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Git>>,
    github: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Github>>,
    gitlab: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Gitlab>>,
    log_destination: std::option::Option<terrars::DynamicBlock<App_Spec_Function_LogDestination>>,
    routes: std::option::Option<terrars::DynamicBlock<App_Spec_Function_Routes>>,
}

#[derive(Serialize)]
pub struct App_Spec_Function {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<App_Spec_Function_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<std::vec::Vec<App_Spec_Function_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<App_Spec_Function_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<App_Spec_Function_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<App_Spec_Function_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<App_Spec_Function_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<App_Spec_Function_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<App_Spec_Function_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<std::vec::Vec<App_Spec_Function_Routes>>,
    dynamic: App_Spec_FunctionDynamic,
}

impl App_Spec_Function {
    #[doc = "Set the field `source_dir`.\nAn optional path to the working directory to use for the build."]
    pub fn set_source_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alert`.\n"]
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Alert>>) -> Self {
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

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Bitbucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bitbucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Cors>>) -> Self {
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
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Env>>) -> Self {
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
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Git>>) -> Self {
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
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Github>>) -> Self {
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
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Gitlab>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_LogDestination>>,
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
    pub fn set_routes(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function_Routes>>) -> Self {
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

impl terrars::ToListMappable for App_Spec_Function {
    type O = terrars::BlockAssignable<App_Spec_Function>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Function {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Function {
    pub fn build(self) -> App_Spec_Function {
        App_Spec_Function {
            name: self.name,
            source_dir: core::default::Default::default(),
            alert: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct App_Spec_FunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_FunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_FunctionRef {
        App_Spec_FunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_FunctionRef {
    #[allow(dead_code)]
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
    pub fn alert(&self) -> terrars::ListRef<App_Spec_Function_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<App_Spec_Function_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<App_Spec_Function_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<App_Spec_Function_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<App_Spec_Function_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<App_Spec_Function_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<App_Spec_Function_LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<App_Spec_Function_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_Rule_Component {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Ingress_Rule_Component {
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

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Component {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Component>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Component {}

impl BuildApp_Spec_Ingress_Rule_Component {
    pub fn build(self) -> App_Spec_Ingress_Rule_Component {
        App_Spec_Ingress_Rule_Component {
            name: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
            rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_ComponentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_ComponentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_ComponentRef {
        App_Spec_Ingress_Rule_ComponentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_ComponentRef {
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
pub struct App_Spec_Ingress_Rule_Cors_AllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Ingress_Rule_Cors_AllowOrigins {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match."]
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

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Cors_AllowOrigins {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Cors_AllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Cors_AllowOrigins {}

impl BuildApp_Spec_Ingress_Rule_Cors_AllowOrigins {
    pub fn build(self) -> App_Spec_Ingress_Rule_Cors_AllowOrigins {
        App_Spec_Ingress_Rule_Cors_AllowOrigins {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_Cors_AllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_Cors_AllowOriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_Cors_AllowOriginsRef {
        App_Spec_Ingress_Rule_Cors_AllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_Cors_AllowOriginsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Ingress_Rule_CorsDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Cors_AllowOrigins>>,
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_Rule_Cors {
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
    allow_origins: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Cors_AllowOrigins>>,
    dynamic: App_Spec_Ingress_Rule_CorsDynamic,
}

impl App_Spec_Ingress_Rule_Cors {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Cors_AllowOrigins>>,
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

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Cors {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Cors {}

impl BuildApp_Spec_Ingress_Rule_Cors {
    pub fn build(self) -> App_Spec_Ingress_Rule_Cors {
        App_Spec_Ingress_Rule_Cors {
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

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_CorsRef {
        App_Spec_Ingress_Rule_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_CorsRef {
    #[allow(dead_code)]
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
    pub fn allow_origins(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_Cors_AllowOriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_Rule_Match_Authority {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Ingress_Rule_Match_Authority {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Match_Authority {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Match_Authority>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Match_Authority {}

impl BuildApp_Spec_Ingress_Rule_Match_Authority {
    pub fn build(self) -> App_Spec_Ingress_Rule_Match_Authority {
        App_Spec_Ingress_Rule_Match_Authority { exact: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_Match_AuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_Match_AuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_Match_AuthorityRef {
        App_Spec_Ingress_Rule_Match_AuthorityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_Match_AuthorityRef {
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
pub struct App_Spec_Ingress_Rule_Match_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Ingress_Rule_Match_Path {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Match_Path {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Match_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Match_Path {}

impl BuildApp_Spec_Ingress_Rule_Match_Path {
    pub fn build(self) -> App_Spec_Ingress_Rule_Match_Path {
        App_Spec_Ingress_Rule_Match_Path { prefix: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_Match_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_Match_PathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_Match_PathRef {
        App_Spec_Ingress_Rule_Match_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_Match_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Ingress_Rule_MatchDynamic {
    authority: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Match_Authority>>,
    path: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Match_Path>>,
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_Rule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authority: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Match_Authority>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Match_Path>>,
    dynamic: App_Spec_Ingress_Rule_MatchDynamic,
}

impl App_Spec_Ingress_Rule_Match {
    #[doc = "Set the field `authority`.\n"]
    pub fn set_authority(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Match_Authority>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authority = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authority = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Match_Path>>,
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

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Match {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Match {}

impl BuildApp_Spec_Ingress_Rule_Match {
    pub fn build(self) -> App_Spec_Ingress_Rule_Match {
        App_Spec_Ingress_Rule_Match {
            authority: core::default::Default::default(),
            path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_MatchRef {
        App_Spec_Ingress_Rule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_Match_AuthorityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authority", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_Match_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_Rule_Redirect {
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

impl App_Spec_Ingress_Rule_Redirect {
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

impl terrars::ToListMappable for App_Spec_Ingress_Rule_Redirect {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule_Redirect {}

impl BuildApp_Spec_Ingress_Rule_Redirect {
    pub fn build(self) -> App_Spec_Ingress_Rule_Redirect {
        App_Spec_Ingress_Rule_Redirect {
            authority: core::default::Default::default(),
            port: core::default::Default::default(),
            redirect_code: core::default::Default::default(),
            scheme: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_Rule_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_Rule_RedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_Rule_RedirectRef {
        App_Spec_Ingress_Rule_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_Rule_RedirectRef {
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

#[derive(Serialize, Default)]
struct App_Spec_Ingress_RuleDynamic {
    component: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Component>>,
    cors: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Cors>>,
    match_: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Match>>,
    redirect: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule_Redirect>>,
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Component>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Cors>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule_Redirect>>,
    dynamic: App_Spec_Ingress_RuleDynamic,
}

impl App_Spec_Ingress_Rule {
    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Component>>,
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
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Cors>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Match>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule_Redirect>>,
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

impl terrars::ToListMappable for App_Spec_Ingress_Rule {
    type O = terrars::BlockAssignable<App_Spec_Ingress_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_Rule {}

impl BuildApp_Spec_Ingress_Rule {
    pub fn build(self) -> App_Spec_Ingress_Rule {
        App_Spec_Ingress_Rule {
            component: core::default::Default::default(),
            cors: core::default::Default::default(),
            match_: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_RuleRef {
        App_Spec_Ingress_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_ComponentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<App_Spec_Ingress_Rule_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Ingress_SecureHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Ingress_SecureHeader {
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

impl terrars::ToListMappable for App_Spec_Ingress_SecureHeader {
    type O = terrars::BlockAssignable<App_Spec_Ingress_SecureHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress_SecureHeader {}

impl BuildApp_Spec_Ingress_SecureHeader {
    pub fn build(self) -> App_Spec_Ingress_SecureHeader {
        App_Spec_Ingress_SecureHeader {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Ingress_SecureHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Ingress_SecureHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Ingress_SecureHeaderRef {
        App_Spec_Ingress_SecureHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Ingress_SecureHeaderRef {
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

#[derive(Serialize, Default)]
struct App_Spec_IngressDynamic {
    rule: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_Rule>>,
    secure_header: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress_SecureHeader>>,
}

#[derive(Serialize)]
pub struct App_Spec_Ingress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<App_Spec_Ingress_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secure_header: std::option::Option<std::vec::Vec<App_Spec_Ingress_SecureHeader>>,
    dynamic: App_Spec_IngressDynamic,
}

impl App_Spec_Ingress {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_Rule>>) -> Self {
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

    #[doc = "Set the field `secure_header`.\n"]
    pub fn set_secure_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress_SecureHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secure_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secure_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Ingress {
    type O = terrars::BlockAssignable<App_Spec_Ingress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Ingress {}

impl BuildApp_Spec_Ingress {
    pub fn build(self) -> App_Spec_Ingress {
        App_Spec_Ingress {
            rule: core::default::Default::default(),
            secure_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_IngressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_IngressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_IngressRef {
        App_Spec_IngressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_IngressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<App_Spec_Ingress_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }

    #[doc = "Get a reference to the value of field `secure_header` after provisioning.\n"]
    pub fn secure_header(&self) -> terrars::ListRef<App_Spec_Ingress_SecureHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secure_header", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Job_Alert_Destinations_SlackWebhooks {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl App_Spec_Job_Alert_Destinations_SlackWebhooks { }

impl terrars::ToListMappable for App_Spec_Job_Alert_Destinations_SlackWebhooks {
    type O = terrars::BlockAssignable<App_Spec_Job_Alert_Destinations_SlackWebhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Alert_Destinations_SlackWebhooks {
    #[doc = "The Slack channel to send notifications to."]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The Slack webhook URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_Alert_Destinations_SlackWebhooks {
    pub fn build(self) -> App_Spec_Job_Alert_Destinations_SlackWebhooks {
        App_Spec_Job_Alert_Destinations_SlackWebhooks {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_Alert_Destinations_SlackWebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_Alert_Destinations_SlackWebhooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Job_Alert_Destinations_SlackWebhooksRef {
        App_Spec_Job_Alert_Destinations_SlackWebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_Alert_Destinations_SlackWebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send notifications to."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Slack webhook URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Job_Alert_DestinationsDynamic {
    slack_webhooks: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Alert_Destinations_SlackWebhooks>>,
}

#[derive(Serialize)]
pub struct App_Spec_Job_Alert_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_webhooks: std::option::Option<std::vec::Vec<App_Spec_Job_Alert_Destinations_SlackWebhooks>>,
    dynamic: App_Spec_Job_Alert_DestinationsDynamic,
}

impl App_Spec_Job_Alert_Destinations {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Alert_Destinations_SlackWebhooks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack_webhooks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_webhooks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_Alert_Destinations {
    type O = terrars::BlockAssignable<App_Spec_Job_Alert_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Alert_Destinations {}

impl BuildApp_Spec_Job_Alert_Destinations {
    pub fn build(self) -> App_Spec_Job_Alert_Destinations {
        App_Spec_Job_Alert_Destinations {
            emails: core::default::Default::default(),
            slack_webhooks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_Alert_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_Alert_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_Alert_DestinationsRef {
        App_Spec_Job_Alert_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_Alert_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_webhooks` after provisioning.\n"]
    pub fn slack_webhooks(&self) -> terrars::ListRef<App_Spec_Job_Alert_Destinations_SlackWebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_webhooks", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Job_AlertDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Alert_Destinations>>,
}

#[derive(Serialize)]
pub struct App_Spec_Job_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<App_Spec_Job_Alert_Destinations>>,
    dynamic: App_Spec_Job_AlertDynamic,
}

impl App_Spec_Job_Alert {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Alert_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_Alert {
    type O = terrars::BlockAssignable<App_Spec_Job_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Alert {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_Alert {
    pub fn build(self) -> App_Spec_Job_Alert {
        App_Spec_Job_Alert {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
            destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_AlertRef {
        App_Spec_Job_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_AlertRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_Bitbucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_Bitbucket {
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

impl terrars::ToListMappable for App_Spec_Job_Bitbucket {
    type O = terrars::BlockAssignable<App_Spec_Job_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Bitbucket {}

impl BuildApp_Spec_Job_Bitbucket {
    pub fn build(self) -> App_Spec_Job_Bitbucket {
        App_Spec_Job_Bitbucket {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_BitbucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_BitbucketRef {
        App_Spec_Job_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_BitbucketRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_Env {
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

impl terrars::ToListMappable for App_Spec_Job_Env {
    type O = terrars::BlockAssignable<App_Spec_Job_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Env {}

impl BuildApp_Spec_Job_Env {
    pub fn build(self) -> App_Spec_Job_Env {
        App_Spec_Job_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_EnvRef {
        App_Spec_Job_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_EnvRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_Git {
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

impl terrars::ToListMappable for App_Spec_Job_Git {
    type O = terrars::BlockAssignable<App_Spec_Job_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Git {}

impl BuildApp_Spec_Job_Git {
    pub fn build(self) -> App_Spec_Job_Git {
        App_Spec_Job_Git {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_GitRef {
        App_Spec_Job_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_GitRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_Github {
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

impl terrars::ToListMappable for App_Spec_Job_Github {
    type O = terrars::BlockAssignable<App_Spec_Job_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Github {}

impl BuildApp_Spec_Job_Github {
    pub fn build(self) -> App_Spec_Job_Github {
        App_Spec_Job_Github {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_GithubRef {
        App_Spec_Job_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_GithubRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_Gitlab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_Gitlab {
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

impl terrars::ToListMappable for App_Spec_Job_Gitlab {
    type O = terrars::BlockAssignable<App_Spec_Job_Gitlab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Gitlab {}

impl BuildApp_Spec_Job_Gitlab {
    pub fn build(self) -> App_Spec_Job_Gitlab {
        App_Spec_Job_Gitlab {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_GitlabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_GitlabRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_GitlabRef {
        App_Spec_Job_GitlabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_GitlabRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_Image_DeployOnPush {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl App_Spec_Job_Image_DeployOnPush {
    #[doc = "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_Image_DeployOnPush {
    type O = terrars::BlockAssignable<App_Spec_Job_Image_DeployOnPush>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Image_DeployOnPush {}

impl BuildApp_Spec_Job_Image_DeployOnPush {
    pub fn build(self) -> App_Spec_Job_Image_DeployOnPush {
        App_Spec_Job_Image_DeployOnPush { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_Image_DeployOnPushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_Image_DeployOnPushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_Image_DeployOnPushRef {
        App_Spec_Job_Image_DeployOnPushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_Image_DeployOnPushRef {
    #[allow(dead_code)]
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
struct App_Spec_Job_ImageDynamic {
    deploy_on_push: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Image_DeployOnPush>>,
}

#[derive(Serialize)]
pub struct App_Spec_Job_Image {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_type: terrars::PrimField<std::string::String>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<std::vec::Vec<App_Spec_Job_Image_DeployOnPush>>,
    dynamic: App_Spec_Job_ImageDynamic,
}

impl App_Spec_Job_Image {
    #[doc = "Set the field `digest`.\nThe image digest. Cannot be specified if tag is provided."]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_credentials`.\nAccess credentials for third-party registries"]
    pub fn set_registry_credentials(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.registry_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided. Cannot be specified if digest is provided."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Image_DeployOnPush>>,
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

impl terrars::ToListMappable for App_Spec_Job_Image {
    type O = terrars::BlockAssignable<App_Spec_Job_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Image {
    #[doc = "The registry type."]
    pub registry_type: terrars::PrimField<std::string::String>,
    #[doc = "The repository name."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_Image {
    pub fn build(self) -> App_Spec_Job_Image {
        App_Spec_Job_Image {
            digest: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_credentials: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_ImageRef {
        App_Spec_Job_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `digest` after provisioning.\nThe image digest. Cannot be specified if tag is provided."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry_credentials` after provisioning.\nAccess credentials for third-party registries"]
    pub fn registry_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_credentials", self.base))
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
        "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided. Cannot be specified if digest is provided."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<App_Spec_Job_Image_DeployOnPushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Job_LogDestination_Datadog {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_LogDestination_Datadog {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_LogDestination_Datadog {
    type O = terrars::BlockAssignable<App_Spec_Job_LogDestination_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_LogDestination_Datadog {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_LogDestination_Datadog {
    pub fn build(self) -> App_Spec_Job_LogDestination_Datadog {
        App_Spec_Job_LogDestination_Datadog {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_LogDestination_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_LogDestination_DatadogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_LogDestination_DatadogRef {
        App_Spec_Job_LogDestination_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_LogDestination_DatadogRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Job_LogDestination_Logtail {
    token: terrars::PrimField<std::string::String>,
}

impl App_Spec_Job_LogDestination_Logtail { }

impl terrars::ToListMappable for App_Spec_Job_LogDestination_Logtail {
    type O = terrars::BlockAssignable<App_Spec_Job_LogDestination_Logtail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_LogDestination_Logtail {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_LogDestination_Logtail {
    pub fn build(self) -> App_Spec_Job_LogDestination_Logtail {
        App_Spec_Job_LogDestination_Logtail { token: self.token }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_LogDestination_LogtailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_LogDestination_LogtailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_LogDestination_LogtailRef {
        App_Spec_Job_LogDestination_LogtailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_LogDestination_LogtailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    #[doc = "Set the field `password`.\nPassword for basic authentication."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nuser for basic authentication."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    type O = terrars::BlockAssignable<App_Spec_Job_LogDestination_OpenSearch_BasicAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {}

impl BuildApp_Spec_Job_LogDestination_OpenSearch_BasicAuth {
    pub fn build(self) -> App_Spec_Job_LogDestination_OpenSearch_BasicAuth {
        App_Spec_Job_LogDestination_OpenSearch_BasicAuth {
            password: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
        App_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_LogDestination_OpenSearch_BasicAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for basic authentication."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nuser for basic authentication."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Job_LogDestination_OpenSearchDynamic {
    basic_auth: std::option::Option<terrars::DynamicBlock<App_Spec_Job_LogDestination_OpenSearch_BasicAuth>>,
}

#[derive(Serialize)]
pub struct App_Spec_Job_LogDestination_OpenSearch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth: std::option::Option<std::vec::Vec<App_Spec_Job_LogDestination_OpenSearch_BasicAuth>>,
    dynamic: App_Spec_Job_LogDestination_OpenSearchDynamic,
}

impl App_Spec_Job_LogDestination_OpenSearch {
    #[doc = "Set the field `cluster_name`.\nOpenSearch cluster name."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nOpenSearch endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\nOpenSearch index name."]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_LogDestination_OpenSearch_BasicAuth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_LogDestination_OpenSearch {
    type O = terrars::BlockAssignable<App_Spec_Job_LogDestination_OpenSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_LogDestination_OpenSearch {}

impl BuildApp_Spec_Job_LogDestination_OpenSearch {
    pub fn build(self) -> App_Spec_Job_LogDestination_OpenSearch {
        App_Spec_Job_LogDestination_OpenSearch {
            cluster_name: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            index_name: core::default::Default::default(),
            basic_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_LogDestination_OpenSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_LogDestination_OpenSearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_LogDestination_OpenSearchRef {
        App_Spec_Job_LogDestination_OpenSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_LogDestination_OpenSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nOpenSearch cluster name."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nOpenSearch endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\nOpenSearch index name."]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::ListRef<App_Spec_Job_LogDestination_OpenSearch_BasicAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Job_LogDestination_Papertrail {
    endpoint: terrars::PrimField<std::string::String>,
}

impl App_Spec_Job_LogDestination_Papertrail { }

impl terrars::ToListMappable for App_Spec_Job_LogDestination_Papertrail {
    type O = terrars::BlockAssignable<App_Spec_Job_LogDestination_Papertrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_LogDestination_Papertrail {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_LogDestination_Papertrail {
    pub fn build(self) -> App_Spec_Job_LogDestination_Papertrail {
        App_Spec_Job_LogDestination_Papertrail { endpoint: self.endpoint }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_LogDestination_PapertrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_LogDestination_PapertrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_LogDestination_PapertrailRef {
        App_Spec_Job_LogDestination_PapertrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_LogDestination_PapertrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Job_LogDestinationDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<App_Spec_Job_LogDestination_Datadog>>,
    logtail: std::option::Option<terrars::DynamicBlock<App_Spec_Job_LogDestination_Logtail>>,
    open_search: std::option::Option<terrars::DynamicBlock<App_Spec_Job_LogDestination_OpenSearch>>,
    papertrail: std::option::Option<terrars::DynamicBlock<App_Spec_Job_LogDestination_Papertrail>>,
}

#[derive(Serialize)]
pub struct App_Spec_Job_LogDestination {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<App_Spec_Job_LogDestination_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<App_Spec_Job_LogDestination_Logtail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_search: std::option::Option<std::vec::Vec<App_Spec_Job_LogDestination_OpenSearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<App_Spec_Job_LogDestination_Papertrail>>,
    dynamic: App_Spec_Job_LogDestinationDynamic,
}

impl App_Spec_Job_LogDestination {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_LogDestination_Datadog>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_LogDestination_Logtail>>,
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

    #[doc = "Set the field `open_search`.\n"]
    pub fn set_open_search(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_LogDestination_OpenSearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_search = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_LogDestination_Papertrail>>,
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

impl terrars::ToListMappable for App_Spec_Job_LogDestination {
    type O = terrars::BlockAssignable<App_Spec_Job_LogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_LogDestination {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job_LogDestination {
    pub fn build(self) -> App_Spec_Job_LogDestination {
        App_Spec_Job_LogDestination {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            open_search: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_LogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_LogDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_LogDestinationRef {
        App_Spec_Job_LogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_LogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<App_Spec_Job_LogDestination_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<App_Spec_Job_LogDestination_LogtailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `open_search` after provisioning.\n"]
    pub fn open_search(&self) -> terrars::ListRef<App_Spec_Job_LogDestination_OpenSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_search", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<App_Spec_Job_LogDestination_PapertrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Job_Termination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl App_Spec_Job_Termination {
    #[doc =
        "Set the field `grace_period_seconds`.\nThe number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. Default: 120, Minimum 1, Maximum 600."]
    pub fn set_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grace_period_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job_Termination {
    type O = terrars::BlockAssignable<App_Spec_Job_Termination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job_Termination {}

impl BuildApp_Spec_Job_Termination {
    pub fn build(self) -> App_Spec_Job_Termination {
        App_Spec_Job_Termination { grace_period_seconds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Job_TerminationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Job_TerminationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Job_TerminationRef {
        App_Spec_Job_TerminationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Job_TerminationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `grace_period_seconds` after provisioning.\nThe number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. Default: 120, Minimum 1, Maximum 600."]
    pub fn grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_period_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_JobDynamic {
    alert: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Alert>>,
    bitbucket: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Bitbucket>>,
    env: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Env>>,
    git: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Git>>,
    github: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Github>>,
    gitlab: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Gitlab>>,
    image: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Image>>,
    log_destination: std::option::Option<terrars::DynamicBlock<App_Spec_Job_LogDestination>>,
    termination: std::option::Option<terrars::DynamicBlock<App_Spec_Job_Termination>>,
}

#[derive(Serialize)]
pub struct App_Spec_Job {
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
    alert: std::option::Option<std::vec::Vec<App_Spec_Job_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<std::vec::Vec<App_Spec_Job_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<App_Spec_Job_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<App_Spec_Job_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<App_Spec_Job_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<App_Spec_Job_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<App_Spec_Job_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<App_Spec_Job_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination: std::option::Option<std::vec::Vec<App_Spec_Job_Termination>>,
    dynamic: App_Spec_JobDynamic,
}

impl App_Spec_Job {
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
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Alert>>) -> Self {
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

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Bitbucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bitbucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Env>>) -> Self {
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
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Git>>) -> Self {
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
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Github>>) -> Self {
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
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Gitlab>>) -> Self {
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
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Image>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_LogDestination>>,
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

    #[doc = "Set the field `termination`.\n"]
    pub fn set_termination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job_Termination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.termination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.termination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Job {
    type O = terrars::BlockAssignable<App_Spec_Job>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Job {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Job {
    pub fn build(self) -> App_Spec_Job {
        App_Spec_Job {
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
            bitbucket: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            image: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            termination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_JobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_JobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_JobRef {
        App_Spec_JobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_JobRef {
    #[allow(dead_code)]
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
    pub fn alert(&self) -> terrars::ListRef<App_Spec_Job_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<App_Spec_Job_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<App_Spec_Job_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<App_Spec_Job_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<App_Spec_Job_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<App_Spec_Job_ImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<App_Spec_Job_LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `termination` after provisioning.\n"]
    pub fn termination(&self) -> terrars::ListRef<App_Spec_Job_TerminationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Maintenance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offline_page_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Maintenance {
    #[doc =
        "Set the field `archive`.\nIndicates whether the app should be archived. Setting this to true implies that enabled is set to true."]
    pub fn set_archive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.archive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nIndicates whether maintenance mode should be enabled for the app."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `offline_page_url`.\nA custom offline page to display when maintenance mode is enabled or the app is archived."]
    pub fn set_offline_page_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.offline_page_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Maintenance {
    type O = terrars::BlockAssignable<App_Spec_Maintenance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Maintenance {}

impl BuildApp_Spec_Maintenance {
    pub fn build(self) -> App_Spec_Maintenance {
        App_Spec_Maintenance {
            archive: core::default::Default::default(),
            enabled: core::default::Default::default(),
            offline_page_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_MaintenanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_MaintenanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_MaintenanceRef {
        App_Spec_MaintenanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_MaintenanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive` after provisioning.\nIndicates whether the app should be archived. Setting this to true implies that enabled is set to true."]
    pub fn archive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether maintenance mode should be enabled for the app."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `offline_page_url` after provisioning.\nA custom offline page to display when maintenance mode is enabled or the app is archived."]
    pub fn offline_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offline_page_url", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_Alert_Destinations_SlackWebhooks {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl App_Spec_Service_Alert_Destinations_SlackWebhooks { }

impl terrars::ToListMappable for App_Spec_Service_Alert_Destinations_SlackWebhooks {
    type O = terrars::BlockAssignable<App_Spec_Service_Alert_Destinations_SlackWebhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Alert_Destinations_SlackWebhooks {
    #[doc = "The Slack channel to send notifications to."]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The Slack webhook URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_Alert_Destinations_SlackWebhooks {
    pub fn build(self) -> App_Spec_Service_Alert_Destinations_SlackWebhooks {
        App_Spec_Service_Alert_Destinations_SlackWebhooks {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_Alert_Destinations_SlackWebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_Alert_Destinations_SlackWebhooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Service_Alert_Destinations_SlackWebhooksRef {
        App_Spec_Service_Alert_Destinations_SlackWebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_Alert_Destinations_SlackWebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send notifications to."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Slack webhook URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_Alert_DestinationsDynamic {
    slack_webhooks: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Alert_Destinations_SlackWebhooks>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_Alert_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_webhooks: std::option::Option<std::vec::Vec<App_Spec_Service_Alert_Destinations_SlackWebhooks>>,
    dynamic: App_Spec_Service_Alert_DestinationsDynamic,
}

impl App_Spec_Service_Alert_Destinations {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Alert_Destinations_SlackWebhooks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack_webhooks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_webhooks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_Alert_Destinations {
    type O = terrars::BlockAssignable<App_Spec_Service_Alert_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Alert_Destinations {}

impl BuildApp_Spec_Service_Alert_Destinations {
    pub fn build(self) -> App_Spec_Service_Alert_Destinations {
        App_Spec_Service_Alert_Destinations {
            emails: core::default::Default::default(),
            slack_webhooks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_Alert_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_Alert_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_Alert_DestinationsRef {
        App_Spec_Service_Alert_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_Alert_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_webhooks` after provisioning.\n"]
    pub fn slack_webhooks(&self) -> terrars::ListRef<App_Spec_Service_Alert_Destinations_SlackWebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_webhooks", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_AlertDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Alert_Destinations>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<App_Spec_Service_Alert_Destinations>>,
    dynamic: App_Spec_Service_AlertDynamic,
}

impl App_Spec_Service_Alert {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Alert_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_Alert {
    type O = terrars::BlockAssignable<App_Spec_Service_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Alert {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_Alert {
    pub fn build(self) -> App_Spec_Service_Alert {
        App_Spec_Service_Alert {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
            destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_AlertRef {
        App_Spec_Service_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_AlertRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_Autoscaling_Metrics_Cpu {
    percent: terrars::PrimField<f64>,
}

impl App_Spec_Service_Autoscaling_Metrics_Cpu { }

impl terrars::ToListMappable for App_Spec_Service_Autoscaling_Metrics_Cpu {
    type O = terrars::BlockAssignable<App_Spec_Service_Autoscaling_Metrics_Cpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Autoscaling_Metrics_Cpu {
    #[doc = "The average target CPU utilization for the component."]
    pub percent: terrars::PrimField<f64>,
}

impl BuildApp_Spec_Service_Autoscaling_Metrics_Cpu {
    pub fn build(self) -> App_Spec_Service_Autoscaling_Metrics_Cpu {
        App_Spec_Service_Autoscaling_Metrics_Cpu { percent: self.percent }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_Autoscaling_Metrics_CpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_Autoscaling_Metrics_CpuRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_Autoscaling_Metrics_CpuRef {
        App_Spec_Service_Autoscaling_Metrics_CpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_Autoscaling_Metrics_CpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nThe average target CPU utilization for the component."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_Autoscaling_MetricsDynamic {
    cpu: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Autoscaling_Metrics_Cpu>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_Autoscaling_Metrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<std::vec::Vec<App_Spec_Service_Autoscaling_Metrics_Cpu>>,
    dynamic: App_Spec_Service_Autoscaling_MetricsDynamic,
}

impl App_Spec_Service_Autoscaling_Metrics {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Autoscaling_Metrics_Cpu>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cpu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_Autoscaling_Metrics {
    type O = terrars::BlockAssignable<App_Spec_Service_Autoscaling_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Autoscaling_Metrics {}

impl BuildApp_Spec_Service_Autoscaling_Metrics {
    pub fn build(self) -> App_Spec_Service_Autoscaling_Metrics {
        App_Spec_Service_Autoscaling_Metrics {
            cpu: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_Autoscaling_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_Autoscaling_MetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_Autoscaling_MetricsRef {
        App_Spec_Service_Autoscaling_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_Autoscaling_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::ListRef<App_Spec_Service_Autoscaling_Metrics_CpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_AutoscalingDynamic {
    metrics: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Autoscaling_Metrics>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_Autoscaling {
    max_instance_count: terrars::PrimField<f64>,
    min_instance_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<std::vec::Vec<App_Spec_Service_Autoscaling_Metrics>>,
    dynamic: App_Spec_Service_AutoscalingDynamic,
}

impl App_Spec_Service_Autoscaling {
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Autoscaling_Metrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_Autoscaling {
    type O = terrars::BlockAssignable<App_Spec_Service_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Autoscaling {
    #[doc = "The maximum amount of instances for this component. Must be more than min_instance_count."]
    pub max_instance_count: terrars::PrimField<f64>,
    #[doc = "The minimum amount of instances for this component. Must be less than max_instance_count."]
    pub min_instance_count: terrars::PrimField<f64>,
}

impl BuildApp_Spec_Service_Autoscaling {
    pub fn build(self) -> App_Spec_Service_Autoscaling {
        App_Spec_Service_Autoscaling {
            max_instance_count: self.max_instance_count,
            min_instance_count: self.min_instance_count,
            metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_AutoscalingRef {
        App_Spec_Service_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_AutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_instance_count` after provisioning.\nThe maximum amount of instances for this component. Must be more than min_instance_count."]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instance_count` after provisioning.\nThe minimum amount of instances for this component. Must be less than max_instance_count."]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<App_Spec_Service_Autoscaling_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_Bitbucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_Bitbucket {
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

impl terrars::ToListMappable for App_Spec_Service_Bitbucket {
    type O = terrars::BlockAssignable<App_Spec_Service_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Bitbucket {}

impl BuildApp_Spec_Service_Bitbucket {
    pub fn build(self) -> App_Spec_Service_Bitbucket {
        App_Spec_Service_Bitbucket {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_BitbucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_BitbucketRef {
        App_Spec_Service_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_BitbucketRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_Cors_AllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_Cors_AllowOrigins {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match."]
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

impl terrars::ToListMappable for App_Spec_Service_Cors_AllowOrigins {
    type O = terrars::BlockAssignable<App_Spec_Service_Cors_AllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Cors_AllowOrigins {}

impl BuildApp_Spec_Service_Cors_AllowOrigins {
    pub fn build(self) -> App_Spec_Service_Cors_AllowOrigins {
        App_Spec_Service_Cors_AllowOrigins {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_Cors_AllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_Cors_AllowOriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_Cors_AllowOriginsRef {
        App_Spec_Service_Cors_AllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_Cors_AllowOriginsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_CorsDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Cors_AllowOrigins>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_Cors {
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
    allow_origins: std::option::Option<std::vec::Vec<App_Spec_Service_Cors_AllowOrigins>>,
    dynamic: App_Spec_Service_CorsDynamic,
}

impl App_Spec_Service_Cors {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Cors_AllowOrigins>>,
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

impl terrars::ToListMappable for App_Spec_Service_Cors {
    type O = terrars::BlockAssignable<App_Spec_Service_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Cors {}

impl BuildApp_Spec_Service_Cors {
    pub fn build(self) -> App_Spec_Service_Cors {
        App_Spec_Service_Cors {
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

#[allow(dead_code)]
pub struct App_Spec_Service_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_CorsRef {
        App_Spec_Service_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_CorsRef {
    #[allow(dead_code)]
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
    pub fn allow_origins(&self) -> terrars::ListRef<App_Spec_Service_Cors_AllowOriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_Env {
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

impl terrars::ToListMappable for App_Spec_Service_Env {
    type O = terrars::BlockAssignable<App_Spec_Service_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Env {}

impl BuildApp_Spec_Service_Env {
    pub fn build(self) -> App_Spec_Service_Env {
        App_Spec_Service_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_EnvRef {
        App_Spec_Service_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_EnvRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_Git {
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

impl terrars::ToListMappable for App_Spec_Service_Git {
    type O = terrars::BlockAssignable<App_Spec_Service_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Git {}

impl BuildApp_Spec_Service_Git {
    pub fn build(self) -> App_Spec_Service_Git {
        App_Spec_Service_Git {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_GitRef {
        App_Spec_Service_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_GitRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_Github {
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

impl terrars::ToListMappable for App_Spec_Service_Github {
    type O = terrars::BlockAssignable<App_Spec_Service_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Github {}

impl BuildApp_Spec_Service_Github {
    pub fn build(self) -> App_Spec_Service_Github {
        App_Spec_Service_Github {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_GithubRef {
        App_Spec_Service_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_GithubRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_Gitlab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_Gitlab {
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

impl terrars::ToListMappable for App_Spec_Service_Gitlab {
    type O = terrars::BlockAssignable<App_Spec_Service_Gitlab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Gitlab {}

impl BuildApp_Spec_Service_Gitlab {
    pub fn build(self) -> App_Spec_Service_Gitlab {
        App_Spec_Service_Gitlab {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_GitlabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_GitlabRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_GitlabRef {
        App_Spec_Service_GitlabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_GitlabRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_HealthCheck {
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

impl App_Spec_Service_HealthCheck {
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

impl terrars::ToListMappable for App_Spec_Service_HealthCheck {
    type O = terrars::BlockAssignable<App_Spec_Service_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_HealthCheck {}

impl BuildApp_Spec_Service_HealthCheck {
    pub fn build(self) -> App_Spec_Service_HealthCheck {
        App_Spec_Service_HealthCheck {
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
pub struct App_Spec_Service_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_HealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_HealthCheckRef {
        App_Spec_Service_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_HealthCheckRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_Image_DeployOnPush {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl App_Spec_Service_Image_DeployOnPush {
    #[doc = "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_Image_DeployOnPush {
    type O = terrars::BlockAssignable<App_Spec_Service_Image_DeployOnPush>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Image_DeployOnPush {}

impl BuildApp_Spec_Service_Image_DeployOnPush {
    pub fn build(self) -> App_Spec_Service_Image_DeployOnPush {
        App_Spec_Service_Image_DeployOnPush { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_Image_DeployOnPushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_Image_DeployOnPushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_Image_DeployOnPushRef {
        App_Spec_Service_Image_DeployOnPushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_Image_DeployOnPushRef {
    #[allow(dead_code)]
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
struct App_Spec_Service_ImageDynamic {
    deploy_on_push: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Image_DeployOnPush>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_Image {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_type: terrars::PrimField<std::string::String>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<std::vec::Vec<App_Spec_Service_Image_DeployOnPush>>,
    dynamic: App_Spec_Service_ImageDynamic,
}

impl App_Spec_Service_Image {
    #[doc = "Set the field `digest`.\nThe image digest. Cannot be specified if tag is provided."]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_credentials`.\nAccess credentials for third-party registries"]
    pub fn set_registry_credentials(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.registry_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided. Cannot be specified if digest is provided."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Image_DeployOnPush>>,
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

impl terrars::ToListMappable for App_Spec_Service_Image {
    type O = terrars::BlockAssignable<App_Spec_Service_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Image {
    #[doc = "The registry type."]
    pub registry_type: terrars::PrimField<std::string::String>,
    #[doc = "The repository name."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_Image {
    pub fn build(self) -> App_Spec_Service_Image {
        App_Spec_Service_Image {
            digest: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_credentials: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_ImageRef {
        App_Spec_Service_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `digest` after provisioning.\nThe image digest. Cannot be specified if tag is provided."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry_credentials` after provisioning.\nAccess credentials for third-party registries"]
    pub fn registry_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_credentials", self.base))
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
        "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided. Cannot be specified if digest is provided."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<App_Spec_Service_Image_DeployOnPushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_LivenessHealthCheck {
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

impl App_Spec_Service_LivenessHealthCheck {
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

impl terrars::ToListMappable for App_Spec_Service_LivenessHealthCheck {
    type O = terrars::BlockAssignable<App_Spec_Service_LivenessHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LivenessHealthCheck {}

impl BuildApp_Spec_Service_LivenessHealthCheck {
    pub fn build(self) -> App_Spec_Service_LivenessHealthCheck {
        App_Spec_Service_LivenessHealthCheck {
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
pub struct App_Spec_Service_LivenessHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LivenessHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_LivenessHealthCheckRef {
        App_Spec_Service_LivenessHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LivenessHealthCheckRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_LogDestination_Datadog {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_LogDestination_Datadog {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_LogDestination_Datadog {
    type O = terrars::BlockAssignable<App_Spec_Service_LogDestination_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LogDestination_Datadog {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_LogDestination_Datadog {
    pub fn build(self) -> App_Spec_Service_LogDestination_Datadog {
        App_Spec_Service_LogDestination_Datadog {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_LogDestination_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LogDestination_DatadogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_LogDestination_DatadogRef {
        App_Spec_Service_LogDestination_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LogDestination_DatadogRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Service_LogDestination_Logtail {
    token: terrars::PrimField<std::string::String>,
}

impl App_Spec_Service_LogDestination_Logtail { }

impl terrars::ToListMappable for App_Spec_Service_LogDestination_Logtail {
    type O = terrars::BlockAssignable<App_Spec_Service_LogDestination_Logtail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LogDestination_Logtail {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_LogDestination_Logtail {
    pub fn build(self) -> App_Spec_Service_LogDestination_Logtail {
        App_Spec_Service_LogDestination_Logtail { token: self.token }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_LogDestination_LogtailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LogDestination_LogtailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_LogDestination_LogtailRef {
        App_Spec_Service_LogDestination_LogtailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LogDestination_LogtailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_LogDestination_OpenSearch_BasicAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Service_LogDestination_OpenSearch_BasicAuth {
    #[doc = "Set the field `password`.\nPassword for basic authentication."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nuser for basic authentication."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_LogDestination_OpenSearch_BasicAuth {
    type O = terrars::BlockAssignable<App_Spec_Service_LogDestination_OpenSearch_BasicAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LogDestination_OpenSearch_BasicAuth {}

impl BuildApp_Spec_Service_LogDestination_OpenSearch_BasicAuth {
    pub fn build(self) -> App_Spec_Service_LogDestination_OpenSearch_BasicAuth {
        App_Spec_Service_LogDestination_OpenSearch_BasicAuth {
            password: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_LogDestination_OpenSearch_BasicAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LogDestination_OpenSearch_BasicAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Service_LogDestination_OpenSearch_BasicAuthRef {
        App_Spec_Service_LogDestination_OpenSearch_BasicAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LogDestination_OpenSearch_BasicAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for basic authentication."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nuser for basic authentication."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_LogDestination_OpenSearchDynamic {
    basic_auth: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LogDestination_OpenSearch_BasicAuth>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_LogDestination_OpenSearch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth: std::option::Option<std::vec::Vec<App_Spec_Service_LogDestination_OpenSearch_BasicAuth>>,
    dynamic: App_Spec_Service_LogDestination_OpenSearchDynamic,
}

impl App_Spec_Service_LogDestination_OpenSearch {
    #[doc = "Set the field `cluster_name`.\nOpenSearch cluster name."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nOpenSearch endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\nOpenSearch index name."]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LogDestination_OpenSearch_BasicAuth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_LogDestination_OpenSearch {
    type O = terrars::BlockAssignable<App_Spec_Service_LogDestination_OpenSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LogDestination_OpenSearch {}

impl BuildApp_Spec_Service_LogDestination_OpenSearch {
    pub fn build(self) -> App_Spec_Service_LogDestination_OpenSearch {
        App_Spec_Service_LogDestination_OpenSearch {
            cluster_name: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            index_name: core::default::Default::default(),
            basic_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_LogDestination_OpenSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LogDestination_OpenSearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_LogDestination_OpenSearchRef {
        App_Spec_Service_LogDestination_OpenSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LogDestination_OpenSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nOpenSearch cluster name."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nOpenSearch endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\nOpenSearch index name."]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::ListRef<App_Spec_Service_LogDestination_OpenSearch_BasicAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_LogDestination_Papertrail {
    endpoint: terrars::PrimField<std::string::String>,
}

impl App_Spec_Service_LogDestination_Papertrail { }

impl terrars::ToListMappable for App_Spec_Service_LogDestination_Papertrail {
    type O = terrars::BlockAssignable<App_Spec_Service_LogDestination_Papertrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LogDestination_Papertrail {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_LogDestination_Papertrail {
    pub fn build(self) -> App_Spec_Service_LogDestination_Papertrail {
        App_Spec_Service_LogDestination_Papertrail { endpoint: self.endpoint }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_LogDestination_PapertrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LogDestination_PapertrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_LogDestination_PapertrailRef {
        App_Spec_Service_LogDestination_PapertrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LogDestination_PapertrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Service_LogDestinationDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LogDestination_Datadog>>,
    logtail: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LogDestination_Logtail>>,
    open_search: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LogDestination_OpenSearch>>,
    papertrail: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LogDestination_Papertrail>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service_LogDestination {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<App_Spec_Service_LogDestination_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<App_Spec_Service_LogDestination_Logtail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_search: std::option::Option<std::vec::Vec<App_Spec_Service_LogDestination_OpenSearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<App_Spec_Service_LogDestination_Papertrail>>,
    dynamic: App_Spec_Service_LogDestinationDynamic,
}

impl App_Spec_Service_LogDestination {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LogDestination_Datadog>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LogDestination_Logtail>>,
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

    #[doc = "Set the field `open_search`.\n"]
    pub fn set_open_search(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LogDestination_OpenSearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_search = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LogDestination_Papertrail>>,
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

impl terrars::ToListMappable for App_Spec_Service_LogDestination {
    type O = terrars::BlockAssignable<App_Spec_Service_LogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_LogDestination {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service_LogDestination {
    pub fn build(self) -> App_Spec_Service_LogDestination {
        App_Spec_Service_LogDestination {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            open_search: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_LogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_LogDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_LogDestinationRef {
        App_Spec_Service_LogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_LogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<App_Spec_Service_LogDestination_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<App_Spec_Service_LogDestination_LogtailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `open_search` after provisioning.\n"]
    pub fn open_search(&self) -> terrars::ListRef<App_Spec_Service_LogDestination_OpenSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_search", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<App_Spec_Service_LogDestination_PapertrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Service_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl App_Spec_Service_Routes {
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

impl terrars::ToListMappable for App_Spec_Service_Routes {
    type O = terrars::BlockAssignable<App_Spec_Service_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Routes {}

impl BuildApp_Spec_Service_Routes {
    pub fn build(self) -> App_Spec_Service_Routes {
        App_Spec_Service_Routes {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_RoutesRef {
        App_Spec_Service_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_RoutesRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct App_Spec_Service_Termination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drain_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl App_Spec_Service_Termination {
    #[doc =
        "Set the field `drain_seconds`.\nThe number of seconds to wait between selecting a container instance for termination and issuing the TERM signal. Selecting a container instance for termination begins an asynchronous drain of new requests on upstream load-balancers. Default: 15 seconds, Minimum 1, Maximum 110."]
    pub fn set_drain_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.drain_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `grace_period_seconds`.\nThe number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. Default: 120, Minimum 1, Maximum 600."]
    pub fn set_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grace_period_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service_Termination {
    type O = terrars::BlockAssignable<App_Spec_Service_Termination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service_Termination {}

impl BuildApp_Spec_Service_Termination {
    pub fn build(self) -> App_Spec_Service_Termination {
        App_Spec_Service_Termination {
            drain_seconds: core::default::Default::default(),
            grace_period_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Service_TerminationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Service_TerminationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Service_TerminationRef {
        App_Spec_Service_TerminationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Service_TerminationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drain_seconds` after provisioning.\nThe number of seconds to wait between selecting a container instance for termination and issuing the TERM signal. Selecting a container instance for termination begins an asynchronous drain of new requests on upstream load-balancers. Default: 15 seconds, Minimum 1, Maximum 110."]
    pub fn drain_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `grace_period_seconds` after provisioning.\nThe number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. Default: 120, Minimum 1, Maximum 600."]
    pub fn grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_period_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_ServiceDynamic {
    alert: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Alert>>,
    autoscaling: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Autoscaling>>,
    bitbucket: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Bitbucket>>,
    cors: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Cors>>,
    env: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Env>>,
    git: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Git>>,
    github: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Github>>,
    gitlab: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Gitlab>>,
    health_check: std::option::Option<terrars::DynamicBlock<App_Spec_Service_HealthCheck>>,
    image: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Image>>,
    liveness_health_check: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LivenessHealthCheck>>,
    log_destination: std::option::Option<terrars::DynamicBlock<App_Spec_Service_LogDestination>>,
    routes: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Routes>>,
    termination: std::option::Option<terrars::DynamicBlock<App_Spec_Service_Termination>>,
}

#[derive(Serialize)]
pub struct App_Spec_Service {
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
    alert: std::option::Option<std::vec::Vec<App_Spec_Service_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<App_Spec_Service_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<std::vec::Vec<App_Spec_Service_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<App_Spec_Service_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<App_Spec_Service_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<App_Spec_Service_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<App_Spec_Service_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<App_Spec_Service_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<std::vec::Vec<App_Spec_Service_HealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<App_Spec_Service_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_health_check: std::option::Option<std::vec::Vec<App_Spec_Service_LivenessHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<App_Spec_Service_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<std::vec::Vec<App_Spec_Service_Routes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination: std::option::Option<std::vec::Vec<App_Spec_Service_Termination>>,
    dynamic: App_Spec_ServiceDynamic,
}

impl App_Spec_Service {
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
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Alert>>) -> Self {
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

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Autoscaling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Bitbucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bitbucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Cors>>) -> Self {
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
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Env>>) -> Self {
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
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Git>>) -> Self {
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
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Github>>) -> Self {
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
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Gitlab>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_HealthCheck>>,
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
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Image>>) -> Self {
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

    #[doc = "Set the field `liveness_health_check`.\n"]
    pub fn set_liveness_health_check(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LivenessHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.liveness_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_LogDestination>>,
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
    pub fn set_routes(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Routes>>) -> Self {
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

    #[doc = "Set the field `termination`.\n"]
    pub fn set_termination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service_Termination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.termination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.termination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Service {
    type O = terrars::BlockAssignable<App_Spec_Service>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Service {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Service {
    pub fn build(self) -> App_Spec_Service {
        App_Spec_Service {
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
            autoscaling: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
            cors: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            health_check: core::default::Default::default(),
            image: core::default::Default::default(),
            liveness_health_check: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            routes: core::default::Default::default(),
            termination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_ServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_ServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_ServiceRef {
        App_Spec_ServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_ServiceRef {
    #[allow(dead_code)]
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
    pub fn alert(&self) -> terrars::ListRef<App_Spec_Service_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<App_Spec_Service_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<App_Spec_Service_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<App_Spec_Service_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<App_Spec_Service_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<App_Spec_Service_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<App_Spec_Service_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<App_Spec_Service_HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<App_Spec_Service_ImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `liveness_health_check` after provisioning.\n"]
    pub fn liveness_health_check(&self) -> terrars::ListRef<App_Spec_Service_LivenessHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<App_Spec_Service_LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<App_Spec_Service_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }

    #[doc = "Get a reference to the value of field `termination` after provisioning.\n"]
    pub fn termination(&self) -> terrars::ListRef<App_Spec_Service_TerminationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_StaticSite_Bitbucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_StaticSite_Bitbucket {
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

impl terrars::ToListMappable for App_Spec_StaticSite_Bitbucket {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Bitbucket {}

impl BuildApp_Spec_StaticSite_Bitbucket {
    pub fn build(self) -> App_Spec_StaticSite_Bitbucket {
        App_Spec_StaticSite_Bitbucket {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_BitbucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_BitbucketRef {
        App_Spec_StaticSite_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_BitbucketRef {
    #[allow(dead_code)]
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
pub struct App_Spec_StaticSite_Cors_AllowOrigins {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_StaticSite_Cors_AllowOrigins {
    #[doc = "Set the field `exact`.\nExact string match."]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nPrefix-based match."]
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

impl terrars::ToListMappable for App_Spec_StaticSite_Cors_AllowOrigins {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Cors_AllowOrigins>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Cors_AllowOrigins {}

impl BuildApp_Spec_StaticSite_Cors_AllowOrigins {
    pub fn build(self) -> App_Spec_StaticSite_Cors_AllowOrigins {
        App_Spec_StaticSite_Cors_AllowOrigins {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_Cors_AllowOriginsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_Cors_AllowOriginsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_Cors_AllowOriginsRef {
        App_Spec_StaticSite_Cors_AllowOriginsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_Cors_AllowOriginsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\nExact string match."]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nPrefix-based match."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\nRE2 style regex-based match."]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_StaticSite_CorsDynamic {
    allow_origins: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Cors_AllowOrigins>>,
}

#[derive(Serialize)]
pub struct App_Spec_StaticSite_Cors {
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
    allow_origins: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Cors_AllowOrigins>>,
    dynamic: App_Spec_StaticSite_CorsDynamic,
}

impl App_Spec_StaticSite_Cors {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Cors_AllowOrigins>>,
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

impl terrars::ToListMappable for App_Spec_StaticSite_Cors {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Cors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Cors {}

impl BuildApp_Spec_StaticSite_Cors {
    pub fn build(self) -> App_Spec_StaticSite_Cors {
        App_Spec_StaticSite_Cors {
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

#[allow(dead_code)]
pub struct App_Spec_StaticSite_CorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_CorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_CorsRef {
        App_Spec_StaticSite_CorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_CorsRef {
    #[allow(dead_code)]
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
    pub fn allow_origins(&self) -> terrars::ListRef<App_Spec_StaticSite_Cors_AllowOriginsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_StaticSite_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_StaticSite_Env {
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

impl terrars::ToListMappable for App_Spec_StaticSite_Env {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Env {}

impl BuildApp_Spec_StaticSite_Env {
    pub fn build(self) -> App_Spec_StaticSite_Env {
        App_Spec_StaticSite_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_EnvRef {
        App_Spec_StaticSite_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_EnvRef {
    #[allow(dead_code)]
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
pub struct App_Spec_StaticSite_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_StaticSite_Git {
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

impl terrars::ToListMappable for App_Spec_StaticSite_Git {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Git {}

impl BuildApp_Spec_StaticSite_Git {
    pub fn build(self) -> App_Spec_StaticSite_Git {
        App_Spec_StaticSite_Git {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_GitRef {
        App_Spec_StaticSite_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_GitRef {
    #[allow(dead_code)]
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
pub struct App_Spec_StaticSite_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_StaticSite_Github {
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

impl terrars::ToListMappable for App_Spec_StaticSite_Github {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Github {}

impl BuildApp_Spec_StaticSite_Github {
    pub fn build(self) -> App_Spec_StaticSite_Github {
        App_Spec_StaticSite_Github {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_GithubRef {
        App_Spec_StaticSite_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_GithubRef {
    #[allow(dead_code)]
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
pub struct App_Spec_StaticSite_Gitlab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_StaticSite_Gitlab {
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

impl terrars::ToListMappable for App_Spec_StaticSite_Gitlab {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Gitlab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Gitlab {}

impl BuildApp_Spec_StaticSite_Gitlab {
    pub fn build(self) -> App_Spec_StaticSite_Gitlab {
        App_Spec_StaticSite_Gitlab {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_GitlabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_GitlabRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_GitlabRef {
        App_Spec_StaticSite_GitlabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_GitlabRef {
    #[allow(dead_code)]
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
pub struct App_Spec_StaticSite_Routes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_path_prefix: std::option::Option<terrars::PrimField<bool>>,
}

impl App_Spec_StaticSite_Routes {
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

impl terrars::ToListMappable for App_Spec_StaticSite_Routes {
    type O = terrars::BlockAssignable<App_Spec_StaticSite_Routes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite_Routes {}

impl BuildApp_Spec_StaticSite_Routes {
    pub fn build(self) -> App_Spec_StaticSite_Routes {
        App_Spec_StaticSite_Routes {
            path: core::default::Default::default(),
            preserve_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_StaticSite_RoutesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSite_RoutesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSite_RoutesRef {
        App_Spec_StaticSite_RoutesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSite_RoutesRef {
    #[allow(dead_code)]
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
struct App_Spec_StaticSiteDynamic {
    bitbucket: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Bitbucket>>,
    cors: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Cors>>,
    env: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Env>>,
    git: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Git>>,
    github: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Github>>,
    gitlab: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Gitlab>>,
    routes: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite_Routes>>,
}

#[derive(Serialize)]
pub struct App_Spec_StaticSite {
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
    bitbucket: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Cors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routes: std::option::Option<std::vec::Vec<App_Spec_StaticSite_Routes>>,
    dynamic: App_Spec_StaticSiteDynamic,
}

impl App_Spec_StaticSite {
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

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Bitbucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bitbucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors`.\n"]
    pub fn set_cors(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Cors>>) -> Self {
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
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Env>>) -> Self {
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
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Git>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Github>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Gitlab>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite_Routes>>,
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

impl terrars::ToListMappable for App_Spec_StaticSite {
    type O = terrars::BlockAssignable<App_Spec_StaticSite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_StaticSite {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_StaticSite {
    pub fn build(self) -> App_Spec_StaticSite {
        App_Spec_StaticSite {
            build_command: core::default::Default::default(),
            catchall_document: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            error_document: core::default::Default::default(),
            index_document: core::default::Default::default(),
            name: self.name,
            output_dir: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct App_Spec_StaticSiteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_StaticSiteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_StaticSiteRef {
        App_Spec_StaticSiteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_StaticSiteRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<App_Spec_StaticSite_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `cors` after provisioning.\n"]
    pub fn cors(&self) -> terrars::ListRef<App_Spec_StaticSite_CorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<App_Spec_StaticSite_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<App_Spec_StaticSite_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<App_Spec_StaticSite_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `routes` after provisioning.\n"]
    pub fn routes(&self) -> terrars::ListRef<App_Spec_StaticSite_RoutesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routes", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Vpc {
    id: terrars::PrimField<std::string::String>,
}

impl App_Spec_Vpc { }

impl terrars::ToListMappable for App_Spec_Vpc {
    type O = terrars::BlockAssignable<App_Spec_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Vpc {
    #[doc = "The ID of the VPC."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Vpc {
    pub fn build(self) -> App_Spec_Vpc {
        App_Spec_Vpc { id: self.id }
    }
}

#[allow(dead_code)]
pub struct App_Spec_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_VpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_VpcRef {
        App_Spec_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the VPC."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Alert_Destinations_SlackWebhooks {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl App_Spec_Worker_Alert_Destinations_SlackWebhooks { }

impl terrars::ToListMappable for App_Spec_Worker_Alert_Destinations_SlackWebhooks {
    type O = terrars::BlockAssignable<App_Spec_Worker_Alert_Destinations_SlackWebhooks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Alert_Destinations_SlackWebhooks {
    #[doc = "The Slack channel to send notifications to."]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The Slack webhook URL."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_Alert_Destinations_SlackWebhooks {
    pub fn build(self) -> App_Spec_Worker_Alert_Destinations_SlackWebhooks {
        App_Spec_Worker_Alert_Destinations_SlackWebhooks {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_Alert_Destinations_SlackWebhooksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_Alert_Destinations_SlackWebhooksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Worker_Alert_Destinations_SlackWebhooksRef {
        App_Spec_Worker_Alert_Destinations_SlackWebhooksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_Alert_Destinations_SlackWebhooksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send notifications to."]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe Slack webhook URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Worker_Alert_DestinationsDynamic {
    slack_webhooks: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Alert_Destinations_SlackWebhooks>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Alert_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack_webhooks: std::option::Option<std::vec::Vec<App_Spec_Worker_Alert_Destinations_SlackWebhooks>>,
    dynamic: App_Spec_Worker_Alert_DestinationsDynamic,
}

impl App_Spec_Worker_Alert_Destinations {
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Alert_Destinations_SlackWebhooks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack_webhooks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack_webhooks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_Alert_Destinations {
    type O = terrars::BlockAssignable<App_Spec_Worker_Alert_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Alert_Destinations {}

impl BuildApp_Spec_Worker_Alert_Destinations {
    pub fn build(self) -> App_Spec_Worker_Alert_Destinations {
        App_Spec_Worker_Alert_Destinations {
            emails: core::default::Default::default(),
            slack_webhooks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_Alert_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_Alert_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_Alert_DestinationsRef {
        App_Spec_Worker_Alert_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_Alert_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `emails` after provisioning.\n"]
    pub fn emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.emails", self.base))
    }

    #[doc = "Get a reference to the value of field `slack_webhooks` after provisioning.\n"]
    pub fn slack_webhooks(&self) -> terrars::ListRef<App_Spec_Worker_Alert_Destinations_SlackWebhooksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack_webhooks", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Worker_AlertDynamic {
    destinations: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Alert_Destinations>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Alert {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    operator: terrars::PrimField<std::string::String>,
    rule: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<App_Spec_Worker_Alert_Destinations>>,
    dynamic: App_Spec_Worker_AlertDynamic,
}

impl App_Spec_Worker_Alert {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Alert_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_Alert {
    type O = terrars::BlockAssignable<App_Spec_Worker_Alert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Alert {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_Alert {
    pub fn build(self) -> App_Spec_Worker_Alert {
        App_Spec_Worker_Alert {
            disabled: core::default::Default::default(),
            operator: self.operator,
            rule: self.rule,
            value: self.value,
            window: self.window,
            destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_AlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_AlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_AlertRef {
        App_Spec_Worker_AlertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_AlertRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_Autoscaling_Metrics_Cpu {
    percent: terrars::PrimField<f64>,
}

impl App_Spec_Worker_Autoscaling_Metrics_Cpu { }

impl terrars::ToListMappable for App_Spec_Worker_Autoscaling_Metrics_Cpu {
    type O = terrars::BlockAssignable<App_Spec_Worker_Autoscaling_Metrics_Cpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Autoscaling_Metrics_Cpu {
    #[doc = "The average target CPU utilization for the component."]
    pub percent: terrars::PrimField<f64>,
}

impl BuildApp_Spec_Worker_Autoscaling_Metrics_Cpu {
    pub fn build(self) -> App_Spec_Worker_Autoscaling_Metrics_Cpu {
        App_Spec_Worker_Autoscaling_Metrics_Cpu { percent: self.percent }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_Autoscaling_Metrics_CpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_Autoscaling_Metrics_CpuRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_Autoscaling_Metrics_CpuRef {
        App_Spec_Worker_Autoscaling_Metrics_CpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_Autoscaling_Metrics_CpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nThe average target CPU utilization for the component."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Worker_Autoscaling_MetricsDynamic {
    cpu: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Autoscaling_Metrics_Cpu>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Autoscaling_Metrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<std::vec::Vec<App_Spec_Worker_Autoscaling_Metrics_Cpu>>,
    dynamic: App_Spec_Worker_Autoscaling_MetricsDynamic,
}

impl App_Spec_Worker_Autoscaling_Metrics {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Autoscaling_Metrics_Cpu>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cpu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_Autoscaling_Metrics {
    type O = terrars::BlockAssignable<App_Spec_Worker_Autoscaling_Metrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Autoscaling_Metrics {}

impl BuildApp_Spec_Worker_Autoscaling_Metrics {
    pub fn build(self) -> App_Spec_Worker_Autoscaling_Metrics {
        App_Spec_Worker_Autoscaling_Metrics {
            cpu: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_Autoscaling_MetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_Autoscaling_MetricsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_Autoscaling_MetricsRef {
        App_Spec_Worker_Autoscaling_MetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_Autoscaling_MetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::ListRef<App_Spec_Worker_Autoscaling_Metrics_CpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Worker_AutoscalingDynamic {
    metrics: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Autoscaling_Metrics>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Autoscaling {
    max_instance_count: terrars::PrimField<f64>,
    min_instance_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<std::vec::Vec<App_Spec_Worker_Autoscaling_Metrics>>,
    dynamic: App_Spec_Worker_AutoscalingDynamic,
}

impl App_Spec_Worker_Autoscaling {
    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Autoscaling_Metrics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_Autoscaling {
    type O = terrars::BlockAssignable<App_Spec_Worker_Autoscaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Autoscaling {
    #[doc = "The maximum amount of instances for this component. Must be more than min_instance_count."]
    pub max_instance_count: terrars::PrimField<f64>,
    #[doc = "The minimum amount of instances for this component. Must be less than max_instance_count."]
    pub min_instance_count: terrars::PrimField<f64>,
}

impl BuildApp_Spec_Worker_Autoscaling {
    pub fn build(self) -> App_Spec_Worker_Autoscaling {
        App_Spec_Worker_Autoscaling {
            max_instance_count: self.max_instance_count,
            min_instance_count: self.min_instance_count,
            metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_AutoscalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_AutoscalingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_AutoscalingRef {
        App_Spec_Worker_AutoscalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_AutoscalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_instance_count` after provisioning.\nThe maximum amount of instances for this component. Must be more than min_instance_count."]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instance_count` after provisioning.\nThe minimum amount of instances for this component. Must be less than max_instance_count."]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<App_Spec_Worker_Autoscaling_MetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Bitbucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_Bitbucket {
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

impl terrars::ToListMappable for App_Spec_Worker_Bitbucket {
    type O = terrars::BlockAssignable<App_Spec_Worker_Bitbucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Bitbucket {}

impl BuildApp_Spec_Worker_Bitbucket {
    pub fn build(self) -> App_Spec_Worker_Bitbucket {
        App_Spec_Worker_Bitbucket {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_BitbucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_BitbucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_BitbucketRef {
        App_Spec_Worker_BitbucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_BitbucketRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_Env {
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

impl terrars::ToListMappable for App_Spec_Worker_Env {
    type O = terrars::BlockAssignable<App_Spec_Worker_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Env {}

impl BuildApp_Spec_Worker_Env {
    pub fn build(self) -> App_Spec_Worker_Env {
        App_Spec_Worker_Env {
            key: core::default::Default::default(),
            scope: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_EnvRef {
        App_Spec_Worker_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_EnvRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_clone_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_Git {
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

impl terrars::ToListMappable for App_Spec_Worker_Git {
    type O = terrars::BlockAssignable<App_Spec_Worker_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Git {}

impl BuildApp_Spec_Worker_Git {
    pub fn build(self) -> App_Spec_Worker_Git {
        App_Spec_Worker_Git {
            branch: core::default::Default::default(),
            repo_clone_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_GitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_GitRef {
        App_Spec_Worker_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_GitRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_Github {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_Github {
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

impl terrars::ToListMappable for App_Spec_Worker_Github {
    type O = terrars::BlockAssignable<App_Spec_Worker_Github>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Github {}

impl BuildApp_Spec_Worker_Github {
    pub fn build(self) -> App_Spec_Worker_Github {
        App_Spec_Worker_Github {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_GithubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_GithubRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_GithubRef {
        App_Spec_Worker_GithubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_GithubRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_Gitlab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_Gitlab {
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

impl terrars::ToListMappable for App_Spec_Worker_Gitlab {
    type O = terrars::BlockAssignable<App_Spec_Worker_Gitlab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Gitlab {}

impl BuildApp_Spec_Worker_Gitlab {
    pub fn build(self) -> App_Spec_Worker_Gitlab {
        App_Spec_Worker_Gitlab {
            branch: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            repo: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_GitlabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_GitlabRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_GitlabRef {
        App_Spec_Worker_GitlabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_GitlabRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_Image_DeployOnPush {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl App_Spec_Worker_Image_DeployOnPush {
    #[doc = "Set the field `enabled`.\nWhether to automatically deploy images pushed to DOCR."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_Image_DeployOnPush {
    type O = terrars::BlockAssignable<App_Spec_Worker_Image_DeployOnPush>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Image_DeployOnPush {}

impl BuildApp_Spec_Worker_Image_DeployOnPush {
    pub fn build(self) -> App_Spec_Worker_Image_DeployOnPush {
        App_Spec_Worker_Image_DeployOnPush { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_Image_DeployOnPushRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_Image_DeployOnPushRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_Image_DeployOnPushRef {
        App_Spec_Worker_Image_DeployOnPushRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_Image_DeployOnPushRef {
    #[allow(dead_code)]
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
struct App_Spec_Worker_ImageDynamic {
    deploy_on_push: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Image_DeployOnPush>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Image {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_credentials: std::option::Option<terrars::PrimField<std::string::String>>,
    registry_type: terrars::PrimField<std::string::String>,
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy_on_push: std::option::Option<std::vec::Vec<App_Spec_Worker_Image_DeployOnPush>>,
    dynamic: App_Spec_Worker_ImageDynamic,
}

impl App_Spec_Worker_Image {
    #[doc = "Set the field `digest`.\nThe image digest. Cannot be specified if tag is provided."]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry`.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn set_registry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registry_credentials`.\nAccess credentials for third-party registries"]
    pub fn set_registry_credentials(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.registry_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nThe repository tag. Defaults to latest if not provided. Cannot be specified if digest is provided."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deploy_on_push`.\n"]
    pub fn set_deploy_on_push(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Image_DeployOnPush>>,
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

impl terrars::ToListMappable for App_Spec_Worker_Image {
    type O = terrars::BlockAssignable<App_Spec_Worker_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Image {
    #[doc = "The registry type."]
    pub registry_type: terrars::PrimField<std::string::String>,
    #[doc = "The repository name."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_Image {
    pub fn build(self) -> App_Spec_Worker_Image {
        App_Spec_Worker_Image {
            digest: core::default::Default::default(),
            registry: core::default::Default::default(),
            registry_credentials: core::default::Default::default(),
            registry_type: self.registry_type,
            repository: self.repository,
            tag: core::default::Default::default(),
            deploy_on_push: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_ImageRef {
        App_Spec_Worker_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `digest` after provisioning.\nThe image digest. Cannot be specified if tag is provided."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry` after provisioning.\nThe registry name. Must be left empty for the DOCR registry type."]
    pub fn registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry", self.base))
    }

    #[doc =
        "Get a reference to the value of field `registry_credentials` after provisioning.\nAccess credentials for third-party registries"]
    pub fn registry_credentials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_credentials", self.base))
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
        "Get a reference to the value of field `tag` after provisioning.\nThe repository tag. Defaults to latest if not provided. Cannot be specified if digest is provided."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `deploy_on_push` after provisioning.\n"]
    pub fn deploy_on_push(&self) -> terrars::ListRef<App_Spec_Worker_Image_DeployOnPushRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy_on_push", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Worker_LivenessHealthCheck {
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

impl App_Spec_Worker_LivenessHealthCheck {
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

impl terrars::ToListMappable for App_Spec_Worker_LivenessHealthCheck {
    type O = terrars::BlockAssignable<App_Spec_Worker_LivenessHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LivenessHealthCheck {}

impl BuildApp_Spec_Worker_LivenessHealthCheck {
    pub fn build(self) -> App_Spec_Worker_LivenessHealthCheck {
        App_Spec_Worker_LivenessHealthCheck {
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
pub struct App_Spec_Worker_LivenessHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LivenessHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_LivenessHealthCheckRef {
        App_Spec_Worker_LivenessHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LivenessHealthCheckRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_LogDestination_Datadog {
    api_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_LogDestination_Datadog {
    #[doc = "Set the field `endpoint`.\nDatadog HTTP log intake endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_LogDestination_Datadog {
    type O = terrars::BlockAssignable<App_Spec_Worker_LogDestination_Datadog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LogDestination_Datadog {
    #[doc = "Datadog API key."]
    pub api_key: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_LogDestination_Datadog {
    pub fn build(self) -> App_Spec_Worker_LogDestination_Datadog {
        App_Spec_Worker_LogDestination_Datadog {
            api_key: self.api_key,
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_LogDestination_DatadogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LogDestination_DatadogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_LogDestination_DatadogRef {
        App_Spec_Worker_LogDestination_DatadogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LogDestination_DatadogRef {
    #[allow(dead_code)]
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
pub struct App_Spec_Worker_LogDestination_Logtail {
    token: terrars::PrimField<std::string::String>,
}

impl App_Spec_Worker_LogDestination_Logtail { }

impl terrars::ToListMappable for App_Spec_Worker_LogDestination_Logtail {
    type O = terrars::BlockAssignable<App_Spec_Worker_LogDestination_Logtail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LogDestination_Logtail {
    #[doc = "Logtail token."]
    pub token: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_LogDestination_Logtail {
    pub fn build(self) -> App_Spec_Worker_LogDestination_Logtail {
        App_Spec_Worker_LogDestination_Logtail { token: self.token }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_LogDestination_LogtailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LogDestination_LogtailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_LogDestination_LogtailRef {
        App_Spec_Worker_LogDestination_LogtailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LogDestination_LogtailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\nLogtail token."]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Worker_LogDestination_OpenSearch_BasicAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Spec_Worker_LogDestination_OpenSearch_BasicAuth {
    #[doc = "Set the field `password`.\nPassword for basic authentication."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nuser for basic authentication."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_LogDestination_OpenSearch_BasicAuth {
    type O = terrars::BlockAssignable<App_Spec_Worker_LogDestination_OpenSearch_BasicAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LogDestination_OpenSearch_BasicAuth {}

impl BuildApp_Spec_Worker_LogDestination_OpenSearch_BasicAuth {
    pub fn build(self) -> App_Spec_Worker_LogDestination_OpenSearch_BasicAuth {
        App_Spec_Worker_LogDestination_OpenSearch_BasicAuth {
            password: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_LogDestination_OpenSearch_BasicAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LogDestination_OpenSearch_BasicAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> App_Spec_Worker_LogDestination_OpenSearch_BasicAuthRef {
        App_Spec_Worker_LogDestination_OpenSearch_BasicAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LogDestination_OpenSearch_BasicAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for basic authentication."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nuser for basic authentication."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Worker_LogDestination_OpenSearchDynamic {
    basic_auth: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LogDestination_OpenSearch_BasicAuth>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_LogDestination_OpenSearch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth: std::option::Option<std::vec::Vec<App_Spec_Worker_LogDestination_OpenSearch_BasicAuth>>,
    dynamic: App_Spec_Worker_LogDestination_OpenSearchDynamic,
}

impl App_Spec_Worker_LogDestination_OpenSearch {
    #[doc = "Set the field `cluster_name`.\nOpenSearch cluster name."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nOpenSearch endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_name`.\nOpenSearch index name."]
    pub fn set_index_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_auth`.\n"]
    pub fn set_basic_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LogDestination_OpenSearch_BasicAuth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_LogDestination_OpenSearch {
    type O = terrars::BlockAssignable<App_Spec_Worker_LogDestination_OpenSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LogDestination_OpenSearch {}

impl BuildApp_Spec_Worker_LogDestination_OpenSearch {
    pub fn build(self) -> App_Spec_Worker_LogDestination_OpenSearch {
        App_Spec_Worker_LogDestination_OpenSearch {
            cluster_name: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            index_name: core::default::Default::default(),
            basic_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_LogDestination_OpenSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LogDestination_OpenSearchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_LogDestination_OpenSearchRef {
        App_Spec_Worker_LogDestination_OpenSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LogDestination_OpenSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nOpenSearch cluster name."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nOpenSearch endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\nOpenSearch index name."]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_auth` after provisioning.\n"]
    pub fn basic_auth(&self) -> terrars::ListRef<App_Spec_Worker_LogDestination_OpenSearch_BasicAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_auth", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Worker_LogDestination_Papertrail {
    endpoint: terrars::PrimField<std::string::String>,
}

impl App_Spec_Worker_LogDestination_Papertrail { }

impl terrars::ToListMappable for App_Spec_Worker_LogDestination_Papertrail {
    type O = terrars::BlockAssignable<App_Spec_Worker_LogDestination_Papertrail>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LogDestination_Papertrail {
    #[doc = "Papertrail syslog endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_LogDestination_Papertrail {
    pub fn build(self) -> App_Spec_Worker_LogDestination_Papertrail {
        App_Spec_Worker_LogDestination_Papertrail { endpoint: self.endpoint }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_LogDestination_PapertrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LogDestination_PapertrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_LogDestination_PapertrailRef {
        App_Spec_Worker_LogDestination_PapertrailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LogDestination_PapertrailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nPapertrail syslog endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_Worker_LogDestinationDynamic {
    datadog: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LogDestination_Datadog>>,
    logtail: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LogDestination_Logtail>>,
    open_search: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LogDestination_OpenSearch>>,
    papertrail: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LogDestination_Papertrail>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker_LogDestination {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datadog: std::option::Option<std::vec::Vec<App_Spec_Worker_LogDestination_Datadog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logtail: std::option::Option<std::vec::Vec<App_Spec_Worker_LogDestination_Logtail>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_search: std::option::Option<std::vec::Vec<App_Spec_Worker_LogDestination_OpenSearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    papertrail: std::option::Option<std::vec::Vec<App_Spec_Worker_LogDestination_Papertrail>>,
    dynamic: App_Spec_Worker_LogDestinationDynamic,
}

impl App_Spec_Worker_LogDestination {
    #[doc = "Set the field `datadog`.\n"]
    pub fn set_datadog(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LogDestination_Datadog>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LogDestination_Logtail>>,
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

    #[doc = "Set the field `open_search`.\n"]
    pub fn set_open_search(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LogDestination_OpenSearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.open_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.open_search = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `papertrail`.\n"]
    pub fn set_papertrail(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LogDestination_Papertrail>>,
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

impl terrars::ToListMappable for App_Spec_Worker_LogDestination {
    type O = terrars::BlockAssignable<App_Spec_Worker_LogDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_LogDestination {
    #[doc = "Name of the log destination"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker_LogDestination {
    pub fn build(self) -> App_Spec_Worker_LogDestination {
        App_Spec_Worker_LogDestination {
            name: self.name,
            datadog: core::default::Default::default(),
            logtail: core::default::Default::default(),
            open_search: core::default::Default::default(),
            papertrail: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_LogDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_LogDestinationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_LogDestinationRef {
        App_Spec_Worker_LogDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_LogDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the log destination"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `datadog` after provisioning.\n"]
    pub fn datadog(&self) -> terrars::ListRef<App_Spec_Worker_LogDestination_DatadogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datadog", self.base))
    }

    #[doc = "Get a reference to the value of field `logtail` after provisioning.\n"]
    pub fn logtail(&self) -> terrars::ListRef<App_Spec_Worker_LogDestination_LogtailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logtail", self.base))
    }

    #[doc = "Get a reference to the value of field `open_search` after provisioning.\n"]
    pub fn open_search(&self) -> terrars::ListRef<App_Spec_Worker_LogDestination_OpenSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_search", self.base))
    }

    #[doc = "Get a reference to the value of field `papertrail` after provisioning.\n"]
    pub fn papertrail(&self) -> terrars::ListRef<App_Spec_Worker_LogDestination_PapertrailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.papertrail", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Spec_Worker_Termination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grace_period_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl App_Spec_Worker_Termination {
    #[doc =
        "Set the field `grace_period_seconds`.\nThe number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. Default: 120, Minimum 1, Maximum 600."]
    pub fn set_grace_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grace_period_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker_Termination {
    type O = terrars::BlockAssignable<App_Spec_Worker_Termination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker_Termination {}

impl BuildApp_Spec_Worker_Termination {
    pub fn build(self) -> App_Spec_Worker_Termination {
        App_Spec_Worker_Termination { grace_period_seconds: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_Spec_Worker_TerminationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_Worker_TerminationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_Worker_TerminationRef {
        App_Spec_Worker_TerminationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_Worker_TerminationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `grace_period_seconds` after provisioning.\nThe number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. Default: 120, Minimum 1, Maximum 600."]
    pub fn grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grace_period_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_Spec_WorkerDynamic {
    alert: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Alert>>,
    autoscaling: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Autoscaling>>,
    bitbucket: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Bitbucket>>,
    env: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Env>>,
    git: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Git>>,
    github: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Github>>,
    gitlab: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Gitlab>>,
    image: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Image>>,
    liveness_health_check: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LivenessHealthCheck>>,
    log_destination: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_LogDestination>>,
    termination: std::option::Option<terrars::DynamicBlock<App_Spec_Worker_Termination>>,
}

#[derive(Serialize)]
pub struct App_Spec_Worker {
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
    alert: std::option::Option<std::vec::Vec<App_Spec_Worker_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling: std::option::Option<std::vec::Vec<App_Spec_Worker_Autoscaling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bitbucket: std::option::Option<std::vec::Vec<App_Spec_Worker_Bitbucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<App_Spec_Worker_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<App_Spec_Worker_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    github: std::option::Option<std::vec::Vec<App_Spec_Worker_Github>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gitlab: std::option::Option<std::vec::Vec<App_Spec_Worker_Gitlab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<App_Spec_Worker_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_health_check: std::option::Option<std::vec::Vec<App_Spec_Worker_LivenessHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_destination: std::option::Option<std::vec::Vec<App_Spec_Worker_LogDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination: std::option::Option<std::vec::Vec<App_Spec_Worker_Termination>>,
    dynamic: App_Spec_WorkerDynamic,
}

impl App_Spec_Worker {
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
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Alert>>) -> Self {
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

    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Autoscaling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bitbucket`.\n"]
    pub fn set_bitbucket(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Bitbucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bitbucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bitbucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Env>>) -> Self {
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
    pub fn set_git(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Git>>) -> Self {
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
    pub fn set_github(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Github>>) -> Self {
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
    pub fn set_gitlab(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Gitlab>>) -> Self {
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
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Image>>) -> Self {
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

    #[doc = "Set the field `liveness_health_check`.\n"]
    pub fn set_liveness_health_check(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LivenessHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.liveness_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_destination`.\n"]
    pub fn set_log_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_LogDestination>>,
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

    #[doc = "Set the field `termination`.\n"]
    pub fn set_termination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker_Termination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.termination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.termination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for App_Spec_Worker {
    type O = terrars::BlockAssignable<App_Spec_Worker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec_Worker {
    #[doc = "The name of the component"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec_Worker {
    pub fn build(self) -> App_Spec_Worker {
        App_Spec_Worker {
            build_command: core::default::Default::default(),
            dockerfile_path: core::default::Default::default(),
            environment_slug: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_size_slug: core::default::Default::default(),
            name: self.name,
            run_command: core::default::Default::default(),
            source_dir: core::default::Default::default(),
            alert: core::default::Default::default(),
            autoscaling: core::default::Default::default(),
            bitbucket: core::default::Default::default(),
            env: core::default::Default::default(),
            git: core::default::Default::default(),
            github: core::default::Default::default(),
            gitlab: core::default::Default::default(),
            image: core::default::Default::default(),
            liveness_health_check: core::default::Default::default(),
            log_destination: core::default::Default::default(),
            termination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_Spec_WorkerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_Spec_WorkerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_Spec_WorkerRef {
        App_Spec_WorkerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_Spec_WorkerRef {
    #[allow(dead_code)]
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
    pub fn alert(&self) -> terrars::ListRef<App_Spec_Worker_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> terrars::ListRef<App_Spec_Worker_AutoscalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `bitbucket` after provisioning.\n"]
    pub fn bitbucket(&self) -> terrars::ListRef<App_Spec_Worker_BitbucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bitbucket", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<App_Spec_Worker_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `github` after provisioning.\n"]
    pub fn github(&self) -> terrars::ListRef<App_Spec_Worker_GithubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.github", self.base))
    }

    #[doc = "Get a reference to the value of field `gitlab` after provisioning.\n"]
    pub fn gitlab(&self) -> terrars::ListRef<App_Spec_Worker_GitlabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gitlab", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<App_Spec_Worker_ImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `liveness_health_check` after provisioning.\n"]
    pub fn liveness_health_check(&self) -> terrars::ListRef<App_Spec_Worker_LivenessHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::ListRef<App_Spec_Worker_LogDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `termination` after provisioning.\n"]
    pub fn termination(&self) -> terrars::ListRef<App_Spec_Worker_TerminationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.termination", self.base))
    }
}

#[derive(Serialize, Default)]
struct App_SpecDynamic {
    alert: std::option::Option<terrars::DynamicBlock<App_Spec_Alert>>,
    database: std::option::Option<terrars::DynamicBlock<App_Spec_Database>>,
    domain: std::option::Option<terrars::DynamicBlock<App_Spec_Domain>>,
    egress: std::option::Option<terrars::DynamicBlock<App_Spec_Egress>>,
    env: std::option::Option<terrars::DynamicBlock<App_Spec_Env>>,
    function: std::option::Option<terrars::DynamicBlock<App_Spec_Function>>,
    ingress: std::option::Option<terrars::DynamicBlock<App_Spec_Ingress>>,
    job: std::option::Option<terrars::DynamicBlock<App_Spec_Job>>,
    maintenance: std::option::Option<terrars::DynamicBlock<App_Spec_Maintenance>>,
    service: std::option::Option<terrars::DynamicBlock<App_Spec_Service>>,
    static_site: std::option::Option<terrars::DynamicBlock<App_Spec_StaticSite>>,
    vpc: std::option::Option<terrars::DynamicBlock<App_Spec_Vpc>>,
    worker: std::option::Option<terrars::DynamicBlock<App_Spec_Worker>>,
}

#[derive(Serialize)]
pub struct App_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_edge_cache: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_email_obfuscation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enhanced_threat_control_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alert: std::option::Option<std::vec::Vec<App_Spec_Alert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<std::vec::Vec<App_Spec_Database>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<std::vec::Vec<App_Spec_Domain>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<std::vec::Vec<App_Spec_Egress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<App_Spec_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<std::vec::Vec<App_Spec_Function>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress: std::option::Option<std::vec::Vec<App_Spec_Ingress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job: std::option::Option<std::vec::Vec<App_Spec_Job>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance: std::option::Option<std::vec::Vec<App_Spec_Maintenance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<std::vec::Vec<App_Spec_Service>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_site: std::option::Option<std::vec::Vec<App_Spec_StaticSite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<std::vec::Vec<App_Spec_Vpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker: std::option::Option<std::vec::Vec<App_Spec_Worker>>,
    dynamic: App_SpecDynamic,
}

impl App_Spec {
    #[doc =
        "Set the field `disable_edge_cache`.\nWhether to disable the edge cache for the app. Default is false, which enables the edge cache."]
    pub fn set_disable_edge_cache(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_edge_cache = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_email_obfuscation`.\nEmail obfuscation configuration for the app. Default is false, which keeps the email obfuscated."]
    pub fn set_disable_email_obfuscation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_email_obfuscation = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `enhanced_threat_control_enabled`.\nWhether to enable enhanced threat control for the app. Default is false. Set to true to enable enhanced threat control, putting additional security measures for Layer 7 DDoS attacks."]
    pub fn set_enhanced_threat_control_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enhanced_threat_control_enabled = std::option::Option::Some(v.into());
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
    pub fn set_alert(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Alert>>) -> Self {
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
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Database>>) -> Self {
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
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Domain>>) -> Self {
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

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Egress>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Env>>) -> Self {
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
    pub fn set_function(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Function>>) -> Self {
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
    pub fn set_ingress(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Ingress>>) -> Self {
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
    pub fn set_job(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Job>>) -> Self {
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

    #[doc = "Set the field `maintenance`.\n"]
    pub fn set_maintenance(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Maintenance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Service>>) -> Self {
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
    pub fn set_static_site(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_StaticSite>>) -> Self {
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

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Vpc>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `worker`.\n"]
    pub fn set_worker(mut self, v: impl std::convert::Into<terrars::BlockAssignable<App_Spec_Worker>>) -> Self {
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

impl terrars::ToListMappable for App_Spec {
    type O = terrars::BlockAssignable<App_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Spec {
    #[doc = "The name of the app. Must be unique across all apps in the same account."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApp_Spec {
    pub fn build(self) -> App_Spec {
        App_Spec {
            disable_edge_cache: core::default::Default::default(),
            disable_email_obfuscation: core::default::Default::default(),
            domains: core::default::Default::default(),
            enhanced_threat_control_enabled: core::default::Default::default(),
            features: core::default::Default::default(),
            name: self.name,
            region: core::default::Default::default(),
            alert: core::default::Default::default(),
            database: core::default::Default::default(),
            domain: core::default::Default::default(),
            egress: core::default::Default::default(),
            env: core::default::Default::default(),
            function: core::default::Default::default(),
            ingress: core::default::Default::default(),
            job: core::default::Default::default(),
            maintenance: core::default::Default::default(),
            service: core::default::Default::default(),
            static_site: core::default::Default::default(),
            vpc: core::default::Default::default(),
            worker: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct App_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_SpecRef {
        App_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_edge_cache` after provisioning.\nWhether to disable the edge cache for the app. Default is false, which enables the edge cache."]
    pub fn disable_edge_cache(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_edge_cache", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_email_obfuscation` after provisioning.\nEmail obfuscation configuration for the app. Default is false, which keeps the email obfuscated."]
    pub fn disable_email_obfuscation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_email_obfuscation", self.base))
    }

    #[doc = "Get a reference to the value of field `domains` after provisioning.\n"]
    pub fn domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enhanced_threat_control_enabled` after provisioning.\nWhether to enable enhanced threat control for the app. Default is false. Set to true to enable enhanced threat control, putting additional security measures for Layer 7 DDoS attacks."]
    pub fn enhanced_threat_control_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enhanced_threat_control_enabled", self.base))
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

    #[doc = "Get a reference to the value of field `alert` after provisioning.\n"]
    pub fn alert(&self) -> terrars::ListRef<App_Spec_AlertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alert", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::ListRef<App_Spec_DatabaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::ListRef<App_Spec_DomainRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::ListRef<App_Spec_EgressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::ListRef<App_Spec_FunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> terrars::ListRef<App_Spec_IngressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress", self.base))
    }

    #[doc = "Get a reference to the value of field `job` after provisioning.\n"]
    pub fn job(&self) -> terrars::ListRef<App_Spec_JobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance` after provisioning.\n"]
    pub fn maintenance(&self) -> terrars::ListRef<App_Spec_MaintenanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::ListRef<App_Spec_ServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `static_site` after provisioning.\n"]
    pub fn static_site(&self) -> terrars::ListRef<App_Spec_StaticSiteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_site", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<App_Spec_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `worker` after provisioning.\n"]
    pub fn worker(&self) -> terrars::ListRef<App_Spec_WorkerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker", self.base))
    }
}

#[derive(Serialize)]
pub struct App_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl App_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for App_Timeouts {
    type O = terrars::BlockAssignable<App_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApp_Timeouts {}

impl BuildApp_Timeouts {
    pub fn build(self) -> App_Timeouts {
        App_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct App_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for App_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> App_TimeoutsRef {
        App_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl App_TimeoutsRef {
    #[allow(dead_code)]
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
    dedicated_ips: std::option::Option<terrars::DynamicBlock<App_DedicatedIps>>,
    spec: std::option::Option<terrars::DynamicBlock<App_Spec>>,
}
