use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Gauge, List, ListItem, Paragraph, Row, Table, Tabs, Wrap,
    },
    Frame,
};

use super::app::{App, Tab, SortBy, ScoredDevice, DeviceSource};

/// Main draw function
pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header/tabs
            Constraint::Min(0),      // Main content
            Constraint::Length(3),  // Footer/status
        ])
        .split(f.area());
    
    draw_header(f, app, chunks[0]);
    
    match app.tab {
        Tab::Dashboard => draw_dashboard(f, app, chunks[1]),
        Tab::WiFi => draw_wifi_tab(f, app, chunks[1]),
        Tab::Bluetooth => draw_ble_tab(f, app, chunks[1]),
        Tab::Alerts => draw_alerts_tab(f, app, chunks[1]),
        Tab::Threats => draw_threats_tab(f, app, chunks[1]),
        Tab::Help => draw_help_tab(f, chunks[1]),
    }
    
    draw_footer(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec!["[1]Dashboard", "[2]WiFi", "[3]BLE", "[4]Alerts", "[5]Threats", "[?]Help"];
    let selected = match app.tab {
        Tab::Dashboard => 0,
        Tab::WiFi => 1,
        Tab::Bluetooth => 2,
        Tab::Alerts => 3,
        Tab::Threats => 4,
        Tab::Help => 5,
    };
    
    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" SIGINT-Deck "))
        .select(selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD));
    
    f.render_widget(tabs, area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(area);
    
    // Left: Sort info
    let sort_info = Paragraph::new(format!(
        " Sort: {} {} | [s]ort [S]order",
        app.sort_by.label(),
        if app.sort_ascending { "↑" } else { "↓" }
    ))
    .style(Style::default().fg(Color::DarkGray))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(sort_info, chunks[0]);
    
    // Middle: Hardware status
    let hw_status = format!(
        " WiFi:{} BLE:{} GPS:{}",
        if app.wifi_enabled { "✓" } else { "✗" },
        if app.ble_enabled { "✓" } else { "✗" },
        if app.gps_enabled { "✓" } else { "✗" },
    );
    let status = Paragraph::new(hw_status)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
    
    // Right: Battery and quit hint
    let battery_str = app.battery_percent
        .map(|b| format!("🔋{}%", b))
        .unwrap_or_else(|| "🔌".to_string());
    let right_info = Paragraph::new(format!(" {} | [q]uit", battery_str))
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(right_info, chunks[2]);
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Stats row
            Constraint::Min(0),       // Threat list
        ])
        .split(area);
    
    // Stats row
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(chunks[0]);
    
    draw_stat_box(f, "WiFi Devices", app.total_wifi, Color::Cyan, stats_chunks[0]);
    draw_stat_box(f, "BLE Devices", app.total_ble, Color::Blue, stats_chunks[1]);
    draw_stat_box(f, "Trackers", app.trackers_detected, Color::Red, stats_chunks[2]);
    draw_stat_box(f, "New", app.new_devices, Color::Yellow, stats_chunks[3]);
    draw_stat_box(f, "Threats", app.threats_detected, Color::Magenta, stats_chunks[4]);
    
    // Threat summary
    let threats = app.get_threat_devices();
    let threat_items: Vec<ListItem> = threats.iter().take(15).map(|d| {
        let style = if d.is_tracker {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else if d.threat_score > 0.5 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        
        let source = match d.source {
            DeviceSource::WiFi => "📶",
            DeviceSource::Bluetooth => "🔵",
        };
        
        let name = d.name.as_deref().unwrap_or(&d.mac);
        let reason = d.threat_reason.as_deref().unwrap_or("");
        
        ListItem::new(format!(
            "{} {} ({} dBm) - {}",
            source, name, d.rssi, reason
        )).style(style)
    }).collect();
    
    let threat_list = List::new(threat_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" ⚠ Concerning Devices "));
    f.render_widget(threat_list, chunks[1]);
}

fn draw_stat_box(f: &mut Frame, title: &str, value: usize, color: Color, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title))
        .style(Style::default().fg(color));
    
    let text = Paragraph::new(format!("{}", value))
        .style(Style::default()
            .fg(color)
            .add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);
    
    f.render_widget(text, area);
}

fn draw_wifi_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let devices: Vec<ScoredDevice> = app.get_sorted_devices()
        .into_iter()
        .filter(|d| matches!(d.source, DeviceSource::WiFi))
        .collect();
    
    draw_device_table(f, app, area, &devices, "WiFi Devices", &["MAC", "SSID/Name", "Ch", "RSSI", "Type", "Vendor"]);
}

fn draw_ble_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let devices: Vec<ScoredDevice> = app.get_sorted_devices()
        .into_iter()
        .filter(|d| matches!(d.source, DeviceSource::Bluetooth))
        .collect();
    
    draw_device_table(f, app, area, &devices, "Bluetooth Devices", &["MAC", "Name", "Type", "RSSI", "Status", "Vendor"]);
}

