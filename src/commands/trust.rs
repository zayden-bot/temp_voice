use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    GuildChannel, PermissionOverwrite, PermissionOverwriteType, Permissions, ResolvedOption,
    ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;
use crate::VoiceChannelManager;

pub async fn trust(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _member)) => user,
        _ => unreachable!("User option is required"),
    };

    {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get_mut(&channel.id)
            .expect("Expected channel in manager");
        channel_data.add_trusted(user.id);
    }

    channel
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL
                    | Permissions::MANAGE_CHANNELS
                    | Permissions::CONNECT
                    | Permissions::SET_VOICE_CHANNEL_STATUS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Set user to trusted.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
