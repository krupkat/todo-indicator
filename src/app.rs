// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::{Limits, Subscription};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::widget;
use cosmic::{Application, Element};
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::gitlab::GitLabClient;

/// The main application struct for the GitLab TODO indicator
#[derive(Default)]
pub struct GitLabTodoApp {
    /// Application state managed by COSMIC runtime
    core: Core,
    /// The popup window ID
    popup: Option<Id>,
    /// GitLab client for API calls
    gitlab_client: Option<GitLabClient>,
    /// Current TODO count
    todo_count: u32,
    /// Last error message
    last_error: Option<String>,
    /// Last update time
    last_update: Option<Instant>,
    /// Configuration
    config: Option<Config>,
    /// Whether we're currently fetching data
    is_fetching: bool,
    /// Current label for the button
    current_label: String,
}

/// Messages that the application can handle
#[derive(Debug, Clone)]
pub enum Message {
    /// Toggle the popup window
    TogglePopup,
    /// Popup was closed
    PopupClosed(Id),
    /// Refresh TODO count immediately
    RefreshNow,
    /// Open GitLab TODOs in browser
    OpenGitLab,
    /// TODO count was updated
    TodoCountUpdated(Result<u32, String>),
    /// Periodic tick for auto-refresh
    Tick,
}

impl Application for GitLabTodoApp {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.gitlab.todo-indicator";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = GitLabTodoApp {
            core,
            current_label: "⏳".to_string(), // Initial loading state
            ..Default::default()
        };

        // Try to load configuration
        match Config::load(Config::default_path()) {
            Ok(config) => {
                let gitlab_client = GitLabClient::new(
                    config.gitlab.url.clone(),
                    config.gitlab.access_token.clone(),
                );
                app.gitlab_client = Some(gitlab_client);
                app.config = Some(config);
            }
            Err(e) => {
                app.last_error = Some(format!("Configuration error: {}", e));
            }
        }

        (app, Task::none())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // Set up periodic refresh based on config interval
        let interval = self
            .config
            .as_ref()
            .map(|c| c.app.refresh_interval)
            .unwrap_or(300); // Default 5 minutes

        cosmic::iced::time::every(Duration::from_secs(interval))
            .map(|_| Message::Tick)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(400.0)
                        .min_width(300.0)
                        .min_height(150.0)
                        .max_height(600.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::RefreshNow | Message::Tick => {
                if let Some(client) = &self.gitlab_client {
                    if !self.is_fetching {
                        self.is_fetching = true;
                        let client = client.clone();
                        return Task::perform(
                            async move {
                                match client.fetch_todo_count().await {
                                    Ok(count) => Ok(count),
                                    Err(e) => Err(e.to_string()),
                                }
                            },
                            |result| cosmic::Action::App(Message::TodoCountUpdated(result)),
                        );
                    }
                }
            }
            Message::TodoCountUpdated(result) => {
                self.is_fetching = false;
                match result {
                    Ok(count) => {
                        self.todo_count = count;
                        self.last_error = None;
                        self.last_update = Some(Instant::now());
                        // Update the label
                        self.current_label = if count == 0 {
                            "✓".to_string()
                        } else {
                            count.to_string()
                        };
                    }
                    Err(error) => {
                        self.last_error = Some(error);
                        self.current_label = "❌".to_string();
                    }
                }
            }
            Message::OpenGitLab => {
                if let Some(client) = &self.gitlab_client {
                    let url = client.get_todos_url();
                    let _ = std::process::Command::new("xdg-open")
                        .arg(&url)
                        .spawn();
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Main applet button - shows the TODO count or error indicator
        self.core
            .applet
            .icon_button(&self.current_label)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let mut content_list = widget::list_column().padding(5).spacing(10);

        // Status section
        let status_text = if let Some(ref error) = self.last_error {
            format!("❌ Error: {}", error)
        } else if self.is_fetching {
            "🔄 Fetching...".to_string()
        } else {
            format!("📝 TODOs: {}", self.todo_count)
        };

        content_list = content_list.add(
            widget::text(status_text)
                .size(16)
        );

        // Last update info
        if let Some(last_update) = self.last_update {
            let elapsed = last_update.elapsed();
            let time_str = if elapsed.as_secs() < 60 {
                format!("Updated {} seconds ago", elapsed.as_secs())
            } else {
                format!("Updated {} minutes ago", elapsed.as_secs() / 60)
            };
            content_list = content_list.add(
                widget::text(time_str)
                    .size(12)
            );
        }

        // Action buttons
        let refresh_button = widget::button::standard("🔄 Refresh Now")
            .on_press(Message::RefreshNow);

        content_list = content_list.add(refresh_button);

        if self.gitlab_client.is_some() && self.last_error.is_none() {
            let gitlab_button = widget::button::standard("🌐 Open GitLab TODOs")
                .on_press(Message::OpenGitLab);
            content_list = content_list.add(gitlab_button);
        }

        // Configuration info
        if let Some(ref config) = self.config {
            content_list = content_list.add(
                widget::text(format!("GitLab: {}", config.gitlab.url))
                    .size(10)
            );
            content_list = content_list.add(
                widget::text(format!("Refresh: {}s", config.app.refresh_interval))
                    .size(10)
            );
        }

        self.core.applet.popup_container(content_list).into()
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}