fn draw_device_table(
    f: &mut Frame,
    app: &mut App,
    area: Rect,
    devices: &[ScoredDevice],
    title: &str,
    headers: &[&str],
) {
    // Clamp selection
    if !devices.is_empty() {
        app.selected_index = app.selected_index.min(devices.len() - 1);
    } else {
        app.selected_index = 0;
    }
    
    let header_cells = headers.iter().map(|h| {
        Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    });
    let header = Row::new(header_cells).height(1);
    
    let rows: Vec<Row> = devices.iter().enumerate().map(|(i, device)| {
        let style = if device.is_tracker {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else if device.is_new {
            Style::default().fg(Color::Green)
        } else if device.threat_score > 0.5 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        
        let selected_style = if i == app.selected_index {
            style.add_modifier(Modifier::REVERSED)
        } else {
            style
        };
        
        let ch_str = device.channel.map(|c| c.to_string()).unwrap_or_else(|| "-".to_string());
        let status = if device.is_tracker { "⚠TRACKER" } 
            else if device.is_new { "NEW" } 
            else { "" };
        
        let cells = vec![
            Cell::from(device.mac.clone()),
            Cell::from(device.name.clone().unwrap_or_else(|| "-".to_string())),
            Cell::from(ch_str),
            Cell::from(format!("{}", device.rssi)),
            Cell::from(device.device_type.clone()),
            Cell::from(device.vendor.clone().unwrap_or_else(|| "-".to_string())),
        ];
        
        Row::new(cells).style(selected_style)
    }).collect();
    
    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(25),
        Constraint::Percentage(5),
        Constraint::Percentage(8),
        Constraint::Percentage(15),
        Constraint::Percentage(27),
    ];
    
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ({}) ", title, devices.len())));
    
    f.render_widget(table, area);
}

fn draw_alerts_tab(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.alerts.iter().take(50).map(|alert| {
        let style = match alert.priority.as_str() {
            "Critical" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            "High" => Style::default().fg(Color::Yellow),
            "Medium" => Style::default().fg(Color::Cyan),
            _ => Style::default().fg(Color::White),
        };
        
        let time = chrono::DateTime::from_timestamp(alert.timestamp, 0)
            .map(|t| t.format("%H:%M:%S").to_string())
            .unwrap_or_else(|| "??:??:??".to_string());
        
        ListItem::new(format!(
            "[{}] [{}] {}",
            time, alert.priority, alert.message
        )).style(style)
    }).collect();
    
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" Alerts ({}) ", app.alerts.len())));
    
    f.render_widget(list, area);
}

fn draw_threats_tab(f: &mut Frame, app: &App, area: Rect) {
    let threats = app.get_threat_devices();
    
    let items: Vec<ListItem> = threats.iter().map(|d| {
        let style = if d.is_tracker {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else if d.threat_score > 0.7 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else if d.threat_score > 0.5 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        
        let source = match d.source {
            DeviceSource::WiFi => "WiFi",
            DeviceSource::Bluetooth => "BLE ",
        };
        
        let name = d.name.as_deref().unwrap_or(&d.mac);
        let reason = d.threat_reason.as_deref().unwrap_or("Unknown");
        let tracker = if d.is_tracker { " [TRACKER]" } else { "" };
        
        ListItem::new(format!(
            "[{:.0}%] {} {} ({} dBm){} - {}",
            d.threat_score * 100.0,
            source,
            name,
            d.rssi,
            tracker,
            reason
        )).style(style)
    }).collect();
    
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(format!(" ⚠ Threat Assessment ({}) ", threats.len())));
    
    f.render_widget(list, area);
}

fn draw_help_tab(f: &mut Frame, area: Rect) {
    let help_text = vec![
        "",
        "  NAVIGATION",
        "  ─────────────────────────────",
        "  Tab / Shift+Tab    Next/prev tab",
        "  1-5                Jump to tab",
        "  ?/F1               Help",
        "",
        "  LIST NAVIGATION",
        "  ─────────────────────────────",
        "  ↑/k, ↓/j           Move up/down",
        "  PgUp, PgDn         Page up/down",
        "  Home/g, End/G      Top/bottom",
        "",
        "  SORTING",
        "  ─────────────────────────────",
        "  s                  Cycle sort mode",
        "  S                  Toggle asc/desc",
        "",
        "  Sort modes: Signal > Channel > Type > Threat > Recent > Name",
        "",
        "  OTHER",
        "  ─────────────────────────────",
        "  r/F5               Refresh",
        "  q/Esc              Quit",
        "",
        "  THREAT INDICATORS",
        "  ─────────────────────────────",
        "  Red text           Tracker detected",
        "  Yellow text        Elevated threat",
        "  Green text         New device",
        "",
    ];
    
    let paragraph = Paragraph::new(help_text.join("\n"))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Help "))
        .wrap(Wrap { trim: false });
    
    f.render_widget(paragraph, area);
}
