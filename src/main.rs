use kernel_runtime::{App, AppConfig, LogConfig, init_tracing};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    init_tracing(&LogConfig::from_env());
    let config = AppConfig::from_env().map_err(|e| e.report())?;
    App::builder(config)
        .module(mod_audit::AuditModule::new())
        .module(mod_identity::IdentityModule::new(
            mod_identity::IdentityConfig::from_env()?,
        ))
        .build()
        .await
        .map_err(|e| e.report())?
        .run()
        .await
        .map_err(|e| e.report())?;
    Ok(())
}
