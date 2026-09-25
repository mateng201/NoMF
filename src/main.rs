use alloy::providers::ProviderBuilder;
use erc8004::{
    Erc8004, Network,//*
    types::{Registration, RegistrationFile, ServiceEndpoint},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let network = Network::EthereumMainnet;//*
    let provider = ProviderBuilder::new().connect_http("https://localhost:1".parse()?);//*
    let client = Erc8004::new(provider).with_network(network);//*new

    println!("== ERC-8004 Network Inspector ==");
    println!("network: Ethereum mainnet");//*
    println!("chain id: {}", network.chain_id());
    println!("identity registry: {:?}", client.identity_address());
    println!("reputation registry: {:?}", client.reputation_address());
    println!("agent registry prefix: {}", network.agent_registry_prefix());

    let mut file = RegistrationFile::new(//*
        "WeatherBot",
        "A tiny demo agent registration file built after reading erc8004.",
    );

    file.services.push(
        ServiceEndpoint::new(//*
            "A2A",
            "https://weather.example.com/.well-known/agent-card.json",
        )
        .with_version("1.0"),
    );
    file.services.push(
        ServiceEndpoint::new("MCP", "https://weather.example.com/mcp").with_version("2025-03-26"),
    );//*
    file.x402_support = true;
    file.supported_trust = vec!["reputation".into(), "validation".into()];
    file.registrations.push(Registration {
        agent_id: 1,
        agent_registry: network.agent_registry_prefix(),
    });

    println!("\n== Registration JSON ==");
    let json = file.to_json()?;
    println!("{json}");

    println!("\n== Round Trip Check ==");
    let parsed = RegistrationFile::from_json(&json)?;//*
    println!("agent name: {}", parsed.name);
    println!("service count: {}", parsed.services.len());
    println!("supports x402: {}", parsed.x402_support);

    println!("\n== Expected Error Demo ==");
    let unconfigured_provider = ProviderBuilder::new().connect_http("https://localhost:1".parse()?);
    let unconfigured = Erc8004::new(unconfigured_provider);//*
    match unconfigured.identity() {
        Ok(_) => println!("unexpected: identity registry is configured"),
        Err(err) => println!("identity() before with_network(): {err}"),
    }

    Ok(())
}
