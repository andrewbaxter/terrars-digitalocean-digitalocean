use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDropletData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDroplet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDropletData>,
}

#[derive(Clone)]
pub struct DataDroplet(std::rc::Rc<DataDroplet_>);

impl DataDroplet {
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

    #[doc = "Set the field `id`.\nid of the Droplet"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nname of the Droplet"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nunique tag of the Droplet"]
    pub fn set_tag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backups` after provisioning.\nwhether the Droplet has backups enabled"]
    pub fn backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nthe creation date for the Droplet"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk` after provisioning.\nthe size of the Droplets disk in gigabytes"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nid of the Droplet"]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nthe image id or slug of the Droplet"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_address` after provisioning.\nthe Droplets public ipv4 address"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv4_address_private(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\nwhether the Droplet has ipv6 enabled"]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address` after provisioning.\nthe Droplets public ipv6 address"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv6_address_private(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_private", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locked` after provisioning.\nwhether the Droplet has been locked"]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\nmemory of the Droplet in megabytes"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring` after provisioning.\nwhether the Droplet has monitoring enabled"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the Droplet"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_hourly` after provisioning.\nthe Droplets hourly price"]
    pub fn price_hourly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_monthly` after provisioning.\nthe Droplets monthly price"]
    pub fn price_monthly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_networking` after provisioning.\nwhether the Droplet has private networking enabled"]
    pub fn private_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nthe region that the Droplet instance is deployed in"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nthe current size of the Droplet"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nstate of the Droplet instance"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\nunique tag of the Droplet"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the Droplet"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vcpus` after provisioning.\nthe number of virtual cpus"]
    pub fn vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `volume_ids` after provisioning.\nlist of volumes attached to the Droplet"]
    pub fn volume_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the Droplet is located"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataDroplet {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDroplet { }

impl terrars::ToListMappable for DataDroplet {
    type O = terrars::ListRef<DataDropletRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDroplet_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_droplet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDroplet {
    pub tf_id: std::string::String,
}

impl BuildDataDroplet {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDroplet {
        let out = DataDroplet(std::rc::Rc::new(DataDroplet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDropletData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDropletRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDropletRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `backups` after provisioning.\nwhether the Droplet has backups enabled"]
    pub fn backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nthe creation date for the Droplet"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk` after provisioning.\nthe size of the Droplets disk in gigabytes"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nid of the Droplet"]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nthe image id or slug of the Droplet"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_address` after provisioning.\nthe Droplets public ipv4 address"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv4_address_private(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\nwhether the Droplet has ipv6 enabled"]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address` after provisioning.\nthe Droplets public ipv6 address"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address_private` after provisioning.\nthe Droplets private ipv4 address"]
    pub fn ipv6_address_private(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_private", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locked` after provisioning.\nwhether the Droplet has been locked"]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\nmemory of the Droplet in megabytes"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring` after provisioning.\nwhether the Droplet has monitoring enabled"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the Droplet"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_hourly` after provisioning.\nthe Droplets hourly price"]
    pub fn price_hourly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_monthly` after provisioning.\nthe Droplets monthly price"]
    pub fn price_monthly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_networking` after provisioning.\nwhether the Droplet has private networking enabled"]
    pub fn private_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nthe region that the Droplet instance is deployed in"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nthe current size of the Droplet"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nstate of the Droplet instance"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\nunique tag of the Droplet"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the Droplet"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vcpus` after provisioning.\nthe number of virtual cpus"]
    pub fn vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `volume_ids` after provisioning.\nlist of volumes attached to the Droplet"]
    pub fn volume_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the Droplet is located"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}
