use super::{functions::*, graphql::blocks_query::BlocksQueryBlocks, models::*};
use crate::{
    blocks::graphql::blocks_query::BlocksQueryBlocksTransactionsFeeTransfer,
    common::{components::*, constants::*, functions::*, models::*, spotlight::*, table::*},
    icons::*,
};
use charming::{
    component::{Legend, Title},
    series::*,
    Chart, WasmRenderer,
};
use gloo_timers::future::TimeoutFuture;
use leptos::*;
use leptos_router::*;
use leptos_use::{use_interval, UseIntervalReturn};
use std::collections::HashMap;
#[component]
pub fn BlockTabContainer(content: BlockContent) -> impl IntoView {
    let option_block = use_context::<ReadSignal<Option<BlocksQueryBlocks>>>()
        .expect("there to be an optional block signal provided");

    let content_for_fallback = content.clone();

    let (placeholder_metadata, _) = create_signal(Some(TableMetadata::default()));

    view! {
        <PageContainer>
            <ErrorBoundary fallback=move |_| ().into_view()>
                <Suspense fallback=move || {
                    let content_clone = content_for_fallback.clone();
                    match content_clone {
                        BlockContent::Spotlight => view! { <BlockSpotlightPlaceholder/> },
                        BlockContent::UserCommands => {
                            view! {
                                <TableSection
                                    metadata=placeholder_metadata
                                    section_heading="User Commands"
                                    controls=|| ().into_view()
                                >
                                    <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::SNARKJobs => {
                            view! {
                                <TableSection
                                    metadata=placeholder_metadata
                                    section_heading="SNARK Jobs"
                                    controls=|| ().into_view()
                                >
                                    <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::FeeTransfers => {
                            view! {
                                <TableSection
                                    metadata=placeholder_metadata
                                    section_heading="Internal Commands"
                                    controls=|| ().into_view()
                                >
                                    <DeprecatedTable data=DeprecatedLoadingPlaceholder {}/>
                                </TableSection>
                            }
                        }
                        BlockContent::Analytics => {
                            view! {
                                <TableSection
                                    metadata=placeholder_metadata
                                    section_heading="Analytics"
                                    controls=|| ().into_view()
                                >
                                    <span></span>
                                </TableSection>
                            }
                        }
                    }
                }>

                    {
                        let content_clone = content.clone();
                        move || {
                            match (option_block.get(), content_clone.clone()) {
                                (Some(block), BlockContent::Spotlight) => {
                                    view! { <BlockSpotlight block=block/> }
                                }
                                (Some(block), BlockContent::UserCommands) => {
                                    view! { <BlockUserCommands block=block/> }
                                }
                                (Some(block), BlockContent::SNARKJobs) => {
                                    view! { <BlockSnarkJobs block=block/> }
                                }
                                (Some(block), BlockContent::FeeTransfers) => {
                                    view! { <BlockInternalCommands block=block/> }
                                }
                                (Some(block), BlockContent::Analytics) => {
                                    view! { <BlockAnalytics block=block/> }
                                }
                                _ => ().into_view(),
                            }
                        }
                    }

                </Suspense>
            </ErrorBoundary>
        </PageContainer>
    }
}

#[component]
pub fn BlockUserCommands(block: BlocksQueryBlocks) -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata {
        total_records: "all".to_string(),
        displayed_records: get_user_commands(&block)
            .map(|uc| uc.len() as i64)
            .unwrap_or(0),
    }));

    view! {
        <TableSection metadata section_heading="User Commands" controls=|| ().into_view()>

            {move || match get_user_commands(&block) {
                Some(user_commands) => {
                    view! { <DeprecatedTable data=user_commands/> }
                }
                None => ().into_view(),
            }}

        </TableSection>
    }
}

#[component]
pub fn BlockSnarkJobs(block: BlocksQueryBlocks) -> impl IntoView {
    let (metadata, _) = create_signal(Some(TableMetadata {
        total_records: "all".to_string(),
        displayed_records: get_snark_job_count(&block)
            .map(|sj| sj as i64)
            .unwrap_or_default(),
    }));
    view! {
        <TableSection metadata section_heading="SNARK Jobs" controls=|| ().into_view()>
            <BlockSpotlightSnarkJobTable block=block/>
        </TableSection>
    }
}

#[component]
pub fn BlockInternalCommands(block: BlocksQueryBlocks) -> impl IntoView {
    let block_clone = block.clone();
    let (metadata, _) = create_signal(Some(TableMetadata {
        total_records: "all".to_string(),
        displayed_records: match (
            block_clone
                .transactions
                .clone()
                .and_then(|txn| txn.fee_transfer),
            block_clone
                .transactions
                .clone()
                .and_then(|txn| txn.coinbase_receiver_account.and_then(|ra| ra.public_key)),
        ) {
            (Some(feetransfers), Some(_)) => (feetransfers.len() + 1) as i64,
            (Some(feetransfers), None) => feetransfers.len() as i64,
            (_, _) => 0_i64,
        },
    }));
    view! {
        <TableSection metadata section_heading="Internal Commands" controls=|| ().into_view()>
            <BlockInternalCommandsTable block=block.clone()/>
        </TableSection>
    }
}

#[component]
pub fn BlockInternalCommandsTable(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        {move || match (
            block.transactions.clone().and_then(|txn| txn.fee_transfer),
            block.transactions.clone().and_then(|txn| txn.coinbase),
            block
                .transactions
                .clone()
                .and_then(|txn| txn.coinbase_receiver_account.and_then(|ra| ra.public_key)),
        ) {
            (Some(mut feetransfers), Some(coinbase), Some(coinbase_receiver)) => {
                feetransfers
                    .push(
                        Some(BlocksQueryBlocksTransactionsFeeTransfer {
                            fee: Some(coinbase),
                            type_: Some("Coinbase".to_string()),
                            recipient: Some(coinbase_receiver),
                        }),
                    );
                view! { <DeprecatedTable data=feetransfers/> }
            }
            (_, _, _) => {
                view! { <EmptyTable message="No internal commands for this block"/> }
            }
        }}
    }
}

