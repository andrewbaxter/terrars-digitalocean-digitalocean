use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DropletData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    droplet_agent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graceful_shutdown: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resize_disk: Option<PrimField<bool>>,
    size: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssh_keys: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_uuid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DropletTimeoutsEl>,
}

struct Droplet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DropletData>,
}

#[derive(Clone)]
pub struct Droplet(Rc<Droplet_>);

impl Droplet {
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

    #[doc= "Set the field `backups`.\n"]
    pub fn set_backups(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().backups = Some(v.into());
        self
    }

    #[doc= "Set the field `droplet_agent`.\n"]
    pub fn set_droplet_agent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().droplet_agent = Some(v.into());
        self
    }

    #[doc= "Set the field `graceful_shutdown`.\n"]
    pub fn set_graceful_shutdown(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().graceful_shutdown = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6`.\n"]
    pub fn set_ipv6(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `monitoring`.\n"]
    pub fn set_monitoring(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `private_networking`.\n"]
    pub fn set_private_networking(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_networking = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `resize_disk`.\n"]
    pub fn set_resize_disk(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().resize_disk = Some(v.into());
        self
    }

    #[doc= "Set the field `ssh_keys`.\n"]
    pub fn set_ssh_keys(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ssh_keys = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `user_data`.\n"]
    pub fn set_user_data(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_data = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_ids`.\n"]
    pub fn set_volume_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().volume_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DropletTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `backups` after provisioning.\n"]
    pub fn backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_agent` after provisioning.\n"]
    pub fn droplet_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_agent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `graceful_shutdown` after provisioning.\n"]
    pub fn graceful_shutdown(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.graceful_shutdown", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address_private` after provisioning.\n"]
    pub fn ipv4_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_networking` after provisioning.\n"]
    pub fn private_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resize_disk` after provisioning.\n"]
    pub fn resize_disk(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resize_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_ids` after provisioning.\n"]
    pub fn volume_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DropletTimeoutsElRef {
        DropletTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for Droplet {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Droplet {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Droplet {
    type O = ListRef<DropletRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Droplet_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_droplet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDroplet {
    pub tf_id: String,
    #[doc= ""]
    pub image: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub size: PrimField<String>,
}

impl BuildDroplet {
    pub fn build(self, stack: &mut Stack) -> Droplet {
        let out = Droplet(Rc::new(Droplet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DropletData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backups: core::default::Default::default(),
                droplet_agent: core::default::Default::default(),
                graceful_shutdown: core::default::Default::default(),
                id: core::default::Default::default(),
                image: self.image,
                ipv6: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                name: self.name,
                private_networking: core::default::Default::default(),
                region: core::default::Default::default(),
                resize_disk: core::default::Default::default(),
                size: self.size,
                ssh_keys: core::default::Default::default(),
                tags: core::default::Default::default(),
                user_data: core::default::Default::default(),
                volume_ids: core::default::Default::default(),
                vpc_uuid: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DropletRef {
    shared: StackShared,
    base: String,
}

impl Ref for DropletRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DropletRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backups` after provisioning.\n"]
    pub fn backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_agent` after provisioning.\n"]
    pub fn droplet_agent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_agent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `graceful_shutdown` after provisioning.\n"]
    pub fn graceful_shutdown(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.graceful_shutdown", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address_private` after provisioning.\n"]
    pub fn ipv4_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_networking` after provisioning.\n"]
    pub fn private_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resize_disk` after provisioning.\n"]
    pub fn resize_disk(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.resize_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_ids` after provisioning.\n"]
    pub fn volume_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DropletTimeoutsElRef {
        DropletTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DropletTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DropletTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for DropletTimeoutsEl {
    type O = BlockAssignable<DropletTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDropletTimeoutsEl {}

impl BuildDropletTimeoutsEl {
    pub fn build(self) -> DropletTimeoutsEl {
        DropletTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DropletTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DropletTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DropletTimeoutsElRef {
        DropletTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DropletTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
