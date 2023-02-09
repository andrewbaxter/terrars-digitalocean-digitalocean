use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDropletsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataDropletsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataDropletsSortEl>>,
    dynamic: DataDropletsDynamic,
}

struct DataDroplets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDropletsData>,
}

#[derive(Clone)]
pub struct DataDroplets(Rc<DataDroplets_>);

impl DataDroplets {
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

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataDropletsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sort`.\n"]
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataDropletsSortEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `droplets` after provisioning.\n"]
    pub fn droplets(&self) -> ListRef<DataDropletsDropletsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.droplets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataDropletsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Datasource for DataDroplets {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataDroplets {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataDroplets {
    type O = ListRef<DataDropletsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDroplets_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_droplets".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDroplets {
    pub tf_id: String,
}

impl BuildDataDroplets {
    pub fn build(self, stack: &mut Stack) -> DataDroplets {
        let out = DataDroplets(Rc::new(DataDroplets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDropletsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDropletsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDropletsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDropletsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `droplets` after provisioning.\n"]
    pub fn droplets(&self) -> ListRef<DataDropletsDropletsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.droplets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataDropletsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDropletsDropletsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_address_private: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address_private: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_hourly: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_monthly: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_networking: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    urn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_uuid: Option<PrimField<String>>,
}

impl DataDropletsDropletsEl {
    #[doc= "Set the field `backups`.\n"]
    pub fn set_backups(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.backups = Some(v.into());
        self
    }

    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv4_address`.\n"]
    pub fn set_ipv4_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_address = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv4_address_private`.\n"]
    pub fn set_ipv4_address_private(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv4_address_private = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_address_private`.\n"]
    pub fn set_ipv6_address_private(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address_private = Some(v.into());
        self
    }

    #[doc= "Set the field `locked`.\n"]
    pub fn set_locked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.locked = Some(v.into());
        self
    }

    #[doc= "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }

    #[doc= "Set the field `monitoring`.\n"]
    pub fn set_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `price_hourly`.\n"]
    pub fn set_price_hourly(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.price_hourly = Some(v.into());
        self
    }

    #[doc= "Set the field `price_monthly`.\n"]
    pub fn set_price_monthly(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.price_monthly = Some(v.into());
        self
    }

    #[doc= "Set the field `private_networking`.\n"]
    pub fn set_private_networking(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.private_networking = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc= "Set the field `urn`.\n"]
    pub fn set_urn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.urn = Some(v.into());
        self
    }

    #[doc= "Set the field `vcpus`.\n"]
    pub fn set_vcpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vcpus = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_ids`.\n"]
    pub fn set_volume_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.volume_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_uuid = Some(v.into());
        self
    }
}

impl ToListMappable for DataDropletsDropletsEl {
    type O = BlockAssignable<DataDropletsDropletsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDropletsDropletsEl {}

impl BuildDataDropletsDropletsEl {
    pub fn build(self) -> DataDropletsDropletsEl {
        DataDropletsDropletsEl {
            backups: core::default::Default::default(),
            created_at: core::default::Default::default(),
            disk: core::default::Default::default(),
            id: core::default::Default::default(),
            image: core::default::Default::default(),
            ipv4_address: core::default::Default::default(),
            ipv4_address_private: core::default::Default::default(),
            ipv6: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            ipv6_address_private: core::default::Default::default(),
            locked: core::default::Default::default(),
            memory: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            name: core::default::Default::default(),
            price_hourly: core::default::Default::default(),
            price_monthly: core::default::Default::default(),
            private_networking: core::default::Default::default(),
            region: core::default::Default::default(),
            size: core::default::Default::default(),
            status: core::default::Default::default(),
            tags: core::default::Default::default(),
            urn: core::default::Default::default(),
            vcpus: core::default::Default::default(),
            volume_ids: core::default::Default::default(),
            vpc_uuid: core::default::Default::default(),
        }
    }
}

pub struct DataDropletsDropletsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDropletsDropletsElRef {
    fn new(shared: StackShared, base: String) -> DataDropletsDropletsElRef {
        DataDropletsDropletsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDropletsDropletsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backups` after provisioning.\n"]
    pub fn backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backups", self.base))
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv4_address_private` after provisioning.\n"]
    pub fn ipv4_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_address_private` after provisioning.\n"]
    pub fn ipv6_address_private(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_private", self.base))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.base))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc= "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.base))
    }

    #[doc= "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.base))
    }

    #[doc= "Get a reference to the value of field `private_networking` after provisioning.\n"]
    pub fn private_networking(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.base))
    }

    #[doc= "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_ids` after provisioning.\n"]
    pub fn volume_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDropletsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataDropletsFilterEl {
    #[doc= "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }

    #[doc= "Set the field `match_by`.\n"]
    pub fn set_match_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_by = Some(v.into());
        self
    }
}

impl ToListMappable for DataDropletsFilterEl {
    type O = BlockAssignable<DataDropletsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDropletsFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataDropletsFilterEl {
    pub fn build(self) -> DataDropletsFilterEl {
        DataDropletsFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataDropletsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDropletsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataDropletsFilterElRef {
        DataDropletsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDropletsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_by` after provisioning.\n"]
    pub fn match_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_by", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDropletsSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataDropletsSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataDropletsSortEl {
    type O = BlockAssignable<DataDropletsSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDropletsSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataDropletsSortEl {
    pub fn build(self) -> DataDropletsSortEl {
        DataDropletsSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataDropletsSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDropletsSortElRef {
    fn new(shared: StackShared, base: String) -> DataDropletsSortElRef {
        DataDropletsSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDropletsSortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataDropletsDynamic {
    filter: Option<DynamicBlock<DataDropletsFilterEl>>,
    sort: Option<DynamicBlock<DataDropletsSortEl>>,
}
