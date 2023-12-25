export const idlFactory = ({ IDL }) => {
  const CulinaryAdventurePayload = IDL.Record({
    'dish_name' : IDL.Text,
    'visit_date' : IDL.Nat64,
    'notes' : IDL.Text,
    'rating' : IDL.Float64,
    'restaurant_name' : IDL.Text,
  });
  const CulinaryAdventure = IDL.Record({
    'id' : IDL.Nat64,
    'dish_name' : IDL.Text,
    'visit_date' : IDL.Nat64,
    'notes' : IDL.Text,
    'rating' : IDL.Float64,
    'restaurant_name' : IDL.Text,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : CulinaryAdventure, 'Err' : Error });
  return IDL.Service({
    'add_culinary_adventure' : IDL.Func(
        [CulinaryAdventurePayload],
        [IDL.Opt(CulinaryAdventure)],
        [],
      ),
    'delete_culinary_adventure' : IDL.Func([IDL.Nat64], [Result], []),
    'get_all_culinary_adventures' : IDL.Func(
        [],
        [IDL.Vec(CulinaryAdventure)],
        ['query'],
      ),
    'get_culinary_adventure' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_culinary_adventures_before_date' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(CulinaryAdventure)],
        ['query'],
      ),
    'get_culinary_adventures_by_restaurant' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(CulinaryAdventure)],
        ['query'],
      ),
    'get_culinary_adventures_count_before_date' : IDL.Func(
        [IDL.Nat64],
        [IDL.Nat64],
        ['query'],
      ),
    'get_top_rated_culinary_adventures' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(CulinaryAdventure)],
        ['query'],
      ),
    'get_total_culinary_adventures' : IDL.Func([], [IDL.Nat64], ['query']),
    'search_culinary_adventures_by_notes' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(CulinaryAdventure)],
        ['query'],
      ),
    'search_culinary_adventures_by_partial_dish_name' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(CulinaryAdventure)],
        ['query'],
      ),
    'update_culinary_adventure' : IDL.Func(
        [IDL.Nat64, CulinaryAdventurePayload],
        [Result],
        [],
      ),
    'update_visit_date' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
