use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct UptimeAlertData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    check_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comparison: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notifications: Option<Vec<UptimeAlertNotificationsEl>>,
    dynamic: UptimeAlertDynamic,
}

struct UptimeAlert_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<UptimeAlertData>,
}

#[derive(Clone)]
pub struct UptimeAlert(Rc<UptimeAlert_>);

impl UptimeAlert {
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

    #[doc= "Set the field `comparison`.\nThe comparison operator used against the alert's threshold. Enum: 'greater_than' 'less_than"]
    pub fn set_comparison(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comparison = Some(v.into());
        self
    }

    #[doc= "Set the field `period`.\nPeriod of time the threshold must be exceeded to trigger the alert. Enum '2m' '3m' '5m' '10m' '15m' '30m' '1h'"]
    pub fn set_period(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().period = Some(v.into());
        self
    }

    #[doc= "Set the field `threshold`.\nThe threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type."]
    pub fn set_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `notifications`.\n"]
    pub fn set_notifications(self, v: impl Into<BlockAssignable<UptimeAlertNotificationsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notifications = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notifications = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `check_id` after provisioning.\nA unique identifier for a check."]
    pub fn check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comparison` after provisioning.\nThe comparison operator used against the alert's threshold. Enum: 'greater_than' 'less_than"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA human-friendly display name for the alert."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\nPeriod of time the threshold must be exceeded to trigger the alert. Enum '2m' '3m' '5m' '10m' '15m' '30m' '1h'"]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `threshold` after provisioning.\nThe threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of health check to perform. Enum: 'latency' 'down' 'down_global' 'ssl_expiry'"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> ListRef<UptimeAlertNotificationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notifications", self.extract_ref()))
    }
}

impl Referable for UptimeAlert {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for UptimeAlert { }

impl ToListMappable for UptimeAlert {
    type O = ListRef<UptimeAlertRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for UptimeAlert_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_uptime_alert".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildUptimeAlert {
    pub tf_id: String,
    #[doc= "A unique identifier for a check."]
    pub check_id: PrimField<String>,
    #[doc= "A human-friendly display name for the alert."]
    pub name: PrimField<String>,
    #[doc= "The type of health check to perform. Enum: 'latency' 'down' 'down_global' 'ssl_expiry'"]
    pub type_: PrimField<String>,
}

impl BuildUptimeAlert {
    pub fn build(self, stack: &mut Stack) -> UptimeAlert {
        let out = UptimeAlert(Rc::new(UptimeAlert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(UptimeAlertData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                check_id: self.check_id,
                comparison: core::default::Default::default(),
                name: self.name,
                period: core::default::Default::default(),
                threshold: core::default::Default::default(),
                type_: self.type_,
                notifications: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct UptimeAlertRef {
    shared: StackShared,
    base: String,
}

impl Ref for UptimeAlertRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl UptimeAlertRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_id` after provisioning.\nA unique identifier for a check."]
    pub fn check_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `comparison` after provisioning.\nThe comparison operator used against the alert's threshold. Enum: 'greater_than' 'less_than"]
    pub fn comparison(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA human-friendly display name for the alert."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\nPeriod of time the threshold must be exceeded to trigger the alert. Enum '2m' '3m' '5m' '10m' '15m' '30m' '1h'"]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `threshold` after provisioning.\nThe threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type."]
    pub fn threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of health check to perform. Enum: 'latency' 'down' 'down_global' 'ssl_expiry'"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notifications` after provisioning.\n"]
    pub fn notifications(&self) -> ListRef<UptimeAlertNotificationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.notifications", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct UptimeAlertNotificationsElSlackEl {
    channel: PrimField<String>,
    url: PrimField<String>,
}

impl UptimeAlertNotificationsElSlackEl { }

impl ToListMappable for UptimeAlertNotificationsElSlackEl {
    type O = BlockAssignable<UptimeAlertNotificationsElSlackEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUptimeAlertNotificationsElSlackEl {
    #[doc= "The Slack channel to send alerts to"]
    pub channel: PrimField<String>,
    #[doc= "The webhook URL for Slack"]
    pub url: PrimField<String>,
}

impl BuildUptimeAlertNotificationsElSlackEl {
    pub fn build(self) -> UptimeAlertNotificationsElSlackEl {
        UptimeAlertNotificationsElSlackEl {
            channel: self.channel,
            url: self.url,
        }
    }
}

pub struct UptimeAlertNotificationsElSlackElRef {
    shared: StackShared,
    base: String,
}

impl Ref for UptimeAlertNotificationsElSlackElRef {
    fn new(shared: StackShared, base: String) -> UptimeAlertNotificationsElSlackElRef {
        UptimeAlertNotificationsElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UptimeAlertNotificationsElSlackElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `channel` after provisioning.\nThe Slack channel to send alerts to"]
    pub fn channel(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe webhook URL for Slack"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct UptimeAlertNotificationsElDynamic {
    slack: Option<DynamicBlock<UptimeAlertNotificationsElSlackEl>>,
}

#[derive(Serialize)]
pub struct UptimeAlertNotificationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack: Option<Vec<UptimeAlertNotificationsElSlackEl>>,
    dynamic: UptimeAlertNotificationsElDynamic,
}

impl UptimeAlertNotificationsEl {
    #[doc= "Set the field `email`.\nList of email addresses to sent notifications to"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `slack`.\n"]
    pub fn set_slack(mut self, v: impl Into<BlockAssignable<UptimeAlertNotificationsElSlackEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.slack = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.slack = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for UptimeAlertNotificationsEl {
    type O = BlockAssignable<UptimeAlertNotificationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildUptimeAlertNotificationsEl {}

impl BuildUptimeAlertNotificationsEl {
    pub fn build(self) -> UptimeAlertNotificationsEl {
        UptimeAlertNotificationsEl {
            email: core::default::Default::default(),
            slack: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct UptimeAlertNotificationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for UptimeAlertNotificationsElRef {
    fn new(shared: StackShared, base: String) -> UptimeAlertNotificationsElRef {
        UptimeAlertNotificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl UptimeAlertNotificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nList of email addresses to sent notifications to"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> ListRef<UptimeAlertNotificationsElSlackElRef> {
        ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }
}

#[derive(Serialize, Default)]
struct UptimeAlertDynamic {
    notifications: Option<DynamicBlock<UptimeAlertNotificationsEl>>,
}
