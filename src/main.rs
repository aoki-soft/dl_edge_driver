use anyhow::{Result, Context, Ok};


fn main() -> Result<()> {
    println!("Edge Driverの確認・ダウンロードを行います");
    let (is_fetch, driver_path) = fetch_edge_driver::check_get_driver_with_exe()?;
    match is_fetch {
        fetch_edge_driver::FetchNew::Yes => {
            println!("Edge Driverのダウンロードを完了しました");
            println!("Edge Driverファイルパス: {}",
            driver_path.to_str().context("Edge Driverパスの文字列化に失敗した")?);
        },
        _ => {
            println!("Edge Driverは最新でした");
            println!("Edge Driverファイルパス: {}",
            driver_path.to_str().context("Edge Driverパスの文字列化に失敗した")?);
        }
    }
    Ok(())
}
