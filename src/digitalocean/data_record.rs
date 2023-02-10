use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataRecordData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataRecord_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRecordData>,
}

#[derive(Clone)]
pub struct DataRecord(Rc<DataRecord_>);

impl DataRecord {
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

    #[doc= "Get a reference to the value of field `data` after provisioning.\nname record data"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\ndomain of the name record"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flags` after provisioning.\nflags of the name record"]
    pub fn flags(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the record"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nport of the name record"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\npriority of the name record"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\ntag of the name record"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nttl of the name record"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\ntype of the name record"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nweight of the name record"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }
}

impl Datasource for DataRecord {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataRecord {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataRecord {
    type O = ListRef<DataRecordRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataRecord_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_record".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRecord {
    pub tf_id: String,
    #[doc= "domain of the name record"]
    pub domain: PrimField<String>,
    #[doc= "name of the record"]
    pub name: PrimField<String>,
}

impl BuildDataRecord {
    pub fn build(self, stack: &mut Stack) -> DataRecord {
        let out = DataRecord(Rc::new(DataRecord_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRecordData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                domain: self.domain,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRecordRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRecordRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRecordRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\nname record data"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\ndomain of the name record"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flags` after provisioning.\nflags of the name record"]
    pub fn flags(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the record"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\nport of the name record"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\npriority of the name record"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tag` after provisioning.\ntag of the name record"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nttl of the name record"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\ntype of the name record"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nweight of the name record"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }
}
