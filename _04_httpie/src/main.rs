use anyhow::anyhow;
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, Response, Url, header};
use std::collections::HashMap;
use std::str::FromStr;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);

    let mut headers = header::HeaderMap::new();
    // 为我们的 http 客户端添加一些缺省的 HTTP 头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = match cli.cmd {
        SubCommand::Get(ref args) => get(&client, args).await,
        SubCommand::Post(ref args) => post(&client, args).await,
    };

    Ok(result?)
}

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "JX <xongxg@gmail.com>")]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Parser)]
struct Get {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}

#[derive(Debug, Parser)]
struct Post {
    #[clap(parse(try_from_str=parse_url))]
    url: String,

    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

fn parse_url(url: &str) -> anyhow::Result<String> {
    let _ = url.parse::<Url>()?;
    Ok(url.into())
}

#[derive(Debug, PartialEq)]
struct KvPair {
    key: String,
    value: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(KvPair {
            key: split.next().ok_or_else(err)?.to_string(),
            value: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> anyhow::Result<KvPair> {
    s.parse()
}

async fn get(client: &Client, args: &Get) -> anyhow::Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(())
}

async fn post(client: &Client, args: &Post) -> anyhow::Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!();
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

/// 打印整个响应
async fn print_resp(resp: Response) -> anyhow::Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(m) if m == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(m) if m == mime::TEXT_HTML => print_syntect(body, "html"),
        _ => print!("{}", body),
    }
}
