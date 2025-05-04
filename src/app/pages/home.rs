use leptos::{either::Either, prelude::*};
use leptos_meta::Title;

use crate::db::{get_todos, AddTodo, DeleteTodo};

stylance::import_crate_style!(css, "src/app/pages/home.module.scss");

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // // Creates a reactive value to update the button
    // let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;

    let add_todo = ServerMultiAction::<AddTodo>::new();
    let delete_todo = ServerAction::<DeleteTodo>::new();
    let submissions = add_todo.submissions();
    
    let todos = Resource::new(move || {
        (
            delete_todo.version().get(),
            add_todo.version().get(),
        )
    }, move |_| get_todos());

    let existing_todos = move || {
        Suspend::new(async move {
            todos
                .await
                .map(|todos| {
                    if todos.is_empty() {
                        Either::Left(view! { <p>"No tasks were found."</p> })
                    } else {
                        Either::Right(
                            todos
                                .iter()
                                .map(move |todo| {
                                    let id = todo.id;
                                    view! {
                                        <li>
                                            {todo.title.clone()} <ActionForm action=delete_todo>
                                                <input type="hidden" name="id" value=id />
                                                <input type="submit" value="del" />
                                            </ActionForm>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>(),
                        )
                    }
                })
        })
    };

    view! {
        <Title text="Home" />
        <div class=css::home>
            <MultiActionForm action=add_todo>
                <div class=css::header>
                    <label>"Add A Job" <input type="text" name="title" /></label>
                    <input type="submit" value="Add" />
                </div>
            </MultiActionForm>
            <div class=css::todos>
                <Transition fallback=move || view! { <p>"Loading..."</p> }>
                    <ErrorBoundary fallback=|_errors| "error">
                        <ul>
                            {existing_todos}
                            {move || {
                                submissions
                                    .get()
                                    .into_iter()
                                    .filter(|submission| submission.pending().get())
                                    .map(|submission| {
                                        view! {
                                            <li class=css::pending>
                                                {move || submission.input().get().map(|data| data.title)}
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}

                        </ul>
                    </ErrorBoundary>
                </Transition>
            </div>
        </div>
    }
}
