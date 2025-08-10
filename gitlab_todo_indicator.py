#!/usr/bin/env python3

"""
GitLab TODO Indicator
A system tray application that displays the number of GitLab TODO items.
"""

import sys
import time
import threading
import requests
import subprocess
import yaml
import textwrap

from pathlib import Path
from typing import Optional, Dict, Any

import gi
gi.require_version('Gtk', '3.0')
gi.require_version('AppIndicator3', '0.1')
from gi.repository import Gtk, GLib, AppIndicator3  # nopep8


class GitLabTodoIndicator:
    def __init__(self, config_path: str = "config.yaml"):
        # Load configuration from YAML file
        self.config = self.load_config(config_path)
        
        # Extract configuration values
        self.gitlab_url = self.config['gitlab']['url']
        self.access_token = self.config['gitlab']['access_token']
        self.refresh_interval = self.config['app']['refresh_interval']

        if not self.access_token or self.access_token == "your-token-here":
            print("Error: Please set a valid GitLab access token in config.yaml")
            sys.exit(1)

        # Initialize the indicator
        self.indicator = AppIndicator3.Indicator.new(
            "gitlab-todo-indicator",
            "dialog-information",
            AppIndicator3.IndicatorCategory.APPLICATION_STATUS
        )
        self.indicator.set_status(AppIndicator3.IndicatorStatus.ACTIVE)

        # Create menu
        self.menu = Gtk.Menu()
        self.create_menu()
        self.indicator.set_menu(self.menu)

        # Initialize TODO count
        self.todo_count = 0
        self.last_error = None
        self.update_indicator_label()

        # Start background thread for fetching TODOs
        self.stop_thread = False
        self.refresh_condition = threading.Condition()
        self.fetch_thread = threading.Thread(
            target=self.fetch_todos_loop, daemon=True)
        self.fetch_thread.start()

    def load_config(self, config_path: str) -> Dict[str, Any]:
        """Load configuration from YAML file"""
        config_file = Path(config_path)
        
        if not config_file.exists():
            print(f"Error: Configuration file '{config_path}' not found")
            print("Please copy config.yaml.example to config.yaml and customize it")
            sys.exit(1)
        
        try:
            with open(config_file, 'r') as f:
                config = yaml.safe_load(f)
            
            # Validate required fields
            required_fields = {
                'gitlab': ['url', 'access_token'],
                'app': ['refresh_interval']
            }
            
            for section, fields in required_fields.items():
                if section not in config:
                    raise ValueError(f"Missing required section '{section}' in config")
                for field in fields:
                    if field not in config[section]:
                        raise ValueError(f"Missing required field '{section}.{field}' in config")
            
            return config
            
        except yaml.YAMLError as e:
            print(f"Error parsing YAML config file: {e}")
            sys.exit(1)
        except (KeyError, ValueError) as e:
            print(f"Configuration error: {e}")
            sys.exit(1)

    def create_menu(self):
        """Create the context menu for the indicator"""
        # Refresh item
        refresh_item = Gtk.MenuItem(label="Refresh Now")
        refresh_item.connect("activate", self.on_refresh_clicked)
        self.menu.append(refresh_item)

        # Separator
        separator = Gtk.SeparatorMenuItem()
        self.menu.append(separator)

        # Open GitLab TODOs
        open_gitlab_item = Gtk.MenuItem(label="Open GitLab TODOs")
        open_gitlab_item.connect("activate", self.on_open_gitlab_clicked)
        self.menu.append(open_gitlab_item)

        # Separator
        separator2 = Gtk.SeparatorMenuItem()
        self.menu.append(separator2)

        # Status item
        self.status_item = Gtk.MenuItem(label="Status: Initializing...")
        self.status_item.set_sensitive(False)
        self.menu.append(self.status_item)

        # Separator
        separator3 = Gtk.SeparatorMenuItem()
        self.menu.append(separator3)

        # Quit item
        quit_item = Gtk.MenuItem(label="Quit")
        quit_item.connect("activate", self.on_quit_clicked)
        self.menu.append(quit_item)

        self.menu.show_all()

    def fetch_todos(self) -> Optional[int]:
        """Fetch TODO count from GitLab API"""
        try:
            headers = {
                'Authorization': f'Bearer {self.access_token}',
                'Content-Type': 'application/json'
            }

            # GitLab API endpoint for TODOs
            url = f"{self.gitlab_url}/api/v4/todos"
            params = {
                'state': 'pending',  # Only fetch pending TODOs
                'per_page': 1  # We only need the count, not the actual data
            }

            response = requests.get(
                url, headers=headers, params=params, timeout=10)
            response.raise_for_status()

            # Get total count from headers
            total_count = response.headers.get('X-Total', '0')
            return int(total_count)

        except requests.exceptions.RequestException as e:
            print(f"Error fetching TODOs: {e}")
            self.last_error = str(e)
            return None
        except ValueError as e:
            print(f"Error parsing TODO count: {e}")
            self.last_error = f"Parse error: {e}"
            return None

    def fetch_todos_loop(self):
        """Background thread loop for fetching TODOs using condition variable"""
        while not self.stop_thread:
            todo_count = self.fetch_todos()

            if todo_count is not None:
                self.todo_count = todo_count
                self.last_error = None

            # Update UI in main thread
            GLib.idle_add(self.update_indicator_label)

            # Wait for next refresh using condition variable
            with self.refresh_condition:
                if not self.stop_thread:
                    self.refresh_condition.wait(timeout=self.refresh_interval)

    def update_indicator_label(self):
        """Update the indicator label with current TODO count"""
        if self.last_error:
            self.indicator.set_label("❌", "Error")
            formatted_error = textwrap.fill(self.last_error, width=80)
            self.status_item.set_label(f"Error: {formatted_error}")
        else:
            # Display TODO count
            if self.todo_count == 0:
                self.indicator.set_label("✓", "No TODOs")
            else:
                self.indicator.set_label(
                    str(self.todo_count), f"{self.todo_count} TODOs")

            self.status_item.set_label(
                f"TODOs: {self.todo_count} (Updated: {time.strftime('%H:%M:%S')})")

    def on_refresh_clicked(self, widget):
        """Handle refresh button click"""
        # Trigger immediate refresh by notifying the condition variable
        with self.refresh_condition:
            self.refresh_condition.notify()

    def on_open_gitlab_clicked(self, widget):
        """Handle open GitLab button click"""
        try:
            subprocess.run(
                ['xdg-open', f"{self.gitlab_url}/dashboard/todos"], check=True)
        except subprocess.CalledProcessError as e:
            print(f"Error opening GitLab: {e}")

    def on_quit_clicked(self, widget):
        """Handle quit button click"""
        self.stop_thread = True
        with self.refresh_condition:
            self.refresh_condition.notify()
        Gtk.main_quit()


def main():
    """Main entry point"""
    import argparse
    
    parser = argparse.ArgumentParser(description='GitLab TODO Indicator')
    parser.add_argument('--config', default='config.yaml',
                        help='Path to configuration file (default: config.yaml)')
    args = parser.parse_args()

    try:
        indicator = GitLabTodoIndicator(args.config)
        print(f"GitLab TODO Indicator started")
        print(f"GitLab URL: {indicator.gitlab_url}")
        print(f"Refresh interval: {indicator.refresh_interval} seconds")
        print("Right-click the indicator for options")

        # Run the GTK main loop
        Gtk.main()

    except KeyboardInterrupt:
        print("\nShutting down...")
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
