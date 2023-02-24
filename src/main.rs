use exitfailure::ExitFailure;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, ExecuteWebhook};
use serenity::http::Http;
use serenity::json::Value;
use serenity::model::prelude::Embed;
use serenity::model::webhook::Webhook;
use serenity::utils::Colour;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let http: Http = Http::new("");

    const DISCORD_ID: &str = "1078478546005147689";
    const DISCORD_TOKEN: &str =
        "rGYvHEv4DswykuhWVIN6yxc2TP8jf0m92PtPWiPa5sQ2AHZmEstXEYXK34JTx3530c2z";

    let discordURL = format!(
        "https://discord.com/api/webhooks/{}/{}",
        DISCORD_ID, DISCORD_TOKEN
    );

    let webhook: Webhook = Webhook::from_url(&http, &discordURL).await?;

    let embed: Value = Embed::fake(|e: &mut CreateEmbed| {
        e.color(Colour::DARK_GREEN);
        e.title("Rust - Discord Webhook");
        e.thumbnail("https://avatars.githubusercontent.com/u/5430905?s=200&v=4");
        e.author(|f: &mut CreateEmbedAuthor| {
            f.icon_url("https://avatars.githubusercontent.com/u/77025415?v=4");
            f.name("Gabriel Alonso");
            f
        });
        e.field("", "", false);
        e.field("language", "rust", false);
        e.field("feature", "webhook", false);
        e
    });

    webhook
        .execute(&http, false, |w: &mut ExecuteWebhook| w.embeds(vec![embed]))
        .await?;

    Ok(())
}
