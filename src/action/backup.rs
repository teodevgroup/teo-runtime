

impl Action {

    pub(crate) fn empty() -> Self {
        Self { value: 0 }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub(crate) fn from_name(name: &str) -> Self {
        Action {
            value: match name {
                "create" => CREATE,
                "update" => UPDATE,
                "delete" => DELETE,
                "find" => FIND,
                "connect" => CONNECT,
                "disconnect" => DISCONNECT,
                "set" => SET,
                "join" => JOIN,
                "first" => FIRST,
                "count" => COUNT,
                "aggregate" => AGGREGATE,
                "groupBy" => GROUP_BY,
                "entry" => ENTRY,
                "nested" => NESTED,
                "internalLocation" => INTERNAL_POSITION,
                "single" => SINGLE,
                "many" => MANY,
                "internalAmount" => INTERNAL_AMOUNT,
                "programCode" => PROGRAM_CODE,
                "identity" => IDENTITY,
                _ => panic!("Unrecognized action option name '{}'.", name)
            }
        }
    }

    pub(crate) const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub(crate) fn to_u32(&self) -> u32 {
        self.value
    }

    pub(crate) fn finalized(&self) -> Self {
        let mut value = self.value;
        if !self.contains_name_bits() {
            value = value | ALL_NAMES;
        }
        if !self.contains_position_bits() {
            value = value | ALL_POSITIONS;
        }
        if !self.contains_amount_bits() {
            value = value | ALL_AMOUNTS;
        }
        Self { value }
    }

    pub(crate) fn redirect(&self, action: Action) -> Self {
        let mut result = self.value;
        let new_names_bits = action.value & ALL_NAMES;
        if new_names_bits != 0 {
            result = (result & !ALL_NAMES) | new_names_bits;
        }
        let new_position_bits = action.value & ALL_POSITIONS;
        if new_position_bits != 0 {
            result = (result & !ALL_POSITIONS) | new_position_bits;
        }
        let new_amount_bits = action.value & ALL_AMOUNTS;
        if new_amount_bits != 0 {
            result = (result & !ALL_AMOUNTS) | new_amount_bits;
        }
        Self { value: result }
    }

    #[inline(always)]
    fn contains_name_bits(&self) -> bool {
        self.value & ALL_NAMES != 0
    }

    #[inline(always)]
    fn contains_position_bits(&self) -> bool {
        self.value & ALL_POSITIONS != 0
    }

    #[inline(always)]
    fn contains_amount_bits(&self) -> bool {
        self.value & ALL_AMOUNTS != 0
    }

    pub(crate) fn neg(&self) -> Self {
        let restore_name_bits = !self.contains_name_bits();
        let restore_entry_nested_bits = !self.contains_position_bits();
        let restore_single_many_bits = !self.contains_amount_bits();
        let mut value = !self.value;
        if restore_name_bits {
            value = value & NOT_ALL_NAMES;
        }
        if restore_entry_nested_bits {
            value = value & NOT_ENTRY_NESTED;
        }
        if restore_single_many_bits {
            value = value & NOT_SINGLE_MANY;
        }
        Self { value }
    }

    pub(crate) fn and(&self, other: Action) -> Self {
        Self { value: self.value & other.value }
    }

    pub(crate) fn or(&self, other: Action) -> Self {
        Self { value: self.value | other.value }
    }

    pub(crate) fn xor(&self, other: Action) -> Self {
        Self { value: self.value ^ other.value }
    }

    pub(crate) fn passes(&self, matchers: &Vec<Action>) -> bool {
        for matcher in matchers {
            let copy = self.finalized();
            let finalized_matcher = matcher.finalized().value;
            let result = finalized_matcher & copy.value;
            return ((result & ALL_NAMES) != 0) &&
                ((result & ALL_POSITIONS) != 0) &&
                ((result & ALL_AMOUNTS) != 0)
        }
        false
    }

    // handler
    pub(crate) fn handler_allowed_input_json_keys(&self) -> &HashSet<&str> {
        match self.value {
            FIND_UNIQUE_HANDLER => &FIND_UNIQUE_INPUT_JSON_KEYS,
            FIND_FIRST_HANDLER => &FIND_FIRST_INPUT_JSON_KEYS,
            FIND_MANY_HANDLER => &FIND_MANY_INPUT_JSON_KEYS,
            CREATE_HANDLER => &CREATE_INPUT_JSON_KEYS,
            UPDATE_HANDLER => &UPDATE_INPUT_JSON_KEYS,
            UPSERT_HANDLER => &UPSERT_INPUT_JSON_KEYS,
            DELETE_HANDLER => &DELETE_INPUT_JSON_KEYS,
            CREATE_MANY_HANDLER => &CREATE_MANY_INPUT_JSON_KEYS,
            UPDATE_MANY_HANDLER => &UPDATE_MANY_INPUT_JSON_KEYS,
            DELETE_MANY_HANDLER => &DELETE_MANY_INPUT_JSON_KEYS,
            COUNT_HANDLER => &COUNT_INPUT_JSON_KEYS,
            AGGREGATE_HANDLER => &AGGREGATE_INPUT_JSON_KEYS,
            GROUP_BY_HANDLER => &GROUP_BY_INPUT_JSON_KEYS,
            SIGN_IN_HANDLER => &SIGN_IN_INPUT_JSON_KEYS,
            IDENTITY_HANDLER => &IDENTITY_INPUT_JSON_KEYS,
            _ => unreachable!()
        }
    }

    pub(crate) fn handler_requires_aggregates(&self) -> bool {
        self.value == GROUP_BY_HANDLER || self.value == AGGREGATE_HANDLER
    }

    pub(crate) fn handler_requires_by_and_having(&self) -> bool {
        self.value == GROUP_BY_HANDLER
    }

    pub(crate) fn handler_requires_credentials(&self) -> bool {
        self.value == SIGN_IN_HANDLER
    }


    pub(crate) fn handler_requires_update(&self) -> bool {
        match self.value {
            UPDATE_HANDLER | UPSERT_HANDLER | UPDATE_MANY_HANDLER => true,
            _ => false,
        }
    }


    pub(crate) fn handler_requires_create(&self) -> bool {
        match self.value {
            CREATE_HANDLER | UPSERT_HANDLER | CREATE_MANY_HANDLER => true,
            _ => false,
        }
    }

    pub(crate) fn handler_requires_where_unique(&self) -> bool {
        match self.value {
            FIND_UNIQUE_HANDLER | UPDATE_HANDLER | UPSERT_HANDLER | DELETE_HANDLER => true,
            _ => false,
        }
    }

    pub(crate) fn handler_requires_where(&self) -> bool {
        match self.value {
            FIND_FIRST_HANDLER | FIND_MANY_HANDLER | UPDATE_MANY_HANDLER | DELETE_MANY_HANDLER | COUNT_HANDLER | AGGREGATE_HANDLER | GROUP_BY_HANDLER => true,
            _ => false,
        }
    }

    pub(crate) fn handler_res_meta(&self) -> ResMeta {
        match self.value {
            FIND_UNIQUE_HANDLER => ResMeta::NoMeta,
            FIND_FIRST_HANDLER => ResMeta::NoMeta,
            FIND_MANY_HANDLER => ResMeta::PagingInfo,
            CREATE_HANDLER => ResMeta::NoMeta,
            UPDATE_HANDLER => ResMeta::NoMeta,
            UPSERT_HANDLER => ResMeta::NoMeta,
            DELETE_HANDLER => ResMeta::NoMeta,
            CREATE_MANY_HANDLER => ResMeta::NoMeta,
            UPDATE_MANY_HANDLER => ResMeta::NoMeta,
            DELETE_MANY_HANDLER => ResMeta::NoMeta,
            COUNT_HANDLER => ResMeta::NoMeta,
            AGGREGATE_HANDLER => ResMeta::NoMeta,
            GROUP_BY_HANDLER => ResMeta::NoMeta,
            SIGN_IN_HANDLER => ResMeta::TokenInfo,
            IDENTITY_HANDLER => ResMeta::NoMeta,
            _ => unreachable!()
        }
    }

    pub(crate) fn handler_res_data(&self) -> ResData {
        match self.value {
            FIND_UNIQUE_HANDLER => ResData::Single,
            FIND_FIRST_HANDLER => ResData::Single,
            FIND_MANY_HANDLER => ResData::Vec,
            CREATE_HANDLER => ResData::Single,
            UPDATE_HANDLER => ResData::Single,
            UPSERT_HANDLER => ResData::Single,
            DELETE_HANDLER => ResData::Single,
            CREATE_MANY_HANDLER => ResData::Vec,
            UPDATE_MANY_HANDLER => ResData::Vec,
            DELETE_MANY_HANDLER => ResData::Vec,
            COUNT_HANDLER => ResData::Number,
            AGGREGATE_HANDLER => ResData::Other,
            GROUP_BY_HANDLER => ResData::Other,
            SIGN_IN_HANDLER => ResData::Single,
            IDENTITY_HANDLER => ResData::Single,
            _ => unreachable!()
        }
    }

    pub(crate) fn as_handler_str(&self) -> &'static str {
        match self.to_u32() {
            FIND_UNIQUE_HANDLER => "findUnique",
            FIND_FIRST_HANDLER => "findFirst",
            FIND_MANY_HANDLER => "findMany",
            CREATE_HANDLER => "create",
            UPDATE_HANDLER => "update",
            UPSERT_HANDLER => "upsert",
            DELETE_HANDLER => "delete",
            CREATE_MANY_HANDLER => "createMany",
            UPDATE_MANY_HANDLER => "updateMany",
            DELETE_MANY_HANDLER => "deleteMany",
            COUNT_HANDLER => "count",
            AGGREGATE_HANDLER => "aggregate",
            GROUP_BY_HANDLER => "groupBy",
            SIGN_IN_HANDLER => "signIn",
            IDENTITY_HANDLER => "identity",
            _ => unreachable!()
        }
    }

