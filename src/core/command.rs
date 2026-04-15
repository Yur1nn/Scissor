//! Command system for Scissor (minimal, for toggling build mode)
use crate::core::permissions::Permissions;

pub struct CommandContext<'a> {
    pub player: &'a str,
    pub permissions: &'a mut Permissions,
}

pub fn handle_command(ctx: &mut CommandContext, input: &str) {
    let args: Vec<&str> = input.trim().split_whitespace().collect();
    if args.is_empty() {
        println!("No command entered.");
        return;
    }
    match args[0] {
        "/buildmode" => {
            if ctx.permissions.is_build_mode(ctx.player) {
                ctx.permissions.disable_build_mode(ctx.player);
                println!("Build mode disabled for {}", ctx.player);
            } else {
                ctx.permissions.enable_build_mode(ctx.player);
                println!("Build mode enabled for {}", ctx.player);
            }
        }
        _ => println!("Unknown command: {}", args[0]),
    }
}
