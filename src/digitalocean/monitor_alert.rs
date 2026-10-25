#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct MonitorAlertData_ {
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
    compare: terrars::PrimField<std::string::String>,
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    window: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alerts: std::option::Option<std::vec::Vec<MonitorAlert_Alerts>>,
    dynamic: MonitorAlertDynamic,
}

struct MonitorAlert_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MonitorAlertData_>,
}

#[derive(Clone)]
pub struct MonitorAlert(std::rc::Rc<MonitorAlert_>);

impl MonitorAlert {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entities`.\nThe droplets to apply the alert policy to"]
    pub fn set_entities(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().entities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alerts`.\n"]
    pub fn set_alerts(self, v: impl std::convert::Into<terrars::BlockAssignable<MonitorAlert_Alerts>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alerts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alerts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `compare` after provisioning.\nThe comparison operator to use for value"]
    pub fn compare(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compare", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the alert policy"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entities` after provisioning.\nThe droplets to apply the alert policy to"]
    pub fn entities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alerts` after provisioning.\n"]
    pub fn alerts(&self) -> terrars::ListRef<MonitorAlert_AlertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alerts", self.extract_ref()))
    }
}

impl terrars::Referable for MonitorAlert {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MonitorAlert { }

impl terrars::ToListMappable for MonitorAlert {
    type O = terrars::ListRef<MonitorAlertRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MonitorAlert_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_monitor_alert".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMonitorAlert {
    pub tf_id: std::string::String,
    #[doc = "The comparison operator to use for value"]
    pub compare: terrars::PrimField<std::string::String>,
    #[doc = "Description of the alert policy"]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
    #[doc = ""]
    pub window: terrars::PrimField<std::string::String>,
}

impl BuildMonitorAlert {
    pub fn build(self, stack: &mut terrars::Stack) -> MonitorAlert {
        let out = MonitorAlert(std::rc::Rc::new(MonitorAlert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MonitorAlertData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                compare: self.compare,
                description: self.description,
                enabled: core::default::Default::default(),
                entities: core::default::Default::default(),
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                type_: self.type_,
                value: self.value,
                window: self.window,
                alerts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MonitorAlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitorAlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MonitorAlertRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compare` after provisioning.\nThe comparison operator to use for value"]
    pub fn compare(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compare", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the alert policy"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entities` after provisioning.\nThe droplets to apply the alert policy to"]
    pub fn entities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.entities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alerts` after provisioning.\n"]
    pub fn alerts(&self) -> terrars::ListRef<MonitorAlert_AlertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alerts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MonitorAlertDynamic {
    alerts: std::option::Option<terrars::DynamicBlock<MonitorAlert_Alerts>>,
}

#[derive(Serialize)]
pub struct MonitorAlert_Alerts_Slack {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl MonitorAlert_Alerts_Slack { }

impl terrars::ToListMappable for MonitorAlert_Alerts_Slack {
    type O = terrars::BlockAssignable<MonitorAlert_Alerts_Slack>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitorAlert_Alerts_Slack {
    #[doc = "The Slack channel to send alerts to"]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The webhook URL for Slack"]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildMonitorAlert_Alerts_Slack {
    pub fn build(self) -> MonitorAlert_Alerts_Slack {
        MonitorAlert_Alerts_Slack {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct MonitorAlert_Alerts_SlackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitorAlert_Alerts_SlackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitorAlert_Alerts_SlackRef {
        MonitorAlert_Alerts_SlackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitorAlert_Alerts_SlackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send alerts to"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe webhook URL for Slack"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitorAlert_AlertsDynamic {
    slack: std::option::Option<terrars::DynamicBlock<MonitorAlert_Alerts_Slack>>,
}

#[derive(Serialize)]
pub struct MonitorAlert_Alerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std::option::Option<std::vec::Vec<MonitorAlert_Alerts_Slack>>,
    dynamic: MonitorAlert_AlertsDynamic,
}

impl MonitorAlert_Alerts {
    #[doc = "Set the field `email`.\nList of email addresses to sent notifications to"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(mut self, v: impl std::convert::Into<terrars::BlockAssignable<MonitorAlert_Alerts_Slack>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slack = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitorAlert_Alerts {
    type O = terrars::BlockAssignable<MonitorAlert_Alerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitorAlert_Alerts {}

impl BuildMonitorAlert_Alerts {
    pub fn build(self) -> MonitorAlert_Alerts {
        MonitorAlert_Alerts {
            email: core::default::Default::default(),
            slack: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitorAlert_AlertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitorAlert_AlertsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitorAlert_AlertsRef {
        MonitorAlert_AlertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitorAlert_AlertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nList of email addresses to sent notifications to"]
    pub fn email(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> terrars::ListRef<MonitorAlert_Alerts_SlackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }
}
