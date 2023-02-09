use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct MonitorAlertData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    compare: PrimField<String>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<f64>,
    window: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alerts: Option<Vec<MonitorAlertAlertsEl>>,
    dynamic: MonitorAlertDynamic,
}

struct MonitorAlert_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MonitorAlertData>,
}

#[derive(Clone)]
pub struct MonitorAlert(Rc<MonitorAlert_>);

impl MonitorAlert {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `entities`.\nThe droplets to apply the alert policy to"]
    pub fn set_entities(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().entities = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `alerts`.\n"]
    pub fn set_alerts(self, v: impl Into<BlockAssignable<MonitorAlertAlertsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alerts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alerts = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `compare` after provisioning.\nThe comparison operator to use for value"]
    pub fn compare(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compare", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the alert policy"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entities` after provisioning.\nThe droplets to apply the alert policy to"]
    pub fn entities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.entities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alerts` after provisioning.\n"]
    pub fn alerts(&self) -> ListRef<MonitorAlertAlertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alerts", self.extract_ref()))
    }
}

impl Resource for MonitorAlert {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for MonitorAlert {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for MonitorAlert {
    type O = ListRef<MonitorAlertRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for MonitorAlert_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_monitor_alert".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMonitorAlert {
    pub tf_id: String,
    #[doc= "The comparison operator to use for value"]
    pub compare: PrimField<String>,
    #[doc= "Description of the alert policy"]
    pub description: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
    #[doc= ""]
    pub value: PrimField<f64>,
    #[doc= ""]
    pub window: PrimField<String>,
}

impl BuildMonitorAlert {
    pub fn build(self, stack: &mut Stack) -> MonitorAlert {
        let out = MonitorAlert(Rc::new(MonitorAlert_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MonitorAlertData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MonitorAlertRef {
    shared: StackShared,
    base: String,
}

impl Ref for MonitorAlertRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MonitorAlertRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `compare` after provisioning.\nThe comparison operator to use for value"]
    pub fn compare(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compare", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the alert policy"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `entities` after provisioning.\nThe droplets to apply the alert policy to"]
    pub fn entities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.entities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.window", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `alerts` after provisioning.\n"]
    pub fn alerts(&self) -> ListRef<MonitorAlertAlertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alerts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MonitorAlertAlertsElSlackEl {
    channel: PrimField<String>,
    url: PrimField<String>,
}

impl MonitorAlertAlertsElSlackEl { }

impl ToListMappable for MonitorAlertAlertsElSlackEl {
    type O = BlockAssignable<MonitorAlertAlertsElSlackEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitorAlertAlertsElSlackEl {
    #[doc= "The Slack channel to send alerts to"]
    pub channel: PrimField<String>,
    #[doc= "The webhook URL for Slack"]
    pub url: PrimField<String>,
}

impl BuildMonitorAlertAlertsElSlackEl {
    pub fn build(self) -> MonitorAlertAlertsElSlackEl {
        MonitorAlertAlertsElSlackEl {
            channel: self.channel,
            url: self.url,
        }
    }
}

pub struct MonitorAlertAlertsElSlackElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MonitorAlertAlertsElSlackElRef {
    fn new(shared: StackShared, base: String) -> MonitorAlertAlertsElSlackElRef {
        MonitorAlertAlertsElSlackElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitorAlertAlertsElSlackElRef {
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
struct MonitorAlertAlertsElDynamic {
    slack: Option<DynamicBlock<MonitorAlertAlertsElSlackEl>>,
}

#[derive(Serialize)]
pub struct MonitorAlertAlertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slack: Option<Vec<MonitorAlertAlertsElSlackEl>>,
    dynamic: MonitorAlertAlertsElDynamic,
}

impl MonitorAlertAlertsEl {
    #[doc= "Set the field `email`.\nList of email addresses to sent notifications to"]
    pub fn set_email(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `slack`.\n"]
    pub fn set_slack(mut self, v: impl Into<BlockAssignable<MonitorAlertAlertsElSlackEl>>) -> Self {
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

impl ToListMappable for MonitorAlertAlertsEl {
    type O = BlockAssignable<MonitorAlertAlertsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitorAlertAlertsEl {}

impl BuildMonitorAlertAlertsEl {
    pub fn build(self) -> MonitorAlertAlertsEl {
        MonitorAlertAlertsEl {
            email: core::default::Default::default(),
            slack: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct MonitorAlertAlertsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for MonitorAlertAlertsElRef {
    fn new(shared: StackShared, base: String) -> MonitorAlertAlertsElRef {
        MonitorAlertAlertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitorAlertAlertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nList of email addresses to sent notifications to"]
    pub fn email(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `slack` after provisioning.\n"]
    pub fn slack(&self) -> ListRef<MonitorAlertAlertsElSlackElRef> {
        ListRef::new(self.shared().clone(), format!("{}.slack", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitorAlertDynamic {
    alerts: Option<DynamicBlock<MonitorAlertAlertsEl>>,
}
