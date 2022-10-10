// // fn main() {
// //     println!("Hello, world!");
// // }

// // 今回のサンプルが必要とする`warp.Filter` traitをimportします。
// use warp::Filter;

// // 今回tokioのランタイムを利用する
// // 非同期ランタイムの上で実行されるためmain関数はasyncをつけて定義します
// #[tokio::main]
// async fn main() {
//     // GET /hello/warp => 200 OK with body "Hello, warp!"
//     let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

//     // Serverの起動
//     warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
// }

use std::convert::Infallible;
use warp::Filter;
use tokio;

mod search_dirctory;

async fn hello(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("hello {}!\n", name))
}

async fn searchdir() -> Result<impl warp::Reply, Infallible> {
    search_dirctory::search_dir("/videos");
}

#[tokio::main]
async fn main() { 
    let addr = [0, 0, 0, 0];
    let port = 3030;

    // fiter必要なfilterを個別に用意して、ruteにまとめる必要がある
    let example = warp::path!("example" / String).and(warp::get()).and_then(hello);

    // ２つ目以降を作成したら.or(filter)のように追加する。
    let routes = warp::get().and(
        example
    );
    warp::serve( routes ).run((addr, port)).await;
}