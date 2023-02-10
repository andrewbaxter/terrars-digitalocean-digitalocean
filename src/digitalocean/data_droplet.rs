use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDropletData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}

struct DataDroplet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDropletData>,
}

#[derive(Clone)]
pub struct DataDroplet(Rc<DataDroplet_>);

impl DataDroplet {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\nid of the Droplet"]
    pub fn set_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nname of the Droplet"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `tag`.\nunique tag of the Droplet"]
    pub fn set_tag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tag = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `backups` after provisioning.\nwhether the Droplet has backups enabled"]
    pub fn backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nthe creation date for the Droplet"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\nthe size of the Droplets disk in gigabytes"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nid of the Droplet"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nthe image id or slug of the Droplet"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\nthe Droplets public ipv4 address"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv4_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\nwhether the Droplet has ipv6 enabled"]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\nthe Droplets public ipv6 address"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv6_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nwhether the Droplet has been locked"]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nmemory of the Droplet in megabytes"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring` after provisioning.\nwhether the Droplet has monitoring enabled"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the Droplet"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_hourly` after provisioning.\nthe Droplets hourly price"]
    pub fn price_hourly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_monthly` after provisioning.\nthe Droplets monthly price"]
    pub fn price_monthly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_networking` after provisioning.\nwhether the Droplet has private networking enabled"]
    pub fn private_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nthe region that the Droplet instance is deployed in"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nthe current size of the Droplet"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nstate of the Droplet instance"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nunique tag of the Droplet"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the Droplet"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vcpus` after provisioning.\nthe number of virtual cpus"]
    pub fn vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_ids` after provisioning.\nlist of volumes attached to the Droplet"]
    pub fn volume_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the Droplet is located"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl Referable for DataDroplet {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDroplet { }

impl ToListMappable for DataDroplet {
    type O = ListRef<DataDropletRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDroplet_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_droplet".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDroplet {
    pub tf_id: String,
}

impl BuildDataDroplet {
    pub fn build(self, stack: &mut Stack) -> DataDroplet {
        let out = DataDroplet(Rc::new(DataDroplet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDropletData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                tag: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDropletRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDropletRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDropletRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `backups` after provisioning.\nwhether the Droplet has backups enabled"]
    pub fn backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\nthe creation date for the Droplet"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\nthe size of the Droplets disk in gigabytes"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nid of the Droplet"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nthe image id or slug of the Droplet"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\nthe Droplets public ipv4 address"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv4_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv4_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\nwhether the Droplet has ipv6 enabled"]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\nthe Droplets public ipv6 address"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv6_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nwhether the Droplet has been locked"]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\nmemory of the Droplet in megabytes"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `monitoring` after provisioning.\nwhether the Droplet has monitoring enabled"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the Droplet"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_hourly` after provisioning.\nthe Droplets hourly price"]
    pub fn price_hourly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_monthly` after provisioning.\nthe Droplets monthly price"]
    pub fn price_monthly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_networking` after provisioning.\nwhether the Droplet has private networking enabled"]
    pub fn private_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nthe region that the Droplet instance is deployed in"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nthe current size of the Droplet"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nstate of the Droplet instance"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\nunique tag of the Droplet"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the Droplet"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vcpus` after provisioning.\nthe number of virtual cpus"]
    pub fn vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_ids` after provisioning.\nlist of volumes attached to the Droplet"]
    pub fn volume_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the Droplet is located"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}
