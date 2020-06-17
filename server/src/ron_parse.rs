use crate::type_checker::DataTypes;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Event<'a> {
    pub id: &'a str,
    pub description: &'a str,
    pub object: Option<Vec<DataTypes>>,
    pub param: Option<Vec<DataTypes>>,
    pub param1: Option<Vec<DataTypes>>,
    pub param2: Option<Vec<DataTypes>>,
    pub param3: Option<Vec<DataTypes>>,
}
// #[derive(Debug, Default, Clone)]
// pub struct EventList<'a> {
//     pub events: Vec<Event<'a>>,
// }
// impl<'a> Deserialize<'a> for EventList<'a>{
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'a> {
//         todo!()
//     }
    
// }

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Method<'a> {
    pub id: &'a str,
    pub description: &'a str,
    pub output: Vec<Output<'a>>,
}
impl<'a> Method<'a> {
    pub fn search_output(&self, node: &'a str, attr: &'a str, mult: Option<&str>) -> Option<Multiple> {
        match self.id {
            node => {
                for out in self.output.iter() {
                    match out.attr {
                        attr=> {
                            match mult {
                                Some(m) => {}
                                None => {match out.single.clone() {
                                    Some(single) => return Some(Multiple::Single(single)),
                                    None => {
                                        match out.multiple.clone() {
                                            Some(m) if m == DataTypes::List => {
                                                match out.inside.clone() {
                                                    Some(i) => return Some(Multiple::List(i)),
                                                    None => return Some(Multiple::List(vec![DataTypes::Unknown]))
                                                }
                                            }
                                            Some(m) if m == DataTypes::Group => {
                                                match out.inside.clone() {
                                                    Some(i) => return Some(Multiple::Group(i)),
                                                    None => return Some(Multiple::Group(vec![DataTypes::Unknown]))
                                                }
                                            }
                                            Some(_) => return None,
                                            None => return None
                                        }
                                    }
                                }}
                            }
                        }
                        _ => return None
                    }
                }
                None
            }
            _ => return None
        }
    }
}

// #[derive(Debug, Default, Clone, Deserialize)]
// pub struct MethodList<'a> {
//     pub methods: Vec<Method<'a>>,
// }
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Output<'a> {
    pub attr: &'a str,
    pub multiple: Option<DataTypes>,
    pub inside: Option<Vec<DataTypes>>,
    pub single: Option<Vec<DataTypes>>,
}

pub fn parse_method_ron(string: &str) -> Vec<Method> {
    let methods = match ron::from_str(string) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    methods
}

pub fn parse_event_ron(string: &str) -> Vec<Event> {
    let events = match ron::from_str(string) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    events
}

// #[derive(Debug, Default, Clone, Deserialize)]
// pub struct MethodsParsed<'a> {
//     pub methods: Vec<Method<'a>>,
// }
// #[derive(Debug, Default, Clone, Deserialize)]
// pub struct MethodParsed<'a> {
//     pub name: &'a str,
//     pub description: &'a str,
//     pub values: (
//         Option<VarType<'a>>,
//         Option<VarType<'a>>,
//         Option<VarType<'a>>,
//         Option<VarType<'a>>,
//         Option<VarType<'a>>,
//     ),
// }
// #[derive(Debug, Default, Clone, Deserialize)]
// pub struct VarType<'a> {
//     pub multiple: Multiple,
//     pub attr: &'a str,
//     pub datatype: DataTypes,
// }
#[derive(Debug, Clone, Deserialize)]
pub enum Multiple {
    List(Vec<DataTypes>),
    Group(Vec<DataTypes>),
    Single(Vec<DataTypes>),
}
// impl Default for Multiple {
//     fn default() -> Self {
//         Multiple::Single
//     }
// }
