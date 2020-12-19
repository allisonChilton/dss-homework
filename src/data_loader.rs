mod home;

use serde_json::{Value, Map};
use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::Read;
use serde::Deserialize;
use home::HomeData;
use crate::data_loader::home::Program23;

#[derive(Debug)]
struct ConvertError(String);
impl fmt::Display for ConvertError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Conversion Error: {}", self.0)
    }
}
impl Error for ConvertError{}

enum SetStyle{
    BecauseYouSet,
    TrendingSet,
    PersonalizedCuratedSet,
    editorial
}

impl SetStyle{
    fn from_str(input: &str) -> Result<SetStyle, Box<dyn Error>>{
        match input.to_lowercase().as_str(){
            "becauseyouset" => Ok(SetStyle::BecauseYouSet),
            "trendingset" => Ok(SetStyle::TrendingSet),
            "personalizedcuratedset" => Ok(SetStyle::PersonalizedCuratedSet),
            "editorial" => Ok(SetStyle::editorial),
            _ => Err(Box::new(ConvertError(format!("SetStyle can't format {}", input))))
        }

    }
}

enum SetType{
    SetRef,
    CuratedSet
}


impl SetType{
    fn from_str(input: &str) -> Result<SetType, Box<dyn Error>>{
        match input.to_lowercase().as_str(){
            "setref" => Ok(SetType::SetRef),
            "curatedset" => Ok(SetType::CuratedSet),
            _ => Err(Box::new(ConvertError(format!("SetType can't format {}", input))))
        }
    }
}

const emsg: &str = "JSON Parsing error";

struct Title{
    id: String,
    name: String,
    release_date: String,
    rating: String,
    image_url: String

}

impl Title{
    fn from_json(node: &Value) -> Result<Title, Box<dyn Error>>{
        let tid = node.get("contentId").ok_or(emsg)?.as_str().ok_or(emsg)?.to_string();

        let t = Title{
            id: tid,
            name: json_iter_find(node,"content")?.as_str().ok_or(emsg)?.to_string(),
            release_date: match json_iter_find(node, "releaseDate"){
                Ok(e) => e.as_str().or(Some("")),
                Err(_) => Some("")
            }.unwrap().to_string(),
            rating: node.get("ratings").ok_or(emsg)?.as_array().ok_or(emsg)?[0]
                .get("system").ok_or(emsg)?.as_str().ok_or(emsg)?.to_string(),
            image_url:  json_iter_find(json_drill_down(
                node, vec!["image","tile","1.78"])?, "url")?
                .as_str().ok_or(emsg)?.to_string()
        };
        Ok(t)
    }
}

struct TitleContainer{
    set_id: String,
    name: String,
    set_type: SetType,
    style: SetStyle,
    items: Vec<Title>
}

fn json_iter_find<'a>(val: &'a Value, search_key: &str) -> Result<&'a Value, Box<dyn Error>>{

    if val.is_object(){
        for (key, v) in val.as_object().ok_or(emsg)?.iter() {
            if key.eq(search_key) {
                return Ok(v);
            } else {
                let mut val_vec: Vec<&Value> = Vec::new();
                if v.is_array() {
                    val_vec.extend(v.as_array().ok_or(emsg)?);
                } else {
                    val_vec.push(v);
                }
                for vv in val_vec {
                    match json_iter_find(vv, search_key) {
                        Ok(res) => return Ok(res),
                        Err(_) => {}
                    }
                }
            }
        }
    }

    Err(Box::new(ConvertError(format!("Cannot find search key {} in {}", search_key, val.as_str().ok_or(emsg)?))))
}

fn json_drill_down<'a>(val: &'a Value, keys: Vec<&str>) -> Result<&'a Value, Box<dyn Error>>{
    let mut node = val;
    for key in keys{
        node = node.get(key).ok_or(format!("Key error: {}", key))?;
    }
    Ok(node)
}