#[component]
pub fn BlockAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (block_sig, _) = create_signal(block);

    let user_command_amount_total = move || {
        if let Some(user_commands) = get_user_commands(&block_sig.get()) {
            user_commands
                .iter()
                .filter_map(|transaction_option| {
                    transaction_option
                        .as_ref()
                        .map(|transaction| transaction.amount.unwrap_or(0.0))
                        .map(|f| f.round() as u64)
                })
                .sum()
        } else {
            0
        }
    };
    let (metadata, _) = create_signal(Some(TableMetadata {
        displayed_records: user_command_amount_total() as i64,
        total_records: "all".to_string(),
    }));

    view! {
        <TableSection metadata section_heading="Analytics" controls=|| ().into_view()>
            <AnalyticsLayout>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total User Amounts Transferred".into())
                        value=decorate_with_mina_tag(nanomina_to_mina(user_command_amount_total()))

                        variant=ColorVariant::Transparent
                    />

                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total Internal Fees Transferred".into())
                        value=decorate_with_mina_tag(get_transaction_fees(&block_sig.get()))

                        variant=ColorVariant::Transparent
                    />
                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <AnalyticsSimpleInfo
                        label=convert_to_span("Total SNARK Fees".into())
                        value=wrap_in_pill(
                            decorate_with_mina_tag(get_snark_fees(&block_sig.get())),
                            ColorVariant::Blue,
                        )

                        variant=ColorVariant::Blue
                    />
                </AnalyticsSmContainer>
                <AnalyticsSmContainer>
                    <span></span>
                </AnalyticsSmContainer>
                <AnalyticsLgContainer>
                    <BlockSpotlightFeeTransferAnalytics block=block_sig.get()/>
                </AnalyticsLgContainer>
                <AnalyticsLgContainer>
                    <BlockSpotlightUserCommandAnalytics block=block_sig.get()/>
                </AnalyticsLgContainer>
            </AnalyticsLayout>
        </TableSection>
    }
}

#[component]
pub fn BlockSpotlightFeeTransferAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (block_sig, _) = create_signal(block);
    let (data, set_data) = create_signal(HashMap::new());

    create_effect(move |_| {
        if let Some(transactions) = block_sig.get().transactions.as_ref() {
            if let Some(fee_transfer) = transactions.fee_transfer.as_ref() {
                let pie_hashmap = fee_transfer
                    .iter()
                    .filter_map(|row| {
                        let r = row.as_ref()?;
                        let (Some(fee), Some(recipient)) = (r.fee.as_ref(), r.recipient.as_ref())
                        else {
                            return None;
                        };
                        let parsed_fee = str::parse::<i32>(fee).unwrap_or(0);
                        let sixth_to_last = recipient.len() - 6;
                        let recip = [
                            recipient[..6].to_string(),
                            recipient[sixth_to_last..].to_string(),
                        ];
                        Some((recip.join("..."), parsed_fee))
                    })
                    .fold(HashMap::new(), |mut acc, (recipient, fee)| {
                        *acc.entry(recipient).or_insert(0) += fee;
                        acc
                    });
                set_data.set(pie_hashmap);
            }
        }
    });

    create_effect(move |_| {
        if !data.get().is_empty() {
            setup_and_render_chart(&data.get(), "chart", "Top Internal Transfers");
        }
    });

    view! { <div id="chart" class="p-4 md:p-8"></div> }
}

