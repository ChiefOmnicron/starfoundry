use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Discord {
    pub content: String,
    pub embeds: Vec<DiscordEmbed>,
}

impl Discord {
    pub fn new() -> Self {
        Self {
            content: "".into(),
            embeds: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DiscordEmbed {
    pub title:       String,
    pub description: String,
    pub color:       i32,
    pub fields:      Vec<DiscordField>,
}

impl DiscordEmbed {
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
}

pub struct DiscordEmbedBuilder {
    embed: DiscordEmbed,
}

impl DiscordEmbedBuilder {
    pub fn new(
        embed: DiscordEmbed,
    ) -> Self {
        Self {
            embed,
        }
    }

    pub fn build_two_fields(
        self,
        field1: DiscordAddField,
        field2: DiscordAddField,
    ) -> Vec<DiscordEmbed> {
        let mut embeds = Vec::new();
        let mut field1_merged = String::new();
        let mut field2_merged = String::new();

        for (field1_entry, field2_entry) in field1
                                                    .entries
                                                    .iter()
                                                    .zip(field2.entries.iter()) {

            field1_merged += &format!("{field1_entry}\n");
            field2_merged += &format!("{field2_entry}\n");

            // 1024 is the limit from discord, and 950 gives us enough room
            if field1_merged.len() + field2_merged.len() >= 950 {
                let mut embed_clone = self.embed.clone();

                embed_clone.fields.push(DiscordField::new(field1.name.clone(), field1_merged.clone()));
                embed_clone.fields.push(DiscordField::new(field2.name.clone(), field2_merged.clone()));

                if field1_merged.len() > 0 && field2_merged.len() > 0 {
                    embeds.push(embed_clone);
                }

                field1_merged = String::new();
                field2_merged = String::new();
            }
        }

        let mut embed_clone = self.embed.clone();
        embed_clone.fields.push(DiscordField::new(field1.name.clone(), field1_merged));
        embed_clone.fields.push(DiscordField::new(field2.name.clone(), field2_merged));
        embeds.push(embed_clone);

        embeds
    }
}

pub struct DiscordAddField {
    pub name: String,
    pub entries: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DiscordField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

impl DiscordField {
    pub fn new<S: Into<String>>(
        name: S,
        value: S,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline: true,
        }
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
