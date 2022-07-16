use anyhow::{Result, Context};


fn main() -> Result<()> {
    println!("edge driverのダウンロードを開始します");
    let driver_path = fetch_edge_driver::save_driver_with_exe()?;
    println!("edge driverのダウンロードを完了しました");
    println!("edge driverファイルパス: {}",
        driver_path.to_str().context("edge driverパスの文字列化に失敗した")?);
    Ok(())
}
