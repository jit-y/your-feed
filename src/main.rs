use anyhow::Result;
use clap::Parser;
use scraper::{Html, Selector};

#[derive(Parser)]
#[clap(version = "0.1.0")]
struct Opts {
    #[clap(short, long)]
    url: String,
    #[clap(long)]
    list_group_selector: String,
    #[clap(long)]
    list_item_selector: String,
    #[clap(long)]
    title_selector: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let res = reqwest::get(opts.url).await?;
    let body = res.text().await?;
    let doc = Html::parse_document(body.as_str());
    let group_selector = Selector::parse(opts.list_group_selector.as_str()).expect("parse error");
    let item_selector = Selector::parse(opts.list_item_selector.as_str()).expect("parse error");
    let title_selector = Selector::parse(opts.title_selector.as_str()).expect("parse error");

    for list_group in doc.select(&group_selector) {
        for list_item in list_group.select(&item_selector) {
            for title in list_item.select(&title_selector) {
                println!("{}", title.inner_html());
            }
        }
    }

    Ok(())
}
