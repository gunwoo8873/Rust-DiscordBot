use aws_config::BehaviorVersion;

pub async fn aws_run() {
  let config = aws_config::defaults(BehaviorVersion::latest())
  .load()
  .await;

  println!("AWS Config: {:?}", config);
}