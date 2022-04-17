use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"RustVault"}</h1>
            <p>{"RustVault is a secure way to store your plaintext on the cloud. Completely free and open source. Licensed under GPL v3.0"}</p>
            <h2>{"QNA"}</h2>
            <h2>{"Q. Why does this website look like this?"}</h2>
            <p>{"A. Look around you dumbass. The web is filled with shitty unresponsive websites that depend on 80MB JQuery animations just so they can animate a div. This website follows the http://motherfuckingwebsite.com/ philosophy."} </p>
            <h3>{"Q. How many accounts and plaintext files can I store?"}</h3>
            <p>{"A. OVER 9000!!! /s"}</p>
            <code>{"infinite"}</code>
            <h3>{"Q. Can I redistribute this website?"}</h3>
            <p>{"Yes. Just don't slap a proprietary license over it you poo-poo head."}</p>
        </div>
    }
}
