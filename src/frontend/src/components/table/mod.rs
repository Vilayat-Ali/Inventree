use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[function_component]
pub fn Table(props: &Props) -> Html {
    let t_headers = props
        .header
        .iter()
        .map(|t_head| {
            html! {<th>{t_head.clone()}</th>}
        })
        .collect::<Html>();

    let t_rows = props
        .rows
        .iter()
        .map(|t_row| {
            html! {
                <tr>
                    {
                        t_row.iter().map(|row_entry| {
                            html!{
                                <td>{row_entry.clone()}</td>
                            }
                        }).collect::<Html>()
                    }
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <div class="overflow-x-auto w-80">
            <table class="table w-full">
                <thead>
                <tr>
                    {t_headers}
                </tr>
                </thead>

                <tbody>
                    {t_rows}
                </tbody>
            </table>
        </div>
    }
}
