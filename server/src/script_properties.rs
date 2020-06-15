use crate::type_checker::{match_datatypes, match_keyword, DataTypes, Keywords};
use serde::Deserialize;

const SCRIPTPS: &str = include_str!("reference/scriptproperties.xml");
const EVENTS: &str = include_str!("reference/events.ron");
const METHODS: &str = include_str!("reference/methods.ron");

#[derive(Debug, Default)]
pub struct ScriptProperties<'a> {
    pub datatypes: Vec<DataTypeProperties<'a>>,
    pub keywords: Vec<KeywordProperties<'a>>,
}

impl<'a> ScriptProperties<'a> {
    pub fn parse(doc: &'a roxmltree::Document) -> ScriptProperties<'a> {
        let mut flag = Flag::Datatype;
        let mut sp = ScriptProperties::default();
        for node in doc.descendants() {
            match node.tag_name().name() {
                "datatype" => {
                    flag = Flag::Datatype;
                    let mut dp = DataTypeProperties::default();
                    if let Some(attr) = node.attribute("name") {
                        dp.datatype = match_datatypes(attr)
                    }
                    if let Some(parent_type) = node.attribute("type") {
                        dp.parent_type = Some(match_datatypes(parent_type))
                    }
                    sp.datatypes.push(dp);
                }
                "keyword" => {
                    flag = Flag::Keyword;
                    let mut kp = KeywordProperties::default();
                    if let Some(attr) = node.attribute("name") {
                        kp.keyword = match_keyword(attr)
                    }
                    if let Some(desc) = node.attribute("description") {
                        kp.description = desc
                    }
                    sp.keywords.push(kp);
                }
                "property" if flag == Flag::Datatype => {
                    if let Some(dp) = sp.datatypes.last_mut() {
                        let mut prop = DProperty {
                            datatype: dp.datatype.clone(),
                            prop_name: node.attribute("name"),
                            prop_result: node.attribute("result"),
                            prop_type: None,
                        };
                        if let Some(attr) = node.attribute("type") {
                            prop.prop_type = Some(match_datatypes(attr))
                        }
                        dp.properties.push(prop)
                    }
                }
                "property" if flag == Flag::Keyword => {
                    if let Some(kp) = sp.keywords.last_mut() {
                        let mut prop = KProperty {
                            keyword: kp.keyword.clone(),
                            prop_name: node.attribute("name"),
                            prop_result: node.attribute("result"),
                            prop_type: None,
                        };
                        if let Some(attr) = node.attribute("type") {
                            prop.prop_type = Some(match_datatypes(attr))
                        }
                        kp.properties.push(prop)
                    }
                }
                _ => {}
            }
        }
        sp
    }
}

#[derive(Debug, PartialEq)]
enum Flag {
    Keyword,
    Datatype,
}

#[derive(Debug, Default, Clone)]
pub struct DataTypeProperties<'a> {
    pub datatype: DataTypes,
    pub parent_type: Option<DataTypes>,
    pub properties: Vec<DProperty<'a>>,
}

#[derive(Debug, Default, Clone)]
pub struct DProperty<'a> {
    pub datatype: DataTypes,
    pub prop_name: Option<&'a str>,
    pub prop_result: Option<&'a str>,
    pub prop_type: Option<DataTypes>,
}
#[derive(Debug, Default, Clone)]
pub struct KProperty<'a> {
    pub keyword: Keywords,
    pub prop_name: Option<&'a str>,
    pub prop_result: Option<&'a str>,
    pub prop_type: Option<DataTypes>,
}
#[derive(Debug, Default, Clone)]
pub struct KeywordProperties<'a> {
    pub keyword: Keywords,
    pub description: &'a str,
    pub properties: Vec<KProperty<'a>>,
}