#[component]
pub fn BlockSpotlightUserCommandAnalytics(block: BlocksQueryBlocks) -> impl IntoView {
    let (data, set_data) = create_signal(HashMap::new());
    create_effect(move |_| {
        if let Some(transactions) = block.transactions.as_ref() {
            if let Some(user_commands) = transactions.user_commands.as_ref() {
                let pie_hashmap = user_commands
                    .iter()
                    .filter_map(|row| {
                        let r = row.as_ref()?;
                        let (Some(amount), Some(recipient)) = (r.amount, r.to.as_ref()) else {
                            return None;
                        };
                        let sixth_to_last = recipient.len() - 6;
                        let recip = [
                            recipient[..6].to_string(),
                            recipient[sixth_to_last..].to_string(),
                        ];
                        Some((recip.join("..."), amount as i64))
                    })
                    .fold(HashMap::new(), |mut acc, (recipient, amount)| {
                        *acc.entry(recipient).or_insert(0) += amount;
                        acc
                    });
                set_data.set(pie_hashmap);
            }
        }
    });

    create_effect(move |_| {
        if !data.get().is_empty() {
            setup_and_render_chart(&data.get(), "chart2", "Top Payments");
        }
    });

    view! { <div id="chart2" class="p-4 md:p-8"></div> }
}

fn setup_and_render_chart<T>(data: &HashMap<String, T>, chart_id: &str, chart_title: &str)
where
    T: Into<i64> + Copy + 'static,
{
    let d = data.clone();
    let ch_id = chart_id.to_string();
    let ch_tl = chart_title.to_string();

    let action = create_action(move |_: &()| {
        let d_cloned = d.clone();
        let ch_id_cloned = ch_id.clone();
        let ch_tl_cloned = ch_tl.clone();

        async move { render_pie_chart(&d_cloned, &ch_id_cloned, &ch_tl_cloned).await }
    });

    action.dispatch(());
}

// Asynchronous function to render the chart
async fn render_pie_chart<T>(data: &HashMap<String, T>, chart_id: &str, chart_title: &str)
where
    T: Into<i64> + Copy,
{
    let mut sorted_data = data
        .iter()
        .map(|(key, &val)| (Into::<i64>::into(val), key))
        .collect::<Vec<_>>();
    sorted_data.sort_by(|a, b| b.0.cmp(&a.0));

    let size = sorted_data.len();
    let (top_items, rest) = sorted_data.split_at_mut(5.min(size));

    let binding = String::from("Other");
    let aggregated = rest.iter().fold((0, &binding), |mut acc, tup| {
        acc.0 += tup.0;
        acc
    });

    let mut result = top_items.to_vec();
    if !rest.is_empty() {
        result.push(aggregated);
    }

    let series = Pie::new()
        .radius(vec!["50", "100"])
        .center(vec!["50%", "50%"])
        .data(result);
    let chart = Chart::new()
        .title(Title::new().text(chart_title))
        .legend(Legend::new().top("bottom"))
        .series(series);
    let renderer = WasmRenderer::new(375, 375);

    TimeoutFuture::new(1_000).await;
    renderer.render(chart_id, &chart).unwrap();
}

