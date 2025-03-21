use cosmwasm_schema::write_api;

use ai_bouncer_caller::msg::{ExecuteMsg, QueryMsg};
use cosmwasm_std::Empty;

fn main() {
    write_api! {
        instantiate: Empty,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
