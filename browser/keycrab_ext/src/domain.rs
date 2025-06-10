use crate::button::Button;
use leptos::prelude::*;

#[derive(Clone)]
struct DomainInfo {
    pub id: i32,
    pub domain: String,
    pub username: String,
    pub password: String,
}

#[component]
fn Domain(#[prop(into)] domain_info: DomainInfo) -> impl IntoView {
    view! {
        <div class="flex gap-4 py-4 items-center hover:bg-slate-700 focus:rounded-xl focus:border-1 focus:border-slate-600">
            <div class="size-7 rounded-md bg-red-900 text-center text-2xl grid place-items-center">
                <i class="iconoir-lock-square"></i>
            </div>
            <div class="flex flex-col grow">
                <span class="text-lg">{{ domain_info.domain }}</span>
                <div class="flex gap-2 text-md">
                    <span class="font-semibold">username:</span>
                    {{ domain_info.username }}
                </div>
            </div>
            <Button>"Fill"</Button>
        </div>
    }
}

#[component]
pub fn Domains() -> impl IntoView {
    let (domains, _) = signal(vec![
        DomainInfo {
            id: 0,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 1,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 2,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 3,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 4,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 5,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 6,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 7,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 8,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 9,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 10,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 11,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 122,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 13,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 14,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 15,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
        DomainInfo {
            id: 16,
            domain: "https://gmail.com".to_string(),
            username: "test@gmail.com".to_string(),
            password: "test123".to_string(),
        },
    ]);
    view! {
        <div class="flex flex-col divide-y divide-slate-600">
            <For each=move || domains.get() key=|x| x.id let:child>
                <Domain domain_info=child />
            </For>
        </div>
    }
}
