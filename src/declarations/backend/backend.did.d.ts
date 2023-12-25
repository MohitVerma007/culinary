import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CulinaryAdventure {
  'id' : bigint,
  'dish_name' : string,
  'visit_date' : bigint,
  'notes' : string,
  'rating' : number,
  'restaurant_name' : string,
}
export interface CulinaryAdventurePayload {
  'dish_name' : string,
  'visit_date' : bigint,
  'notes' : string,
  'rating' : number,
  'restaurant_name' : string,
}
export type Error = { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : CulinaryAdventure } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_culinary_adventure' : ActorMethod<
    [CulinaryAdventurePayload],
    [] | [CulinaryAdventure]
  >,
  'delete_culinary_adventure' : ActorMethod<[bigint], Result>,
  'get_all_culinary_adventures' : ActorMethod<[], Array<CulinaryAdventure>>,
  'get_culinary_adventure' : ActorMethod<[bigint], Result>,
  'get_culinary_adventures_before_date' : ActorMethod<
    [bigint],
    Array<CulinaryAdventure>
  >,
  'get_culinary_adventures_by_restaurant' : ActorMethod<
    [string],
    Array<CulinaryAdventure>
  >,
  'get_culinary_adventures_count_before_date' : ActorMethod<[bigint], bigint>,
  'get_top_rated_culinary_adventures' : ActorMethod<
    [bigint],
    Array<CulinaryAdventure>
  >,
  'get_total_culinary_adventures' : ActorMethod<[], bigint>,
  'search_culinary_adventures_by_notes' : ActorMethod<
    [string],
    Array<CulinaryAdventure>
  >,
  'search_culinary_adventures_by_partial_dish_name' : ActorMethod<
    [string],
    Array<CulinaryAdventure>
  >,
  'update_culinary_adventure' : ActorMethod<
    [bigint, CulinaryAdventurePayload],
    Result
  >,
  'update_visit_date' : ActorMethod<[bigint, bigint], Result>,
}
