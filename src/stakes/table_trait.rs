use crate::common::components::*;
use crate::common::functions::*;
use crate::common::models::*;

use super::{functions::*, graphql::stakes_query};
use leptos::*;
use stakes_query::StakesQueryStakes;

impl TableData for Vec<Option<StakesQueryStakes>> {
    fn get_columns(&self) -> Vec<String> {
        ["Kye", "Stake", "Delegate", "Delegators", "Ledger Hash"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn get_rows(&self) -> Vec<Vec<HtmlElement<html::AnyElement>>> {
        self.iter()
            .map(|opt_stake| match opt_stake {
                Some(stake) => vec![
                    convert_to_span(get_public_key(stake)),
                    convert_to_pill(get_balance(stake), PillVariant::Green),
                    convert_to_link(
                        get_delegate(stake),
                        format!("/accounts/{}", get_delegate(stake)),
                    ),
                    convert_to_pill(get_delegators_count(stake), PillVariant::Blue),
                    convert_to_span(get_ledger_hash(stake)),
                ],
                None => vec![],
            })
            .collect::<Vec<_>>()
    }
}
