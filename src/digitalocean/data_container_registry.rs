use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataContainerRegistryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataContainerRegistry_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataContainerRegistryData>,
}

#[derive(Clone)]
pub struct DataContainerRegistry(Rc<DataContainerRegistry_>);

impl DataContainerRegistry {
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

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the container registry"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_url` after provisioning.\n"]
    pub fn server_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_usage_bytes` after provisioning.\n"]
    pub fn storage_usage_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_usage_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription_tier_slug` after provisioning.\n"]
    pub fn subscription_tier_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription_tier_slug", self.extract_ref()))
    }
}

impl Datasource for DataContainerRegistry {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataContainerRegistry {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataContainerRegistry {
    type O = ListRef<DataContainerRegistryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataContainerRegistry_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_container_registry".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataContainerRegistry {
    pub tf_id: String,
    #[doc= "name of the container registry"]
    pub name: PrimField<String>,
}

impl BuildDataContainerRegistry {
    pub fn build(self, stack: &mut Stack) -> DataContainerRegistry {
        let out = DataContainerRegistry(Rc::new(DataContainerRegistry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataContainerRegistryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataContainerRegistryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataContainerRegistryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataContainerRegistryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the container registry"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_url` after provisioning.\n"]
    pub fn server_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_usage_bytes` after provisioning.\n"]
    pub fn storage_usage_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_usage_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription_tier_slug` after provisioning.\n"]
    pub fn subscription_tier_slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription_tier_slug", self.extract_ref()))
    }
}
