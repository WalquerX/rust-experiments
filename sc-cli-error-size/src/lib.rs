//! Finds where the 176 bytes of `sc_cli::Error` come from.
//!
//! `sc_cli::Error` holds `sc_service::Error`, which holds more error enums.
//! This test measures all three levels and prints each level sorted by size,
//! so the fat path is visible without reading every row.
//!
//! Run with:
//!   cargo test -- --nocapture

#[cfg(test)]
mod tests {
	use std::mem::size_of;

	/// Prints a heading and its rows, largest first.
	fn table(heading: &str, mut rows: Vec<(&str, usize)>) {
		rows.sort_by(|a, b| b.1.cmp(&a.1));
		println!("\n{heading}");
		println!("{}", "=".repeat(heading.len()));
		for (name, bytes) in rows {
			println!("{bytes:>5}  {name}");
		}
	}

	#[test]
	fn report_error_sizes() {
		table(
			"level 0: what the call site moves",
			vec![
				("sc_cli::Error", size_of::<sc_cli::Error>()),
				("sp_inherents::InherentData (the Ok side)", size_of::<sp_inherents::InherentData>()),
				(
					"Result<InherentData, sc_cli::Error>",
					size_of::<Result<sp_inherents::InherentData, sc_cli::Error>>(),
				),
			],
		);

		table(
			"level 1: sc_cli::Error variant payloads",
			vec![
				("Service(sc_service::Error)", size_of::<sc_service::Error>()),
				("Client(sp_blockchain::Error)", size_of::<sp_blockchain::Error>()),
				("InvalidUri(sp_core::crypto::PublicError)", size_of::<sp_core::crypto::PublicError>()),
				("Codec(codec::Error)", size_of::<codec::Error>()),
				("KeyStorage(sc_keystore::Error)", size_of::<sc_keystore::Error>()),
				("Input(String)", size_of::<String>()),
				("NetworkKeyNotFound(PathBuf)", size_of::<std::path::PathBuf>()),
				("HexDataConversion(hex::FromHexError)", size_of::<hex::FromHexError>()),
				("Application(Box<dyn Error>)", size_of::<Box<dyn std::error::Error>>()),
				("Cli(clap::Error)", size_of::<clap::Error>()),
				("Io(std::io::Error)", size_of::<std::io::Error>()),
			],
		);

		table(
			"level 2: sc_service::Error variant payloads",
			vec![
				("Client(sp_blockchain::Error)", size_of::<sp_blockchain::Error>()),
				("Consensus(sp_consensus::Error)", size_of::<sp_consensus::Error>()),
				("Network(sc_network::error::Error)", size_of::<sc_network::error::Error>()),
				("Keystore(sp_keystore::Error)", size_of::<sp_keystore::Error>()),
				("Telemetry(sc_telemetry::Error)", size_of::<sc_telemetry::Error>()),
				(
					"Prometheus(PrometheusError)",
					size_of::<substrate_prometheus_endpoint::PrometheusError>(),
				),
				("Other(String)", size_of::<String>()),
				("Application(Box<dyn Error>)", size_of::<Box<dyn std::error::Error>>()),
				("Io(std::io::Error)", size_of::<std::io::Error>()),
			],
		);

		table(
			"level 3: sc_network::error::Error variant payloads",
			vec![
				(
					"DuplicateBootnode { Multiaddr, PeerId, PeerId }",
					size_of::<(
						sc_network_types::multiaddr::Multiaddr,
						sc_network_types::PeerId,
						sc_network_types::PeerId,
					)>(),
				),
				(
					"AddressesForAnotherTransport { TransportConfig, Vec<Multiaddr> }",
					size_of::<(
						sc_network::config::TransportConfig,
						Vec<sc_network_types::multiaddr::Multiaddr>,
					)>(),
				),
				("Litep2p(litep2p::Error)", size_of::<litep2p::Error>()),
				("PeerDoesntExist(PeerId)", size_of::<sc_network_types::PeerId>()),
				(
					"DuplicateRequestResponseProtocol { ProtocolName }",
					size_of::<sc_network::ProtocolName>(),
				),
				(
					"Prometheus(PrometheusError)",
					size_of::<substrate_prometheus_endpoint::PrometheusError>(),
				),
				("Client(Box<sp_blockchain::Error>)  <- already boxed", size_of::<Box<sp_blockchain::Error>>()),
				("Io(std::io::Error)", size_of::<std::io::Error>()),
				("ChannelClosed / ConnectionClosed", 0),
			],
		);

		table(
			"parts, for reference",
			vec![
				("sc_network_types::multiaddr::Multiaddr", size_of::<sc_network_types::multiaddr::Multiaddr>()),
				("sc_network_types::PeerId", size_of::<sc_network_types::PeerId>()),
				("sc_network::config::TransportConfig", size_of::<sc_network::config::TransportConfig>()),
			],
		);

		println!();
	}
}