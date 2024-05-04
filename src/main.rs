use aws_config::BehaviorVersion;
use aws_sdk_cloudformation as cf;
use aws_sdk_ec2 as ec2;
use cf::types::OnFailure;

mod template;

#[::tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let _ec2_client = ec2::Client::new(&config);
    let cf_client = cf::Client::new(&config);

    //TODO make configurable
    let _image_id = "ami-019a292cfb114a776";

    //TODO make configurable
    let _docker_image = "nginx:latest";

    //TODO implement
    let _key_name = "";

    let tp = template::Template {};

    //TODO Generate stack name automatically (hash?)
    let create_stack_output = cf_client
        .create_stack()
        .stack_name("showers")
        .template_body(serde_json::to_string(&tp)?)
        .on_failure(OnFailure::Delete)
        .send()
        .await?;

    let stack_id = create_stack_output
        .stack_id
        .expect("AWS Api failed to provide a stack ID");

    cf_client.delete_stack().stack_name(stack_id).send().await?;

    Ok(())
}
