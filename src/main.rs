mod hub;
use crate::hub::Hub;
use color_eyre::{Report, Result};
use rss::{Channel, ChannelBuilder, GuidBuilder, ItemBuilder};
use tokio::signal::unix::{signal, SignalKind};
use warp::reject::Reject;
use warp::{Filter, Rejection};

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = dotenv::var("PORT")
        .map(|port| port.parse())
        .unwrap_or(Ok(80))?;

    let hub = Hub::default();
    let hub = warp::any().map(move || hub.clone());

    let feed_route = warp::path!(String / String).and(hub).and_then(feed);
    let routes = feed_route.clone().or(warp::path!("r" / ..).and(feed_route));

    let mut int = signal(SignalKind::interrupt())?;
    warp::serve(routes)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], port), async move {
            int.recv().await;
        })
        .1
        .await;

    Ok(())
}

#[derive(Debug)]
struct ReportRejection(Report);

impl Reject for ReportRejection {}

async fn feed(user: String, repo: String, hub: Hub) -> Result<impl warp::Reply, Rejection> {
    feed_inner(user, repo, hub)
        .await
        .map_err(ReportRejection)
        .map_err(warp::reject::custom)
}

async fn feed_inner(user: String, repo: String, hub: Hub) -> Result<impl warp::Reply> {
    let repo = repo.trim_end_matches(".atom").to_string();
    let mut channel: Channel = ChannelBuilder::default()
        .title(format!("{}/{} | Docker Hub Images", user, repo))
        .link(format!("https://hub.docker.com/r/{}/{}", user, repo))
        .description(format!("Image updates for {}/{}", user, repo))
        .build();

    let tags = hub.tags(&user, &repo).await?;

    channel.items = tags
        .into_iter()
        .map(|tag| {
            ItemBuilder::default()
                .title(format!("{}/{}:{}", user, repo, tag.name))
                .link(format!(
                    "https://hub.docker.com/r/{}/{}/tags?name={}",
                    user, repo, tag.name
                ))
                .guid(
                    GuidBuilder::default()
                        .value(format!("{}-{}", tag.id, tag.last_updated.timestamp()))
                        .permalink(false)
                        .build(),
                )
                .pub_date(tag.last_updated.to_rfc2822())
                .build()
        })
        .collect();

    Ok(channel.to_string())
}