    pub(crate) fn handlers_iter() -> Iter<'static, Action> {
        static HANDLER_TYPES: [Action; 15] = [
            Action::from_u32(FIND_UNIQUE_HANDLER),
            Action::from_u32(FIND_FIRST_HANDLER),
            Action::from_u32(FIND_MANY_HANDLER),
            Action::from_u32(CREATE_HANDLER),
            Action::from_u32(UPDATE_HANDLER),
            Action::from_u32(UPSERT_HANDLER),
            Action::from_u32(DELETE_HANDLER),
            Action::from_u32(CREATE_MANY_HANDLER),
            Action::from_u32(UPDATE_MANY_HANDLER),
            Action::from_u32(DELETE_MANY_HANDLER),
            Action::from_u32(COUNT_HANDLER),
            Action::from_u32(AGGREGATE_HANDLER),
            Action::from_u32(GROUP_BY_HANDLER),
            Action::from_u32(SIGN_IN_HANDLER),
            Action::from_u32(IDENTITY_HANDLER),
        ];
        HANDLER_TYPES.iter()
    }

    pub(crate) fn handlers_default() -> HashSet<Action> {
        HashSet::from_iter(vec![
            Action::from_u32(FIND_UNIQUE_HANDLER),
            Action::from_u32(FIND_FIRST_HANDLER),
            Action::from_u32(FIND_MANY_HANDLER),
            Action::from_u32(CREATE_HANDLER),
            Action::from_u32(UPDATE_HANDLER),
            Action::from_u32(UPSERT_HANDLER),
            Action::from_u32(DELETE_HANDLER),
            Action::from_u32(CREATE_MANY_HANDLER),
            Action::from_u32(UPDATE_MANY_HANDLER),
            Action::from_u32(DELETE_MANY_HANDLER),
            Action::from_u32(COUNT_HANDLER),
            Action::from_u32(AGGREGATE_HANDLER),
            Action::from_u32(GROUP_BY_HANDLER),
        ].iter().map(|x| *x))
    }

    pub(crate) fn nested_action_from_name(name: &str) -> Option<Self> {
        Some(Action {
            value: match name {
                "create" => NESTED_CREATE_ACTION,
                "update" => NESTED_UPDATE_ACTION,
                "upsert" => NESTED_UPSERT_ACTION,
                "delete" => NESTED_DELETE_ACTION,
                "connect" => NESTED_CONNECT_ACTION,
                "connectOrCreate" => NESTED_CONNECT_OR_CREATE_ACTION,
                "disconnect" => NESTED_DISCONNECT_ACTION,
                "set" => NESTED_SET_ACTION,
                "createMany" => NESTED_CREATE_ACTION,
                "updateMany" => NESTED_UPDATE_MANY_ACTION,
                "deleteMany" => NESTED_DELETE_MANY_ACTION,
                _ => None?
            }
        })
    }
}

#[derive(PartialEq)]
pub enum ResMeta {
    PagingInfo,
    TokenInfo,
    NoMeta,
    Other,
}

#[derive(PartialEq)]
pub enum ResData {
    Single,
    Vec,
    Other,
    Number,
}
