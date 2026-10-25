#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct UptimeAlertData_ {
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
    check_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparison: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notifications: std::option::Option<std::vec::Vec<UptimeAlert_Notifications>>,
    dynamic: UptimeAlertDynamic,
}

struct UptimeAlert_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<UptimeAlertData_>,
}

#[derive(Clone)]
pub struct UptimeAlert(std::rc::Rc<UptimeAlert_>);

impl UptimeAlert {
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
        "Set the field `comparison`.\nThe comparison operator used against the alert's threshold. Enum: 'greater_than' 'less_than"]
    pub fn set_comparison(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comparison = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period`.\nPeriod of time the threshold must be exceeded to trigger the alert. Enum '2m' '3m' '5m' '10m' '15m' '30m' '1h'"]
    pub fn set_period(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `threshold`.\nThe threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type."]
    pub fn set_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notifications`.\n"]
    pub fn set_notifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<UptimeAlert_Notifications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `check_id` after provisioning.\nA unique identifier for a check."]
    pub fn check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comparison` after provisioning.\nThe comparison operator used against the alert's threshold. Enum: 'greater_than' 'less_than"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-friendly display name for the alert."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nPeriod of time the threshold must be exceeded to trigger the alert. Enum '2m' '3m' '5m' '10m' '15m' '30m' '1h'"]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of health check to perform. Enum: 'latency' 'down' 'down_global' 'ssl_expiry'"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> terrars::ListRef<UptimeAlert_NotificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notifications", self.extract_ref()))
    }
}

impl terrars::Referable for UptimeAlert {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for UptimeAlert { }

impl terrars::ToListMappable for UptimeAlert {
    type O = terrars::ListRef<UptimeAlertRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for UptimeAlert_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_uptime_alert".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildUptimeAlert {
    pub tf_id: std::string::String,
    #[doc = "A unique identifier for a check."]
    pub check_id: terrars::PrimField<std::string::String>,
    #[doc = "A human-friendly display name for the alert."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The type of health check to perform. Enum: 'latency' 'down' 'down_global' 'ssl_expiry'"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildUptimeAlert {
    pub fn build(self, stack: &mut terrars::Stack) -> UptimeAlert {
        let out = UptimeAlert(std::rc::Rc::new(UptimeAlert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(UptimeAlertData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                check_id: self.check_id,
                comparison: core::default::Default::default(),
                name: self.name,
                period: core::default::Default::default(),
                threshold: core::default::Default::default(),
                type_: self.type_,
                notifications: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct UptimeAlertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UptimeAlertRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl UptimeAlertRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_id` after provisioning.\nA unique identifier for a check."]
    pub fn check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `comparison` after provisioning.\nThe comparison operator used against the alert's threshold. Enum: 'greater_than' 'less_than"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-friendly display name for the alert."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nPeriod of time the threshold must be exceeded to trigger the alert. Enum '2m' '3m' '5m' '10m' '15m' '30m' '1h'"]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type."]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of health check to perform. Enum: 'latency' 'down' 'down_global' 'ssl_expiry'"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> terrars::ListRef<UptimeAlert_NotificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notifications", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct UptimeAlertDynamic {
    notifications: std::option::Option<terrars::DynamicBlock<UptimeAlert_Notifications>>,
}

#[derive(Serialize)]
pub struct UptimeAlert_Notifications_Slack {
    channel: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl UptimeAlert_Notifications_Slack { }

impl terrars::ToListMappable for UptimeAlert_Notifications_Slack {
    type O = terrars::BlockAssignable<UptimeAlert_Notifications_Slack>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUptimeAlert_Notifications_Slack {
    #[doc = "The Slack channel to send alerts to"]
    pub channel: terrars::PrimField<std::string::String>,
    #[doc = "The webhook URL for Slack"]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildUptimeAlert_Notifications_Slack {
    pub fn build(self) -> UptimeAlert_Notifications_Slack {
        UptimeAlert_Notifications_Slack {
            channel: self.channel,
            url: self.url,
        }
    }
}

#[allow(dead_code)]
pub struct UptimeAlert_Notifications_SlackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UptimeAlert_Notifications_SlackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> UptimeAlert_Notifications_SlackRef {
        UptimeAlert_Notifications_SlackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UptimeAlert_Notifications_SlackRef {
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
struct UptimeAlert_NotificationsDynamic {
    slack: std::option::Option<terrars::DynamicBlock<UptimeAlert_Notifications_Slack>>,
}

#[derive(Serialize)]
pub struct UptimeAlert_Notifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slack: std::option::Option<std::vec::Vec<UptimeAlert_Notifications_Slack>>,
    dynamic: UptimeAlert_NotificationsDynamic,
}

impl UptimeAlert_Notifications {
    #[doc = "Set the field `email`.\nList of email addresses to sent notifications to"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slack`.\n"]
    pub fn set_slack(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<UptimeAlert_Notifications_Slack>>,
    ) -> Self {
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

impl terrars::ToListMappable for UptimeAlert_Notifications {
    type O = terrars::BlockAssignable<UptimeAlert_Notifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUptimeAlert_Notifications {}

impl BuildUptimeAlert_Notifications {
    pub fn build(self) -> UptimeAlert_Notifications {
        UptimeAlert_Notifications {
            email: core::default::Default::default(),
            slack: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct UptimeAlert_NotificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for UptimeAlert_NotificationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> UptimeAlert_NotificationsRef {
        UptimeAlert_NotificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UptimeAlert_NotificationsRef {
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
    pub fn slack(&self) -> terrars::ListRef<UptimeAlert_Notifications_SlackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }
}
