use super::CommandResult;
use codewhale_command_contract::facets::CommandSessionControlContext;
use codewhale_command_contract::handler::{CommandContexts, CommandHandler};
use codewhale_command_contract::metadata::{
    CommandInfo as ContractInfo, RegisterCommand as ContractRegisterCommand,
};

pub(in crate::commands) const COMPOSER_COLORS: [&str; 8] = [
    "red", "orange", "amber", "green", "cyan", "blue", "purple", "pink",
];

pub(in crate::commands) struct ColorCmd;

pub(in crate::commands) const CONTRACT_INFO: ContractInfo = ContractInfo {
    name: "color",
    aliases: &["colour"],
    usage: "/color [red|orange|amber|green|cyan|blue|purple|pink|off]",
    description_key: "cmd_color_description",
};

impl ContractRegisterCommand<CommandResult> for ColorCmd {
    fn info() -> &'static ContractInfo {
        &CONTRACT_INFO
    }
    fn handler() -> CommandHandler<CommandResult> {
        CommandHandler::Contextual {
            capabilities: codewhale_command_contract::handler::CommandCapabilities::SESSION_CONTROL,
            handler: color_contextual,
        }
    }
}

pub(in crate::commands) fn color_contextual(
    contexts: CommandContexts<'_>,
    arg: Option<&str>,
) -> CommandResult {
    let mut parts = contexts.into_parts();
    let Some(control) = parts.control.as_deref_mut() else {
        return CommandResult::error("Command capability unavailable: session_control".to_string());
    };
    color_portable(control, arg)
}

pub(in crate::commands) fn color_portable(
    control: &mut dyn CommandSessionControlContext,
    arg: Option<&str>,
) -> CommandResult {
    let requested = arg
        .map(str::trim)
        .filter(|arg| !arg.is_empty())
        .map(str::to_ascii_lowercase);
    let next = match requested.as_deref() {
        None => {
            let current = control.composer_color();
            let position = current
                .as_deref()
                .and_then(|name| COMPOSER_COLORS.iter().position(|color| *color == name));
            Some(COMPOSER_COLORS[position.map_or(0, |index| (index + 1) % COMPOSER_COLORS.len())])
        }
        Some("off" | "clear" | "none" | "default") => None,
        Some(name) => match COMPOSER_COLORS.iter().find(|color| **color == name) {
            Some(color) => Some(*color),
            None => {
                return CommandResult::error(format!(
                    "Unknown color. Choose one of: {}, or off.",
                    COMPOSER_COLORS.join(", ")
                ));
            }
        },
    };
    match control.set_composer_color(next.map(str::to_string)) {
        Ok(()) => CommandResult::message(match next {
            Some(color) => format!("Composer color for this chat: {color}"),
            None => "Composer color for this chat reset to the theme default".to_string(),
        }),
        Err(error) => CommandResult::error(error),
    }
}
