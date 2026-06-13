use serde::Serialize;
use crate::error::{Error, Result};

#[derive(Debug, Serialize)]
pub struct Discord {
    pub content:    String,
    pub embeds:     Vec<DiscordEmbedding>,
}

impl Discord {
    pub fn new() -> Self {
        Self {
            content:    "".into(),
            embeds:     Vec::new(),
        }
    }

    pub fn add_embedding(
        &mut self,
        embedding: DiscordEmbedding,
    ) {
        self.embeds.push(embedding);
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DiscordEmbedding {
    pub title:       String,
    pub description: String,
    pub color:       i32,
    pub fields:      Vec<DiscordField>,
}

impl DiscordEmbedding {
    pub fn new<S: Into<String>>(
        title:       S,
        description: S,
        color:       DiscordColor,
    ) -> Self {
        Self {
            title:       title.into(),
            description: description.into(),
            color:       color.as_code(),
            fields:      Vec::new(),
        }
    }

    /// Adds an additional field to the discord message.
    /// The limit is 25 limits, afterwards an error will be thrown
    pub fn add_field(
        &mut self,
        field: DiscordField,
    ) -> Result<()> {
        if self.fields.len() == 25 {
            tracing::error!("Max discord fields of 25 reached");
            Err(Error::TooManyFields)
        } else {
            self.fields.push(field);
            Ok(())
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DiscordField {
    pub name:   String,
    pub value:  String,
    pub inline: bool,
}

impl DiscordField {
    pub fn new<S: Into<String>>(
        name:   S,
        value:  S,
        inline: bool,
    ) -> Result<Self, Error> {
        let name = name.into();
        let value = value.into();

        if name.len() > 256 {
            return Err(Error::TooManyFieldCharacters);
        }
        if value.len() > 256 {
            return Err(Error::TooManyValueCharacters);
        }

        Ok(Self {
            name,
            value,
            inline,
        })
    }
}

/// https://gist.github.com/thomasbnt/b6f455e2c7d743b796917fa3c205f812
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub enum DiscordColor {
    Default,
    Aqua,
    DarkAqua,
    Green,
    DarkGreen,
    Blue,
    DarkBlue,
    Purple,
    DarkPurple,
    LuminousVividPink,
    DarkVividPink,
    Gold,
    DarkGold,
    Orange,
    DarkOrange,
    Red,
    DarkRed,
    Grey,
    DarkGrey,
    DarkerGrey,
    LightGrey,
    Navy,
    DarkNavy,
    Yellow,
}

impl DiscordColor {
    pub fn as_code(self) -> i32 {
        match self {
            Self::Default           => 0,
            Self::Aqua              => 1752220,
            Self::DarkAqua          => 1146986,
            Self::Green             => 5763719,
            Self::DarkGreen         => 2067276,
            Self::Blue              => 3447003,
            Self::DarkBlue          => 2123412,
            Self::Purple            => 10181046,
            Self::DarkPurple        => 7419530,
            Self::LuminousVividPink => 15277667,
            Self::DarkVividPink     => 11342935,
            Self::Gold              => 15844367,
            Self::DarkGold          => 12745742,
            Self::Orange            => 15105570,
            Self::DarkOrange        => 11027200,
            Self::Red               => 15548997,
            Self::DarkRed           => 10038562,
            Self::Grey              => 9807270,
            Self::DarkGrey          => 9936031,
            Self::DarkerGrey        => 8359053,
            Self::LightGrey         => 12370112,
            Self::Navy              => 3426654,
            Self::DarkNavy          => 2899536,
            Self::Yellow            => 16776960,
        }
    }
}
