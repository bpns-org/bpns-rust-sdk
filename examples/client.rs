// Copyright (c) 2021-2022 Yuki Kishimoto
// Distributed under the MIT software license

extern crate bpns_sdk;

use bpns_sdk::client::Client;

#[tokio::main]
async fn main() {
    // Generate new token
    let new_token = Client::generate_new_token("http://127.0.0.1:50055").await.unwrap();
    println!("New Token: {}", new_token);

    // Init client
    let client = Client::new("http://127.0.0.1:50055", new_token.as_str());

    // Subscribe token
    client.subscribe().await.unwrap();

    // Get notifications without delete
    println!("Notifications: {:?}", client.notifications().await.unwrap());

    // Delete notification by id
    client.delete_notification_by_id("notification_id").await.unwrap();

    // Delete all notifications
    client.delete_notifications().await.unwrap();

    // Add new addresses
    let addresses: Vec<&str> =
        vec!["bc1q7ug4w4as2sefar89q057hnmxkakp58a25535ttlmurn6cncs8tms4e7gp2"];
    client.add_addresses(&addresses).await.unwrap();

    // Add new addresses from singlesig
    client.add_addresses_from_singlesig("zpub6s1rSuNVVpH88zXPyXdtCduh8XwyaE9eCBYiCXM29iF9gHpDznAU2F4GeYZe7qi3SwdZ9BJm1gkDD8C3SGp7qnA9D2hJjyFRU8b6EeYnTH9", 0, 250, false).await.unwrap();

    // Add new addresses from multisig
    client.add_addresses_from_multisig("p2wsh", 2, &["Zpub748ymTifcW1UhCiJHKmXcpRe5AGKsYnbYFyecW7Wbbwm2jghz8SaJ7sNEQMEHovqv3xaHMWCzPFkmRSEqgLNYaiHBtP26KsNDgaF8eRjTWq", "Zpub747CZL1obhcxYuenctciPW6Y2WzMf9eYQuqQbCQGDEqfYFZkMz3gCRB7qGwZifZwqAaQRQDUwed8UztrVp62o2BqTjDKh716UTuMnmtrJoh"], 0, 250, false).await.unwrap();

    // Get addresses
    println!("Addresses: {:?}", client.addresses().await.unwrap());

    // Delete addresses
    client.delete_addresses(&addresses).await.unwrap();

    // Delete addresses of singlesig
    client.delete_addresses_from_singlesig("zpub6s1rSuNVVpH88zXPyXdtCduh8XwyaE9eCBYiCXM29iF9gHpDznAU2F4GeYZe7qi3SwdZ9BJm1gkDD8C3SGp7qnA9D2hJjyFRU8b6EeYnTH9", 0, 250, false).await.unwrap();

    // Delete addresses from multisig
    client.delete_addresses_from_multisig("p2wsh", 2, &["Zpub748ymTifcW1UhCiJHKmXcpRe5AGKsYnbYFyecW7Wbbwm2jghz8SaJ7sNEQMEHovqv3xaHMWCzPFkmRSEqgLNYaiHBtP26KsNDgaF8eRjTWq", "Zpub747CZL1obhcxYuenctciPW6Y2WzMf9eYQuqQbCQGDEqfYFZkMz3gCRB7qGwZifZwqAaQRQDUwed8UztrVp62o2BqTjDKh716UTuMnmtrJoh"], 0, 250, false).await.unwrap();

    // Get addresses
    println!("Addresses: {:?}", client.addresses().await.unwrap());

    // Delete account and data
    client.unsubscribe().await.unwrap();
}