// with more time I'd make this a less generic error
pub fn load_home_data()  -> Result<f64, Box<dyn Error>> {
    let json_str = if false {
        let resp = reqwest::blocking::get("https://cd-static.bamgrid.com/dp-117731241344/home.json")?;
        resp.text().unwrap()
    }else{
        let mut f = File::open("home.json")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        s
    };

    let hd: HomeData = serde_json::from_str(json_str.as_str()).unwrap();
    println!("{}", hd.data.standard_collection.containers[0].set.items[0].content_id.as_ref().unwrap());
    let mut t_containers = Vec::new();

    for con in hd.data.standard_collection.containers{

        let stype = SetType::from_str(con.set.type_field.as_str())?;
        let mut itemv = Vec::new();

        for i in con.set.items{
            let tname = match i.text.title.full.program{
                Some(e) => e.default.content,
                None => match i.text.title.full.collection{
                    Some(e) => e.default.content,
                    None => match i.text.title.full.series{
                        Some(e) => e.default.content,
                        None => return Err(Box::new(ConvertError("Could not determine title".to_string())))
                    }
                }
            };
            let tid = match i.content_id{
                Some(e) => e,
                None => match i.collection_id{
                    Some(e) => e,
                    None => match i.program_id{
                        Some(e) => e,
                        None => return Err(Box::new(ConvertError("Could not determine id".to_string())))
                    }
                }
            };

            let t = Title{
                id: tid,
                name: tname,
                release_date: match i.releases.get(0) {
                    None => "".to_string(),
                    Some(e) => e.release_date.as_ref().unwrap_or(&"".to_string()).to_string()
                },
                rating: match i.ratings.get(0){
                    None => "".to_string(),
                    Some(e) => e.value.clone()
                },
                image_url: match i.image.tile.n178.default{
                    Some(e) => e.default.url,
                    None => match i.image.tile.n178.series{
                        Some(e) => e.default.url,
                        None => match i.image.tile.n178.program{
                            Some(e) => e.default.url,
                            None => return Err(Box::new(ConvertError("Could not determine title".to_string())))
                        }
                    }
                }
            };

            itemv.push(t);
        }

        let tc = TitleContainer{
            set_id: match stype{
                SetType::SetRef => con.set.ref_id.expect("Couldn't unpack set type"),
                SetType::CuratedSet => con.set.set_id.expect("Couldn't unpack set type"),
            },
            set_type: stype,
            style: SetStyle::from_str(con.style.as_str())?,
            name: con.set.text.title.full.set.default.content,
            items: itemv
        };

        t_containers.push(tc);
    }

    /*
    let data: Value = serde_json::from_str(json_str.as_str())?;
    let container_json = json_drill_down(&data, vec!["data", "StandardCollection", "containers"])?.as_array().ok_or(emsg)?;
    let mut t_containers = Vec::new();
    for con in container_json{
        println!("======\n{}", con);
        let set = con.get("set").ok_or(emsg)?;
        let stype = SetType::from_str(set.get("type").ok_or(emsg)?.as_str().ok_or(emsg)?)?;
        let mut itemv = Vec::new();
        for i in set.get("items").ok_or(emsg)?.as_array().ok_or(emsg)?{
            match Title::from_json(i){
                Ok(e) => itemv.push(e),
                Err(e) => return Err(Box::new(ConvertError(format!("Error parsing {}, {}", i, e))))
            }
        }
        let container = TitleContainer{
            set_id: match stype{
                SetType::SetRef => set.get("refId").ok_or(emsg)?.as_str().ok_or(emsg)?.to_string(),
                SetType::CuratedSet => set.get("setId").ok_or(emsg)?.as_str().ok_or(emsg)?.to_string(),
            },
            set_type: stype,
            style: SetStyle::from_str(con.get("style").ok_or(emsg)?.as_str().ok_or(emsg)?)?,
            name: json_drill_down(set, vec!["text","title","full","set","default","content"])?.as_str().ok_or(emsg)?.to_string(),
            items: itemv
        };// ok_or(emsg)?.as_array().ok_or(emsg)?.
        t_containers.push(container);
    }

     */
    Ok(1.)
}
