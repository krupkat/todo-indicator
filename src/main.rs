// SPDX-License-Identifier: GPL-3.0-only

use app::GitLabTodoApp;

mod app;
mod config;
mod gitlab;
mod localization;

/// Main entry point for the GitLab TODO indicator applet
fn main() -> cosmic::iced::Result {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    // Run the COSMIC applet
    cosmic::applet::run::<GitLabTodoApp>(())
}