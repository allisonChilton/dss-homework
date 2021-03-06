extern crate futures;
mod home;
mod dynset;

use futures::future::{join_all};
use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::{Read};
use home::HomeData;
use home::Container;
use dynset::Dynset;
use std::collections::{HashMap, HashSet};
use image::{RgbImage};
use std::fmt::Display;
use serde::export::Formatter;

#[derive(Debug)]
struct ConvertError(String);
impl fmt::Display for ConvertError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Conversion Error: {}", self.0)
    }
}
impl Error for ConvertError{}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum SetStyle{
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

#[derive(Copy, Clone)]
pub enum SetType{
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

const EMSG: &str = "JSON Parsing error";

#[allow(dead_code)]
pub struct Title{
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) release_date: String,
    pub(crate) rating: String,
    pub(crate) image_url: String

}

impl Display for Title {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}\nRelease Date: {}\nRating: {}", self.name, self.release_date, self.rating)
    }
}


pub struct TitleContainer{
    pub set_id: String,
    pub name: String,
    pub set_type: SetType,
    pub style: SetStyle,
    pub items: Vec<Title>
}


fn get_dynamic_set_ref(set_ref: &str) -> Result<Vec<Title>, Box<dyn Error>>{
    let url = format!("https://cd-static.bamgrid.com/dp-117731241344/sets/{}.json", set_ref);
    let resp = reqwest::blocking::get(url.as_str())?;
    let json_str = resp.text().unwrap();
    let mut itemv: Vec<Title> = Vec::new();
    let ds: Dynset = serde_json::from_str(json_str.as_str()).unwrap();
    if ds.data.trending_set.is_some(){
        for i in ds.data.trending_set.unwrap().items{
            let t = Title{
                id: i.content_id,
                name: match i.text.title.full.series{
                    Some(e) => e.default.content,
                    None => i.text.title.full.program.ok_or(EMSG)?.default.content
                },
                release_date: match i.releases.get(0){
                    Some(e) => e.release_date.clone(),
                    None => "".to_string()
                },
                rating: match i.ratings.get(0){
                    Some(e) => e.value.clone(),
                    None => "".to_string()

                },
                image_url: match i.image.tile.n178.program{
                    Some(e) => e.default.url,
                    None => match i.image.tile.n178.series{
                        Some(e) => e.default.url,
                        None => return Err(Box::new(ConvertError("Could not determine title".to_string())))
                    }
                }
            };
            itemv.push(t);
        }
    }else if ds.data.personalized_curated_set.is_some() {
        let set = ds.data.personalized_curated_set.unwrap();
        for i in set.items{
            let t = Title{
                id: i.content_id,
                name: match i.text.title.full.series{
                    Some(e) => e.default.content,
                    None => i.text.title.full.program.ok_or(EMSG)?.default.content
                },
                release_date: match i.releases.get(0){
                    Some(e) => e.release_date.clone(),
                    None => "".to_string()
                },
                rating: match i.ratings.get(0){
                    Some(e) => e.value.clone(),
                    None => "".to_string()

                },
                image_url: match i.image.tile.n178.program{
                    Some(e) => e.default.url,
                    None => match i.image.tile.n178.series{
                        Some(e) => e.default.url,
                        None => return Err(Box::new(ConvertError("Could not determine title".to_string())))
                    }
                }
            };
            itemv.push(t);
        }
    }else if ds.data.curated_set.is_some(){
        for i in ds.data.curated_set.unwrap().items {
            let t = Title {
                id: i.content_id,
                name: match i.text.title.full.series {
                    Some(e) => e.default.content,
                    None => i.text.title.full.program.ok_or(EMSG)?.default.content
                },
                release_date: match i.releases.get(0) {
                    Some(e) => e.release_date.clone().unwrap_or_else(||"".to_string()),
                    None => "".to_string()
                },
                rating: match i.ratings.get(0) {
                    Some(e) => e.value.clone(),
                    None => "".to_string()
                },
                image_url: match i.image.tile.n178.program {
                    Some(e) => e.default.url,
                    None => match i.image.tile.n178.series {
                        Some(e) => e.default.url,
                        None => return Err(Box::new(ConvertError("Could not determine title".to_string())))
                    }
                }
            };
            itemv.push(t);
        }

    }else{
        return Err(Box::new(ConvertError("Could not determine items for dynset".to_string())))
    }

    Ok(itemv)
}

fn get_curated_items(con: Container) -> Result<Vec<Title>, Box<dyn Error>>{
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

    Ok(itemv)
}

pub fn get_image_cache(tc: &[TitleContainer])-> HashMap<String, RgbImage>{
    let mut urls: HashSet<String> = HashSet::new();
    let mut hm = HashMap::new();
    for t in tc{
        for i in &t.items{
            urls.insert(i.image_url.clone());
        }
    }
    let cl = reqwest::Client::new();
    let mut futs = Vec::new();
    for url in urls{
        let r = cl.get(&url);
        let fut = cl.execute(r.build().unwrap());
        futs.push(fut);
    }
    // let retdata = futures::stream::collect
    let af = async {
        let results = join_all(futs).await;
        for r in results{
            let resp = r.unwrap();
            let key = resp.url().to_string().clone();
            let imgdata = resp.bytes().await.unwrap();
            let img = match image::load_from_memory(&imgdata){
                Ok(e) => e.to_rgb8(),
                Err(_) => continue
            };
            hm.insert(key, img);
        }
    };
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(af);

    hm
}

// with more time I'd make this a less generic error
pub fn load_home_data()  -> Result<Vec<TitleContainer>, Box<dyn Error>> {
    let json_str = if true {
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
        let setid = match stype{
            SetType::SetRef => con.set.ref_id.clone().expect("Couldn't unpack set type"),
            SetType::CuratedSet => con.set.set_id.clone().expect("Couldn't unpack set type"),
        };

        let name = con.set.text.title.full.set.default.content.clone();
        let style = SetStyle::from_str(con.style.as_str())?;

        let itemv = match stype{
            SetType::SetRef => {
                get_dynamic_set_ref(&setid)
            },
            SetType::CuratedSet => {
                get_curated_items(con)
            }
        }?;


        let tc = TitleContainer{
            set_id: setid,
            set_type: stype,
            style,
            name,
            items: itemv
        };

        t_containers.push(tc);
    }

    Ok(t_containers)
}
