use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, GuildChannel, PermissionOverwriteType,
    ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn unblock(
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

    channel
        .delete_permission(ctx, PermissionOverwriteType::Member(user.id))
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Removed user from blocked."),
        )
        .await?;

    Ok(())
}
