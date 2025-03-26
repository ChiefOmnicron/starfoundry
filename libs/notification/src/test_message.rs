use crate::{Discord, DiscordEmbed, Error, Notification, NotificationTarget};
use serde::Serialize;

pub struct TestMessage;

impl TestMessage {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(
        &self,
        target: NotificationTarget,
        url:    String,
    ) -> Result<String, String> {
        match target {
            NotificationTarget::Discord => {
                let discord = self.discord_message();

                match self.discord(url, vec![discord]).await {
                    Ok(x)  => Ok(x),
                    Err(Error::ResponseError(x, _)) => Err(format!("{x}")),
                    Err(e) => Err(format!("{e}"))
                }
            },
            NotificationTarget::Json => {
                let json = self.json_message();

                match self.json(url, json).await {
                    Ok(x)  => Ok(x),
                    Err(Error::ResponseError(x, _)) => Err(format!("{x}")),
                    Err(e) => Err(format!("{e}"))
                }
            },
        }
    }

    fn discord_message(
        &self
    ) -> Discord {
        let mut discord = Discord::new();

        let embed = DiscordEmbed::new(
            "Test Message",
            "TestMessagePleaseIgnore",
            crate::DiscordColor::DarkGrey,
        );

        discord.embeds.push(embed);

        discord
    }

    fn json_message(
        &self,
    ) -> serde_json::Value {
        #[derive(Serialize)]
        struct TmpMessage {
            event:  String,
            message: String,
        }

        let message = TmpMessage {
            event: "TEST_MESSAGE".into(),
            message: "TestMessagePleaseIgnore".into(),
        };

        serde_json::to_value(&message).unwrap_or_default()
    }
}

impl Notification for TestMessage {}
