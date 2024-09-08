use panda::Cache;
#[ntex::main]
async fn main() {
    Cache::init().await.run().await
}
