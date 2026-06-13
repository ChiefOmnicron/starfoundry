use crate::discord::Discord;
use crate::error::{Error, Result};

mod discord;
mod error;

pub async fn send_discord(
    url:      String,
    messages: Vec<Discord>,
) -> Result<String> {
    if !url.contains("https://discord.com/api") {
        return Err(Error::InvalidTarget("DISCORD".into(), url));
    }

    for message in messages {
        let value = serde_json::to_value(&message)?;

        if let Err(e) = dbg!(send_json(url.clone(), value).await) {
            return Err(e)
        }
    }

    Ok("OK".into())
}

pub async fn send_json(
    url:   String,
    value: serde_json::Value,
) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&value)
        .send()
        .await?;

    if !response.status().is_success() {
        let message = response
            .text()
            .await
            .map_err(Error::GenericReqwestError)?;
        Err(Error::ResponseError(message, value))
    } else {
        let message = response
            .text()
            .await
            .map_err(Error::GenericReqwestError)?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use crate::discord::{Discord, DiscordColor, DiscordEmbedding, DiscordField};
    use crate::send_discord;

    #[tokio::test]
    async fn test_send_discord_happy_path() {
        let mut discord_message = Discord::new();
        let mut embedding = DiscordEmbedding::new(
            "Test",
            "Message",
            DiscordColor::Aqua,
        );
        embedding.add_field(DiscordField::new(
                "Field 1",
                "Value 1",
                true,
            )
            .unwrap()
        ).unwrap();
        embedding.add_field(DiscordField::new(
                "Field 2",
                "Value 2",
                true,
            )
            .unwrap()
        ).unwrap();
        discord_message.add_embedding(embedding);
        let result = send_discord(
                "https://discord.com/api/webhooks/1515149426711072769/heECjRRXGaSxHj6a-BOssUgRcL7UAfz8YSndxxmZSNBdVzJy15bqAGNscVwvW2lbPcyr".into(),
                vec![discord_message],
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_discord_field_name_too_long() {
        let mut discord_message = Discord::new();
        let mut embedding = DiscordEmbedding::new(
            "Test",
            "Message",
            DiscordColor::Aqua,
        );
        embedding.add_field(DiscordField::new(
                "ThisFieldMessageIsWayTooLongAndShouldThrowAnErrorIfItDoesNotDoThatThenThereIsSomethingVeryWrongAndItNeedsToBeFixedOtherwiseThereMightBeErrorsInALaterStageOfTheApplicationWhichOfCourseIsNotAnDesiredOutcomeIStillHaveACoupleMoreRequiredCharactersBeforeIReachTheGoalAndIAmRunningOutOfIdeas",
                "Value 1",
                true,
            )
            .unwrap()
        ).unwrap();
        discord_message.add_embedding(embedding);
        let result = send_discord(
                "https://discord.com/api/webhooks/1515149426711072769/heECjRRXGaSxHj6a-BOssUgRcL7UAfz8YSndxxmZSNBdVzJy15bqAGNscVwvW2lbPcyr".into(),
                vec![discord_message],
            )
            .await;

        assert!(result.is_err());
    }
}
