use panda::Application;
#[ntex::main]
async fn main() {
    Application::default().run().await
}
