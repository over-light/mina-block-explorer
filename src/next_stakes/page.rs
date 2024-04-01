use super::functions::*;
use crate::{
    common::{components::*, functions::*, search::*, table::*},
    stakes::components::StakesNavButton,
};
use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn NextStakesPage() -> impl IntoView {
    let query_params_map = use_query_map();
    let resource = create_resource(
        move || query_params_map.get(),
        |params_map| async move {
            let public_key = params_map.get("query");
            load_data(50, public_key.cloned()).await
        },
    );

    let records_per_page = 10;
    let (current_page, set_current_page) = create_signal(1);

    view! {
        <Title text="Next Staking Ledger | Search For Stakers"/>
        <SearchBar placeholder="Exact search for public key"/>
        <PageContainer>
            <ErrorBoundary fallback=move |_| view! { <NullView/> }>
                <Suspense fallback=move || {
                    view! {
                        <TableSection
                            section_heading="Next Staking Ledger"
                            controls=move || view! { <NullView/> }
                        >
                            <Table data=LoadingPlaceholder {}/>
                        </TableSection>
                    }
                }>
                    {move || {
                        resource
                            .get()
                            .and_then(|res| res.ok())
                            .map(|data| {
                                let pag = build_pagination(
                                    data.nextstakes.len(),
                                    records_per_page,
                                    current_page.get(),
                                    set_current_page,
                                );
                                let subset = get_subset(
                                    &data.nextstakes,
                                    records_per_page,
                                    current_page.get() - 1,
                                );
                                view! {
                                    <TableSection
                                        section_heading="Next Staking Ledger"
                                        controls=move || {
                                            view! {
                                                <StakesNavButton href="/stakes" text="Current Stakes"/>
                                            }
                                        }
                                    >

                                        <Table data=subset pagination=pag/>
                                    </TableSection>
                                }
                            })
                    }}

                </Suspense>
            </ErrorBoundary>
        </PageContainer>
    }
}