#[component]
pub fn BlockSpotlight(block: BlocksQueryBlocks) -> impl IntoView {
    let state_hash = get_state_hash(&block);
    let date_time = get_date_time(&block);
    let spotlight_items = vec![
        SpotlightEntry {
            label: "State Hash".to_string(),
            any_el: Some(convert_to_link(
                state_hash.clone(),
                format!("/blocks/{}", state_hash),
            )),
            copiable: true,
        },
        SpotlightEntry {
            label: "Previous State Hash".to_string(),
            any_el: Some({
                let prev_state_hash = get_previous_state_hash(&block);
                convert_to_link(
                    prev_state_hash.clone(),
                    format!("/blocks/{}", prev_state_hash),
                )
            }),
            copiable: true,
        },
        SpotlightEntry {
            label: "Staged Ledger Hash".to_string(),
            any_el: Some(convert_to_span(get_staged_ledger_hash(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "Snarked Ledger Hash".to_string(),
            any_el: Some(convert_to_span(get_snarked_ledger_hash(&block))),
            copiable: true,
        },
        SpotlightEntry {
            label: "Coinbase".to_string(),
            any_el: Some(decorate_with_mina_tag(get_coinbase(&block))),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase Receiver".to_string(),
            any_el: Some({
                let coinbase_receiver = get_coinbase_receiver(&block);
                convert_to_link(
                    coinbase_receiver.clone(),
                    format!("/addresses/accounts/{}", coinbase_receiver),
                )
            }),
            copiable: true,
        },
        SpotlightEntry {
            label: "SNARK Fees".to_string(),
            any_el: Some(decorate_with_mina_tag(get_snark_fees(&block))),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Global Slot".to_string(),
            any_el: Some(convert_to_pill(get_global_slot(&block), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Slot".to_string(),
            any_el: Some(convert_to_pill(get_slot(&block), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Epoch".to_string(),
            any_el: Some(convert_to_pill(get_epoch(&block), ColorVariant::Grey)),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Transaction Fees".to_string(),
            any_el: Some(decorate_with_mina_tag(get_transaction_fees(&block))),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Blockchain Length".to_string(),
            any_el: Some(convert_to_pill(
                get_block_height(&block),
                ColorVariant::Grey,
            )),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Total Currency".to_string(),
            any_el: Some(decorate_with_mina_tag(get_total_currency(&block))),
            ..Default::default()
        },
    ];
    view! {
        <SpotlightSection
            header="Block Spotlight".to_string()
            spotlight_items=spotlight_items
            id=Some(get_state_hash(&block))
            meta=Some(format!("{} ({})", date_time, print_time_since(&date_time)))
        >

            <BlockIcon width=40/>
        </SpotlightSection>
    }
}

#[component]
fn BlockSpotlightPlaceholder() -> impl IntoView {
    let spotlight_items = vec![
        SpotlightEntry {
            label: "State Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Previous State Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Staged Ledger Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Snarked Ledger Hash".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Coinbase Receiver".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "SNARK Fees".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Global Slot".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Slot".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Epoch".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Transaction Fees".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Blockchain Length".to_string(),
            ..Default::default()
        },
        SpotlightEntry {
            label: "Total Currency".to_string(),
            ..Default::default()
        },
    ];
    view! {
        <SpotlightSection
            header="Block Spotlight"
            spotlight_items=spotlight_items
            id=None
            meta=None
        >
            <BlockIcon width=40/>
        </SpotlightSection>
    }
}

#[component]
pub fn BlocksSection() -> impl IntoView {
    let query_params_map = use_query_map();
    let (data_sig, set_data_sig) = create_signal(None);
    let (block_height_sig, _) = create_query_signal::<i64>("q-height");
    let (slot_sig, _) = create_query_signal::<i64>("q-slot");
    let (canonical_sig, _) = create_query_signal::<bool>("canonical");
    let UseIntervalReturn { counter, .. } = use_interval(LIVE_RELOAD_INTERVAL);

    let resource = create_resource(
        move || {
            (
                counter.get(),
                query_params_map.get(),
                block_height_sig.get(),
                slot_sig.get(),
                canonical_sig.get(),
            )
        },
        |(_, q_map, block_height, slot, canonical)| async move {
            load_data(
                TABLE_ROW_LIMIT,
                q_map.get("q-block-producer").cloned(),
                q_map.get("q-state-hash").cloned(),
                block_height,
                slot,
                if canonical.is_some() {
                    canonical
                } else {
                    Some(true)
                },
            )
            .await
        },
    );

    let table_columns = vec![
        TableColumn {
            column: "Height".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "State Hash".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Slot".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Age".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Block Producer".to_string(),
            is_searchable: true,
        },
        TableColumn {
            column: "Coinbase".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "User Commands".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "SNARKs".to_string(),
            is_searchable: false,
        },
        TableColumn {
            column: "Coinbase Receiver".to_string(),
            is_searchable: false,
        },
    ];

    create_effect(move |_| {
        if let Some(data) = resource.get().and_then(|res| res.ok()) {
            set_data_sig.set(Some(data.blocks));
        }
    });

    view! {
        <TableSectionTemplate
            table_columns
            data_sig
            section_heading="Blocks"
            is_loading=resource.loading()
            controls=move || {
                view! {
                    <UrlParamSelectMenu
                        id="canonical-selection"
                        query_str_key="canonical"
                        labels=UrlParamSelectOptions {
                            is_boolean_option: true,
                            cases: vec!["Canonical".to_string(), "Non-Canonical".to_string()],
                        }
                    />
                }
            }
        />

        <Outlet/>
    }
}

#[component]
pub fn BlockSpotlightSnarkJobTable(block: BlocksQueryBlocks) -> impl IntoView {
    view! {
        {move || match block.snark_jobs.clone() {
            Some(snark_jobs) => {
                view! { <DeprecatedTable data=snark_jobs/> }
            }
            _ => ().into_view(),
        }}
    }
}
