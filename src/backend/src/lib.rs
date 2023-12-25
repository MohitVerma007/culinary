#[macro_use]
extern crate serde;
use candid::{Decode, Encode};

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CulinaryAdventure {
    id: u64,
    restaurant_name: String,
    dish_name: String,
    rating: f64,
    notes: String,
    visit_date: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct CulinaryAdventurePayload {
    restaurant_name: String,
    dish_name: String,
    rating: f64,
    notes: String,
    visit_date: u64,
}

impl Storable for CulinaryAdventure {
    // Conversion to bytes
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    // Conversion from bytes
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for CulinaryAdventure {
    const MAX_SIZE: u32 = 2048; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

// Existing thread-local variables
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static CULINARY_ADVENTURE_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for culinary adventures")
    );

    static CULINARY_ADVENTURE_STORAGE: RefCell<StableBTreeMap<u64, CulinaryAdventure, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for CulinaryAdventure
fn do_insert_culinary_adventure(item: &CulinaryAdventure) {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service.borrow_mut().insert(item.id, item.clone());
    });
}

#[ic_cdk::query]
fn get_culinary_adventure(id: u64) -> Result<CulinaryAdventure, Error> {
    match _get_culinary_adventure(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("culinary adventure with id={} not found", id),
        }),
    }
}

fn _get_culinary_adventure(id: &u64) -> Option<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|s| s.borrow().get(id))
}

#[ic_cdk::update]
fn add_culinary_adventure(item: CulinaryAdventurePayload) -> Option<CulinaryAdventure> {
    let id = CULINARY_ADVENTURE_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for culinary adventures");
    let culinary_adventure = CulinaryAdventure {
        id,
        restaurant_name: item.restaurant_name,
        dish_name: item.dish_name,
        rating: item.rating,
        notes: item.notes,
        visit_date: item.visit_date,
    };
    do_insert_culinary_adventure(&culinary_adventure);
    Some(culinary_adventure)
}

#[ic_cdk::update]
fn update_culinary_adventure(id: u64, item: CulinaryAdventurePayload) -> Result<CulinaryAdventure, Error> {
    match CULINARY_ADVENTURE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut culinary_adventure) => {
            culinary_adventure.restaurant_name = item.restaurant_name;
            culinary_adventure.dish_name = item.dish_name;
            culinary_adventure.rating = item.rating;
            culinary_adventure.notes = item.notes;
            culinary_adventure.visit_date = item.visit_date;
            do_insert_culinary_adventure(&culinary_adventure);
            Ok(culinary_adventure)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update culinary adventure with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn delete_culinary_adventure(id: u64) -> Result<CulinaryAdventure, Error> {
    match CULINARY_ADVENTURE_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(culinary_adventure) => Ok(culinary_adventure),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete culinary adventure with id={}. item not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

#[ic_cdk::query]
fn get_culinary_adventures_before_date(visit_date: u64) -> Vec<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, adventure)| adventure.visit_date <= visit_date)
            .map(|(_, adventure)| adventure.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_visit_date(id: u64, new_visit_date: u64) -> Result<CulinaryAdventure, Error> {
    match CULINARY_ADVENTURE_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut culinary_adventure) => {
            culinary_adventure.visit_date = new_visit_date;
            do_insert_culinary_adventure(&culinary_adventure);
            Ok(culinary_adventure)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update visit date for culinary adventure with id={}. item not found",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_all_culinary_adventures() -> Vec<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_total_culinary_adventures() -> u64 {
    CULINARY_ADVENTURE_STORAGE.with(|service| service.borrow().len())
}

#[ic_cdk::query]
fn get_culinary_adventures_count_before_date(visit_date: u64) -> usize {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, adventure)| adventure.visit_date <= visit_date)
            .count()
    })
}

#[ic_cdk::query]
fn search_culinary_adventures_by_notes(keyword: String) -> Vec<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, adventure)| adventure.notes.contains(&keyword))
            .map(|(_, adventure)| adventure.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_culinary_adventures_by_restaurant(restaurant_name: String) -> Vec<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, adventure)| adventure.restaurant_name == restaurant_name)
            .map(|(_, adventure)| adventure.clone())
            .collect()
    })
}

#[ic_cdk::query]
fn get_top_rated_culinary_adventures(top_n: usize) -> Vec<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        let mut adventures: Vec<CulinaryAdventure> = service
            .borrow()
            .iter()
            .map(|(_, adventure)| adventure.clone())
            .collect();

        adventures.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
        adventures.truncate(top_n);

        adventures
    })
}

#[ic_cdk::query]
fn search_culinary_adventures_by_partial_dish_name(partial_name: String) -> Vec<CulinaryAdventure> {
    CULINARY_ADVENTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .filter(|(_, adventure)| adventure.dish_name.contains(&partial_name))
            .map(|(_, adventure)| adventure.clone())
            .collect()
    })
}

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
